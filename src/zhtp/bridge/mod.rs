// External crate imports
use anyhow::Result;
use serde::{Serialize, Deserialize};

// Standard library imports
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

// Async runtime imports
use tokio::sync::RwLock;

// Internal imports
use crate::zhtp::{
    contracts::WasmRuntime,
    zk_proofs::{RoutingProof, ByteRoutingProof},
};

/// Cross-chain message format for blockchain interoperability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainMessage {
    /// Source blockchain identifier
    pub source_chain: String,
    
    /// Target blockchain identifier 
    pub target_chain: String,
    
    /// Message sequence number
    pub nonce: u64,
    
    /// Contract bytecode or call data
    pub payload: Vec<u8>,
    
    /// Zero-knowledge proof for routing verification (serializable format)
    pub proof: Option<ByteRoutingProof>,
    
    /// Message state hash
    pub state_hash: [u8; 32],
}

/// Manages verification of cross-chain state transitions
#[derive(Debug)]
pub struct StateVerifier {
    /// Chain identifier
    chain_id: String,
    
    /// Current state root
    state_root: [u8; 32],
    
    /// Map of verified states from other chains
    verified_states: Arc<RwLock<HashMap<String, [u8; 32]>>>,
}

impl StateVerifier {
    pub fn new(chain_id: String) -> Self {
        Self {
            chain_id,
            state_root: [0; 32],
            verified_states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn verify_state(&mut self, chain_id: &str, state_root: [u8; 32]) -> bool {
        let mut states = self.verified_states.write().await;
        states.insert(chain_id.to_string(), state_root);
        true // TODO: Implement proper verification
    }

    pub async fn get_verified_state(&self, chain_id: &str) -> Option<[u8; 32]> {
        let states = self.verified_states.read().await;
        states.get(chain_id).copied()
    }
}

/// Adapter for cross-chain communication and contract execution
#[derive(Debug)]
pub struct ChainAdapter {
    /// Chain identifier
    chain_id: String,
    
    /// WebAssembly runtime for contract execution
    pub(crate) runtime: WasmRuntime,
    
    /// State verification
    pub verifier: StateVerifier,
    
    /// Pending messages queue
    pub message_queue: Arc<RwLock<Vec<CrossChainMessage>>>,
}


impl CrossChainMessage {
    pub fn new(
        source_chain: String,
        target_chain: String,
        nonce: u64,
        payload: Vec<u8>,
        proof: Option<RoutingProof>,
        state_hash: [u8; 32],
    ) -> Self {
        Self {
            source_chain,
            target_chain,
            nonce,
            payload,
            proof: proof.map(ByteRoutingProof::from),
            state_hash,
        }
    }

    pub fn get_proof(&self) -> Option<Result<RoutingProof, ark_serialize::SerializationError>> {
        self.proof.as_ref().map(|p| RoutingProof::try_from(p.clone()))
    }
}

impl CrossChainMessage {
    // Convert to RoutingProof version for processing
    fn to_processing_message(&self) -> Result<Self, ark_serialize::SerializationError> {
        let proof = match &self.proof {
            None => None,
            Some(p) => Some(RoutingProof::try_from(p.clone())?)
        };

        Ok(CrossChainMessage {
            source_chain: self.source_chain.clone(),
            target_chain: self.target_chain.clone(),
            nonce: self.nonce,
            payload: self.payload.clone(),
            proof: Some(ByteRoutingProof::from(proof.unwrap_or_default())),
            state_hash: self.state_hash,
        })
    }
}

impl ChainAdapter {
    pub fn get_address(&self) -> SocketAddr {
        format!("{}:8545", self.chain_id).parse().unwrap()
    }

    pub fn new(chain_id: String) -> Result<Self> {
        Ok(Self {
            chain_id: chain_id.clone(),
            runtime: WasmRuntime::new(),
            verifier: StateVerifier::new(chain_id),
            message_queue: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn queue_message(&self, message: CrossChainMessage) -> Result<()> {
        let mut queue = self.message_queue.write().await;
        queue.push(message);
        Ok(())
    }

    pub async fn process_messages(&mut self) -> Result<Vec<CrossChainMessage>> {
        let mut processed = Vec::new();
        let mut queue = self.message_queue.write().await;
        
        while let Some(message) = queue.pop() {
            let message = match message.to_processing_message() {
                Ok(m) => m,
                Err(e) => {
                    println!("Failed to process message proof: {}", e);
                    continue;
                }
            };
            // Verify target chain
            if message.target_chain != self.chain_id {
                continue;
            }

            // Verify message state if available
            if let Some(source_state) = self.verifier.get_verified_state(&message.source_chain).await {
                if source_state != message.state_hash {
                    println!("Invalid message state from chain {}", message.source_chain);
                    continue;
                }
            }

            // Execute contract code
            if let Err(e) = self.runtime.deploy(&message.payload) {
                println!("Failed to process message: {}", e);
                continue;
            }

            processed.push(message);
        }

        Ok(processed)
    }

    pub fn get_chain_id(&self) -> &str {
        &self.chain_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chain_adapter_basic() -> Result<()> {
        let adapter = ChainAdapter::new("chain1".to_string())?;

        let message = CrossChainMessage::new(
            "chain2".to_string(),
            "chain1".to_string(),
            1,
            vec![1, 2, 3],
            None,
            [0; 32],
        );

        adapter.queue_message(message).await?;
        
        let mut adapter = adapter;
        let processed = adapter.process_messages().await?;
        
        assert_eq!(processed.len(), 1);
        assert_eq!(processed[0].source_chain, "chain2");
        Ok(())
    }

    #[tokio::test]
    async fn test_state_verifier() {
        let mut verifier = StateVerifier::new("chain1".to_string());
        let state = [1; 32];
        
        assert!(verifier.verify_state("chain2", state).await);
        assert_eq!(verifier.get_verified_state("chain2").await.unwrap(), state);
    }
}