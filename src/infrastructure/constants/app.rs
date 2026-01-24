use std::time::Duration;

pub const ENV_PREFIX: &str = "APP";
pub const CODE_LEN: usize = 5;

pub const QUEUE_EMPTY_DELAY_SECS: Duration = Duration::from_secs(60);
pub const COMPLETE_TASK_DELAY_SECS: Duration = Duration::from_secs(10);
pub const MINIMUM_DELAY_TIME: Duration = Duration::from_millis(120);

pub const CHECK_EMAIL_MESSAGE: &str = "Please check you email.";
pub const AUTHORIZATION: &str = "Authorization";
pub const BEARER: &str = "Bearer";

pub const APP_DOMAIN: &str = "";
pub const APP_EMAIL_ADDR: &str = "";
