use std::net::SocketAddr;
use std::sync::Arc;

use rdkafka::producer::FutureProducer;

use crate::core::app_state::AppState;
use crate::infrastructure::error::TechnicalResult;
use crate::infrastructure::runtime::gateway_registry::build_gateway_registry;

use crate::application::address::use_case::address_service::AddressService;
use crate::application::auth::use_case::auth_service::AuthService;
use crate::application::user::use_case::user_service::UserService;
use crate::infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;

use crate::infrastructure::bootstrap::{
    cache::build_cache, database::build_database, event_publishers::build_event_publishers,
    jwt::build_token_service, kafka::build_kafka_producer, redis::build_redis,
    repositories::build_repositories,
};
use crate::infrastructure::context::axum_request_context_provider::AxumRequestContextProvider;
use crate::infrastructure::runtime::config::CONFIG;

pub struct AppStateBuilder;

impl AppStateBuilder {
    pub async fn build() -> TechnicalResult<(AppState, SocketAddr)> {
        let config = Arc::new(CONFIG.clone());
        let deploy_mode = config.deploy_mode;

        // -------- infra resources (INIT ONE TIME) --------
        let db = build_database(&config).await?;
        let redis = build_redis(&config).await?;
        let kafka_producer: Arc<FutureProducer> = build_kafka_producer(&config)?;
        let ctx_provider = Arc::new(AxumRequestContextProvider::new(|| {
            crate::infrastructure::runtime::request_context::get()
        }));
        //let gateway_registry = Arc::new(build_gateway_registry().await);
        let gateway_registry = Arc::new(build_gateway_registry(&config).await);

        // -------- adapters --------
        let cache = build_cache(redis.clone());
        let (user_repo, address_repo) = build_repositories(db.clone(), ctx_provider.clone());
        let token_service = build_token_service(&config)?;
        let password_hasher = Arc::new(Argon2PasswordHasher);
        let (user_events, address_events) = build_event_publishers(kafka_producer.clone());

        // -------- application services --------
        let auth_service = Arc::new(AuthService::new(
            cache.clone(),
            user_repo.clone(),
            token_service,
            password_hasher.clone(),
            user_events.clone(),
        ));

        let user_service = Arc::new(UserService::new(
            cache.clone(),
            user_repo.clone(),
            password_hasher.clone(),
            user_events,
        ));

        let address_service = Arc::new(AddressService::new(
            cache,
            address_repo,
            user_repo,
            address_events,
        ));

        let state = AppState {
            db,
            auth_service,
            user_service,
            address_service,
            gateway_registry,
            deploy_mode,
            ctx_provider,
        };
        let addr = config.server.get_socket_addr()?;

        Ok((state, addr))
    }
}
