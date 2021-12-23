use chrono::prelude::*;
use serde::Serialize;

use crate::dto::user::UserReq;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub status: i8,
    pub create_time: DateTime<Local>,
    pub update_time: Option<DateTime<Local>>,
}

impl User {
    pub const TABLE_NAME: &'static str = "user";
}

pub struct CreateUserData {
    pub username: String,
    pub status: Option<i8>,
}

impl From<&UserReq> for CreateUserData {
    fn from(v: &UserReq) -> Self {
        Self {
            username: v.username.clone(),
            status: v.status,
        }
    }
}
