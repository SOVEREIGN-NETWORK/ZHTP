pub mod blockchain;
pub mod browser;
pub mod consensus;
pub mod contracts;
pub mod network;
pub mod storage;
pub mod zhtp;
pub mod discovery;

pub use blockchain::{Block, Blockchain, Transaction};
pub use consensus::{ConsensusManager, NetworkMetrics, ConsensusRound};
pub use network::{Network, NetworkCondition, NetworkId, Node, Packet};
pub use storage::{
    dht::{DhtNode, DhtNetwork as StorageManager},
    StorageConfig,
    ContentMetadata,
    ContentId,
};
// Re-export key types
pub use std::sync::Arc;
pub use tokio::sync::Mutex;

// Re-export key components
pub use zhtp::{Keypair, ZhtpNode, ZhtpPacket, SharedNode};
pub use browser::ZhtpBrowser;
pub use contracts::ContractExecutor;

#[cfg(test)]
mod tests {
    use super::*;
}
