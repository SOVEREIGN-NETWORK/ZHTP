//! Security test suite for ZHTP protocol
//! Tests for all identified vulnerabilities and attack vectors

#[cfg(test)]
mod security_tests {
    use crate::{
        Blockchain, Transaction,
        discovery::{DiscoveryNode, ContentIndex},
        storage::dht::DhtNetwork,
        storage::content::{ContentId, ContentMetadata},
    };
    use std::net::SocketAddr;
    use anyhow::Result;

    #[tokio::test]
    async fn test_signature_verification_attack_prevention() -> Result<()> {
        let blockchain = Blockchain::new(100.0);
        
        // Create a valid transaction
        let mut tx = Transaction::new("alice".to_string(), "bob".to_string(), 50.0);
        tx.sign("alice_private_key");
        
        // Try to verify with wrong public key - should fail
        assert!(!tx.verify_signature("mallory_public_key"));
        
        // Try to verify with correct "public key" but wrong format - should fail
        let mut malicious_tx = Transaction::new("alice".to_string(), "bob".to_string(), 1000.0);
        malicious_tx.signature = "alice_private_key:fake_hash".to_string();
        assert!(!malicious_tx.verify_signature("alice_private_key"));
        
        // Valid verification should work
        assert!(tx.verify_signature("alice_private_key"));
        
        Ok(())
    }

    #[tokio::test]
    async fn test_find_nodes_prefix_attack_prevention() -> Result<()> {
        let mut discovery = DiscoveryNode::new("127.0.0.1:8000".parse()?)?;
        discovery.start().await?;
        
        // Register legitimate nodes
        discovery.register_node("127.0.0.1:8001".parse()?, "node1".to_string()).await?;
        discovery.register_node("127.0.0.1:8002".parse()?, "node2".to_string()).await?;
        discovery.register_node("127.0.0.1:8003".parse()?, "node123".to_string()).await?;
        
        // Try malicious input - should fail
        assert!(discovery.find_nodes("../../../etc/passwd".to_string()).await.is_err());
        assert!(discovery.find_nodes("node'; DROP TABLE nodes;--".to_string()).await.is_err());
        assert!(discovery.find_nodes("".to_string()).await.is_err());
        assert!(discovery.find_nodes("x".repeat(100)).await.is_err());
        
        // Valid prefix search should work
        let results = discovery.find_nodes("node".to_string()).await?;
        assert_eq!(results.len(), 3);
        
        // Specific prefix should return subset
        let results = discovery.find_nodes("node1".to_string()).await?;
        assert_eq!(results.len(), 2); // node1 and node123
        
        Ok(())
    }

