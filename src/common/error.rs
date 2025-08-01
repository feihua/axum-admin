use crate::common::result::BaseResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
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

    #[error("业务异常: {0}")]
    BusinessError(&'static str),
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
