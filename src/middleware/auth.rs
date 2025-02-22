use crate::common::error::AppError;
use crate::utils::jwt_util::JwtToken;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::{http, response};

pub async fn auth(mut req: Request, next: Next) -> Result<response::Response, StatusCode> {
    log::info!("req {:?}", req.uri());
    let path = req.uri().to_string();
    if path.eq("/system/user/login") {
        return Ok(next.run(req).await);
    }
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let authorization = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let token = authorization.to_string().replace("Bearer ", "");
    let jwt_token_e = JwtToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => data,
        Err(err) => {
            log::info!("{}", err);
            let er = match err {
                AppError::JwtTokenError(s) => s,
                _ => "no math error".to_string(),
            };
            log::error!(
                "Hi from start. You requested path: {}, token: {}",
                path,
                token
            );
            log::info!("{}", er);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let mut flag: bool = false;
    for token_permission in &jwt_token.permissions {
        if token_permission.to_string().replace("/api", "") == path {
            flag = true;
            break;
        }
    }

    req.headers_mut()
        .insert("user_id", jwt_token.id.to_string().parse().unwrap());
    if flag {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
