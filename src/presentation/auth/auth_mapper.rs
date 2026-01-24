use crate::application::auth::auth_command::{DeviceInfoCommand, LoginByEmailCommand, RefreshTokenCommand};
use crate::presentation::auth::auth_request::{LoginByEmailRequest, RefreshTokenRequest};

impl From<LoginByEmailRequest> for LoginByEmailCommand {
    fn from(req: LoginByEmailRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
            device_info: req.device_info.map(|d| DeviceInfoCommand {
                user_agent: d.user_agent,
                ip_address: d.ip_address,
            }),
        }
    }
}

impl From<RefreshTokenRequest> for RefreshTokenCommand {
    fn from(req: RefreshTokenRequest) -> Self {
        Self {
            token: req.token,
        }
    }
}