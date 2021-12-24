use crate::dto::user::UserReq;
use crate::dto::user::UserResp;
use crate::dto::Result;
use crate::models::user::User;
use crate::server::AppState;
use redis::AsyncCommands;

pub struct UserService;

impl UserService {
    pub async fn create(app_state: &AppState, req: &UserReq) -> Result<u64> {
        let data = req.into();
        User::create(&app_state.db_master, &data).await
    }

    pub async fn info(app_state: &AppState, id: u64) -> Result<Option<UserResp>> {
        let key = format!("rodent:user:{}", id);
        let mut redis_conn = app_state.redis_pool.get().await?;
        let exist = redis_conn.exists(&key).await?;
        if exist {
            let cache = redis_conn.get(&key)
                .await
                .map(|x: String| {
                    serde_json::from_str::<Option<User>>(&x)
                })??;
            if cache.is_some() {
                return Ok(cache.map(|x|x.into()));
            }
        }
        let user = User::get_by_id(&app_state.db_master, id).await?;
        if user.is_some() {
            let json = serde_json::to_string(&user)?;
            redis_conn.set(&key, json).await?;
        }
        Ok(user.map(|x| x.into()))
    }
}
