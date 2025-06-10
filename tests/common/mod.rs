use std::sync::Arc;
use std::net::SocketAddr;
use anyhow::Result;
use decentralized_network::{
    zhtp::{
        bridge::ChainAdapter,
        tunnel::HttpsTunnel,
    },
    network::{Network, NetworkCondition},
};

/// Sets up a test network with two nodes and chain adapters
pub async fn setup_test_network() -> Result<(Network, ChainAdapter, ChainAdapter)> {
    let mut network = Network::new();
    
    // Set up two chains with ideal conditions for testing
    network.add_node("chain1", 100.0);
    network.add_node("chain2", 100.0);
    network.connect_nodes("chain1", "chain2");
    
    // Add good network conditions
    let condition = NetworkCondition {
        packet_loss_rate: 0.0,
        latency_multiplier: 1.0,
        bandwidth_cap: None,
    };
    network.set_node_condition("chain1", condition.clone());
    network.set_node_condition("chain2", condition);
    
    // Create chain adapters
    let chain1 = ChainAdapter::new("chain1".to_string())?;
    let chain2 = ChainAdapter::new("chain2".to_string())?;
    
    Ok((network, chain1, chain2))
}

/// Sets up an HTTPS tunnel for testing
pub async fn setup_test_tunnel() -> Result<(Arc<HttpsTunnel>, SocketAddr)> {
    let cert_path = "tests/fixtures/test_cert.pem";
    let key_path = "tests/fixtures/test_key.pem";
    
    // Use random port for testing
    let addr: SocketAddr = "127.0.0.1:0".parse()?;
    let tunnel = HttpsTunnel::new(addr, cert_path, key_path).await?;
    let bound_addr = tunnel.listener.local_addr()?;
    
    Ok((Arc::new(tunnel), bound_addr))
}