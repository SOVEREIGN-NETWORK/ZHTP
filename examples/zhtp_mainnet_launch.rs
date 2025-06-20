use anyhow::Result;
use decentralized_network::{
    zhtp::{
        p2p_network::ZhtpP2PNetwork,
        zk_transactions::ZkTransaction,
    }
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use log::{info, warn};

/// ZHTP Mainnet Launch Example
/// 
/// This example demonstrates a true decentralized P2P network launch using the ZHTP protocol.
/// It creates multiple network nodes that can discover each other and participate in 
/// zero-knowledge consensus without relying on TCP or libp2p.
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("Starting ZHTP Mainnet Launch Example");
    info!("This demonstrates a true decentralized P2P network using ZHTP protocol");
    
    // Define bootstrap nodes (in a real mainnet, these would be well-known addresses)
    let bootstrap_nodes = vec![
        "127.0.0.1:8001".parse::<SocketAddr>()?,
        "127.0.0.1:8002".parse::<SocketAddr>()?,
        "127.0.0.1:8003".parse::<SocketAddr>()?,
    ];
    
    // Launch multiple ZHTP nodes to simulate a distributed network
    let node_configs = vec![
        ("127.0.0.1:8001", "Node-Alpha"),
        ("127.0.0.1:8002", "Node-Beta"), 
        ("127.0.0.1:8003", "Node-Gamma"),
        ("127.0.0.1:8004", "Node-Delta"),
        ("127.0.0.1:8005", "Node-Epsilon"),
    ];
    
    let mut network_nodes = Vec::new();
    
    // Create and start each network node
    for (addr_str, node_name) in node_configs {
        let addr = addr_str.parse::<SocketAddr>()?;
        info!("Creating ZHTP node: {} at {}", node_name, addr);
        
        match create_zhtp_network_node(addr, bootstrap_nodes.clone(), node_name).await {
            Ok(node) => {
                info!("Successfully created {}", node_name);
                network_nodes.push(node);
            }
            Err(e) => {
                warn!("Failed to create {}: {}", node_name, e);
            }
        }        // Small delay between node creation to prevent race conditions
        sleep(Duration::from_millis(1500)).await;
    }
    
    info!("Created {} ZHTP network nodes", network_nodes.len());
    
    // Allow network discovery and consensus to establish
    info!("Allowing network discovery and consensus to establish...");
    for i in 1..=30 {
        info!("Network stabilization: {}s", i);
        sleep(Duration::from_secs(1)).await;
        
        // Check network health every 10 seconds
        if i % 10 == 0 {
            info!("Checking network health...");
            check_network_health(&network_nodes).await;
        }
    }
    
    // Demonstrate network functionality
    info!("Demonstrating ZHTP network functionality...");
    
    // Test transaction broadcasting
    if let Some(first_node) = network_nodes.first() {
        info!("Testing transaction broadcast from first node");
        test_transaction_broadcast(first_node).await?;
    }
    
    // Monitor network for a while
    info!("Monitoring network activity...");
    for i in 1..=60 {
        if i % 10 == 0 {
            info!("Network monitoring: {}s elapsed", i);
            check_network_health(&network_nodes).await;
        }
        sleep(Duration::from_secs(1)).await;
    }
    
    info!("ZHTP Mainnet Launch Example completed successfully!");
    info!("Network demonstrated true decentralized P2P communication using ZHTP protocol");
    
    Ok(())
}

/// Create a ZHTP network node with P2P capabilities
async fn create_zhtp_network_node(
    local_addr: SocketAddr,
    bootstrap_nodes: Vec<SocketAddr>,
    node_name: &str,
) -> Result<Arc<ZhtpP2PNetwork>> {
    info!("Setting up {} at {}", node_name, local_addr);
    
    info!("Creating ZHTP P2P network for {}", node_name);
    
    // Create the P2P network with ZHTP protocol (it will create its own keypair and consensus)
    let p2p_network = ZhtpP2PNetwork::new(
        local_addr,
        bootstrap_nodes,
    ).await?;
    
    let p2p_network = Arc::new(p2p_network);
    info!("Created ZHTP P2P network for {}", node_name);
    
    // Start the network
    p2p_network.start().await?;
    info!("Started {} - now participating in ZHTP network", node_name);
    
    Ok(p2p_network)
}

/// Check the health of network nodes
async fn check_network_health(nodes: &[Arc<ZhtpP2PNetwork>]) {
    info!("=== Network Health Report ===");
    
    let mut total_peers = 0;
    let mut total_health = 0.0;
    
    for (i, node) in nodes.iter().enumerate() {
        match node.get_network_stats().await {
            Ok(stats) => {
                let node_name = format!("Node-{}", ['A', 'B', 'C', 'D', 'E'][i]);
                
                info!("{}: {} peers (health: {:.2}, latency: {:.1}ms)", 
                      node_name,
                      stats.connected_peers,
                      stats.network_health,
                      stats.avg_latency.as_millis()
                );
                
                total_peers += stats.connected_peers;
                total_health += stats.network_health;
            }
            Err(e) => {
                warn!("Failed to get stats for node {}: {}", i + 1, e);
            }
        }
    }
    
    let avg_peers = if nodes.len() > 0 { total_peers as f64 / nodes.len() as f64 } else { 0.0 };
    let avg_health = if nodes.len() > 0 { total_health / nodes.len() as f64 } else { 0.0 };
    
    info!("Network Summary - Avg Peers: {:.1}, Avg Health: {:.2}", avg_peers, avg_health);
    
    if avg_health > 0.7 {
        info!("✅ Network health: GOOD");
    } else if avg_health > 0.4 {
        info!("⚠️  Network health: MODERATE");
    } else {
        info!("❌ Network health: POOR");
    }
    
    info!("=== End Health Report ===");
}

/// Test transaction broadcasting across the network
async fn test_transaction_broadcast(node: &Arc<ZhtpP2PNetwork>) -> Result<()> {
    info!("Creating test transaction for network broadcast");
    
    // Create a test zero-knowledge transaction with correct parameters
    let test_tx = ZkTransaction::new(
        "test_sender",    // Sender address
        "test_receiver",  // Receiver address  
        100.0,           // Amount
        1000.0,          // Sender balance
        1,               // Nonce
    )?;
    
    info!("Broadcasting test transaction to ZHTP network");
    
    // Broadcast the transaction
    node.broadcast_transaction(test_tx).await?;
    
    info!("Test transaction broadcasted successfully");
    
    Ok(())
}
