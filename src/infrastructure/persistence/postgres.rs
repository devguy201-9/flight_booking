use crate::core::configure::app::AppConfig;
use crate::infrastructure::error::TechnicalResult;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub type DatabaseClient = DatabaseConnection;

pub trait DatabaseClientExt: Sized {
    fn build_from_config(
        config: &AppConfig,
    ) -> impl std::future::Future<Output = TechnicalResult<Self>>;
}

impl DatabaseClientExt for DatabaseClient {
    async fn build_from_config(config: &AppConfig) -> TechnicalResult<Self> {
        let mut opt = ConnectOptions::new(config.db.get_url());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

        let db = Database::connect(opt).await?; // DbErr -> TechnicalError (From impl)
        Ok(db)
    }
}
