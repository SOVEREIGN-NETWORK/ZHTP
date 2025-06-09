use serde::{Serialize, Deserialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::RwLock;

/// Network metrics for consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub packets_routed: u32,
    pub delivery_success: u32,
    pub delivery_failures: u32,
    pub average_latency: f64,
    pub reputation_score: f64,
    pub uptime_threshold: f64,
}

impl NetworkMetrics {
    pub fn new(uptime_threshold: f64) -> Self {
        Self {
            packets_routed: 0,
            delivery_success: 0,
            delivery_failures: 0,
            average_latency: 0.0,
            reputation_score: 1.0,
            uptime_threshold,
        }
    }

    pub fn update_routing_metrics(&mut self, latency: f64, packet_size: usize) {
        self.packets_routed += 1;
        self.delivery_success += 1;
        
        // Update average latency with exponential moving average
        const ALPHA: f64 = 0.1;
        self.average_latency = ALPHA * latency + (1.0 - ALPHA) * self.average_latency;
        
        // Increase reputation for successful routing
        self.update_reputation(true);
    }

    pub fn update_failed_routing(&mut self) {
        self.delivery_failures += 1;
        self.update_reputation(false);
    }

    pub fn update_reputation(&mut self, success: bool) {
        const REPUTATION_SCALE: f64 = 0.1;
        
        if success {
            self.reputation_score += REPUTATION_SCALE * (1.0 - self.reputation_score);
        } else {
            self.reputation_score -= REPUTATION_SCALE * self.reputation_score;
        }

        // Ensure reputation stays within [0, 1]
        self.reputation_score = self.reputation_score.clamp(0.0, 1.0);
    }

    pub fn get_delivery_success_rate(&self) -> f64 {
        if self.delivery_success + self.delivery_failures == 0 {
            1.0
        } else {
            self.delivery_success as f64 / 
            (self.delivery_success + self.delivery_failures) as f64
        }
    }
}

/// Consensus round information
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct ConsensusRound {
    pub round: u64,
    pub leader: String,
    pub validators: HashSet<String>,
    pub votes: HashMap<String, bool>,
    pub timestamp: u64,
}

impl ConsensusRound {
    #[allow(dead_code)]
    pub fn new(round: u64, leader: String, validators: HashSet<String>) -> Self {
        Self {
            round,
            leader,
            validators,
            votes: HashMap::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Node information for consensus
#[derive(Debug, Clone)]
pub struct ValidatorInfo {
    pub id: String,
    pub stake: f64,
    pub metrics: NetworkMetrics,
}

impl ValidatorInfo {
    pub fn new(id: String, stake: f64, uptime_threshold: f64) -> Self {
        Self {
            id,
            stake,
            metrics: NetworkMetrics::new(uptime_threshold),
        }
    }
}

/// Consensus manager for coordinating network consensus
pub struct ConsensusManager {
    validators: Arc<RwLock<HashMap<String, ValidatorInfo>>>,
    current_round: Arc<RwLock<ConsensusRound>>,
    base_reward: f64,
    round_duration: u64,
    view_changes: Arc<RwLock<HashMap<u64, HashSet<String>>>>,
    committed_blocks: Arc<RwLock<HashSet<[u8; 32]>>>,
}

impl ConsensusManager {
    pub fn new(base_reward: f64, round_duration: u64) -> Self {
        Self {
            validators: Arc::new(RwLock::new(HashMap::new())),
            current_round: Arc::new(RwLock::new(ConsensusRound::new(
                0,
                String::from("genesis"),
                HashSet::new(),
            ))),
            base_reward,
            round_duration,
            view_changes: Arc::new(RwLock::new(HashMap::new())),
            committed_blocks: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Register a new validator
    pub async fn register_node(&self, node_id: String, stake: f64) {
        let mut validators = self.validators.write().await;
        let validator = ValidatorInfo::new(node_id.clone(), stake, 0.9);
        validators.insert(node_id, validator);
    }

    /// Select next round leader based on stake and metrics
    pub async fn select_leader(&self) -> Option<String> {
        let validators = self.validators.read().await;
        
        if validators.is_empty() {
            return None;
        }

        let mut max_score = 0.0;
        let mut leader = None;

        for (id, info) in validators.iter() {
            let score = info.stake * info.metrics.reputation_score;
            if score > max_score {
                max_score = score;
                leader = Some(id.clone());
            }
        }

        leader
    }

    /// Update network metrics
    pub async fn update_metrics(&self, node: &str, success: bool, latency: Option<f64>) {
        let mut validators = self.validators.write().await;
        if let Some(info) = validators.get_mut(node) {
            if success {
                if let Some(lat) = latency {
                    info.metrics.update_routing_metrics(lat, 0);
                }
            } else {
                info.metrics.update_failed_routing();
            }
        }
    }

    /// Get validator metrics
    pub async fn get_metrics(&self, node: &str) -> Option<NetworkMetrics> {
        let validators = self.validators.read().await;
        validators.get(node).map(|info| info.metrics.clone())
    }

    /// Select validators for the next round based on stake and metrics
    pub async fn select_validators(&self, count: usize) -> Vec<String> {
        let validators = self.validators.read().await;
        
        let mut sorted: Vec<_> = validators.iter().collect();
        sorted.sort_by(|(_, a), (_, b)| {
            let score_a = a.stake * a.metrics.reputation_score;
            let score_b = b.stake * b.metrics.reputation_score;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        sorted.into_iter()
            .take(count)
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Calculate rewards for the current round
    pub async fn calculate_rewards(&self, round: &ConsensusRound) -> HashMap<String, f64> {
        let mut rewards = HashMap::new();
        let validators = self.validators.read().await;

        for (id, info) in validators.iter() {
            let reward = self.base_reward * info.stake * info.metrics.reputation_score;
            if id == &round.leader {
                rewards.insert(id.clone(), reward * 1.5);
            } else {
                rewards.insert(id.clone(), reward);
            }
        }

        rewards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_consensus_basic() {
        let mut manager = ConsensusManager::new(100.0, 3600);

        // Register validators
        manager.register_node("node1".to_string(), 1000.0).await;
        manager.register_node("node2".to_string(), 800.0).await;
        manager.register_node("node3".to_string(), 500.0).await;

        // Initial leader selection
        let leader = manager.select_leader().await.unwrap();
        assert_eq!(leader, "node1"); // Highest stake
    }

    #[tokio::test]
    async fn test_metrics_update() {
        let mut manager = ConsensusManager::new(100.0, 3600);
        manager.register_node("test_node".to_string(), 1000.0).await;

        // Update metrics
        manager.update_metrics("test_node", true, Some(100.0)).await;
        manager.update_metrics("test_node", false, Some(500.0)).await;

        // Check metrics
        let metrics = manager.get_metrics("test_node").await.unwrap();
        assert_eq!(metrics.delivery_success, 1);
        assert_eq!(metrics.delivery_failures, 1);
        assert!(metrics.average_latency > 0.0);
    }

    #[tokio::test]
    async fn test_reward_calculation() {
        let mut manager = ConsensusManager::new(100.0, 3600);

        // Register validators with different stakes
        manager.register_node("node1".to_string(), 1000.0).await;
        manager.register_node("node2".to_string(), 500.0).await;

        let round = ConsensusRound::new(
            1,
            "node1".to_string(),
            vec!["node1".to_string(), "node2".to_string()].into_iter().collect()
        );

        let rewards = manager.calculate_rewards(&round).await;

        // Leader (node1) should get higher reward
        assert!(rewards["node1"] > rewards["node2"]);
    }
}
