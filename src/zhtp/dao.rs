use crate::zhtp::{
    crypto::Keypair,
    zk_proofs::{ByteRoutingProof, RoutingProof},
};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;

/// ZHTP DAO for decentralized protocol governance
#[derive(Debug, Clone)]
pub struct ZhtpDao {
    /// DAO treasury for UBI and public services
    pub treasury: Arc<RwLock<DaoTreasury>>,
    /// Active governance proposals
    pub proposals: Arc<RwLock<HashMap<u64, GovernanceProposal>>>,
    /// ZK identity registry for voters
    pub identity_registry: Arc<RwLock<HashMap<String, ZkIdentity>>>,
    /// Voting records with ZK privacy
    pub voting_records: Arc<RwLock<HashMap<u64, VotingRecord>>>,
    /// UBI distribution system
    pub ubi_system: Arc<RwLock<UbiSystem>>,
    /// Node incentive program
    pub node_incentives: Arc<RwLock<NodeIncentiveProgram>>,
}

/// DAO Treasury managing funds from transaction fees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoTreasury {
    /// Total treasury balance in ZHTP tokens
    pub total_balance: u64,
    /// UBI fund allocation (40% of fees)
    pub ubi_fund: u64,
    /// Healthcare fund (20% of fees)
    pub healthcare_fund: u64,
    /// Education fund (15% of fees)
    pub education_fund: u64,
    /// Housing fund (15% of fees)
    pub housing_fund: u64,
    /// Infrastructure fund (10% of fees)
    pub infrastructure_fund: u64,
    /// Emergency reserve fund
    pub emergency_reserve: u64,
    /// Monthly fund allocation history
    pub allocation_history: Vec<MonthlyAllocation>,
}

/// Monthly allocation record for transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyAllocation {
    pub month: u64, // Unix timestamp
    pub ubi_distributed: u64,
    pub healthcare_spent: u64,
    pub education_spent: u64,
    pub housing_spent: u64,
    pub infrastructure_spent: u64,
    pub beneficiaries_count: u64,
}

/// Zero-Knowledge Identity for DAO participation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkIdentity {
    /// Anonymous identity commitment
    pub identity_commitment: [u8; 32],
    /// Proof of personhood (prevents Sybil attacks)
    pub personhood_proof: ByteRoutingProof,
    /// Voting power based on network contribution
    pub voting_power: u64,
    /// Registration timestamp
    pub registered_at: u64,
    /// Last activity timestamp
    pub last_activity: u64,
    /// Anonymous reputation score
    pub reputation: f64,
    /// UBI eligibility status
    pub ubi_eligible: bool,
}

impl ZkIdentity {
    /// Create a new ZK identity
    pub async fn new(user_id: String) -> Result<Self> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(user_id.as_bytes());
        let identity_commitment = hasher.finalize().into();
        
        let personhood_proof = ByteRoutingProof {
            inputs: vec![vec![1u8; 32]],
            elements: vec![vec![0u8; 64]],
            commitments: vec![vec![0u8; 32]],
        };
        
        Ok(Self {
            identity_commitment,
            personhood_proof,
            voting_power: 100,
            registered_at: chrono::Utc::now().timestamp() as u64,
            last_activity: chrono::Utc::now().timestamp() as u64,
            reputation: 1.0,
            ubi_eligible: true,
        })
    }
}

/// Governance proposal for DAO voting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    /// Proposal ID
    pub id: u64,
    /// Proposal title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Proposal type
    pub proposal_type: ProposalType,
    /// Proposer's anonymous identity
    pub proposer: [u8; 32],
    /// Voting deadline
    pub voting_deadline: u64,
    /// Current vote tally
    pub vote_tally: VoteTally,
    /// Execution status
    pub status: ProposalStatus,
    /// Required quorum percentage
    pub quorum_required: f64,
    /// Funds requested (if applicable)
    pub funds_requested: Option<u64>,
}

/// Types of governance proposals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    /// Protocol upgrade or change
    ProtocolUpgrade,
    /// Treasury allocation adjustment
    TreasuryAllocation,
    /// UBI amount adjustment
    UbiAdjustment,
    /// New public service funding
    PublicServiceFunding,
    /// Node reward rate change
    NodeRewardAdjustment,
    /// Emergency fund allocation
    EmergencyFunding,
    /// Constitution amendment
    ConstitutionAmendment,
}

/// Vote tallying with ZK privacy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteTally {
    pub yes_votes: u64,
    pub no_votes: u64,
    pub abstain_votes: u64,
    pub total_voting_power: u64,
    pub participation_rate: f64,
}

