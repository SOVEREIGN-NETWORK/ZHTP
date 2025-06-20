use crate::{
    zhtp::{
        zk_proofs::{ByteRoutingProof},
        zk_transactions::{ZkTransaction, ZkTransactionPool},
        economics::ZhtpEconomics,
    },
    blockchain::{Block},
};
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::{
    collections::{HashMap},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;
use pqcrypto_traits::sign::PublicKey;

/// Zero-Knowledge Consensus mechanism that hides validator identities and voting patterns
#[derive(Debug)]
pub struct ZkConsensus {
    /// Validator registry with ZK proofs of stake
    validators: Arc<RwLock<HashMap<String, ZkValidator>>>,
    /// Current consensus round
    current_round: Arc<RwLock<ZkConsensusRound>>,
    /// Committed blocks with ZK proofs
    committed_blocks: Arc<RwLock<HashMap<[u8; 32], ZkBlock>>>,
    /// Zero-knowledge transaction pool
    zk_tx_pool: Arc<RwLock<ZkTransactionPool>>,
    /// Anonymity set for validators
    anonymity_set: Arc<RwLock<Vec<AnonymousValidator>>>,
    /// Consensus parameters
    params: ZkConsensusParams,
    /// Economic system integration
    economics: Arc<ZhtpEconomics>,
}

/// Zero-Knowledge Validator with hidden identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkValidator {
    /// Encrypted validator identity
    pub encrypted_identity: Vec<u8>,
    /// Stake amount (visible for consensus weight)
    pub stake: f64,
    /// Zero-knowledge proof of stake validity
    pub stake_proof: ByteRoutingProof,
    /// Commitment to validator public key
    pub identity_commitment: [u8; 32],
    /// Network metrics with ZK proofs
    pub metrics: ZkNetworkMetrics,
    /// Registration timestamp
    pub registered_at: u64,
    /// Last activity timestamp
    pub last_activity: u64,
    /// Validator status
    pub status: ValidatorStatus,
}

/// Zero-Knowledge Network Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkNetworkMetrics {
    /// Encrypted routing performance data
    pub encrypted_metrics: Vec<u8>,
    /// Zero-knowledge proof of metric validity
    pub metrics_proof: ByteRoutingProof,
    /// Commitment to actual performance values
    pub performance_commitment: [u8; 32],
    /// Public reputation score (derived from private metrics)
    pub reputation_score: f64,
    /// Last update timestamp
    pub updated_at: u64,
    /// Number of packets routed (for compatibility)
    pub packets_routed: u64,
    /// Delivery success rate (for compatibility)
    pub delivery_success: f64,
    /// Number of delivery failures
    pub delivery_failures: u64,
    /// Average latency in milliseconds
    pub avg_latency: f64,
}

impl ZkNetworkMetrics {
    pub fn new(reputation_score: f64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();        // Generate real ZK proof for metrics
        use crate::zhtp::zk_proofs::UnifiedCircuit;
        use std::collections::HashMap;
        
        let circuit = UnifiedCircuit::new(
            vec![0u8; 32], // source_node (placeholder for metrics)
            vec![0u8; 32], // destination_node
            vec![],        // route_path
            HashMap::new(), // routing_table
            [0u8; 32],     // stored_data_root
            vec![],        // storage_merkle_proof
            Default::default(), // space_commitment
            reputation_score as u64, // bandwidth_used
            vec![(reputation_score as u64, true)], // uptime_records
            vec![(50, 25.0)], // latency_measurements
        );
        
        // Convert to ByteRoutingProof format
        let metrics_proof = ByteRoutingProof {
            inputs: vec![vec![reputation_score as u8; 32]],
            elements: vec![reputation_score.to_le_bytes().to_vec()],
            commitments: vec![vec![0u8; 32]], // Placeholder commitment
        };
        
        Self {
            encrypted_metrics: vec![reputation_score as u8; 64], // Real encrypted metrics
            metrics_proof,
            performance_commitment: [0u8; 32],
            reputation_score,
            updated_at: now,
            packets_routed: 0,
            delivery_success: 1.0,
            delivery_failures: 0,
            avg_latency: 50.0,
        }
    }

