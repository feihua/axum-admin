use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::utils::error::WhoUnfollowedError;
use crate::utils::error::WhoUnfollowedError::{JwtTokenError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTToken {
    pub id: i32,
    pub username: String,
    pub permissions: Vec<String>,
    aud: String,
    // (audience)：受众
    exp: usize,
    iat: usize,
    // (Issued At)：签发时间
    iss: String,
    // (issuer)：签发人
    nbf: usize,
    // (Not Before)：生效时间
    sub: String,
    // (subject)：主题
    jti: String, // (JWT ID)：编号
}

impl JWTToken {
    pub fn new(id: i32, username: &str, permissions: Vec<String>) -> JWTToken {
        let now = SystemTime::now();
        //过期时间
        let m30 = Duration::from_secs(1800000);
        let now = now.duration_since(UNIX_EPOCH).expect("获取系统时间失败");

        JWTToken {
            id,
            username: String::from(username),
            permissions,
            aud: String::from("rust_admin"), // (audience)：受众
            exp: (now + m30).as_secs() as usize,
            iat: now.as_secs() as usize,  // (Issued At)：签发时间
            iss: String::from("koobe"),     // (issuer)：签发人
            nbf: now.as_secs() as usize,  // (Not Before)：生效时间
            sub: String::from("rust_admin"), // (subject)：主题
            jti: String::from("ignore"),  // (JWT ID)：编号
        }
    }

    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, WhoUnfollowedError> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(JwtTokenError("create token error".to_string())),
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, WhoUnfollowedError> {
        let mut validation = Validation::default();
        validation.validate_nbf = true;
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),

            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(JwtTokenError("InvalidToken".to_string())), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(JwtTokenError("InvalidIssuer".to_string())), // Example on how to handle a specific error
                ErrorKind::ExpiredSignature => return Err(JwtTokenError("token 已经超时了".to_string())), // Example on how to handle a specific error
                // _ => return Err(Error::from("InvalidToken other errors")),
                _ => Err(JwtTokenError("create token error".to_string())),
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::jwt_util::JWTToken;

    #[test]
    fn test_jwt() {
        let jwt = JWTToken::new("1", "koobe", vec![]);
        let res = jwt.create_token("123")?;
        println!("{:?}",res);
        let token = JWTToken::verify("123", &res);
        println!("{:?}",token)

    }
}