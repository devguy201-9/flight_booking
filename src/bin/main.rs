use anyhow::Result;
use flight_booking::infrastructure::http::server::AppServer;
use flight_booking::infrastructure::runtime::config::CONFIG;
use log::{LevelFilter, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .format_target(true)
        .init();

    info!("The initialization of Tracing was successful!");
    let config = CONFIG.clone();
    let server = AppServer::new(config).await?;
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
