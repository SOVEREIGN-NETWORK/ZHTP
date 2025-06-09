use crate::consensus::NetworkMetrics;
use rand::Rng;
use std::collections::{HashMap, HashSet, VecDeque};

pub type NetworkId = String;

#[derive(Debug, Clone)]
pub struct Packet {
    pub source: NetworkId,
    pub destination: NetworkId,
    pub payload: String,
    pub timestamp: i64,
    visited_nodes: HashSet<NetworkId>,
    size: u64,
    max_hops: u32,
    hop_count: u32,
}

impl Packet {
    pub fn new(source: NetworkId, destination: NetworkId, payload: String, timestamp: i64) -> Self {
        let mut visited = HashSet::new();
        visited.insert(source.clone());
        let size = (payload.len() + 100) as u64; // Base packet size + payload

        Packet {
            source,
            destination,
            payload,
            timestamp,
            visited_nodes: visited,
            size,
            max_hops: 10,
            hop_count: 0,
        }
    }

    fn increment_hop(&mut self) -> bool {
        self.hop_count += 1;
        self.hop_count <= self.max_hops
    }

    fn has_visited(&self, node_id: &str) -> bool {
        self.visited_nodes.contains(node_id)
    }

    fn record_visit(&mut self, node_id: String) {
        self.visited_nodes.insert(node_id);
    }
}

#[derive(Debug, Clone)]
pub struct NetworkCondition {
    pub packet_loss_rate: f64,
    pub latency_multiplier: f64,
    pub bandwidth_cap: Option<usize>,
}

impl Default for NetworkCondition {
    fn default() -> Self {
        NetworkCondition {
            packet_loss_rate: 0.0,
            latency_multiplier: 1.0,
            bandwidth_cap: None,
        }
    }
}

impl NetworkCondition {
    /// Calculate effective drop rate considering all factors
    fn calculate_drop_rate(&self, reputation: f64) -> f64 {
        // Base drop rate is increased by latency
        let latency_factor = self.latency_multiplier.max(1.0);
        let base_rate = self.packet_loss_rate * latency_factor;
        
        // Poor reputation severely increases drop rate
        let rep_penalty = (1.0 - reputation).powf(2.0); // Square for more aggressive penalty
        let adjusted_rate = base_rate * (1.0 + rep_penalty * 5.0); // Increased multiplier
        
        // Cap at 95% to always give some chance
        adjusted_rate.min(0.95)
    }
}

