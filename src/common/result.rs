use crate::common::error::AppResult;
use axum::Json;
use rbatis::rbdc::DateTime;
use serde::Serialize;
use std::fmt::Debug;
use utoipa::{ToSchema};

// 统一返回vo
#[derive(Serialize, Debug, Clone,ToSchema)]
pub struct BaseResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

#[derive(Serialize, Debug, Clone,ToSchema)]
pub struct ResponsePage<T> {
    pub code: i32,
    pub msg: String,
    pub total: u64,
    pub success: bool,
    pub data: Option<T>,
}

pub fn ok() -> AppResult<Json<BaseResponse<()>>> {
    ok_result_data(())
}

pub fn ok_result() -> AppResult<Json<BaseResponse<String>>> {
    ok_result_msg("操作成功")
}

pub fn ok_result_msg(msg: &str) -> AppResult<Json<BaseResponse<String>>> {
    Ok(Json(BaseResponse {
        msg: msg.to_string(),
        code: 0,
        data: Some("None".to_string()),
    }))
}

pub fn ok_result_data<T>(data: T) -> AppResult<Json<BaseResponse<T>>> {
    Ok(Json(BaseResponse {
        msg: "操作成功".to_string(),
        code: 0,
        data: Some(data),
    }))
}

pub fn ok_result_page<T>(data: T, total: u64) -> AppResult<Json<ResponsePage<T>>> {
    Ok(Json(ResponsePage {
        msg: "操作成功".to_string(),
        code: 0,
        success: true,
        data: Some(data),
        total,
    }))
}

pub fn err_result_msg(msg: &str) -> AppResult<Json<BaseResponse<String>>> {
    Ok(Json(BaseResponse {
        msg: msg.to_string(),
        code: 1,
        data: Some("None".to_string()),
    }))
}

pub fn serialize_datetime<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match dt {
        Some(datetime) => {
            let formatted = datetime.format("YYYY-MM-DD hh:mm:ss");
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_str(""),
    }
}

