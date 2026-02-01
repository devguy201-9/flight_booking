use crate::core::configure::app::AppConfig;
use crate::core::configure::app::Profile;
use std::str::FromStr;
use std::sync::LazyLock;

fn load_profile() -> Profile {
    match std::env::var("APP_PROFILE") {
        Ok(value) => {
            Profile::from_str(&value).unwrap_or_else(|_| panic!("Invalid APP_PROFILE: {value}"))
        }
        Err(_) => Profile::Local, // DEFAULT LOCAL
    }
}

pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    let profile = load_profile();
    log::info!("Loading app config with profile: {}", profile);
    AppConfig::read(profile).expect("Failed to load app config")
});
