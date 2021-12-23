use crate::dto::user::UserResp;
use crate::dto::Result;
use crate::models::user::User;
use crate::{db::Pool, dto::user::UserReq};

pub struct UserService;

impl UserService {
    pub async fn create(pool: &Pool, req: &UserReq) -> Result<u64> {
        let data = req.into();
        User::create(pool, &data).await
    }

    pub async fn info(pool: &Pool, id: u64) -> Result<Option<UserResp>> {
        User::get_by_id(pool, id).await.map(|x| x.map(|x| x.into()))
    }
}
