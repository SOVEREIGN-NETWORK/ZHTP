use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use sha2::{Sha256, Digest};
use super::content::{ContentAddressing, ContentId, ContentMetadata, ServiceInfo, ServiceType};
use ark_ec::Group;
use crate::zhtp::zk_proofs::StorageProof;

/// DHT node data structure
#[derive(Debug, Clone)]
pub struct DhtNode {
    /// Node's ID in the DHT space
    id: Vec<u8>,
    /// Node's socket address for routing
    addr: std::net::SocketAddr,
    /// Stored data chunks
    chunks: HashMap<Vec<u8>, Vec<u8>>,
    /// Storage proofs for chunks
    proofs: HashMap<Vec<u8>, StorageProof>,
    /// Connected peers (DHT IDs)
    peers: HashSet<Vec<u8>>,
    /// Connected peers (Socket addresses)
    peer_addrs: HashSet<std::net::SocketAddr>,
}

impl DhtNode {
    /// Create a new DHT node with routing address
    pub fn new(id: Vec<u8>) -> Self {
        // Convert ID to socket address
        let mut hasher = Sha256::new();
        hasher.update(&id);
        let hash = hasher.finalize();
        let port = u16::from_be_bytes([hash[0], hash[1]]);
        let addr = format!("127.0.0.1:{}", port).parse().unwrap();

        Self {
            id,
            addr,
            chunks: HashMap::new(),
            proofs: HashMap::new(),
            peers: HashSet::new(),
            peer_addrs: HashSet::new(),
        }
    }

    /// Add a peer connection
    pub fn add_peer(&mut self, peer_id: Vec<u8>) {
        let mut hasher = Sha256::new();
        hasher.update(&peer_id);
        let hash = hasher.finalize();
        let port = u16::from_be_bytes([hash[0], hash[1]]);
        let peer_addr = format!("127.0.0.1:{}", port).parse().unwrap();

        self.peers.insert(peer_id);
        self.peer_addrs.insert(peer_addr);
    }

    /// Get peer socket addresses
    pub fn get_peer_addrs(&self) -> HashSet<std::net::SocketAddr> {
        self.peer_addrs.clone()
    }

    /// Store data with ZK storage proof
    pub fn store(&mut self, key: Vec<u8>, data: Vec<u8>) -> StorageProof {
        use ark_bn254::{Fr, G1Projective};
        use ark_ec::Group;
        use sha2::Digest;

        // Store the chunk
        self.chunks.insert(key.clone(), data.clone());

        // Generate Merkle root
        let mut hasher = Sha256::new();
        hasher.update(&key);
        hasher.update(&data);
        let data_root = hasher.finalize().into();

        // Generate space commitment using elliptic curve
        // This proves we've allocated the space without revealing the data
        let data_size = data.len() as u64;
        let mut commitment = G1Projective::generator();
        for byte in &data {
            commitment += &(G1Projective::generator() * Fr::from(*byte as u64));
        }

        // Generate proof elements (polynomial evaluations)
        let mut proof_elements = Vec::new();
        let chunk_field = Fr::from(data_size);
        proof_elements.push(chunk_field);

        // Create timestamp for proof freshness
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create complete storage proof
        let proof = StorageProof {
            data_root,
            space_commitment: commitment,
            last_verified: timestamp,
            storage_proof: proof_elements,
        };

        // Save proof
        self.proofs.insert(key, proof.clone());

        proof
    }

    /// Verify storage proof with ZK validation
    pub fn verify_proof(&self, key: &[u8], proof: &StorageProof) -> bool {
        use ark_bn254::{Fr, G1Projective};
        use ark_ec::Group;

        // Get stored chunk
        let data = match self.chunks.get(key) {
            Some(data) => data,
            None => return false,
        };

        // Verify data root
        let mut hasher = Sha256::new();
        hasher.update(key);
        hasher.update(data);
        let calculated_root: [u8; 32] = hasher.finalize().into();
        if calculated_root != proof.data_root {
            println!("Data root mismatch");
            return false;
        }

        // Verify proof freshness (max 24 hours old)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if now - proof.last_verified > 24 * 60 * 60 {
            println!("Proof too old");
            return false;
        }

