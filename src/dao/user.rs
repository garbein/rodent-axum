#[allow(unused_imports)]
use sqlx::Executor;
use crate::{
    db::Pool,
    dto::Result,
    models::user::{CreateUserData, User},
};

impl User {
    pub async fn create(pool: &Pool, data: &CreateUserData) -> Result<u64> {
        let sql = format!(
            "INSERT INTO {} (username, status) VALUES (?,?)",
            Self::TABLE_NAME
        );
        let id = sqlx::query(&sql)
            .bind(&data.username)
            .bind(data.status.unwrap_or_default())
            .execute(pool)
            .await?
            .last_insert_id();
        Ok(id)
    }

    pub async fn get_by_id(pool: &Pool, id: u64) -> Result<Option<Self>> {
        let sql = format!("SELECT * FROM {} WHERE id = ?", Self::TABLE_NAME);
        sqlx::query_as::<_, User>(&sql)
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| e.into())
    }
}
