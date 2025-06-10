use decentralized_network::{
    zhtp::{
        bridge::{ChainAdapter, CrossChainMessage},
        contracts::WasmRuntime,
        zk_proofs::{RoutingProof, ProofType},
    },
    network::{Network, NetworkCondition},
    consensus::NetworkMetrics,
};

use anyhow::Result;
use tokio;

async fn setup_test_network() -> Result<Network> {
    let mut network = Network::new();
    
    // Add nodes representing different chains
    network.add_node("chain1", 100.0);
    network.add_node("chain2", 100.0);
    
    // Connect the chains
    network.connect_nodes("chain1", "chain2");
    
    // Set good network conditions
    let condition = NetworkCondition {
        packet_loss_rate: 0.0,
        latency_multiplier: 1.0,
        bandwidth_cap: None,
    };
    
    network.set_node_condition("chain1", condition.clone());
    network.set_node_condition("chain2", condition);
    
    Ok(network)
}

#[tokio::test]
async fn test_cross_chain_messaging() -> Result<()> {
    // Setup test environment
    let network = setup_test_network().await?;
    
    // Create chain adapters
    let mut chain1 = ChainAdapter::new("chain1".to_string())?;
    let mut chain2 = ChainAdapter::new("chain2".to_string())?;
    
    // Create a cross-chain message
    let message = CrossChainMessage {
        source_chain: "chain1".to_string(),
        target_chain: "chain2".to_string(),
        nonce: 1,
        payload: vec![1, 2, 3, 4],
        proof: None,
        state_hash: [0; 32],
    };
    
    // Queue message on source chain
    chain1.queue_message(message.clone()).await?;
    
    // Process messages on target chain
    let processed = chain2.process_messages().await?;
    
    // Verify message was processed
    assert_eq!(processed.len(), 1);
    assert_eq!(processed[0].source_chain, "chain1");
    assert_eq!(processed[0].target_chain, "chain2");
    assert_eq!(processed[0].payload, vec![1, 2, 3, 4]);
    
    Ok(())
}

#[tokio::test]
async fn test_state_verification() -> Result<()> {
    // Setup chain adapters
    let mut chain1 = ChainAdapter::new("chain1".to_string())?;
    let mut chain2 = ChainAdapter::new("chain2".to_string())?;
    
    // Create message with specific state
    let state = [1u8; 32];
    let message = CrossChainMessage {
        source_chain: "chain1".to_string(),
        target_chain: "chain2".to_string(),
        nonce: 1,
        payload: vec![1, 2, 3],
        proof: None,
        state_hash: state,
    };
    
    // Queue message but don't verify state first
    chain2.queue_message(message.clone()).await?;
    let processed = chain2.process_messages().await?;
    
    // Should not process without verified state
    assert_eq!(processed.len(), 0);
    
    // Verify state and try again
    chain2.verifier.verify_state("chain1", state).await;
    chain2.queue_message(message).await?;
    let processed = chain2.process_messages().await?;
    
    // Should process with verified state
    assert_eq!(processed.len(), 1);
    
    Ok(())
}