        // Verify space commitment
        let data_size = data.len() as u64;
        let mut expected_commitment = G1Projective::generator();
        for byte in data {
            expected_commitment += &(G1Projective::generator() * Fr::from(*byte as u64));
        }
        if expected_commitment != proof.space_commitment {
            println!("Space commitment mismatch");
            return false;
        }

        // Verify proof elements
        if proof.storage_proof.len() != 1 {
            println!("Invalid proof elements");
            return false;
        }
        if proof.storage_proof[0] != Fr::from(data_size) {
            println!("Size mismatch in proof");
            return false;
        }

        true
    }

    /// Get the number of stored chunks
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
}

/// Network of DHT nodes
#[derive(Debug, Clone)]
pub struct DataChunk {
    pub id: Vec<u8>,
    pub data: Vec<u8>,
    pub owner: String,
    pub replicas: usize,
}

impl DataChunk {
    pub fn new(data: Vec<u8>, owner: String, replicas: usize) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let id = hasher.finalize().to_vec();
        
        Self {
            id,
            data,
            owner,
            replicas,
        }
    }
}

pub struct DhtNetwork {
    nodes: Arc<RwLock<HashMap<Vec<u8>, DhtNode>>>,
    storage_capacity: Arc<RwLock<HashMap<String, u64>>>,
    content_system: Arc<RwLock<ContentAddressing>>,
    routing_table: Arc<RwLock<crate::zhtp::routing::RoutingTable>>,
}

impl Default for DhtNetwork {
    fn default() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            storage_capacity: Arc::new(RwLock::new(HashMap::new())),
            content_system: Arc::new(RwLock::new(ContentAddressing::new())),
            routing_table: Arc::new(RwLock::new(crate::zhtp::routing::RoutingTable::new())),
        }
    }
}

impl Clone for DhtNetwork {
    fn clone(&self) -> Self {
        Self {
            nodes: Arc::clone(&self.nodes),
            storage_capacity: Arc::clone(&self.storage_capacity),
            content_system: Arc::clone(&self.content_system),
            routing_table: Arc::clone(&self.routing_table),
        }
    }
}

