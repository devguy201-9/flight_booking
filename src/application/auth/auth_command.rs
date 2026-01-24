use validator::Validate;

#[derive(Debug, Validate, Clone)]
pub struct DeviceInfoCommand {
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Validate, Clone)]
pub struct LoginByEmailCommand {
    #[validate(email(message = "Invalid email"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    pub device_info: Option<DeviceInfoCommand>,
}

#[derive(Debug, Validate, Clone)]
pub struct RefreshTokenCommand {
    #[validate(length(min = 30, message = "Invalid refresh token"))]
    pub token: String,
}

#[derive(Debug, Validate, Clone)]
pub struct ForgetPasswordCommand {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
}

#[derive(Debug, Validate, Clone)]
pub struct ResetPasswordCommand {
    #[validate(length(min = 10, message = "Invalid token"))]
    pub token: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub new_password: String,
}
