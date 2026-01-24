use std::sync::Arc;

use rdkafka::producer::FutureProducer;
use utils::redis_client::RedisConnectionPool;

use crate::core::configure::app::AppConfig;
use crate::infrastructure::error::TechnicalResult;
use crate::infrastructure::config::service_registry::ServiceRegistry;
use crate::infrastructure::persistence::postgres::DatabaseClient;
use crate::infrastructure::bootstrap::app_state_builder::AppStateBuilder;

use crate::application::address::use_case::address_service_interface::AddressServiceInterface;
use crate::application::auth::use_case::auth_service_interface::AuthServiceInterface;
use crate::application::user::use_case::user_service_interface::UserServiceInterface;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Arc<DatabaseClient>,
    pub redis: Arc<RedisConnectionPool>,
    pub kafka_producer: Arc<FutureProducer>,

    pub auth_service: Arc<dyn AuthServiceInterface>,
    pub user_service: Arc<dyn UserServiceInterface>,
    pub address_service: Arc<dyn AddressServiceInterface>,

    pub gateway_registry: Arc<ServiceRegistry>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> TechnicalResult<Self> {
        AppStateBuilder::build(config).await
    }
}
