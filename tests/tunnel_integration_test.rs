mod common;

use std::sync::Arc;
use decentralized_network::{
    consensus::ConsensusManager,
    zhtp::{
        bridge::ChainAdapter,
        tunnel::{HttpsTunnel, TunnelMetrics},
        crypto::Keypair,
    }
};
use common::setup_test_network;
use anyhow::Result;
use reqwest;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_https_to_zhtp_bridge() -> Result<()> {
    // Set up test network, chains and consensus manager
    let (network, chain1, chain2) = setup_test_network().await?;
    let manager = ConsensusManager::new(100.0, 3600);
    manager.register_node("tunnel_operator".to_string(), 1000.0).await;
    
    // Set up HTTPS tunnel
    let (tunnel, bound_addr) = common::setup_test_tunnel().await?;
    
    // Add route to chain2
    tunnel.mapper.add_route("/test".into(), chain2.get_address()).await;
    
    // Start tunnel in background
    let tunnel_clone = tunnel.clone();
    tokio::spawn(async move {
        tunnel_clone.run().await.unwrap();
    });

    // Wait for tunnel to be ready
    tunnel.wait_ready().await?;
    
    // Send HTTPS request
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // For test self-signed cert
        .build()?;
        
    let res = client.post(format!("https://{}/test", bound_addr))
        .body("test data")
        .send()
        .await?;
        
    assert!(res.status().is_success());
    
    // Verify metrics
    let metrics = tunnel.mapper.get_metrics().await;
    assert_eq!(metrics.successful_requests, 1);
    assert!(metrics.bytes_proxied > 0);
    
    // Update and verify metrics for successful tunnel operation
    manager.update_metrics("tunnel_operator", true, Some(50.0)).await;
    let metrics = manager.get_metrics("tunnel_operator").await.unwrap();
    assert!(metrics.reputation_score > 0.0, "Operator reputation should be positive");
    assert_eq!(metrics.delivery_success, 1, "Should record successful packet delivery");
    assert!(metrics.average_latency > 0.0, "Should record latency metrics");
    
    Ok(())
}

#[tokio::test]
async fn test_tunnel_packet_rewards() -> Result<()> {
    // Set up consensus and register tunnel operator
    let manager = ConsensusManager::new(100.0, 3600);
    manager.register_node("tunnel_operator".to_string(), 1000.0).await;
    
    // Set up tunnel
    let (tunnel, addr) = common::setup_test_tunnel().await?;
    
    // Start tunnel
    let tunnel_clone = tunnel.clone();
    tokio::spawn(async move {
        tunnel_clone.run().await.unwrap();
    });
    
    // Wait for tunnel to be ready
    tunnel.wait_ready().await?;
    
    // Initial metrics check
    let initial_metrics = manager.get_metrics("tunnel_operator").await.unwrap();
    
    // Make HTTPS request
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
        
    let res = client.get(format!("https://{}/test", addr))
        .send()
        .await?;
    assert!(res.status().is_success());
    
    // Update operator metrics for packet routing
    manager.update_metrics("tunnel_operator", true, Some(50.0)).await;
    
    // Get updated metrics
    let final_metrics = manager.get_metrics("tunnel_operator").await.unwrap();
    
    // Verify reputation increased
    assert!(final_metrics.reputation_score > initial_metrics.reputation_score,
        "Operator reputation should increase after successful packet routing");
    
    // Verify final metrics
    assert!(final_metrics.delivery_success > initial_metrics.delivery_success,
        "Successful packet delivery count should increase");
    
    Ok(())
}
