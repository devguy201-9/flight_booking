use crate::core::app_state::AppState;
use std::net::SocketAddr;

use crate::infrastructure::http::app::build_app;
use anyhow::Result;
use tokio::net::TcpListener;

pub struct AppServer {
    tcp: TcpListener,
    state: AppState,
}

impl AppServer {
    pub async fn build(state: AppState, addr: SocketAddr) -> Result<Self> {
        let tcp = TcpListener::bind(addr).await?;
        log::info!("HTTP server listening on {}", tcp.local_addr()?);

        Ok(Self { tcp, state })
    }

    pub async fn run(self) -> Result<()> {
        let app = build_app(self.state);

        axum::serve(self.tcp, app).await?;

        Ok(())
    }
}
