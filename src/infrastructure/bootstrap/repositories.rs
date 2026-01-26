use crate::core::context::request_context_provider::RequestContextProvider;
use crate::infrastructure::persistence::postgres::DatabaseClient;
use crate::infrastructure::persistence::seaorm::repositories::{
    address_repository::SeaOrmAddressRepository, user_repository::SeaOrmUserRepository,
};
use std::sync::Arc;

pub fn build_repositories(
    db: Arc<DatabaseClient>,
    ctx_provider: Arc<dyn RequestContextProvider>,
) -> (Arc<SeaOrmUserRepository>, Arc<SeaOrmAddressRepository>) {
    (
        Arc::new(SeaOrmUserRepository::new(db.clone(), ctx_provider.clone())),
        Arc::new(SeaOrmAddressRepository::new(db, ctx_provider.clone())),
    )
}
