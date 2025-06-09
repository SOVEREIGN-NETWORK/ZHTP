use anyhow::Result;
use decentralized_network::{
    zhtp::{Keypair, ZhtpNode},
    Network, StorageManager, ConsensusManager, SharedNode
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::signal;

const DISCOVERY_PORT: u16 = 9000;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("\nShutting down discovery node...");
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== ZHTP Discovery Node ===");
    println!("Zero-Knowledge Hidden Transit Protocol Network");

    // Initialize systems
    let _network = Network::new();
    let storage = StorageManager::new();
    let consensus = ConsensusManager::new(500.0, 3600);

    // Create discovery node
    let keypair = Keypair::generate();
    let addr = format!("0.0.0.0:{}", DISCOVERY_PORT).parse()?;
    let node = ZhtpNode::new(addr, keypair).await?;
    let shared = SharedNode::new(node.clone());
    let _node_handle = Arc::new(Mutex::new(node));

    // Register discovery service
    let mut network = Network::new();
    network.add_node("discovery".to_string(), 1000.0);
    consensus.register_node("discovery".to_string(), 1000.0).await;
    if !storage.register_node("discovery".to_string(), 1_000_000).await {
        println!("Warning: Failed to register with storage network");
    }

    println!("\nDiscovery node starting...");
    println!("Listening on: {}", addr);

    // Start continuous listening
    if let Err(e) = shared.start_listening().await {
        println!("Error starting listener: {}", e);
    }

    println!("\nDiscovery node is ready!");
    println!("Other nodes can connect using this address\n");
    println!("Press Ctrl+C to stop");

    // Wait for shutdown signal
    shutdown_signal().await;

    Ok(())
}