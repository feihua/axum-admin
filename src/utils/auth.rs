use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::{http, response};
use crate::utils::error::WhoUnfollowedError;
use crate::utils::jwt_util::JWTToken;

pub async fn auth<B>(req: Request<B>, next: Next<B>) -> Result<response::Response, StatusCode> {
    log::info!("req {:?}",req.uri());
    let path = req.uri().to_string();
    if path.eq("/login") {
        return Ok(next.run(req).await);
    }
    let auth_header = req.headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let authorization = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let token = authorization.to_string().replace("Bearer ", "");
    log::info!("token:{}",token);
    let jwt_token_e = JWTToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => { data }
        Err(err) => {
            log::info!("{}",err);
            let er = match err {
                WhoUnfollowedError::JwtTokenError(s) => { s }
                _ => "no math error".to_string()
            };
            log::error!("Hi from start. You requested path: {}, token: {}", path, token);
            log::info!("{}",er);
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
    if flag {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }

}