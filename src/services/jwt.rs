use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

// JWT Claims 结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // Subject (user ID)
    pub username: String, // 用户名
    pub role: String,     // 用户角色
    pub exp: usize,       // Expiration time (时间戳)
    pub iat: usize,       // Issued at (签发时间)
}

pub struct JwtUtils;

impl JwtUtils {
    // 获取 JWT 密钥
    fn get_secret() -> String {
        env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string())
    }

    // 生成 JWT token
    pub fn generate_token(
        user_id: i64,
        username: &str,
        role: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = chrono::Utc::now();
        let expiration = now + chrono::Duration::hours(1); // 1小时后过期

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            role: role.to_string(),
            exp: expiration.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let secret = Self::get_secret();
        let encoding_key = EncodingKey::from_secret(secret.as_ref());

        encode(&Header::default(), &claims, &encoding_key)
    }

    // 验证 JWT token
    pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = Self::get_secret();
        let decoding_key = DecodingKey::from_secret(secret.as_ref());
        let validation = Validation::default();

        decode::<Claims>(token, &decoding_key, &validation).map(|token_data| token_data.claims)
    }

    // 提取 token 中的用户 ID
    pub fn extract_user_id(token: &str) -> Result<i64, Box<dyn std::error::Error>> {
        let claims = Self::verify_token(token)?;
        claims
            .sub
            .parse::<i64>()
            .map_err(|e| format!("Invalid user ID in token: {e}").into())
    }
}
