use crate::common::error::AppError;
use crate::common::result::BaseResponse;
use crate::utils::jwt_util::JwtToken;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{http, response, Json};

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
        return Ok(next.run(req).await);
    }

    let json = Json(BaseResponse {
        msg: format!("用户还没有授权url:{}", path),
        code: 401,
        data: Some("None".to_string()),
    });
    Ok((StatusCode::OK, json).into_response())
}