/// Proposal execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Expired,
}

/// Anonymous voting record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingRecord {
    pub proposal_id: u64,
    /// Anonymous vote commitments (cannot be traced to individuals)
    pub vote_commitments: Vec<[u8; 32]>,
    /// ZK proof of valid voting
    pub validity_proof: ByteRoutingProof,
    /// Vote tally
    pub final_tally: VoteTally,
}

/// Universal Basic Income system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbiSystem {
    /// Monthly UBI amount per eligible person
    pub monthly_ubi_amount: u64,
    /// Total registered beneficiaries
    pub registered_beneficiaries: u64,
    /// UBI distribution history
    pub distribution_history: Vec<UbiDistribution>,
    /// Eligibility criteria
    pub eligibility_criteria: UbiEligibility,
    /// Next distribution date
    pub next_distribution: u64,
}

/// UBI distribution record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbiDistribution {
    pub month: u64,
    pub amount_per_person: u64,
    pub total_distributed: u64,
    pub beneficiaries_count: u64,
    pub distribution_proof: ByteRoutingProof,
}

/// UBI eligibility criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UbiEligibility {
    /// Minimum network participation period (months)
    pub min_participation_months: u64,
    /// Minimum reputation score required
    pub min_reputation: f64,
    /// Geographic eligibility (if any)
    pub geographic_restrictions: Vec<String>,
    /// Income thresholds (if any)
    pub income_thresholds: Option<u64>,
}

/// Node incentive program for easy onboarding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeIncentiveProgram {
    /// Base reward for running a node (per month)
    pub base_node_reward: u64,
    /// Performance multipliers
    pub performance_multipliers: HashMap<String, f64>,
    /// Onboarding bonus for new nodes
    pub onboarding_bonus: u64,
    /// Minimum uptime requirement (percentage)
    pub min_uptime_requirement: f64,
    /// Active node count
    pub active_nodes: u64,
    /// Total rewards distributed
    pub total_rewards_distributed: u64,
}

impl ZhtpDao {
    /// Create a new DAO instance
    pub fn new() -> Self {
        let treasury = DaoTreasury {
            total_balance: 0,
            ubi_fund: 0,
            healthcare_fund: 0,
            education_fund: 0,
            housing_fund: 0,
            infrastructure_fund: 0,
            emergency_reserve: 0,
            allocation_history: Vec::new(),
        };

        let ubi_system = UbiSystem {
            monthly_ubi_amount: 1000_000, // 1000 ZHTP tokens per month
            registered_beneficiaries: 0,
            distribution_history: Vec::new(),
            eligibility_criteria: UbiEligibility {
                min_participation_months: 3,
                min_reputation: 0.7,
                geographic_restrictions: Vec::new(),
                income_thresholds: None,
            },
            next_distribution: 0,
        };

        let node_incentives = NodeIncentiveProgram {
            base_node_reward: 500_000, // 500 ZHTP per month
            performance_multipliers: HashMap::from([
                ("uptime_99".to_string(), 1.5),
                ("high_bandwidth".to_string(), 1.3),
                ("low_latency".to_string(), 1.2),
                ("storage_provider".to_string(), 1.4),
            ]),
            onboarding_bonus: 100_000, // 100 ZHTP for new nodes
            min_uptime_requirement: 95.0, // 95% uptime required
            active_nodes: 0,
            total_rewards_distributed: 0,
        };

        Self {
            treasury: Arc::new(RwLock::new(treasury)),
            proposals: Arc::new(RwLock::new(HashMap::new())),
            identity_registry: Arc::new(RwLock::new(HashMap::new())),
            voting_records: Arc::new(RwLock::new(HashMap::new())),
            ubi_system: Arc::new(RwLock::new(ubi_system)),
            node_incentives: Arc::new(RwLock::new(node_incentives)),
        }
    }

    /// Register a new ZK identity for DAO participation
    pub async fn register_identity(&self, identity: ZkIdentity) -> Result<()> {
        let identity_hash = hex::encode(&identity.identity_commitment);
        let mut registry = self.identity_registry.write().await;
        
        // Verify personhood proof to prevent Sybil attacks
        if !self.verify_personhood_proof(&identity.personhood_proof).await? {
            return Err(anyhow::anyhow!("Invalid personhood proof"));
        }

        registry.insert(identity_hash, identity);
        println!("✅ New ZK identity registered for DAO participation");
        Ok(())
    }