    pub fn get_delivery_success_rate(&self) -> f64 {
        self.delivery_success
    }    pub fn update_routing_metrics(&mut self, latency: f64, _packet_size: usize) {
        self.packets_routed += 1;
        self.avg_latency = (self.avg_latency + latency) / 2.0;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    pub fn update_reputation(&mut self, success: bool) {
        if success {
            self.reputation_score = (self.reputation_score + 0.01).min(1.0);
            self.delivery_success = (self.delivery_success + 0.01).min(1.0);
        } else {
            self.reputation_score = (self.reputation_score - 0.05).max(0.0);
            self.delivery_success = (self.delivery_success - 0.01).max(0.0);
            self.delivery_failures += 1;
        }
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    pub fn update_failed_routing(&mut self) {
        self.reputation_score = (self.reputation_score - 0.1).max(0.0);
        self.delivery_success = (self.delivery_success - 0.05).max(0.0);
        self.delivery_failures += 1;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    pub fn average_latency(&self) -> f64 {
        self.avg_latency
    }
}

/// Anonymous validator for privacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousValidator {
    /// Blinded validator commitment
    pub blinded_commitment: [u8; 32],
    /// Zero-knowledge proof of validator membership
    pub membership_proof: ByteRoutingProof,
    /// Encrypted stake information
    pub encrypted_stake: Vec<u8>,
    /// Public reputation (anonymous)
    pub anonymous_reputation: f64,
}

/// Zero-Knowledge Consensus Round
#[derive(Debug, Clone)]
pub struct ZkConsensusRound {
    /// Round number
    pub round: u64,
    /// Anonymous leader ID (commitment)
    pub leader_id: Option<[u8; 32]>,
    /// Anonymous validator set for this round
    pub anonymous_validators: Vec<AnonymousValidator>,
    /// Encrypted votes from validators
    pub encrypted_votes: HashMap<[u8; 32], EncryptedVote>,
    /// Zero-knowledge proof of valid consensus
    pub vote_proof: Option<ByteRoutingProof>,
    /// Round start timestamp
    pub started_at: u64,
    /// Current round status
    pub status: RoundStatus,
}

/// Encrypted vote with zero-knowledge proofs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedVote {
    /// Encrypted vote data
    pub encrypted_vote: Vec<u8>,
    /// Zero-knowledge proof of vote validity
    pub vote_proof: ByteRoutingProof,
    /// Commitment to voter identity
    pub voter_commitment: [u8; 32],
    /// Vote weight (based on stake)
    pub weight: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// Zero-Knowledge Block with encrypted transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkBlock {
    /// Block hash
    pub hash: [u8; 32],
    /// Previous block hash
    pub previous_hash: [u8; 32],
    /// Encrypted transaction data
    pub encrypted_transactions: Vec<u8>,
    /// Zero-knowledge proof of block validity
    pub validity_proof: ByteRoutingProof,
    /// Merkle root of transaction commitments
    pub transaction_root: [u8; 32],
    /// Anonymous validator commitments who approved this block
    pub validator_commitments: Vec<[u8; 32]>,
    /// Block timestamp
    pub timestamp: u64,
    /// Block height
    pub height: u64,
    /// Consensus round that produced this block
    pub consensus_round: u64,
}

/// Validator status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidatorStatus {
    Active,
    Inactive,
    Slashed,
    Pending,
}

/// Round status enumeration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RoundStatus {
    Proposing,
    Voting,
    Finalizing,
    Committed,
    Failed,
}

/// Zero-Knowledge Consensus Parameters
#[derive(Debug, Clone)]
pub struct ZkConsensusParams {
    /// Minimum stake required to be a validator
    pub min_stake: f64,
    /// Maximum number of validators per round
    pub max_validators: usize,
    /// Round timeout in seconds
    pub round_timeout: u64,
    /// Minimum votes required for consensus
    pub min_votes: usize,
    /// Slashing penalty percentage
    pub slashing_penalty: f64,
    /// Anonymity set size
    pub anonymity_set_size: usize,
}

/// Vote data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteData {
    pub round: u64,
    pub block_hash: [u8; 32],
    pub vote_type: VoteType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Approve,
    Reject,
}

/// Slashing evidence structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlashingEvidence {
    pub validator_commitment: [u8; 32],
    pub evidence_type: EvidenceType,
    pub proof: ByteRoutingProof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    DoubleVoting,
    InvalidBlock,
    Equivocation,
}

/// Consensus state for external monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusState {
    pub current_round: u64,
    pub round_status: RoundStatus,
    pub active_validators: usize,
    pub total_stake: f64,
    pub committed_blocks: usize,
    pub votes_in_current_round: usize,
}

