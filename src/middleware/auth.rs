use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::utils::jwt_util::JwtToken;
use crate::AppState;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{http, response, Json};
use redis::{Client, Commands};
use std::sync::Arc;

pub async fn auth(State(state): State<Arc<AppState>>, mut req: Request, next: Next) -> Result<response::Response, StatusCode> {
    log::info!("req {:?}", req.uri());
    let path = req.uri().to_string();
    if path.eq("/system/user/login") {
        return Ok(next.run(req).await);
    }
    let auth_header = req.headers().get(http::header::AUTHORIZATION).and_then(|header| header.to_str().ok());

    if auth_header.is_none() {
        let json = Json(BaseResponse {
            msg: "请求头缺少 Authorization 字段".to_string(),
            code: 401,
            data: Some("None".to_string()),
        });
        return Ok((StatusCode::OK, json).into_response());
    }
    let authorization = auth_header.unwrap();

    let token = authorization.to_string().replace("Bearer ", "");
    let jwt_token_e = JwtToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => data,
        Err(err) => {
            let er = match err {
                AppError::JwtTokenError(s) => s,
                _ => "no math error".to_string(),
            };
            let json = Json(BaseResponse {
                msg: er,
                code: 401,
                data: Some("None".to_string()),
            });
            return Ok((StatusCode::OK, json).into_response());
        }
    };

    match validate_and_get_user_info(&state.redis, jwt_token.id).await {
        Ok((user_id, permissions, token_1, is_admin)) => {
            if token_1 != token {
                let json = Json(BaseResponse {
                    msg: "无效的token".to_string(),
                    code: 401,
                    data: Some("None".to_string()),
                });
                return Ok((StatusCode::OK, json).into_response());
            }
            if is_admin || has_permission(&permissions, &path) {
                req.headers_mut().insert("user_id", user_id.to_string().parse().unwrap());

                Ok(next.run(req).await)
            } else {
                let json = Json(BaseResponse {
                    msg: format!("用户还没有授权url:{}", path),
                    code: 401,
                    data: Some("None".to_string()),
                });
                Ok((StatusCode::OK, json).into_response())
            }
        }
        Err(e) => {
            let json = Json(BaseResponse {
                msg: e.to_string(),
                code: 401,
                data: Some("None".to_string()),
            });
            Ok((StatusCode::OK, json).into_response())
        }
    }
}

async fn validate_and_get_user_info(redis_client: &Client, user_id: i64) -> Result<(i64, Vec<String>, String, bool), String> {
    let mut conn = redis_client.get_connection().map_err(|_| "Redis连接失败".to_string())?;

    let key = format!("axum:admin:user:info:{}", user_id);
    let permissions_str: String = conn.hget(&key, "permissions").unwrap_or_else(|_| "".to_string());
    let token: String = conn.hget(&key, "token").map_err(|_| "无效的token".to_string())?;
    let is_admin: bool = conn.hget(&key, "isAdmin").unwrap_or_default();
    let permissions: Vec<String> = if permissions_str.is_empty() {
        Vec::new()
    } else {
        permissions_str.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    };
    Ok((user_id, permissions, token, is_admin))
}

fn has_permission(permissions: &[String], path: &str) -> bool {
    permissions.iter().any(|permission| {
        // 确保权限路径以 /api 开头
        if permission.starts_with("/api") {
            // 移除 /api 前缀进行比较
            let permission_path = &permission[4..]; // 跳过 "/api" 的4个字符
            permission_path == path
        } else {
            // 如果权限路径不以 /api 开头，直接比较
            permission == path
        }
    })
}
