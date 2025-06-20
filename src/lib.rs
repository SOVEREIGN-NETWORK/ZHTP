pub mod blockchain;
pub mod browser;
pub mod contracts;
pub mod network;
pub mod storage;
pub mod zhtp;
pub mod discovery;
pub mod contract_manager;
pub mod security_tests; // Add security tests module

// Backward compatibility consensus module alias
pub mod consensus {
    pub use crate::zhtp::zk_consensus::{
        ZkNetworkMetrics, ConsensusState, ZkConsensusRound, ZkConsensus, ZkValidator, ZkBlock
    };
}

pub use blockchain::{Block, Blockchain, Transaction};
pub use zhtp::zk_consensus::{ZkConsensus, ZkValidator, ZkBlock, ZkNetworkMetrics, ConsensusState, ZkConsensusRound};
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
pub use zhtp::{
    Keypair, ZhtpNode, ZhtpPacket, SharedNode,
    bridge::{ChainAdapter, CrossChainMessage, StateVerifier},
    tunnel::{HttpsTunnel, TunnelMetrics, TunnelReward},
    economics::{ZhtpEconomics, EconomicMetrics, TokenSupply, RewardPool, FeeMarket},
};
pub use browser::ZhtpBrowser;
pub use contracts::ContractExecutor;

#[cfg(test)]
mod tests {
    use super::*;
}

#[cfg(test)]
pub mod integration_tests;