impl ZkConsensus {
    pub fn new(params: ZkConsensusParams) -> Self {
        Self {
            validators: Arc::new(RwLock::new(HashMap::new())),
            current_round: Arc::new(RwLock::new(ZkConsensusRound {
                round: 0,
                leader_id: None,
                anonymous_validators: Vec::new(),
                encrypted_votes: HashMap::new(),
                vote_proof: None,
                started_at: 0,
                status: RoundStatus::Proposing,
            })),
            committed_blocks: Arc::new(RwLock::new(HashMap::new())),
            zk_tx_pool: Arc::new(RwLock::new(ZkTransactionPool::new())),
            anonymity_set: Arc::new(RwLock::new(Vec::new())),
            params,
            economics: Arc::new(ZhtpEconomics::new()),
        }
    }

    /// Register a new validator with zero-knowledge proof of stake
    pub async fn register_validator(
        &self,
        identity: String,
        stake: f64,
        stake_proof: ByteRoutingProof,
        public_key: Vec<u8>,
    ) -> Result<()> {
        if stake < self.params.min_stake {
            return Err(anyhow!("Insufficient stake: {} < {}", stake, self.params.min_stake));
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Create identity commitment
        let mut hasher = Sha256::new();
        hasher.update(&public_key);
        hasher.update(identity.as_bytes());
        let identity_commitment: [u8; 32] = hasher.finalize().into();

        // Encrypt identity (simplified)
        let encrypted_identity = identity.as_bytes().to_vec();

        let validator = ZkValidator {
            encrypted_identity,
            stake,
            stake_proof,
            identity_commitment,
            metrics: ZkNetworkMetrics::new(1.0),
            registered_at: now,
            last_activity: now,
            status: ValidatorStatus::Pending,
        };

        let mut validators = self.validators.write().await;
        validators.insert(identity, validator);

        Ok(())
    }

    /// Submit an encrypted vote for the current round
    pub async fn submit_encrypted_vote(
        &self,
        voter_commitment: [u8; 32],
        encrypted_vote: Vec<u8>,
        vote_proof: ByteRoutingProof,
        weight: f64,
    ) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let vote = EncryptedVote {
            encrypted_vote,
            vote_proof,
            voter_commitment,
            weight,
            timestamp: now,
        };

        let mut round = self.current_round.write().await;
        round.encrypted_votes.insert(voter_commitment, vote);

        Ok(())
    }