#[derive(Debug)]
pub struct Network {
    nodes: HashMap<NetworkId, Node>,
    message_queue: VecDeque<Packet>,
    delivery_tracking: HashMap<String, bool>,
    network_conditions: HashMap<NetworkId, NetworkCondition>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            nodes: HashMap::new(),
            message_queue: VecDeque::new(),
            delivery_tracking: HashMap::new(),
            network_conditions: HashMap::new(),
        }
    }


    pub fn add_node<S: Into<String>>(&mut self, id: S, stake: f64) {
        let id = id.into();
        self.nodes.insert(id.clone(), Node::new(id.clone(), stake));
        self.network_conditions
            .insert(id, NetworkCondition::default());
    }

    pub fn set_node_condition<S: AsRef<str>>(&mut self, node_id: S, condition: NetworkCondition) {
        self.network_conditions
            .insert(node_id.as_ref().to_string(), condition);
    }

    pub fn connect_nodes<S: AsRef<str>>(&mut self, node1: S, node2: S) {
        let node1 = node1.as_ref().to_string();
        let node2 = node2.as_ref().to_string();

        if let Some(n1) = self.nodes.get_mut(&node1) {
            n1.connections.push(node2.clone());
        }
        if let Some(n2) = self.nodes.get_mut(&node2) {
            n2.connections.push(node1);
        }
    }

    pub fn disconnect_node<S: AsRef<str>>(&mut self, node_id: S) {
        let node_id = node_id.as_ref();
        for node in self.nodes.values_mut() {
            node.connections.retain(|conn| conn != node_id);
        }
    }

    pub fn send_packet(&mut self, source: String, destination: String, payload: String) {
        let packet = Packet::new(
            source.clone(),
            destination.clone(),
            payload,
            chrono::Utc::now().timestamp(),
        );

        let tracking_id = format!("{}:{}:{}", source, destination, packet.timestamp);
        self.delivery_tracking.insert(tracking_id, false);

        self.message_queue.push_back(packet);
    }

    fn handle_failed_delivery(&mut self, node_id: &str, packet: &Packet) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.metrics.update_failed_routing();
            node.metrics.update_reputation(false);
        }

        // Mark the delivery as failed in tracking
        let tracking_id = format!(
            "{}:{}:{}",
            packet.source, packet.destination, packet.timestamp
        );
        // Track delivery outcome
        self.delivery_tracking.insert(tracking_id, true);
    }

    fn attempt_delivery(&mut self, packet: &Packet) -> bool {
        let dest_id = packet.destination.clone();
        let source_id = packet.source.clone();
        let tracking_id = format!("{}:{}:{}", source_id, dest_id, packet.timestamp);

        let condition = self.network_conditions.get(&dest_id)
            .cloned()
            .unwrap_or_default();
        
        let reputation = self.nodes.get(&dest_id)
            .map(|n| n.metrics.reputation_score)
            .unwrap_or(1.0);
        
        let base_drop_rate = condition.packet_loss_rate * condition.latency_multiplier;
        let rep_penalty = (1.0 - reputation).powf(2.0);
        let final_drop_rate = (base_drop_rate + (rep_penalty * base_drop_rate)).min(0.95);
        
        println!("Delivery check for {}: base_rate={:.3}, penalty={:.3}, final_rate={:.3}, rep={:.2}",
                dest_id, base_drop_rate, rep_penalty, final_drop_rate, reputation);

        if !self.nodes.contains_key(&dest_id) || rand::thread_rng().gen::<f64>() < final_drop_rate {
            // Handle failed delivery with more nuanced penalties
            if let Some(dest_node) = self.nodes.get_mut(&dest_id) {
                // Only apply reputation penalty under good conditions
                if base_drop_rate < 0.3 && reputation > 0.5 {
                    dest_node.metrics.update_reputation(false);
                }
                
                // Track metrics regardless of conditions
                dest_node.metrics.update_failed_routing();
            }
            return false;
        }

        // Attempt delivery
        let latency = self.calculate_node_latency(&dest_id);
        if let Some(dest_node) = self.nodes.get_mut(&dest_id) {
            dest_node.receive_packet(packet.clone());
            dest_node.metrics.update_routing_metrics(latency, packet.size.try_into().unwrap());
            self.delivery_tracking.insert(tracking_id, true);

            // Update source node reputation
            if let Some(source_node) = self.nodes.get_mut(&source_id) {
                source_node.metrics.update_reputation(true);
            }
            true
        } else {
            false
        }
    }

    fn try_forward_packet(
        &mut self,
        new_messages: &mut VecDeque<Packet>,
        packet: &Packet,
        next_hop: &str,
    ) -> bool {
        // Get network conditions and calculate drop probability
        let condition = self.network_conditions.get(next_hop)
            .cloned()
            .unwrap_or_default();
        
        // Get current reputation
        let reputation = self.nodes.get(next_hop)
            .map(|n| n.metrics.reputation_score)
            .unwrap_or(1.0);
            
        // Calculate effective drop rate including network conditions
        let base_drop_rate = condition.packet_loss_rate * condition.latency_multiplier;
        
        // Adjust drop rate based on reputation
        // Calculate drop rate adjustment based on reputation
        let base_drop_rate = condition.packet_loss_rate * condition.latency_multiplier;
        let modifier = if reputation > 0.8 {
            -0.2  // Good reputation reduces drop rate
        } else if reputation < 0.3 {
            0.2   // Bad reputation increases drop rate
        } else {
            0.0   // Neutral effect for mid-range reputation
        };
        
        let final_drop_rate = (base_drop_rate + modifier).clamp(0.05, 0.95);
        
        println!("Drop check for {}: base_rate={:.3}, modifier={:.3}, final_rate={:.3}, rep={:.2}",
                next_hop, base_drop_rate, modifier, final_drop_rate, reputation);
                
        // Check if packet should be dropped
        if !self.nodes.contains_key(next_hop) ||
           rand::thread_rng().gen::<f64>() < final_drop_rate {
            // Calculate latency even for failed attempts
            let latency = self.calculate_node_latency(next_hop);
            if let Some(next_node) = self.nodes.get_mut(next_hop) {
                // Update metrics with high latency for failed attempt
                next_node.metrics.update_routing_metrics(latency * 2.0, packet.size.try_into().unwrap());
                
                // Apply penalties based on conditions and current performance
                let expected_fails = condition.packet_loss_rate * condition.latency_multiplier;
                if expected_fails < 0.3 && reputation > 0.8 {
                    // Only penalize if conditions are good and reputation is high
                    next_node.metrics.update_reputation(false);
                } else if expected_fails < 0.5 && reputation > 0.6 {
                    // Light penalty for moderate conditions
                    next_node.metrics.update_reputation(false);
                }
                // Track failure but don't penalize reputation under poor conditions
                next_node.metrics.update_failed_routing();
            }
            return false;
        }

        let latency = self.calculate_node_latency(next_hop);
        if let Some(next_node) = self.nodes.get_mut(next_hop) {
            // Update metrics and apply reputation boost based on conditions
            next_node.metrics.update_routing_metrics(latency, packet.size.try_into().unwrap());
            let mut new_packet = packet.clone();
            new_packet.record_visit(next_hop.to_string());
            new_messages.push_back(new_packet);
            
            // Handle successful forward
            let condition = self.network_conditions.get(next_hop)
                .cloned()
                .unwrap_or_default();

            // Calculate difficulty and expected failure rate
            let difficulty = condition.packet_loss_rate * condition.latency_multiplier;
            
            // Calculate reputation boost based on conditions
            let boost_count = if difficulty > 0.8 {
                3  // Major boost for success under extreme conditions
            } else if difficulty > 0.5 {
                2  // Medium boost for difficult conditions
            } else {
                1  // Normal boost for good conditions
            };

            // Apply reputation boosts
            for _ in 0..boost_count {
                next_node.metrics.update_reputation(true);
            }

            // Small additional boost for consistently good performance
            if next_node.metrics.reputation_score > 0.7 && difficulty < 0.3 {
                next_node.metrics.update_reputation(true);
            }
            
            return true;
        }
        false
    }

    fn calculate_node_latency(&self, node_id: &str) -> f64 {
        let base_latency = rand::thread_rng().gen_range(10.0..200.0);
        if let Some(condition) = self.network_conditions.get(node_id) {
            base_latency * condition.latency_multiplier
        } else {
            base_latency
        }
    }

    /// Evaluate a node's current routing score (higher is better)
    fn evaluate_node_score(&self, node_id: &str) -> f64 {
        // Get node's current reputation
        let reputation = self.nodes.get(node_id)
            .map(|n| n.metrics.reputation_score)
            .unwrap_or(0.0);

        // Get network conditions
        let condition = self.network_conditions.get(node_id)
            .cloned()
            .unwrap_or_default();

        // Calculate effective drop rate
        let drop_rate = condition.packet_loss_rate * condition.latency_multiplier;
        
        // Scale down high drop rates less aggressively
        let condition_multiplier = 1.0 - (drop_rate * 1.5).min(0.6);
        
        // Base score on reputation and conditions
        let score = reputation * condition_multiplier;
        
        // Add small base chance but cap maximum
        (score + 0.05).min(0.95).max(0.05)
    }

    pub fn process_messages(&mut self) {
        let mut new_messages = VecDeque::new();

        while let Some(mut packet) = self.message_queue.pop_front() {
            if !packet.increment_hop() {
                self.handle_failed_delivery(&packet.source, &packet);
                continue;
            }

            // Only attempt direct delivery if the destination is a direct neighbor
            let current_id = packet.visited_nodes.iter().last().unwrap_or(&packet.source).clone();
            let can_deliver_direct = if let Some(current_node) = self.nodes.get(&current_id) {
                current_node.connections.contains(&packet.destination)
            } else {
                false
            };

            if can_deliver_direct && self.attempt_delivery(&packet) {
                continue;
            }

            // Get current node and its connections
            let current_id = packet
                .visited_nodes
                .iter()
                .last()
                .unwrap_or(&packet.source)
                .clone();

            // Get and sort available next hops by score
            let mut candidates = Vec::new();
            if let Some(current_node) = self.nodes.get(&current_id) {
                for conn in &current_node.connections {
                    if !packet.has_visited(conn) {
                        let score = self.evaluate_node_score(conn);
                        candidates.push((conn.clone(), score));
                    }
                }
            }

            // Sort by score and packet loss rate
            candidates.sort_by(|(a_id, a_score), (b_id, b_score)| {
                let a_loss = self.network_conditions.get(a_id)
                    .map(|c| c.packet_loss_rate)
                    .unwrap_or(0.0);
                let b_loss = self.network_conditions.get(b_id)
                    .map(|c| c.packet_loss_rate)
                    .unwrap_or(0.0);
                
                // Primary sort by score, secondary by packet loss
                match b_score.partial_cmp(a_score) {
                    Some(ord) if ord == std::cmp::Ordering::Equal => {
                        a_loss.partial_cmp(&b_loss).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    Some(ord) => ord,
                    None => std::cmp::Ordering::Equal
                }
            });

            // Try forwarding through each candidate
            let mut forwarded = false;
            let mut attempted_nodes = Vec::new();

            for (next_hop, score) in candidates {
                attempted_nodes.push(next_hop.clone());
                let condition = self.network_conditions.get(&next_hop)
                    .cloned()
                    .unwrap_or_default();
                println!("Attempting route through {}: score={:.3}, drop_rate={:.3}, latency={:.1}x",
                    next_hop, score, condition.packet_loss_rate, condition.latency_multiplier);
                
                if self.try_forward_packet(&mut new_messages, &packet, &next_hop) {
                    println!("Successfully forwarded through {}", next_hop);
                    forwarded = true;
                    break;
                } else {
                    println!("Failed to forward through {} - packet dropped", next_hop);
                }

                // Penalize based on base conditions and current reputation
                if let Some(node) = self.nodes.get_mut(&next_hop) {
                    let condition = self.network_conditions.get(&next_hop)
                        .cloned()
                        .unwrap_or_default();
                        
                    // Adjust reputation based on failure context
                    let expected_fails = condition.packet_loss_rate * condition.latency_multiplier;
                    
                    // Apply penalties only under good conditions
                    if expected_fails < 0.3 {
                        // Apply penalty if reputation is too high for performance
                        if node.metrics.reputation_score > 0.8 {
                            node.metrics.update_reputation(false);
                        }
                    }
                    
                    // Always track metrics
                    node.metrics.update_failed_routing();
                }
            }

            // Apply penalties only if packet cannot be forwarded through any path
            if !forwarded {
                for next_hop in attempted_nodes {
                    if let Some(node) = self.nodes.get_mut(&next_hop) {
                        let condition = self.network_conditions.get(&next_hop)
                            .cloned()
                            .unwrap_or_default();
                        let expected_fails = condition.packet_loss_rate * condition.latency_multiplier;
                        
                        // Only track metrics and apply penalties under specific conditions
                        if expected_fails < 0.2 && node.metrics.reputation_score > 0.8 {
                            node.metrics.update_reputation(false);
                            node.metrics.update_failed_routing();
                        }
                    }
                }
                self.handle_failed_delivery(&current_id, &packet);
            }
        }

        self.message_queue.extend(new_messages);
    }

    pub fn get_node_metrics<S: AsRef<str>>(&self, node_id: S) -> Option<&NetworkMetrics> {
        self.nodes.get(node_id.as_ref()).map(|node| &node.metrics)
    }

    pub fn get_delivery_success_rate(&self) -> f64 {
        let total = self.delivery_tracking.len();
        if total == 0 {
            return 1.0;
        }

        let successful = self
            .delivery_tracking
            .values()
            .filter(|&&success| success)
            .count();

        successful as f64 / total as f64
    }
}

