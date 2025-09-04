use crate::common::result::BaseResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use redis::RedisError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // #[error("Failed to complete an HTTP request")]
    // Http { #[from] source: reqwest::Error },
    //
    #[error("Failed to read the cache file")]
    DiskCacheRead { source: std::io::Error },
    //
    // #[error("Failed to update the cache file")]
    // DiskCacheWrite { source: std::io::Error },
    #[error("jwt：{0}")]
    JwtTokenError(String),

    #[error("数据库错误: {0}")]
    DbError(#[from] rbatis::Error),

    #[error("redis错误: {0}")]
    RedisError(#[from] RedisError),

    #[error("业务异常: {0}")]
    BusinessError(&'static str),

    #[error("验证异常: {0}")]
    ValidationError(String),
}
pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let response = BaseResponse {
            msg: self.to_string(),
            code: 1,
            data: Some("None".to_string()),
        };
        (StatusCode::OK, Json(response)).into_response()
    }
}


impl AppError {
    pub fn build_validation_error_message(e: &validator::ValidationErrors) -> String {
        e.field_errors().iter().map(|(field, errors)| {
            let messages: Vec<String> = errors.iter().map(|error| {
                if let Some(message) = &error.message {
                    message.to_string()
                } else {
                    format!("字段 '{}' 验证失败", field)
                }
            }).collect();
            messages.join(", ")
        }).collect::<Vec<String>>().join("; ")
    }
}
