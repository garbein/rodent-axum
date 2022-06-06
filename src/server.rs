use crate::{
    db::{self, Pool},
    routes,
    settings::{CliConfig, Setting}, redis::{self, RedisPool},
};
use axum::extract::Extension;
use std::{env, net::SocketAddr};
use structopt::StructOpt;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub db_master: Pool,
    pub redis_pool: RedisPool,
}

impl AppState {
    async fn new(setting: Setting) -> Self {
        Self {
            db_master: db::new_pool(&setting.db).await,
            redis_pool: redis::new_pool(&setting.redis),
        }
    }
}

pub async fn run() {
    let cli_config = CliConfig::from_args();
    if cli_config.current_dir.is_some() {
        let current_dir = cli_config.current_dir.unwrap_or_default();
        env::set_current_dir(&current_dir).expect("fail to set current dir");
    }
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let setting = Setting::new().expect("fail to load settings");
    tracing::info!("setting {:#?}", setting);
    let addr: SocketAddr = format!("{}:{}", setting.server.host, setting.server.port)
        .parse()
        .expect("fail to parse addr");
    tracing::info!("listening on {}", addr);

    let layer = ServiceBuilder::new()
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive())
    .layer(Extension(AppState::new(setting).await))
    .into_inner();
    let app = routes::new().layer(layer);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        //.with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}

// #[cfg(unix)]
// pub async fn graceful_shutdown() {
//     use tokio::signal::unix::SignalKind;
//     async fn terminate() -> std::io::Result<()> {
//         tokio::signal::unix::signal(SignalKind::terminate())?
//             .recv()
//             .await;
//         Ok(())
//     }

//     tokio::select! {
//         _ = terminate() => {},
//         _ = tokio::signal::ctrl_c() => {},
//     }
//     tracing::info!("signal received, starting graceful shutdown");
// }

// #[cfg(windows)]
// pub async fn graceful_shutdown() {
//     tokio::signal::ctrl_c()
//         .await
//         .expect("faild to install CTRL+C handler");
//     tracing::info!("signal received, starting graceful shutdown");
// }