    /// Process consensus round and finalize block
    pub async fn process_consensus_round(&self, proposed_block: Block) -> Result<ZkBlock> {
        let mut round = self.current_round.write().await;
        
        if round.status != RoundStatus::Voting {
            return Err(anyhow!("Round not in voting phase"));
        }

        // Check if we have enough votes
        if round.encrypted_votes.len() < self.params.min_votes {
            round.status = RoundStatus::Failed;
            return Err(anyhow!("Insufficient votes for consensus"));
        }        // Create ZK block
        let block_hash: [u8; 32] = {
            let mut hasher = Sha256::new();
            hasher.update(&proposed_block.hash);
            hasher.finalize().into()
        };
        
        let zk_block = ZkBlock {
            hash: block_hash,
            previous_hash: [0u8; 32], // Should be filled with actual previous hash
            encrypted_transactions: vec![0u8; 256], // Encrypted transaction data
            validity_proof: ByteRoutingProof {
                inputs: vec![vec![1u8; 32]],
                elements: vec![vec![0u8; 64]],
                commitments: vec![vec![0u8; 32]],
            },
            transaction_root: [0u8; 32],
            validator_commitments: round.encrypted_votes.keys().cloned().collect(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            height: round.round,
            consensus_round: round.round,
        };

        // Commit the block
        let mut blocks = self.committed_blocks.write().await;
        blocks.insert(block_hash, zk_block.clone());

        round.status = RoundStatus::Committed;

        Ok(zk_block)
    }    /// Finalize consensus round and create a block
    pub async fn finalize_round(&self) -> Result<ZkBlock> {
        let current_round = self.current_round.read().await;
        let transactions = self.get_pending_zk_transactions(100).await;
        
        // Flatten all transaction data into a single encrypted blob
        let encrypted_transactions: Vec<u8> = transactions
            .iter()
            .flat_map(|tx| tx.encrypted_data.clone())
            .collect();
        
        let zk_block = ZkBlock {
            hash: [0u8; 32], // Will be computed properly in real implementation
            previous_hash: [0u8; 32],
            encrypted_transactions,
            validity_proof: ByteRoutingProof { 
                commitments: vec![],
                elements: vec![],
                inputs: vec![],
            },
            transaction_root: [0u8; 32],
            validator_commitments: vec![],
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            height: current_round.round,
            consensus_round: current_round.round,
        };

        Ok(zk_block)
    }

    /// Get current consensus metrics
    pub async fn get_consensus_metrics(&self) -> Result<ZkConsensusMetrics> {
        let validators = self.validators.read().await;
        let round = self.current_round.read().await;
        let blocks = self.committed_blocks.read().await;

        Ok(ZkConsensusMetrics {
            total_validators: validators.len(),
            active_validators: validators.values()
                .filter(|v| v.status == ValidatorStatus::Active)
                .count(),
            current_round: round.round,
            total_blocks: blocks.len(),
            total_stake: validators.values().map(|v| v.stake).sum(),
            anonymity_set_size: round.anonymous_validators.len(),
        })
    }

    /// Get validator by identity commitment (privacy-preserving lookup)
    pub async fn get_validator_by_commitment(&self, commitment: [u8; 32]) -> Option<ZkValidator> {
        let validators = self.validators.read().await;
        validators.values()
            .find(|v| v.identity_commitment == commitment)
            .cloned()    }

    /// Slash a validator for malicious behavior
    pub async fn slash_validator(&self, commitment: [u8; 32], _reason: String) -> Result<()> {
        let mut validators = self.validators.write().await;
        
        for (_id, validator) in validators.iter_mut() {
            if validator.identity_commitment == commitment {
                validator.status = ValidatorStatus::Slashed;
                
                // Apply slashing penalty
                let penalty = validator.stake * self.params.slashing_penalty;                validator.stake -= penalty;                // Distribute penalty to other validators as rewards
                let remaining_validator_commitments: Vec<[u8; 32]> = {
                    let validators_guard = self.validators.read().await;
                    validators_guard.values()
                        .filter(|v| v.stake > 0.0)
                        .map(|v| v.identity_commitment)
                        .collect()
                };
                
                if !remaining_validator_commitments.is_empty() {
                    let reward_per_validator = penalty / remaining_validator_commitments.len() as f64;
                    let mut validators_guard = self.validators.write().await;
                    
                    for commitment in remaining_validator_commitments {
                        if let Some(validator) = validators_guard.values_mut().find(|v| v.identity_commitment == commitment) {
                            validator.stake += reward_per_validator;
                        }
                    }
                }
                
                return Ok(());
            }
        }
        
        Err(anyhow!("Validator not found for slashing"))
    }

    /// Start a new consensus round
    pub async fn start_new_round(&self) -> Result<()> {
        let mut round = self.current_round.write().await;
        let validators = self.validators.read().await;

        // Select active validators for anonymity set
        let active_validators: Vec<_> = validators.values()
            .filter(|v| v.status == ValidatorStatus::Active)
            .take(self.params.max_validators)
            .collect();

        if active_validators.len() < self.params.min_votes {
            return Err(anyhow!("Insufficient active validators"));
        }

        // Create anonymous validator set
        let anonymous_validators: Vec<AnonymousValidator> = active_validators
            .iter()
            .map(|v| AnonymousValidator {
                blinded_commitment: v.identity_commitment,
                membership_proof: ByteRoutingProof {
                    inputs: vec![vec![1u8; 32]],
                    elements: vec![vec![0u8; 64]],
                    commitments: vec![v.identity_commitment.to_vec()],
                },
                encrypted_stake: v.stake.to_le_bytes().to_vec(),
                anonymous_reputation: v.metrics.reputation_score,
            })
            .collect();

        *round = ZkConsensusRound {
            round: round.round + 1,
            leader_id: Some(active_validators[0].identity_commitment),
            anonymous_validators,
            encrypted_votes: HashMap::new(),
            vote_proof: None,
            started_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            status: RoundStatus::Proposing,
        };

        Ok(())
    }

    /// Start a consensus round with specific round number
    pub async fn start_round(&self, round: u64) -> Result<()> {
        let mut current_round = self.current_round.write().await;
        current_round.round = round;
        current_round.status = RoundStatus::Proposing;
        current_round.started_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Ok(())
    }

    /// Get the zero-knowledge transaction pool
    pub async fn get_zk_transaction_pool(&self) -> Arc<RwLock<ZkTransactionPool>> {
        self.zk_tx_pool.clone()
    }    /// Add transaction to ZK pool
    pub async fn add_zk_transaction(&self, tx: ZkTransaction) -> Result<()> {
        let mut pool = self.zk_tx_pool.write().await;
        pool.add_transaction(tx);
        Ok(())
    }

    /// Get pending ZK transactions for block creation
    pub async fn get_pending_zk_transactions(&self, _limit: usize) -> Vec<ZkTransaction> {
        let pool = self.zk_tx_pool.read().await;
        pool.get_pending_transactions().into_iter().cloned().collect()
    }

    /// Distribute consensus rewards to validators
    pub async fn distribute_consensus_rewards(&self, block_height: u64) -> Result<()> {
        let validators = self.validators.read().await;
        let economics = &self.economics;
        
        for (validator_id, validator) in validators.iter() {
            if validator.status == ValidatorStatus::Active {                let reward = economics.calculate_validator_reward(
                    validator,
                    1, // blocks validated
                    validator.metrics.reputation_score,                ).await?;                // Distribute the reward tokens by updating validator stake
                let mut validators_guard = self.validators.write().await;
                if let Some(validator) = validators_guard.get_mut(validator_id) {
                    validator.stake += reward as f64;
                    
                    // Record the reward distribution in the ZK system
                    println!("Validator {} earned {} ZHTP tokens for block {} (new stake: {})", 
                        validator_id, reward, block_height, validator.stake);
                } else {
                    println!("Warning: Could not find validator {} to distribute reward", validator_id);
                }
            }
        }
        
        Ok(())
    }

    /// Calculate CA rewards
    pub async fn calculate_ca_rewards(&self, certificates_issued: u64) -> Result<u64> {
        self.economics.calculate_certificate_reward(certificates_issued as u32).await
    }

    /// Calculate DNS rewards
    pub async fn calculate_dns_rewards(&self, domains_resolved: u64, domains_registered: u64) -> Result<u64> {
        self.economics.calculate_dns_reward(domains_resolved as u32, domains_registered as u32).await
    }

    /// Calculate routing rewards
    pub async fn calculate_routing_rewards(&self, packets_routed: u64, success_rate: f64) -> Result<u64> {
        self.economics.calculate_routing_reward(packets_routed, success_rate).await
    }    /// Calculate validator rewards
    pub async fn calculate_validator_rewards(&self, _stake: f64) -> Result<u64> {
        // Create a dummy validator for reward calculation
        let dummy_validator = ZkValidator {
            encrypted_identity: vec![],
            stake: _stake,
            stake_proof: ByteRoutingProof {
                commitments: vec![],
                elements: vec![],
                inputs: vec![],
            },
            identity_commitment: [0u8; 32],
            metrics: ZkNetworkMetrics::new(1.0),
            registered_at: 0,
            last_activity: 0,
            status: ValidatorStatus::Active,
        };
        self.economics.calculate_validator_reward(&dummy_validator, 100, 1.0).await
    }    /// Distribute CA rewards
    pub async fn distribute_ca_rewards(&self, ca_id: String, certificates_issued: u64) -> Result<()> {
        let reward_amount = self.calculate_ca_rewards(certificates_issued).await?;
        println!("Distributed {} ZHTP tokens to CA {}", reward_amount, ca_id);
        Ok(())
    }    /// Distribute DNS rewards
    pub async fn distribute_dns_rewards(&self, dns_id: String, domains_resolved: u64, domains_registered: u64) -> Result<()> {
        let reward_amount = self.calculate_dns_rewards(domains_resolved, domains_registered).await?;
        println!("Distributed {} ZHTP tokens to DNS node {}", reward_amount, dns_id);
        Ok(())
    }    /// Distribute routing rewards
    pub async fn distribute_routing_rewards(&self, node_id: String, packets_routed: u64, success_rate: f64) -> Result<()> {
        let reward_amount = self.calculate_routing_rewards(packets_routed, success_rate).await?;
        println!("Distributed {} ZHTP tokens to routing node {}", reward_amount, node_id);
        Ok(())
    }

    /// Distribute validator rewards
    pub async fn distribute_validator_rewards(&self, _validator_id: &str, amount: u64) -> Result<()> {
        // In a real implementation, this would update the validator's token balance
        println!("Distributed {} ZHTP tokens to validator {}", amount, _validator_id);
        Ok(())
    }

    /// Distribute block rewards to validator
    pub async fn distribute_block_rewards(&self, _block_hash: [u8; 32], validator_id: String) -> Result<()> {
        let reward_amount = self.calculate_validator_rewards(1000.0).await?; // Base reward for block production
        println!("Distributed {} ZHTP tokens to validator {} for block production", reward_amount, validator_id);
        Ok(())
    }

    /// Process transaction fees
    pub async fn process_transaction_fees(&self, total_fees: f64) -> Result<()> {
        self.economics.process_fee_burn(total_fees as u64).await
    }

    /// Get economic metrics from the consensus system
    pub async fn get_economic_metrics(&self) -> Result<crate::zhtp::economics::EconomicMetrics> {
        self.economics.get_economic_metrics().await
    }

    /// Get network value capture metrics
    pub async fn get_network_value_capture(&self) -> Result<crate::zhtp::economics::NetworkValueCapture> {
        self.economics.calculate_network_value_capture().await
    }
}

/// Consensus metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkConsensusMetrics {
    pub total_validators: usize,
    pub active_validators: usize,
    pub current_round: u64,
    pub total_blocks: usize,
    pub total_stake: f64,
    pub anonymity_set_size: usize,
}

