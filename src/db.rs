use sqlx::mysql::MySqlPoolOptions;
use sqlx::mysql::MySqlPool;

use crate::settings::DbSetting;

pub type Pool = MySqlPool;

const DEFAULT_MAX_CONNECTIONS: u32 = 10;

pub async fn new_pool(setting: &DbSetting) -> Pool {
    let options = setting.as_connect_options();
    MySqlPoolOptions::new()
        .max_connections(
            setting
                .max_connections
                .unwrap_or(DEFAULT_MAX_CONNECTIONS),
        )
        .connect_lazy_with(options)
}
