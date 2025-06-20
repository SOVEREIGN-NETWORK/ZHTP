pub mod dht;
pub mod content;

pub use dht::{DhtNode, DhtNetwork};
pub use content::{ContentAddressing, ContentId, ContentMetadata};


/// Storage system configuration
pub struct StorageConfig {
    /// Number of replicas for each piece of data
    pub replication_factor: usize,
    /// Minimum number of proofs required for verification
    pub min_proofs: usize,
    /// Maximum storage per node (in bytes)
    pub max_node_storage: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            replication_factor: 3,
            min_proofs: 2,
            max_node_storage: 1024 * 1024 * 1024, // 1GB default
        }
    }
}