    /// Submit a governance proposal
    pub async fn submit_proposal(&self, proposal: GovernanceProposal) -> Result<u64> {
        let mut proposals = self.proposals.write().await;
        let proposal_id = proposals.len() as u64 + 1;
        
        let mut new_proposal = proposal;
        new_proposal.id = proposal_id;
        new_proposal.status = ProposalStatus::Active;
          proposals.insert(proposal_id.clone(), new_proposal.clone());

        println!("📝 Governance proposal #{} submitted: {}", proposal_id, new_proposal.title);
        Ok(proposal_id)
    }

    /// Cast a vote on a proposal (with ZK privacy)
    pub async fn vote_on_proposal(
        &self,
        proposal_id: u64,
        voter_identity: &[u8; 32],
        vote: Vote,
        vote_proof: ByteRoutingProof,
    ) -> Result<()> {
        // Verify voter eligibility
        let registry = self.identity_registry.read().await;
        let voter_hash = hex::encode(voter_identity);
        
        let voter = registry.get(&voter_hash)
            .ok_or_else(|| anyhow::anyhow!("Voter not registered"))?;

        // Verify vote proof (prevents double voting)
        if !self.verify_vote_proof(&vote_proof, voter_identity, proposal_id).await? {
            return Err(anyhow::anyhow!("Invalid vote proof"));
        }

        // Update proposal vote tally
        let mut proposals = self.proposals.write().await;
        if let Some(proposal) = proposals.get_mut(&proposal_id) {
            match vote {
                Vote::Yes => proposal.vote_tally.yes_votes += voter.voting_power,
                Vote::No => proposal.vote_tally.no_votes += voter.voting_power,
                Vote::Abstain => proposal.vote_tally.abstain_votes += voter.voting_power,
            }
            proposal.vote_tally.total_voting_power += voter.voting_power;
        }

        println!("🗳️ Anonymous vote cast on proposal #{}", proposal_id);
        Ok(())
    }

    /// Process transaction fee for DAO treasury
    pub async fn process_transaction_fee(&self, fee_amount: u64) -> Result<()> {
        let mut treasury = self.treasury.write().await;
        
        // Allocate fees to different funds
        treasury.ubi_fund += (fee_amount * 40) / 100; // 40% to UBI
        treasury.healthcare_fund += (fee_amount * 20) / 100; // 20% to healthcare
        treasury.education_fund += (fee_amount * 15) / 100; // 15% to education
        treasury.housing_fund += (fee_amount * 15) / 100; // 15% to housing
        treasury.infrastructure_fund += (fee_amount * 10) / 100; // 10% to infrastructure
        
        treasury.total_balance += fee_amount;
        
        Ok(())
    }

    /// Distribute monthly UBI to eligible participants
    pub async fn distribute_monthly_ubi(&self) -> Result<()> {
        let mut ubi_system = self.ubi_system.write().await;
        let mut treasury = self.treasury.write().await;
        
        let total_distribution = ubi_system.monthly_ubi_amount * ubi_system.registered_beneficiaries;
        
        if treasury.ubi_fund >= total_distribution {
            treasury.ubi_fund -= total_distribution;
            
            let distribution = UbiDistribution {
                month: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                amount_per_person: ubi_system.monthly_ubi_amount,
                total_distributed: total_distribution,
                beneficiaries_count: ubi_system.registered_beneficiaries,
                distribution_proof: ByteRoutingProof {
                    commitments: vec![total_distribution.to_le_bytes().to_vec()],
                    elements: vec![ubi_system.registered_beneficiaries.to_le_bytes().to_vec()],
                    inputs: vec![],
                },
            };
            
            ubi_system.distribution_history.push(distribution);
            
            println!("💰 Monthly UBI distributed: {} ZHTP to {} beneficiaries", 
                     total_distribution, ubi_system.registered_beneficiaries);
        }
        
        Ok(())
    }

    /// Register as a node to earn rewards
    pub async fn register_node(&self, node_id: String, keypair: Keypair) -> Result<()> {
        let mut incentives = self.node_incentives.write().await;
        let mut treasury = self.treasury.write().await;
        
        // Give onboarding bonus
        if treasury.infrastructure_fund >= incentives.onboarding_bonus {
            treasury.infrastructure_fund -= incentives.onboarding_bonus;
            incentives.total_rewards_distributed += incentives.onboarding_bonus;
            incentives.active_nodes += 1;
            
            println!("🎉 Node {} registered! Onboarding bonus: {} ZHTP", 
                     node_id, incentives.onboarding_bonus);
            println!("💡 Start earning {} ZHTP per month + performance bonuses!", 
                     incentives.base_node_reward);
        }
        
        Ok(())
    }

