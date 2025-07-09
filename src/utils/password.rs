use crate::errors::HWSystemError;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

/// 哈希密码
pub fn hash_password(password: &str) -> Result<String, HWSystemError> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| HWSystemError::validation(format!("密码哈希失败: {e}")))?;
    Ok(hash.to_string())
}

/// 验证密码
pub fn verify_password(password: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(_) => false,
    }
}
