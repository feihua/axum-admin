use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use base64::Engine;
use base64::engine::general_purpose;

pub async fn swagger_auth(req: axum::extract::Request, next: Next) -> Response {
    // 账号和密码
    let username = "admin";
    let password = "password";

    // 生成期望的 Basic Auth header
    let expected = format!(
        "Basic {}",
        general_purpose::STANDARD.encode(format!("{}:{}", username, password))
    );

    // 检查 Authorization 头
    if let Some(value) = req.headers().get("Authorization") {
        if let Ok(auth_str) = value.to_str() {
            if auth_str == expected {
                // 认证成功，继续执行后续路由
                return next.run(req).await;
            }
        }
    }

    // 认证失败，返回 401 并提示浏览器弹出登录框
    (
        StatusCode::UNAUTHORIZED,
        [("WWW-Authenticate", "Basic realm=\"Swagger UI\"")],
        "Unauthorized",
    )
        .into_response()
}