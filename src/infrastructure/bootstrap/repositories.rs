use crate::core::context::request_context_provider::RequestContextProvider;
use crate::infrastructure::persistence::postgres::DatabaseClient;
use crate::infrastructure::persistence::seaorm::repositories::{
    address_repository::SeaOrmAddressRepository,
    airport_repository::SeaOrmAirportRepository,
    boarding_pass_repository::SeaOrmBoardingPassRepository,
    booking_repository::SeaOrmBookingRepository,
    checkin_repository::SeaOrmCheckinRepository,
    flight_repository::SeaOrmFlightRepository,
    passenger_repository::SeaOrmPassengerRepository,
    user_repository::SeaOrmUserRepository,
};
use std::sync::Arc;

pub struct Repositories {
    pub user: Arc<SeaOrmUserRepository>,
    pub address: Arc<SeaOrmAddressRepository>,
    pub airport: Arc<SeaOrmAirportRepository>,
    pub flight: Arc<SeaOrmFlightRepository>,
    pub booking: Arc<SeaOrmBookingRepository>,
    pub passenger: Arc<SeaOrmPassengerRepository>,
    pub checkin: Arc<SeaOrmCheckinRepository>,
    pub boarding_pass: Arc<SeaOrmBoardingPassRepository>,
}

pub fn build_repositories(
    db: Arc<DatabaseClient>,
    ctx_provider: Arc<dyn RequestContextProvider>,
) -> Repositories {
    Repositories {
        user: Arc::new(SeaOrmUserRepository::new(db.clone(), ctx_provider.clone())),
        address: Arc::new(SeaOrmAddressRepository::new(db.clone(), ctx_provider.clone())),
        airport: Arc::new(SeaOrmAirportRepository::new(db.clone())),
        flight: Arc::new(SeaOrmFlightRepository::new(db.clone(), ctx_provider.clone())),
        booking: Arc::new(SeaOrmBookingRepository::new(db.clone(), ctx_provider.clone())),
        passenger: Arc::new(SeaOrmPassengerRepository::new(db.clone(), ctx_provider.clone())),
        checkin: Arc::new(SeaOrmCheckinRepository::new(db.clone(), ctx_provider.clone())),
        boarding_pass: Arc::new(SeaOrmBoardingPassRepository::new(db, ctx_provider)),
    }
}
