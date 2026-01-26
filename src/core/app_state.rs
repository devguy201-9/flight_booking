use crate::application::address::use_case::address_service_interface::AddressServiceInterface;
use crate::application::auth::use_case::auth_service_interface::AuthServiceInterface;
use crate::application::user::use_case::user_service_interface::UserServiceInterface;
use crate::core::configure::deploy_mode::DeployMode;
use crate::infrastructure::config::service_registry::ServiceRegistry;
use crate::infrastructure::persistence::postgres::DatabaseClient;
use std::sync::Arc;
use crate::core::context::request_context_provider::RequestContextProvider;

#[derive(Clone)]
pub struct AppState {
    // for check server (future will remove)
    pub db: Arc<DatabaseClient>,

    // used
    pub user_service: Arc<dyn UserServiceInterface>,
    pub auth_service: Arc<dyn AuthServiceInterface>,
    pub address_service: Arc<dyn AddressServiceInterface>,
    pub gateway_registry: Arc<ServiceRegistry>,
    pub deploy_mode: DeployMode,
    pub ctx_provider: Arc<dyn RequestContextProvider>,
}

impl AppState {
    pub fn is_micro(&self) -> bool {
        self.deploy_mode.is_micro()
    }
}
