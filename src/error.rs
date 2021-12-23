use axum::{
    body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::dto::Resp;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
}

impl Error {
    fn get_status(&self) -> StatusCode {
        match self {
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = self.get_status();
        let resp = Resp::from_error(self);
        Response::builder()
            .status(status)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
            )
            .body(body::boxed(body::Full::from(
                serde_json::to_string(&resp).unwrap_or_default(),
            )))
            .unwrap()
    }
}
