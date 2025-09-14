use axum::{BoxError};
use axum::response::IntoResponse;
use crate::common::error::AppError;

pub async fn handle_middleware_error(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        // return (StatusCode::REQUEST_TIMEOUT,"Request took too long".to_string());
        AppError::InternalError("请求超时").into_response()
    }  else {
        // return (StatusCode::INTERNAL_SERVER_ERROR, "Unhandled internal：{err}".to_string());
        AppError::default().into_response()
    }
}