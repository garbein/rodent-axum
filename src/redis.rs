use deadpool_redis::{Config, Pool};

use crate::settings::RedisSetting;

pub type RedisPool = Pool;

pub fn new_pool(setting: &RedisSetting) -> RedisPool {
    Config::from_url(format!("redis://{}:{}", setting.host, setting.port))
        .create_pool(None)
        .expect("fail to create redis pool")
}