use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlSslMode},
    ConnectOptions,
};
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "server")]
pub struct CliConfig {
    #[structopt(name = "current_dir", short, long = "--current_dir")]
    pub current_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DbSetting {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: Option<u32>,
}

impl DbSetting {
    pub fn as_connect_options(&self) -> MySqlConnectOptions {
        let mut options = MySqlConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .database(&self.database)
            .ssl_mode(MySqlSslMode::Disabled);
        options.log_statements(log::LevelFilter::Trace);
        options
    }
}

#[derive(Debug, Deserialize)]
pub struct ServerSetting {
    pub host: String,
    pub port: u16,
    pub app_env: Option<AppEnv>,
}

#[derive(Debug, Deserialize)]
pub enum AppEnv {
    Dev,
    Test,
    Prod,
}

impl AppEnv {
    pub fn as_str(&self) -> &str {
        match self {
            AppEnv::Dev => "dev",
            AppEnv::Test => "test",
            AppEnv::Prod => "prod",
        }
    }
}

impl TryFrom<String> for AppEnv {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "test" => Ok(Self::Test),
            "prod" => Ok(Self::Prod),
            other => Err(format!("{}  is not a supported environment.", other)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RedisSetting {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Setting {
    pub debug: bool,
    pub server: ServerSetting,
    pub db: DbSetting,
    pub redis: RedisSetting,
}

impl Setting {
    pub fn new() -> Result<Self, ConfigError> {
        let app_env: AppEnv = env::var("APP_ENV")
            .unwrap_or_else(|_| "dev".to_string())
            .try_into()
            .expect("failed to parse app environment ");
        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", app_env.as_str())).required(true))
            .add_source(Environment::with_prefix("app").separator("_"))
            .build()?;
        let mut setting: Self = config.try_deserialize()?;
        setting.server.app_env = app_env.into();
        Ok(setting)
    }
}
