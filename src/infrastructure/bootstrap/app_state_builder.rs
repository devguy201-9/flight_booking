use std::net::SocketAddr;
use std::sync::Arc;

use rdkafka::producer::FutureProducer;

use crate::core::app_state::AppState;
use crate::infrastructure::error::TechnicalResult;
use crate::infrastructure::runtime::gateway_registry::build_gateway_registry;

// Application services
use crate::application::address::use_case::address_service::AddressService;
use crate::application::airport::use_case::airport_service::AirportService;
use crate::application::auth::use_case::auth_service::AuthService;
use crate::application::boarding_pass::use_case::boarding_pass_service::BoardingPassService;
use crate::application::booking::use_case::booking_service::BookingService;
use crate::application::checkin::use_case::checkin_service::CheckinService;
use crate::application::flight::use_case::flight_service::FlightService;
use crate::application::passenger::use_case::passenger_service::PassengerService;
use crate::application::user::use_case::user_service::UserService;

// Infrastructure
use crate::infrastructure::bootstrap::{
    cache::build_cache, database::build_database, event_publishers::build_event_publishers,
    jwt::build_token_service, kafka::build_kafka_producer, redis::build_redis,
    repositories::build_repositories,
};
use crate::infrastructure::context::axum_request_context_provider::AxumRequestContextProvider;
use crate::infrastructure::runtime::config::CONFIG;
use crate::infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;

pub struct AppStateBuilder;

impl AppStateBuilder {
    pub async fn build() -> TechnicalResult<(AppState, SocketAddr)> {
        let config = Arc::new(CONFIG.clone());
        let deploy_mode = config.deploy_mode;

        // -------- infrastructure resources --------
        let db = build_database(&config).await?;
        let redis = build_redis(&config).await?;
        let kafka_producer: Arc<FutureProducer> = build_kafka_producer(&config)?;
        let ctx_provider = Arc::new(AxumRequestContextProvider::new(|| {
            crate::infrastructure::runtime::request_context::get()
        }));
        let gateway_registry = Arc::new(build_gateway_registry(&config).await);

        // -------- adapters --------
        let cache = build_cache(redis.clone());
        let repos = build_repositories(db.clone(), ctx_provider.clone());
        let token_service = build_token_service(&config)?;
        let password_hasher = Arc::new(Argon2PasswordHasher);
        let events = build_event_publishers(kafka_producer.clone());

        // -------- application services --------
        let auth_service = Arc::new(AuthService::new(
            cache.clone(),
            repos.user.clone(),
            token_service,
            password_hasher.clone(),
            events.user.clone(),
        ));

        let user_service = Arc::new(UserService::new(
            cache.clone(),
            repos.user.clone(),
            password_hasher.clone(),
            events.user.clone(),
        ));

        let address_service = Arc::new(AddressService::new(
            cache.clone(),
            repos.address.clone(),
            repos.user.clone(),
            events.address.clone(),
        ));

        let airport_service = Arc::new(AirportService::new(
            cache.clone(),
            repos.airport.clone(),
            events.airport.clone(),
        ));

        let flight_service = Arc::new(FlightService::new(
            cache.clone(),
            repos.flight.clone(),
            repos.airport.clone(),
            events.flight.clone(),
        ));

        let booking_service = Arc::new(BookingService::new(
            cache.clone(),
            repos.booking.clone(),
            repos.flight.clone(),
            repos.user.clone(),
            events.booking.clone(),
        ));

        let passenger_service = Arc::new(PassengerService::new(
            cache.clone(),
            repos.passenger.clone(),
            repos.booking.clone(),
            events.passenger.clone(),
        ));

        let checkin_service = Arc::new(CheckinService::new(
            cache.clone(),
            repos.checkin.clone(),
            repos.booking.clone(),
            repos.flight.clone(),
            repos.passenger.clone(),
            repos.boarding_pass.clone(),
            events.checkin.clone(),
        ));

        let boarding_pass_service = Arc::new(BoardingPassService::new(
            cache,
            repos.boarding_pass.clone(),
            repos.checkin.clone(),
            repos.booking.clone(),
            repos.passenger.clone(),
            events.boarding_pass.clone(),
        ));

        let state = AppState {
            db,
            deploy_mode,
            ctx_provider,
            gateway_registry,
            user_service,
            auth_service,
            address_service,
            airport_service,
            flight_service,
            booking_service,
            passenger_service,
            checkin_service,
            boarding_pass_service,
        };
        let addr = config.server.get_socket_addr()?;

        Ok((state, addr))
    }
}
