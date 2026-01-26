use anyhow::Result;
use flight_booking::infrastructure::bootstrap::app_state_builder::AppStateBuilder;
use flight_booking::infrastructure::http::server::AppServer;
use log::{LevelFilter, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .format_target(true)
        .init();

    info!("The initialization of Tracing was successful!");
    let (state, addr) = AppStateBuilder::build().await?;

    let server = AppServer::build(state, addr).await?;
    info!("Starting server...");

    if let Err(err) = server.run().await {
        error!("Server crashed: {:?}", err);
        std::process::exit(1);
    }

    /*let server_task = tokio::spawn(async {
        if let Err(e) = server.run().await {
            error!("HTTP Server error: {:?}", e);
        }
    });

    let _server_result = tokio::join!(server_task);*/

    Ok(())
}
