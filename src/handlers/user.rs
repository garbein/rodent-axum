use axum::extract::Path;
use axum::{extract::Extension, Json};
use validator::Validate;

use crate::dto::user::{UserReq, UserResp};
use crate::dto::{IdResp, JsonResult, Resp};
use crate::server::AppState;
use crate::services::user::UserService;

pub async fn create(
    Json(req): Json<UserReq>,
    Extension(app_state): Extension<AppState>,
) -> JsonResult<IdResp> {
    req.validate()?;
    let id = UserService::create(&app_state, &req).await?;
    Ok(Json(Resp::from_data(IdResp::from_id(id))))
}

pub async fn info(
    Path(id): Path<u64>,
    Extension(app_state): Extension<AppState>,
) -> JsonResult<Option<UserResp>> {
    let user = UserService::info(&app_state, id).await?;
    Ok(Json(Resp::from_data(user)))
}