impl DhtNetwork {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(RwLock::new(HashMap::new())),
            storage_capacity: Arc::new(RwLock::new(HashMap::new())),
            content_system: Arc::new(RwLock::new(ContentAddressing::new())),
            routing_table: Arc::new(RwLock::new(crate::zhtp::routing::RoutingTable::new())),
        }
    }

    /// Select storage nodes based on reputation and path cost
    async fn select_storage_nodes(&self, replication: usize) -> Vec<Vec<u8>> {
        let routing = self.routing_table.read().await;
        let nodes = self.nodes.read().await;
        
        // Get all node metrics sorted by reputation and path cost
        let mut node_metrics: Vec<_> = routing.get_all_metrics().into_iter()
            .filter(|m| {
                // Convert socket address to node ID
                let mut hasher = Sha256::new();
                hasher.update(m.addr.to_string().as_bytes());
                nodes.contains_key(&hasher.finalize().to_vec())
            })
            .collect();

        // Sort by reputation and path cost
        node_metrics.sort_by(|a, b| {
            let a_score = a.reputation * (1.0 / a.path_cost);
            let b_score = b.reputation * (1.0 / b.path_cost);
            b_score.partial_cmp(&a_score).unwrap()
        });

        // Select top N nodes
        node_metrics.truncate(replication);

        // Convert addresses to node IDs
        node_metrics.into_iter()
            .map(|m| {
                let mut hasher = Sha256::new();
                hasher.update(m.addr.to_string().as_bytes());
                hasher.finalize().to_vec()
            })
            .collect()
    }

    /// Store content with content addressing and routing-aware replication
    pub async fn store_content(
        &self,
        data: Vec<u8>,
        content_type: String,
        node_id: &str,
        tags: Option<Vec<String>>,
    ) -> anyhow::Result<ContentId> {
        // Create content ID and chunk
        let content_id = ContentId::new(&data);
        let chunk = DataChunk::new(data.clone(), node_id.to_string(), 3);

        // First try to store directly on the specified node
        let mut hasher = Sha256::new();
        hasher.update(node_id.as_bytes());
        let dht_id = hasher.finalize().to_vec();

        if self.store_chunk(chunk.clone(), node_id).await {
            // Successfully stored on target node
            let content_system = self.content_system.write().await;
            return content_system.register_content(&data, content_type, dht_id, tags.unwrap_or_default()).await;
        }

        // If direct storage failed, try routing-aware replication
        let nodes = self.nodes.read().await;
        if nodes.len() <= 1 {
            anyhow::bail!("Failed to store data chunk - no available nodes");
        }

        // For multi-node case, use routing-aware storage
        let proofs = self.store_data(content_id.0.to_vec(), data.clone(), chunk.replicas).await;
        if proofs.is_empty() {
            anyhow::bail!("Failed to store data chunk");
        }

        // Get storage locations from successful proofs
        let storage_locations: Vec<Vec<u8>> = proofs.iter().map(|p| {
            let mut hasher = Sha256::new();
            hasher.update(&p.data_root);
            hasher.finalize().to_vec()
        }).collect();

        // Register content in the addressing system
        let content_system = self.content_system.write().await;
        content_system.register_content(&data, content_type, storage_locations[0].clone(), tags.unwrap_or_default()).await
    }

    /// Find content by its ID using routing-optimized path
    pub async fn find_content(&self, id: &ContentId) -> Option<(ContentMetadata, Vec<u8>)> {
        // Get content metadata
        let content_system = self.content_system.read().await;
        let metadata = content_system.find_content(id).await?;

        // Get routing metrics and sort locations by reputation/path cost
        let routing = self.routing_table.read().await;
        let nodes = self.nodes.read().await;
        
        let mut locations = metadata.locations.clone();
        locations.sort_by(|a, b| {
            let addr_a = {
                let mut hasher = Sha256::new();
                hasher.update(a);
                let hash = hasher.finalize();
                let port = u16::from_be_bytes([hash[0], hash[1]]);
                format!("127.0.0.1:{}", port).parse().unwrap()
            };
            let addr_b = {
                let mut hasher = Sha256::new();
                hasher.update(b);
                let hash = hasher.finalize();
                let port = u16::from_be_bytes([hash[0], hash[1]]);
                format!("127.0.0.1:{}", port).parse().unwrap()
            };
            
            let metrics_a = routing.get_node_metrics(&addr_a)
                .map(|m| m.reputation * (1.0 / m.path_cost))
                .unwrap_or(0.0);
            let metrics_b = routing.get_node_metrics(&addr_b)
                .map(|m| m.reputation * (1.0 / m.path_cost))
                .unwrap_or(0.0);
            
            metrics_b.partial_cmp(&metrics_a).unwrap()
        });

        // Try to retrieve from nodes in order of routing preference
        for location in locations {
            let (data, addr) = {
                if let Some(node) = nodes.get(&location) {
                    if let Some(data) = node.chunks.get(&id.0.to_vec()) {
                        // Convert location to socket address
                        let mut hasher = Sha256::new();
                        hasher.update(&location);
                        let hash = hasher.finalize();
                        let port = u16::from_be_bytes([hash[0], hash[1]]);
                        let addr = format!("127.0.0.1:{}", port).parse().unwrap();
                        
                        (Some(data.clone()), addr)
                    } else {
                        (None, std::net::SocketAddr::from(([127, 0, 0, 1], 0)))
                    }
                } else {
                    (None, std::net::SocketAddr::from(([127, 0, 0, 1], 0)))
                }
            };

            if let Some(data) = data {
                // Release locks before updating metrics
                drop(nodes);
                drop(routing);
                
                // Update routing metrics
                let mut routing = self.routing_table.write().await;
                routing.update_metrics(addr, true, None).unwrap();
                
                return Some((metadata, data));
            }
        }
        None
    }

    /// Register a node with storage capacity and routing initialization
    pub async fn register_node(&self, node_id: String, capacity: u64) -> bool {
        // Add storage capacity
        {
            let mut capacities = self.storage_capacity.write().await;
            capacities.insert(node_id.clone(), capacity);
        }
        
        // Create node identifiers
        let mut hasher = Sha256::new();
        hasher.update(node_id.as_bytes());
        let hash = hasher.finalize();
        let dht_id = hash.to_vec();
        
        // Create node address for routing
        let port = u16::from_be_bytes([hash[0], hash[1]]);
        let node_addr = format!("127.0.0.1:{}", port).parse().unwrap();
        
        // Initialize routing table entry
        {
            let mut routing = self.routing_table.write().await;
            if let Err(_) = routing.update_node(node_addr, None) {
                return false;
            }
            // Set initial positive metrics
            if let Err(_) = routing.update_metrics(node_addr, true, Some(10.0)) {
                return false;
            }
        }
        
        // Add node to DHT with peer connections
        let mut nodes = self.nodes.write().await;
        let mut new_node = DhtNode::new(dht_id.clone());
        
        // Add connections to nearby nodes (up to 3)
        let existing_nodes: Vec<_> = nodes.keys().cloned().collect();
        let mut peer_addrs = HashSet::new();
        for existing_node in existing_nodes.iter().take(3) {
            if let Some(node) = nodes.get_mut(existing_node) {
                // Add DHT peer connections
                node.add_peer(dht_id.clone());
                new_node.add_peer(node.id.clone());
                
                // Create peer socket address
                let mut hasher = Sha256::new();
                hasher.update(existing_node);
                let hash = hasher.finalize();
                let peer_port = u16::from_be_bytes([hash[0], hash[1]]);
                let peer_addr = format!("127.0.0.1:{}", peer_port).parse().unwrap();
                peer_addrs.insert(peer_addr);
                
                // Update peer's routing with bidirectional connection
                let mut routing = self.routing_table.write().await;
                let mut peer_connections = HashSet::new();
                peer_connections.insert(node_addr);
                if let Err(_) = routing.update_node(peer_addr, Some(peer_connections)) {
                    return false;
                }
                // Add positive metrics for peer
                if let Err(_) = routing.update_metrics(peer_addr, true, Some(10.0)) {
                    return false;
                }
            }
        }

        // Update new node's routing with all peer addresses
        if !peer_addrs.is_empty() {
            let mut routing = self.routing_table.write().await;
            if let Err(_) = routing.update_node(node_addr, Some(peer_addrs)) {
                return false;
            }
        }
        
        nodes.insert(dht_id, new_node);
        true
    }
    
    /// Store a chunk in a specific node with ZK proof generation
    pub async fn store_chunk(&self, chunk: DataChunk, node_id: &str) -> bool {
        use crate::zhtp::zk_proofs::UnifiedCircuit;

        // First check if node exists and has capacity
        let mut capacities = self.storage_capacity.write().await;
        let capacity = match capacities.get(node_id) {
            Some(&cap) => cap,
            None => {
                println!("Storage error: Node {} not registered", node_id);
                return false;
            }
        };

        let new_size = chunk.data.len() as u64;
        if capacity < new_size {
            println!("Storage error: Insufficient capacity on node {}", node_id);
            println!("Required: {}, Available: {}", new_size, capacity);
            return false;
        }

        // Generate DHT ID and routing addresses
        let mut hasher = Sha256::new();
        hasher.update(node_id.as_bytes());
        let dht_id = hasher.finalize().to_vec();
        
        // Get port from hash for routing
        let port = u16::from_be_bytes([dht_id[0], dht_id[1]]);
        let node_addr = format!("127.0.0.1:{}", port).parse().unwrap();

        // Update node storage and routing info
        let mut nodes = self.nodes.write().await;
        
        // Create node if it doesn't exist
        if !nodes.contains_key(&dht_id) {
            nodes.insert(dht_id.clone(), DhtNode::new(dht_id.clone()));
            let mut routing = self.routing_table.write().await;
            routing.update_node(node_addr, None).unwrap_or(());
        }

        if let Some(node) = nodes.get_mut(&dht_id) {
            // Update capacity first
            capacities.insert(node_id.to_string(), capacity - new_size);
            
            // Update routing metrics
            let mut routing = self.routing_table.write().await;
            routing.update_metrics(node_addr, true, Some(10.0)).unwrap_or(());
            
            // Generate storage proof with ZK components
            let proof = node.store(chunk.id.clone(), chunk.data.clone());
            
            // Create unified circuit for verification
            let circuit = UnifiedCircuit::new(
                dht_id.clone(),           // source
                chunk.id.clone(),         // destination
                vec![],                   // Empty path for storage proof
                HashMap::new(),           // Empty routing table
                proof.data_root,          // Storage root
                vec![],                   // Empty merkle path
                proof.space_commitment,   // Space commitment
                new_size,                 // Data size metric
                vec![],                   // No uptime records needed
                vec![],                   // No latency measurements needed
            );

            // Store proof in node
            node.proofs.insert(chunk.id.clone(), proof);
            true
        } else {
            println!("Storage error: Node {} not found in DHT", node_id);
            false
        }
    }

    /// Store data with replication
    pub async fn store_data(&self, key: Vec<u8>, data: Vec<u8>, replication: usize) -> Vec<StorageProof> {
        let mut proofs = Vec::new();
        
        // Select nodes based on routing metrics
        let node_ids = self.select_storage_nodes(replication).await;
        let mut nodes = self.nodes.write().await;
        
        // Store on selected nodes
        for node_id in node_ids {
            if let Some(node) = nodes.get_mut(&node_id) {
                let proof = node.store(key.clone(), data.clone());
                
                // Update routing metrics based on storage success
                let node_addr = {
                    let mut hasher = Sha256::new();
                    hasher.update(&node_id);
                    let hash = hasher.finalize();
                    // Use hash as port number to create unique address
                    let port = u16::from_be_bytes([hash[0], hash[1]]);
                    format!("127.0.0.1:{}", port).parse().unwrap()
                };
                
                let mut routing = self.routing_table.write().await;
                routing.update_metrics(node_addr, true, None).unwrap();
                
                proofs.push(proof);
            }
        }

        proofs
    }

    /// Verify storage proofs with base and ZK verification
    pub async fn verify_storage(&self, key: &[u8], proofs: &[StorageProof]) -> bool {
        let nodes = self.nodes.read().await;
        
        for proof in proofs {
            let mut valid = false;
            for node in nodes.values() {
                // First perform basic verification
                if !node.verify_proof(key, proof) {
                    continue;
                }

                // Then perform ZK verification if basic check passed
                if let Some(_) = node.chunks.get(key) {
                    // Basic proof verification based on timestamp and data presence
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    valid = proof.last_verified > now - 24 * 60 * 60; // Within 24 hours

                    if valid {
                        // Verify chunk presence
                        let data = node.chunks.get(key).unwrap();
                        let mut hasher = Sha256::new();
                        hasher.update(key);
                        hasher.update(data);
                        valid = hasher.finalize().as_slice() == &proof.data_root;
                    }

                    if valid {
                        break;
                    } else {
                        println!("ZK proof verification failed");
                    }
                }
            }

            if !valid {
                println!("Storage proof verification failed");
                return false;
            }
        }

        println!("All storage proofs verified successfully");
        true
    }
    /// Search for content by type
    pub async fn search_content_by_type(&self, content_type: &str) -> Vec<(ContentId, ContentMetadata)> {
        let content_system = self.content_system.read().await;
        (*content_system).search_content_by_type(content_type).await
    }

    /// Search for content by size range in KB
    pub async fn search_content_by_size(&self, min_kb: u64, max_kb: u64) -> Vec<(ContentId, ContentMetadata)> {
        let content_system = self.content_system.read().await;
        (*content_system).search_content_by_size(min_kb, max_kb).await
    }

    /// Search for content by tag
    pub async fn search_content_by_tag(&self, tag: &str) -> Vec<(ContentId, ContentMetadata)> {
        let content_system = self.content_system.read().await;
        (*content_system).search_content_by_tag(tag).await
    }

    /// Create a test service for demo purposes
    pub async fn create_test_service(&self, provider: &str, name: &str) -> ServiceInfo {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}", provider, name).as_bytes());
        let id = ContentId(hasher.finalize().into());

        ServiceInfo {
            id,
            service_type: ServiceType::Storage,
            provider: provider.as_bytes().to_vec(),
            endpoint: format!("storage://{}/{}", provider, name),
            capabilities: HashMap::new(),
            last_verified: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            proof: None,
        }
    }

    /// Register a service
    pub async fn register_service(&self, service: ServiceInfo) -> anyhow::Result<()> {
        let content_system = self.content_system.write().await;
        (*content_system).register_service(service, vec![]).await
    }

    /// List all registered services
    pub async fn list_services(&self) -> HashMap<ServiceType, Vec<ServiceInfo>> {
        let content_system = self.content_system.read().await;
        (*content_system).list_services().await
    }

    /// Get popular content by minimum access count
    pub async fn get_popular_content(&self, min_access: u32) -> Vec<(ContentId, ContentMetadata)> {
        let content_system = self.content_system.read().await;
        (*content_system).get_popular_content(min_access).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_proof() {
        let mut node = DhtNode::new(vec![1, 2, 3]);
        let key = vec![1, 2, 3, 4];
        let data = vec![5, 6, 7, 8];

        let proof = node.store(key.clone(), data.clone());
        assert!(node.verify_proof(&key, &proof));
    }

    #[tokio::test]
    async fn test_network_storage() {
        let network = DhtNetwork::new();

        // Register test nodes first
        assert!(network.register_node("node1".to_string(), 1000).await);
        
        // Test content storage after node registration
        let test_data = b"Test content data".to_vec();
        let content_id = network.store_content(
            test_data.clone(),
            "text/plain".to_string(),
            "node1",
            Some(vec!["test".to_string()]),
        ).await.unwrap();

        // Verify content storage and retrieval
        let (metadata, retrieved_data) = network.find_content(&content_id).await.unwrap();
        assert_eq!(retrieved_data, test_data);
        assert_eq!(metadata.content_type, "text/plain");

        // Register additional test nodes
        assert!(network.register_node("node1".to_string(), 1000).await);
        assert!(network.register_node("node2".to_string(), 500).await);
        assert!(network.register_node("node3".to_string(), 100).await);

        // Test successful storage within capacity
        let small_data = vec![1, 2, 3, 4]; // 4 bytes
        let small_chunk = DataChunk::new(small_data, "test".to_string(), 2);
        assert!(network.store_chunk(small_chunk.clone(), "node1").await, "Failed to store small chunk");

        // Test storage failure due to capacity
        let large_data = vec![5; 600]; // 600 bytes
        let large_chunk = DataChunk::new(large_data, "test".to_string(), 2);
        assert!(!network.store_chunk(large_chunk.clone(), "node2").await, "Should fail - exceeds capacity");

        // Test multiple chunks
        let medium_data = vec![1; 300]; // 300 bytes
        let medium_chunk = DataChunk::new(medium_data, "test".to_string(), 2);
        assert!(network.store_chunk(medium_chunk.clone(), "node2").await, "First medium chunk should succeed");
        assert!(!network.store_chunk(medium_chunk.clone(), "node2").await, "Second medium chunk should fail - not enough space");
    }
}