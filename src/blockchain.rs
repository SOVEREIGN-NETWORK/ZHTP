use crate::consensus::NetworkMetrics;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: i64,
    pub signature: String,
    pub nonce: u64,
    pub data: Vec<u8>,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64) -> Self {
        Transaction {
            from,
            to,
            amount,
            timestamp: Utc::now().timestamp(),
            signature: String::new(),
            nonce: 0,
            data: Vec::new(),
        }
    }

    pub fn with_data(from: String, to: String, amount: f64, data: Vec<u8>) -> Self {
        Transaction {
            from,
            to,
            amount,
            timestamp: Utc::now().timestamp(),
            signature: String::new(),
            nonce: 0,
            data,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}{}",
            self.from, self.to, self.amount, self.timestamp, self.nonce
        );
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn sign(&mut self, private_key: &str) {
        let hash = self.calculate_hash();
        self.signature = format!("{}:{}", private_key, hash);
    }

    pub fn verify_signature(&self, public_key: &str) -> bool {
        if let Some(key) = self.signature.split(':').next() {
            key == public_key
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub validator: String,
    pub validator_score: f64,
    pub network_metrics: Option<NetworkMetrics>,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
        validator: String,
        validator_score: f64,
        network_metrics: Option<NetworkMetrics>,
    ) -> Self {
        let mut block = Block {
            index,
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
            validator,
            validator_score,
            network_metrics,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.timestamp,
            serde_json::to_string(&self.transactions).unwrap(),
            self.previous_hash,
            self.validator,
            self.validator_score
        );
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[derive(Debug, Clone)]
struct ChainState {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    balances: HashMap<String, f64>,
    transaction_nonces: HashMap<String, u64>,
}

impl ChainState {
    fn new() -> Self {
        let mut chain = Vec::new();
        chain.push(Block::new(
            0,
            Vec::new(),
            String::from("0"),
            String::from("genesis"),
            0.0,
            None,
        ));

        Self {
            chain,
            pending_transactions: Vec::new(),
            balances: HashMap::new(),
            transaction_nonces: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    state: Arc<RwLock<ChainState>>,
    pub base_reward: f64,
}

impl Blockchain {
    pub fn new(base_reward: f64) -> Self {
        Self {
            state: Arc::new(RwLock::new(ChainState::new())),
            base_reward,
        }
    }

    pub async fn add_transaction(&self, mut transaction: Transaction) -> bool {
        if transaction.from.is_empty() || transaction.to.is_empty() {
            return false;
        }

        let mut state = self.state.write().await;
        
        // Update nonce
        {
            let entry = state.transaction_nonces
                .entry(transaction.from.clone())
                .or_insert(0);
            transaction.nonce = *entry;
            *entry += 1;
        }

        // Check balance
        if transaction.from != "network" {
            let balance = state.balances.get(&transaction.from).unwrap_or(&0.0);
            if *balance < transaction.amount {
                return false;
            }
        }

        state.pending_transactions.push(transaction);
        true
    }

    pub fn calculate_reward(&self, validator_score: f64, network_metrics: &NetworkMetrics) -> f64 {
        let base = self.base_reward * validator_score;
        let delivery_multiplier = network_metrics.get_delivery_success_rate();
        let latency_multiplier = (1000.0 - network_metrics.average_latency.min(1000.0)) / 1000.0;
        let routing_multiplier = 1.0 + (network_metrics.packets_routed as f64 / 100.0).min(0.2);
        base * delivery_multiplier * latency_multiplier * routing_multiplier
    }

    pub async fn get_latest_block(&self) -> Block {
        let state = self.state.read().await;
        state.chain.last().unwrap().clone()
    }

    pub async fn get_balance(&self, address: &str) -> f64 {
        let state = self.state.read().await;
        *state.balances.get(address).unwrap_or(&0.0)
    }

    pub async fn get_transactions(&self) -> Vec<Transaction> {
        let state = self.state.read().await;
        let mut all_transactions = Vec::new();
        
        // Get transactions from all blocks
        for block in state.chain.iter() {
            all_transactions.extend(block.transactions.clone());
        }
        
        // Add pending transactions
        all_transactions.extend(state.pending_transactions.clone());
        
        all_transactions
    }

    pub async fn create_block(
        &self,
        validator_id: &str,
        validator_score: f64,
        network_metrics: Option<NetworkMetrics>,
    ) {
        let mut state = self.state.write().await;

        // Calculate reward
        let reward = if let Some(metrics) = &network_metrics {
            self.calculate_reward(validator_score, metrics)
        } else {
            self.base_reward * validator_score
        };

        // Create reward transaction
        let mut reward_tx = Transaction::new(
            String::from("network"),
            validator_id.to_string(),
            reward,
        );
        reward_tx.sign("network");

        // Get all transactions
        let mut transactions = Vec::new();
        transactions.push(reward_tx);
        transactions.append(&mut state.pending_transactions);

        // Create new block
        let new_block = Block::new(
            state.chain.len() as u64,
            transactions,
            state.chain.last().unwrap().hash.clone(),
            validator_id.to_string(),
            validator_score,
            network_metrics,
        );

        // Add block and update balances
        state.chain.push(new_block);

        // Update balances
        let mut new_balances = HashMap::new();
        for block in &state.chain {
            for tx in &block.transactions {
                if tx.from != "network" {
                    *new_balances.entry(tx.from.clone()).or_insert(0.0) -= tx.amount;
                }
                *new_balances.entry(tx.to.clone()).or_insert(0.0) += tx.amount;
            }
        }
        state.balances = new_balances;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_rewards() {
        let blockchain = Blockchain::new(100.0);

        let mut good_metrics = NetworkMetrics::new(1000.0);
        good_metrics.packets_routed = 50;
        good_metrics.average_latency = 50.0;
        good_metrics.update_reputation(true);
        good_metrics.update_reputation(true);

        let mut poor_metrics = NetworkMetrics::new(1000.0);
        poor_metrics.packets_routed = 10;
        poor_metrics.average_latency = 500.0;
        poor_metrics.update_reputation(false);
        poor_metrics.update_reputation(false);

        blockchain.create_block("good_node", 0.9, Some(good_metrics.clone())).await;
        blockchain.create_block("poor_node", 0.9, Some(poor_metrics.clone())).await;

        let good_balance = blockchain.get_balance("good_node").await;
        let poor_balance = blockchain.get_balance("poor_node").await;

        assert!(good_balance > poor_balance);
        assert!(good_balance > blockchain.base_reward * 0.9);
        assert!(poor_balance < blockchain.base_reward * 0.9);
    }
}
