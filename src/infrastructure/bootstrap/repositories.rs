use std::sync::Arc;

use crate::infrastructure::persistence::postgres::DatabaseClient;
use crate::infrastructure::persistence::seaorm::repositories::{
    address_repository::SeaOrmAddressRepository, user_repository::SeaOrmUserRepository,
};

pub fn build_repositories(
    db: Arc<DatabaseClient>,
) -> (Arc<SeaOrmUserRepository>, Arc<SeaOrmAddressRepository>) {
    (
        Arc::new(SeaOrmUserRepository::new(db.clone())),
        Arc::new(SeaOrmAddressRepository::new(db)),
    )
}
