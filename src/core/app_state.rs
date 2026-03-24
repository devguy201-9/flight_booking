use crate::application::address::use_case::address_service_interface::AddressServiceInterface;
use crate::application::airport::use_case::airport_service_interface::AirportServiceInterface;
use crate::application::auth::use_case::auth_service_interface::AuthServiceInterface;
use crate::application::boarding_pass::use_case::boarding_pass_service_interface::BoardingPassServiceInterface;
use crate::application::booking::use_case::booking_service_interface::BookingServiceInterface;
use crate::application::checkin::use_case::checkin_service_interface::CheckinServiceInterface;
use crate::application::flight::use_case::flight_service_interface::FlightServiceInterface;
use crate::application::passenger::use_case::passenger_service_interface::PassengerServiceInterface;
use crate::application::user::use_case::user_service_interface::UserServiceInterface;
use crate::core::configure::deploy_mode::DeployMode;
use crate::core::context::request_context_provider::RequestContextProvider;
use crate::infrastructure::config::service_registry::ServiceRegistry;
use crate::infrastructure::persistence::postgres::DatabaseClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    // Infrastructure
    pub db: Arc<DatabaseClient>,
    pub deploy_mode: DeployMode,
    pub ctx_provider: Arc<dyn RequestContextProvider>,
    pub gateway_registry: Arc<ServiceRegistry>,

    // Application services
    pub user_service: Arc<dyn UserServiceInterface>,
    pub auth_service: Arc<dyn AuthServiceInterface>,
    pub address_service: Arc<dyn AddressServiceInterface>,
    pub airport_service: Arc<dyn AirportServiceInterface>,
    pub flight_service: Arc<dyn FlightServiceInterface>,
    pub booking_service: Arc<dyn BookingServiceInterface>,
    pub passenger_service: Arc<dyn PassengerServiceInterface>,
    pub checkin_service: Arc<dyn CheckinServiceInterface>,
    pub boarding_pass_service: Arc<dyn BoardingPassServiceInterface>,
}

impl AppState {
    pub fn is_micro(&self) -> bool {
        self.deploy_mode.is_micro()
    }
}
