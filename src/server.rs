use crate::{
    db::{self, Pool},
    routes,
    settings::{CliConfig, Setting},
};
use axum::AddExtensionLayer;
use std::{env, net::SocketAddr};
use structopt::StructOpt;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
pub struct AppState {
    pub db_master: Pool,
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
    tracing::info!("setting {:?}", setting);

    let pool = db::new_pool(setting.db).await;
    let app_state = AppState { db_master: pool };
    let layer = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(AddExtensionLayer::new(app_state))
        .into_inner();
    let app = routes::new().layer(layer);

    let addr: SocketAddr = format!("{}:{}", setting.server.host, setting.server.port)
        .parse()
        .expect("fail to parse addr");
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}

#[cfg(unix)]
pub async fn graceful_shutdown() {
    use tokio::signal::unix::SignalKind;
    async fn terminate() -> std::io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    tracing::info!("signal received, starting graceful shutdown");
}

#[cfg(windows)]
pub async fn graceful_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("faild to install CTRL+C handler");
    tracing::info!("signal received, starting graceful shutdown");
}
