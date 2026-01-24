use std::sync::LazyLock;
use crate::core::configure::app::{AppConfig, Profile};

pub static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::read(Profile::Local).unwrap());
