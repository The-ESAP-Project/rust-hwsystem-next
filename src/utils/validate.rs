pub fn validate_username(username: &str) -> Result<(), &'static str> {
    // 用户名长度校验：5 <= x <= 16
    if username.len() < 5 || username.len() > 16 {
        return Err("Username length must be between 5 and 16 characters");
    }
    // 用户名格式校验：只能包含字母、数字、下划线或连字符
    let username_re = regex::Regex::new(r"^[A-Za-z0-9_-]+$").unwrap();
    if !username_re.is_match(username) {
        return Err("Username must contain only letters, numbers, underscores or hyphens");
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), &'static str> {
    // 邮箱格式校验：必须包含 @ 和 .
    let email_re = regex::Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$").unwrap();
    if !email_re.is_match(email) {
        return Err("Email format is invalid");
    }
    Ok(())
}
