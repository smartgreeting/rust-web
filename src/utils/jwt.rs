/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:34:05
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 20:42:43
 * @Email: 17719495105@163.com
 */
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{self, TimeDelta, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
    pub token_type: String,
}
#[derive(Clone)] 
pub struct JwtUtil {
     secret: String,
}

impl JwtUtil {
    pub fn new(secret: &str) -> Self {
        Self { secret: secret.to_owned() }
    }

    pub fn generate_access_token(&self, user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(user_id, role, "access", 3600)
    }

    pub fn generate_refresh_token(&self, user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
        self.generate_token(user_id, role, "refresh", 86400)
    }

    fn generate_token(
        &self,
        sub: &str,
        role: &str,
        token_type: &str,
        exp_seconds: i64
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let exp = Utc::now()
            .checked_add_signed(TimeDelta::try_seconds(exp_seconds).unwrap())
            .expect("Invalid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: sub.to_owned(),
            exp,
            role: role.to_owned(),
            token_type: token_type.to_owned(),
        };

        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
    }
}