#[derive(Debug)]
pub struct Node {
    id: NetworkId,
    connections: Vec<NetworkId>,
    metrics: NetworkMetrics,
    received_messages: Vec<String>,
}

impl Node {
    pub fn new<S: Into<String>>(id: S, stake: f64) -> Self {
        Node {
            id: id.into(),
            connections: Vec::new(),
            metrics: NetworkMetrics::new(stake),
            received_messages: Vec::new(),
        }
    }

    pub fn receive_packet(&mut self, packet: Packet) {
        if packet.destination == self.id {
            self.received_messages.push(packet.payload);
        }
    }

    pub fn get_received_messages(&self) -> &[String] {
        &self.received_messages
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degraded_network() {
        let mut network = Network::new();

        // Add nodes in a more complex topology
        network.add_node("node1", 1000.0);
        network.add_node("node2", 1000.0);
        network.add_node("node3", 1000.0);
        network.add_node("node4", 1000.0);

        // Connect nodes in a diamond pattern
        // node1 -> node2 -> node4
        //      \-> node3 -/
        network.connect_nodes("node1", "node2");
        network.connect_nodes("node1", "node3");
        network.connect_nodes("node2", "node4");
        network.connect_nodes("node3", "node4");

        // Clear initial default conditions
        network.set_node_condition("node2", NetworkCondition::default());
        network.set_node_condition("node3", NetworkCondition::default());

        // Set node2 with extremely poor conditions
        network.set_node_condition(
            "node2",
            NetworkCondition {
                packet_loss_rate: 0.9, // 90% base packet loss
                latency_multiplier: 5.0, // 5x normal latency
                bandwidth_cap: Some(100), // Severely limited bandwidth
            },
        );

        // Set node3 with slightly degraded conditions
        network.set_node_condition(
            "node3",
            NetworkCondition {
                packet_loss_rate: 0.05, // 5% base packet loss
                latency_multiplier: 1.1, // Only slight latency increase
                bandwidth_cap: Some(10000), // Better bandwidth
            },
        );

        // Initialize node2 with baseline reputation
        if let Some(node2) = network.nodes.get_mut("node2") {
            // Give some initial reputation to lose
            node2.metrics.update_reputation(true);
        }

        // Get starting conditions
        if let Some(metrics) = network.get_node_metrics("node2") {
            println!("Initial Node2 reputation: {:.2}", metrics.reputation_score);
        }

        // Send messages with immediate processing
        for i in 0..10 {
            // Send packet
            network.send_packet(
                "node1".to_string(),
                "node4".to_string(),
                format!("Message {}", i),
            );

            // Process immediately to adapt to conditions
            // Process messages and track metrics
            network.process_messages();
            
            // Print current metrics after each round
            println!("Messages in queue: {}", network.message_queue.len());
            let success_rate = network.get_delivery_success_rate();
            println!("Current success rate: {:.1}%", success_rate * 100.0);

            // Let the natural packet processing handle reputation updates
            if let Some(metrics) = network.get_node_metrics("node2") {
                println!("Current Node2 reputation: {:.2}", metrics.reputation_score);
            }
        }

        // Final processing rounds to ensure delivery
        for _ in 0..5 {
            network.process_messages();
        }

        // Process final metrics
        let success_rate = network.get_delivery_success_rate();
        println!("\nFinal Network Metrics:");
        println!("Success rate: {:.1}%", success_rate * 100.0);
        println!("Messages delivered: {}", network.delivery_tracking.len());

        // Success rate should be reasonable with alternate path
        assert!(
            success_rate > 0.3,
            "Success rate {} should be higher with alternate path",
            success_rate
        );

        // Verify node2's degraded performance
        if let Some(metrics) = network.get_node_metrics("node2") {
            println!("Node2 metrics:");
            println!("  Delivery failures: {}", metrics.delivery_failures);
            println!("  Average latency: {:.2}ms", metrics.average_latency);
            println!("  Reputation score: {:.2}", metrics.reputation_score);

            assert!(
                metrics.reputation_score < 0.7,
                "Node2 reputation should decrease"
            );
            assert!(
                metrics.delivery_failures > 0,
                "Node2 should have failed packets"
            );
            assert!(
                metrics.average_latency > 100.0,
                "Node2 should show increased latency"
            );
        }

        // Verify node3's better performance
        if let Some(metrics) = network.get_node_metrics("node3") {
            println!("Node3 metrics:");
            println!("  Delivery failures: {}", metrics.delivery_failures);
            println!("  Average latency: {:.2}ms", metrics.average_latency);
            println!("  Reputation score: {:.2}", metrics.reputation_score);

            assert!(
                metrics.reputation_score > 0.7,
                "Node3 reputation should remain high"
            );
            assert!(
                metrics.average_latency < 200.0,
                "Node3 should have lower latency"
            ); // Checking against reasonable threshold
        }

        // Verify that node4 received messages
        if let Some(node4) = network.nodes.get("node4") {
            let received = node4.get_received_messages().len();
            println!("Messages received by node4: {}", received);
            assert!(received > 0, "Node4 should have received some messages");
        }
    }
}
