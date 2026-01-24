use std::time::Duration;

pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(36000);
pub const EXPIRE_INVITATION_CODE_SECS: Duration = Duration::from_secs(86000);
pub const EXPIRE_BLOCKED_EMAIL_SECS: Duration = Duration::from_secs(300);
pub const EXPIRE_FORGET_PASS_CODE_SECS: Duration = Duration::from_secs(300);

pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(36000);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(86400);
