use decentralized_network::{
    network::{Network, NetworkCondition},
    zhtp::{
        bridge::ChainAdapter,
        tunnel::HttpsTunnel,
    },
};

use anyhow::Result;
use std::net::SocketAddr;

pub async fn setup_test_network() -> Result<(Network, ChainAdapter, ChainAdapter)> {
    let mut network = Network::new();
    
    // Set up two chains
    network.add_node("chain1", 100.0);
    network.add_node("chain2", 100.0);
    network.connect_nodes("chain1", "chain2");
    
    // Create chain adapters
    let chain1 = ChainAdapter::new("chain1".to_string())?;
    let chain2 = ChainAdapter::new("chain2".to_string())?;
    
    // Set good network conditions
    let condition = NetworkCondition::default();
    network.set_node_condition("chain1", condition.clone());
    network.set_node_condition("chain2", condition);
    
    Ok((network, chain1, chain2))
}

pub async fn setup_test_tunnel() -> Result<(HttpsTunnel, SocketAddr)> {
    // Create test certificates
    let cert_path = "tests/fixtures/test_cert.pem";
    let key_path = "tests/fixtures/test_key.pem";
    
    // Use random port for testing
    let addr: SocketAddr = "127.0.0.1:0".parse()?;
    let tunnel = HttpsTunnel::new(addr, cert_path, key_path).await?;
    let bound_addr = tunnel.listener.local_addr()?;
    
    Ok((tunnel, bound_addr))
}

// Test certificate contents
pub const TEST_CERT: &[u8] = include_bytes!("../fixtures/test_cert.pem");
pub const TEST_KEY: &[u8] = include_bytes!("../fixtures/test_key.pem");