use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header, Error as ActixError, FromRequest,
    HttpRequest,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use dotenv::dotenv;

// 初始化函数，在模块加载时调用
fn init_dotenv() {
    // 尝试加载.env文件，忽略可能的错误
    let _ = dotenv();
}

// JWT密钥 - 从.env文件加载
lazy_static! {
    static ref JWT_SECRET: String = {
        // 确保.env文件已被加载
        init_dotenv();
        std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            eprintln!("警告: JWT_SECRET 环境变量未设置，使用默认密钥。这在生产环境中不安全！");
            "your_jwt_secret_key".to_string()
        })
    };
}

// JWT声明 - 存储在令牌中的数据
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub token_type:    String,      // 令牌类型
    pub username:      String,      // 用户名
    pub name:          String,      // 用户姓名
    pub major_class:   String,      // 用户专业班级
    pub role:          String,      // 用户角色
    pub exp:           usize,       // 过期时间戳
}

// JWT Refresh Token声明 - 存储在令牌中的数据
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtRefreshToken {
    pub token_type:    String,      // 令牌类型
    pub username:      String,      // 用户名
    pub name:          String,      // 用户姓名
    pub major_class:   String,      // 用户专业班级
    pub role:          String,      // 用户角色
    pub exp:           usize,       // 过期时间戳
}

// 用于从HTTP请求中提取JWT令牌
pub struct JwtMiddleware {
    pub claims: JwtToken,
}

// 从HTTP请求中解析JWT令牌的实现
impl FromRequest for JwtMiddleware {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // 从请求头中获取授权信息
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => header,
            None => {
                return ready(Err(ErrorUnauthorized("缺少授权头")));
            }
        };

        // 提取Bearer令牌
        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                return ready(Err(ErrorUnauthorized("无效的授权头格式")));
            }
        };

        // 验证Bearer前缀并提取令牌
        if !auth_str.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("无效的授权方案，必须是Bearer")));
        }

        let token = &auth_str["Bearer ".len()..];
        
        // 验证并解码JWT令牌
        match decode::<JwtToken>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => {
                ready(Ok(JwtMiddleware {
                    claims: token_data.claims,
                }))
            }
            Err(e) => {
                match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        ready(Err(ErrorUnauthorized("令牌已过期")))
                    }
                    _ => ready(Err(ErrorUnauthorized("无效的令牌"))),
                }
            }
        }
    }
}

// 生成新JWT令牌的辅助函数
pub fn generate_token( token_type: &str, username: &str, name: &str, major_class: &str, role: &str, expires_in: Duration ) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;
    
    let expires_at = now + expires_in.as_secs() as usize;
    
    let claims = JwtToken {
        token_type: token_type.to_string(),
        username: username.to_string(),
        name: name.to_string(),
        major_class: major_class.to_string(),
        role: role.to_string(),
        exp: expires_at,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

// 验证JWT令牌的辅助函数
pub fn validate_token(token: &str) -> Result<JwtToken, jsonwebtoken::errors::Error> {
    let token_data = decode::<JwtToken>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
}