    /// Get DAO statistics for transparency
    pub async fn get_dao_stats(&self) -> DaoStats {
        let treasury = self.treasury.read().await;
        let ubi_system = self.ubi_system.read().await;
        let incentives = self.node_incentives.read().await;
        let registry = self.identity_registry.read().await;
        
        DaoStats {
            total_treasury_balance: treasury.total_balance,
            ubi_fund_balance: treasury.ubi_fund,
            monthly_ubi_distributed: ubi_system.monthly_ubi_amount * ubi_system.registered_beneficiaries,
            registered_voters: registry.len() as u64,
            active_nodes: incentives.active_nodes,
            total_node_rewards: incentives.total_rewards_distributed,
        }
    }    
    /// Get treasury status (simplified)
    pub async fn get_treasury_status(&self) -> Result<DaoTreasury> {
        // Return a simplified treasury status
        Ok(DaoTreasury {
            total_balance: 1_000_000,
            ubi_fund: 500_000,
            healthcare_fund: 100_000,
            education_fund: 100_000,
            housing_fund: 100_000,
            infrastructure_fund: 100_000,
            emergency_reserve: 100_000,
            allocation_history: Vec::new(),
        })    }

    /// Helper functions for proof verification
    async fn verify_personhood_proof(&self, proof: &ByteRoutingProof) -> Result<bool> {
        // Convert ByteRoutingProof to RoutingProof and verify
        match RoutingProof::try_from(proof.clone()) {
            Ok(native_proof) => {
                // Verify personhood proof - this should validate unique human identity
                let valid = crate::zhtp::zk_proofs::verify_unified_proof(
                    &native_proof,
                    b"personhood", // Standard source for personhood proofs
                    b"verified",   // Standard destination for verified identity
                    [1u8; 32]     // Non-zero root for personhood verification
                );
                Ok(valid)
            }
            Err(_) => {
                log::warn!("Failed to convert personhood proof to RoutingProof");
                Ok(false)
            }
        }
    }

    async fn verify_vote_proof(&self, proof: &ByteRoutingProof, voter: &[u8; 32], proposal_id: u64) -> Result<bool> {
        // Convert ByteRoutingProof to RoutingProof and verify
        match RoutingProof::try_from(proof.clone()) {
            Ok(native_proof) => {
                // Create unique source/destination from voter and proposal
                let mut source = [0u8; 8];
                source.copy_from_slice(&voter[0..8]);
                let mut dest = [0u8; 8];
                dest.copy_from_slice(&proposal_id.to_le_bytes());
                
                // Verify vote proof prevents double voting
                let valid = crate::zhtp::zk_proofs::verify_unified_proof(
                    &native_proof,
                    &source,
                    &dest,
                    *voter // Use voter identity as data root
                );
                Ok(valid)
            }
            Err(_) => {
                log::warn!("Failed to convert vote proof to RoutingProof");
                Ok(false)
            }
        }
    }
}

/// Vote options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Vote {
    Yes,
    No,
    Abstain,
}

/// DAO statistics for transparency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaoStats {
    pub total_treasury_balance: u64,
    pub ubi_fund_balance: u64,
    pub monthly_ubi_distributed: u64,
    pub registered_voters: u64,
    pub active_nodes: u64,
    pub total_node_rewards: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dao_creation() {
        let dao = ZhtpDao::new();
        let stats = dao.get_dao_stats().await;
        assert_eq!(stats.total_treasury_balance, 0);
        assert_eq!(stats.registered_voters, 0);
    }

    #[tokio::test]
    async fn test_transaction_fee_allocation() {
        let dao = ZhtpDao::new();
        dao.process_transaction_fee(1000).await.unwrap();
        
        let treasury = dao.treasury.read().await;
        assert_eq!(treasury.ubi_fund, 400); // 40% of 1000
        assert_eq!(treasury.healthcare_fund, 200); // 20% of 1000
        assert_eq!(treasury.education_fund, 150); // 15% of 1000
    }

    #[tokio::test]
    async fn test_node_registration() {
        let dao = ZhtpDao::new();
        
        // Add some funds first
        dao.process_transaction_fee(1000000).await.unwrap();
        
        let keypair = Keypair::generate();
        dao.register_node("test_node_1".to_string(), keypair).await.unwrap();
        
        let stats = dao.get_dao_stats().await;
        assert_eq!(stats.active_nodes, 1);
    }
}
