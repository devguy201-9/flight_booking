use crate::core::app_state::AppState;
use crate::core::configure::app::AppConfig;

use crate::infrastructure::http::app::build_app;
use anyhow::Result;
use tokio::net::TcpListener;

pub struct AppServer {
    tcp: TcpListener,
    state: AppState,
}

impl AppServer {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let (tcp, config) = Self::bind_socket(config).await?;
        let state = AppState::new(config).await?;

        Ok(Self { tcp, state })
    }

    pub async fn run(self) -> Result<()> {
        let app = build_app(self.state);

        axum::serve(self.tcp, app.into_make_service()).await?;

        //axum::serve(self.tcp, app).await?;
        Ok(())
    }

    async fn bind_socket(mut config: AppConfig) -> Result<(TcpListener, AppConfig)> {
        let tcp = TcpListener::bind(config.server.get_socket_addr()?).await?;
        let addr = tcp.local_addr()?;

        log::info!("HTTP server listening on {}", addr);
        config.server.port = addr.port();

        Ok((tcp, config))
    }
}
