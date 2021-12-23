use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserReq {
    #[validate(length(min = 1, max = 20))]
    pub username: String,
    #[validate(range(min = 0, max = 1))]
    pub status: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResp {
    id: u64,
    username: String,
    status: i8,
    create_time: i64,
    update_time: Option<i64>,
}

impl From<User> for UserResp {
    fn from(v: User) -> Self {
        Self {
            id: v.id,
            username: v.username.clone(),
            status: v.status,
            create_time: v.create_time.timestamp(),
            update_time: v.update_time.map(|x| x.timestamp()),
        }
    }
}
