use std::sync::Arc;

use crate::core::configure::app::AppConfig;
use crate::infrastructure::error::TechnicalResult;
use crate::infrastructure::persistence::postgres::{DatabaseClient, DatabaseClientExt};

pub async fn build_database(config: &AppConfig) -> TechnicalResult<Arc<DatabaseClient>> {
    Ok(Arc::new(DatabaseClient::build_from_config(config).await?))
}
