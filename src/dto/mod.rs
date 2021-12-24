use crate::error::Error;
use axum::Json;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

pub type JsonResult<T> = Result<Json<Resp<T>>>;

const ERROR_CODE: i32 = 1;

const SUCCESS_CODE: i32 = 0;
const SUCCESS_MESSAGE: &str = "success";

#[derive(Debug, Serialize)]
pub struct Resp<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

impl<T> Resp<T> {
    pub fn from_data(data: T) -> Self {
        Self {
            code: SUCCESS_CODE,
            message: SUCCESS_MESSAGE.to_string(),
            data,
        }
    }
}

impl Resp<()> {
    pub fn from_error(e: Error) -> Self {
        Self {
            code: ERROR_CODE,
            message: e.to_string(),
            data: (),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct IdResp {
    pub id: u64,
}

impl IdResp {
    pub fn from_id(id: u64) -> Self {
        IdResp { id }
    }
}

pub mod user;