    #[tokio::test]
    async fn test_nonce_replay_attack_prevention() -> Result<()> {
        let blockchain = Blockchain::new(100.0);
        
        // Add initial balance
        let mut genesis_tx = Transaction::new("network".to_string(), "alice".to_string(), 1000.0);
        genesis_tx.sign("network");
        assert!(blockchain.add_transaction(genesis_tx).await);
        blockchain.create_block("genesis", 1.0, None).await;
        
        // Create transaction with specific nonce
        let mut tx1 = Transaction::new("alice".to_string(), "bob".to_string(), 50.0);
        tx1.nonce = 0; // First transaction should have nonce 0
        tx1.sign("alice");
        assert!(blockchain.add_transaction(tx1).await);
        
        // Try to replay the same nonce - should fail
        let mut tx2 = Transaction::new("alice".to_string(), "bob".to_string(), 100.0);
        tx2.nonce = 0; // Same nonce as before
        tx2.sign("alice");
        assert!(!blockchain.add_transaction(tx2).await);
        
        // Valid next nonce should work
        let mut tx3 = Transaction::new("alice".to_string(), "bob".to_string(), 25.0);
        tx3.nonce = 1; // Correct next nonce
        tx3.sign("alice");
        assert!(blockchain.add_transaction(tx3).await);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_node_registration_validation() -> Result<()> {
        let mut discovery = DiscoveryNode::new("127.0.0.1:8000".parse()?)?;
        discovery.start().await?;
        
        let addr: SocketAddr = "127.0.0.1:8001".parse()?;
        
        // Invalid node names should fail
        assert!(discovery.register_node(addr, "".to_string()).await.is_err());
        assert!(discovery.register_node(addr, "x".repeat(100)).await.is_err());
        assert!(discovery.register_node(addr, "node with spaces".to_string()).await.is_err());
        assert!(discovery.register_node(addr, "node$pecial".to_string()).await.is_err());
        
        // Valid node name should work
        assert!(discovery.register_node(addr, "valid-node_1".to_string()).await.is_ok());
        
        // Duplicate name from different address should fail
        let addr2: SocketAddr = "127.0.0.1:8002".parse()?;
        assert!(discovery.register_node(addr2, "valid-node_1".to_string()).await.is_err());
        
        Ok(())
    }

    #[tokio::test] 
    async fn test_storage_node_registration_security() -> Result<()> {
        let dht = DhtNetwork::new();
        
        // Invalid node IDs should fail
        assert!(!dht.register_node("".to_string(), 1000).await);
        assert!(!dht.register_node("x".repeat(100), 1000).await);
        assert!(!dht.register_node("node with spaces".to_string(), 1000).await);
        assert!(!dht.register_node("node$pecial".to_string(), 1000).await);
        
        // Invalid capacity should fail
        assert!(!dht.register_node("valid-node".to_string(), 0).await);
        assert!(!dht.register_node("valid-node".to_string(), u64::MAX).await);
        
        // Valid registration should work
        assert!(dht.register_node("valid-node_1".to_string(), 1000).await);
        
        // Duplicate registration should fail
        assert!(!dht.register_node("valid-node_1".to_string(), 2000).await);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_cross_chain_replay_protection() -> Result<()> {
        use crate::zhtp::bridge::{ChainAdapter, CrossChainMessage};
        
        let adapter = ChainAdapter::new("chain1".to_string())?;
        
        // Create test messages with same nonce
        let wasm = wat::parse_str("(module)").unwrap();
        
        let msg1 = CrossChainMessage::new(
            "chain2".to_string(),
            "chain1".to_string(),
            1, // nonce 1
            wasm.clone(),
            None,
            [0; 32],
        );
        
        let msg2 = CrossChainMessage::new(
            "chain2".to_string(),
            "chain1".to_string(),
            1, // same nonce - should be rejected
            wasm.clone(),
            None,
            [0; 32],
        );
        
        let msg3 = CrossChainMessage::new(
            "chain2".to_string(),
            "chain1".to_string(),
            2, // correct next nonce
            wasm,
            None,
            [0; 32],
        );
        
        // Queue messages
        adapter.queue_message(msg1).await?;
        adapter.queue_message(msg2).await?;
        adapter.queue_message(msg3).await?;
        
        // Process messages
        let mut adapter = adapter;
        let processed = adapter.process_messages().await?;
        
        // Only first and third messages should be processed
        assert_eq!(processed.len(), 2);
        assert_eq!(processed[0].nonce, 1);
        assert_eq!(processed[1].nonce, 2);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_content_indexing_rate_limiting() -> Result<()> {
        use crate::storage::content::{ContentId, ContentMetadata};
        use crate::discovery::ContentIndex;
          let index = ContentIndex::new();
        let content_id = ContentId::new(b"test_content");
        let metadata = ContentMetadata {
            id: content_id.clone(),
            content_type: "text/plain".to_string(),
            size: 1024,
            locations: vec![],
            last_verified: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            tags: vec!["test".to_string()],
        };
          // Index the same content many times rapidly to trigger rate limiting
        let mut success_count = 0;
        for _i in 0..150 {
            // Use the same content ID to trigger rate limiting
            if index.index_content(content_id.clone(), &metadata).await.is_ok() {
                success_count += 1;
            }
        }
        
        // Should be rate limited after 100 operations
        assert!(success_count <= 100);
        
        Ok(())
    }    #[test]
    fn test_input_sanitization() {
        // Test various malicious inputs
        let long_string = "very_long_string".repeat(1000);
        let malicious_inputs: Vec<&str> = vec![
            "../../../etc/passwd",
            "'; DROP TABLE users;--",
            "<script>alert('xss')</script>",
            "\0\0\0\0",
            &long_string,
            "unicode_\u{202e}attack",
        ];        for input in malicious_inputs {
            // Test against our validation function
            let is_valid = crate::security_tests::security_utils::validate_node_id(input);
            if is_valid {
                panic!("Input '{}' should be rejected but was accepted", input);
            }
        }
    }
}

/// Additional security utilities
pub mod security_utils {
    use sha2::{Sha256, Digest};
    
    /// Secure input validation for node identifiers
    pub fn validate_node_id(id: &str) -> bool {
        !id.is_empty() && 
        id.len() <= 64 && 
        id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    }
    
    /// Generate secure node hash with collision resistance
    pub fn generate_secure_node_hash(input: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hasher.update(b"ZHTP_NODE_SALT"); // Add salt to prevent rainbow table attacks
        hasher.finalize().into()
    }
    
    /// Validate transaction nonce sequence
    pub fn validate_nonce_sequence(current: u64, expected: u64) -> bool {
        current == expected
    }
    
    /// Rate limiting implementation
    pub struct RateLimiter {
        requests: std::collections::HashMap<String, (u32, std::time::Instant)>,
        max_requests: u32,
        window: std::time::Duration,
    }
    
    impl RateLimiter {
        pub fn new(max_requests: u32, window: std::time::Duration) -> Self {
            Self {
                requests: std::collections::HashMap::new(),
                max_requests,
                window,
            }
        }
        
        pub fn check_rate_limit(&mut self, key: &str) -> bool {
            let now = std::time::Instant::now();
            let (count, last_reset) = self.requests.entry(key.to_string()).or_insert((0, now));
            
            if now.duration_since(*last_reset) >= self.window {
                *count = 0;
                *last_reset = now;
            }
            
            if *count >= self.max_requests {
                return false;
            }
            
            *count += 1;
            true
        }
    }
}