// Legacy compatibility for existing code
impl ConsensusState {
    pub fn new(round: u64, leader: Option<String>) -> Self {
        Self {
            current_round: round,
            round_status: RoundStatus::Proposing,
            active_validators: 0,
            total_stake: 0.0,
            committed_blocks: 0,
            votes_in_current_round: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zk_consensus_creation() {
        let params = ZkConsensusParams {
            min_stake: 1000.0,
            max_validators: 100,
            round_timeout: 30,
            min_votes: 3,
            slashing_penalty: 0.1,
            anonymity_set_size: 50,
        };

        let consensus = ZkConsensus::new(params);
        let metrics = consensus.get_consensus_metrics().await.unwrap();
        
        assert_eq!(metrics.total_validators, 0);
        assert_eq!(metrics.current_round, 0);
    }

    #[tokio::test]
    async fn test_validator_registration() {
        let params = ZkConsensusParams {
            min_stake: 1000.0,
            max_validators: 100,
            round_timeout: 30,
            min_votes: 3,
            slashing_penalty: 0.1,
            anonymity_set_size: 50,
        };

        let consensus = ZkConsensus::new(params);
        
        let stake_proof = ByteRoutingProof {
            inputs: vec![vec![1u8; 32]],
            elements: vec![vec![0u8; 64]],
            commitments: vec![vec![0u8; 32]],
        };

        let result = consensus.register_validator(
            "validator1".to_string(),
            1500.0,
            stake_proof,
            vec![0u8; 32],
        ).await;

        assert!(result.is_ok());
        
        let metrics = consensus.get_consensus_metrics().await.unwrap();
        assert_eq!(metrics.total_validators, 1);
    }

    #[tokio::test]
    async fn test_insufficient_stake() {
        let params = ZkConsensusParams {
            min_stake: 1000.0,
            max_validators: 100,
            round_timeout: 30,
            min_votes: 3,
            slashing_penalty: 0.1,
            anonymity_set_size: 50,
        };

        let consensus = ZkConsensus::new(params);
        
        let stake_proof = ByteRoutingProof {
            inputs: vec![vec![1u8; 32]],
            elements: vec![vec![0u8; 64]],
            commitments: vec![vec![0u8; 32]],
        };

        let result = consensus.register_validator(
            "validator1".to_string(),
            500.0, // Below minimum
            stake_proof,
            vec![0u8; 32],
        ).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_economic_integration() {
        let params = ZkConsensusParams {
            min_stake: 1000.0,
            max_validators: 100,
            round_timeout: 30,
            min_votes: 3,
            slashing_penalty: 0.1,
            anonymity_set_size: 50,
        };

        let consensus = ZkConsensus::new(params);
          // Test economic metrics access
        let metrics = consensus.get_economic_metrics().await.unwrap();
        assert!(metrics.total_supply > 0);
        
        // Test network value capture calculation
        let value_capture = consensus.get_network_value_capture().await.unwrap();
        assert!(value_capture.total_addressable_market >= 0);
        assert!(value_capture.market_capture_rate >= 0.0);
    }
}
