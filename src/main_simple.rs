use anyhow::Result;
use decentralized_network::{
    zhtp::{
        network_service::NetworkService,
        api_server::ApiServer,
    }
};
use log::{info, error};
use std::sync::Arc;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();

    info!("🚀 Starting ZHTP Network Service");
    
    // Initialize the network service
    let network_service = Arc::new(NetworkService::new().await?);
    
    // Start the API server
    let api_server = ApiServer::new(network_service.clone());
    let api_handle = tokio::spawn(async move {
        if let Err(e) = api_server.start().await {
            error!("API server error: {}", e);
        }
    });
    
    // Start the network service
    let network_handle = tokio::spawn(async move {
        if let Err(e) = network_service.start().await {
            error!("Network service error: {}", e);
        }
    });
    
    info!("✅ ZHTP Network Service started successfully");
    info!("🌐 API Server: http://localhost:4444");
    info!("📱 Browser: Start 'python -m http.server 4000' and visit http://localhost:4000/browser/");
    info!("Press Ctrl+C to stop");
    
    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("🛑 Shutdown signal received");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }
    
    // Graceful shutdown
    info!("🔄 Shutting down services...");
    api_handle.abort();
    network_handle.abort();
    
    info!("✅ ZHTP Network Service stopped");
    Ok(())
}
