use anyhow::Result;
use decentralized_network::{Keypair, ZhtpNode, SharedNode, NetworkMetrics};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

async fn print_node_metrics(node: &ZhtpNode, name: &str) {
    let metrics = node.get_routing_metrics();
    println!("=== {} Metrics ===", name);
    println!("Success Rate: {:.1}%", metrics.get_delivery_success_rate() * 100.0);
    println!("Messages Routed: {}", metrics.packets_routed);
    println!("Average Latency: {:.1}ms", metrics.average_latency);
    println!("Reputation Score: {:.1}%", metrics.reputation_score * 100.0);
    println!("================");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Create nodes
    let addr_a: SocketAddr = "127.0.0.1:9001".parse()?;
    let addr_b: SocketAddr = "127.0.0.1:9002".parse()?;
    let addr_c: SocketAddr = "127.0.0.1:9003".parse()?;
    let addr_d: SocketAddr = "127.0.0.1:9004".parse()?;

    // Create network topology:
    //     B
    //   /   \
    // A       C
    //   \   /
    //     D

    // Create a shared keypair for testing
    let shared_keypair = Keypair::generate();

    println!("Creating nodes with shared keypair...");
    let mut node_a = ZhtpNode::new(addr_a, shared_keypair.clone()).await?;
    let node_b = ZhtpNode::new(addr_b, shared_keypair.clone()).await?;
    let node_c = ZhtpNode::new(addr_c, shared_keypair.clone()).await?;
    let node_d = ZhtpNode::new(addr_d, shared_keypair.clone()).await?;

    // Set up connections
    println!("Setting up network connections...");
    node_a.connect(addr_b).await?;
    node_a.connect(addr_d).await?;

    // Create listening nodes
    let node_b = Arc::new(Mutex::new(node_b));
    let node_c = Arc::new(Mutex::new(node_c));
    let node_d = Arc::new(Mutex::new(node_d));

    // Connect nodes with proper locking
    {
        let mut node_b = node_b.lock().await;
        node_b.connect(addr_c).await?;
    }
    {
        let mut node_d = node_d.lock().await;
        node_d.connect(addr_c).await?;
    }

    // Start nodes listening in separate tasks
    let node_b_shared = SharedNode::new({
        let n = node_b.lock().await;
        n.clone()
    });
    let node_b_handle = tokio::spawn(async move {
        println!("Node B listening on {}", addr_b);
        if let Err(e) = node_b_shared.start_listening().await {
            eprintln!("Node B error: {}", e);
        }
    });

    let node_c_shared = SharedNode::new({
        let n = node_c.lock().await;
        n.clone()
    });
    let node_c_handle = tokio::spawn(async move {
        println!("Node C listening on {}", addr_c);
        if let Err(e) = node_c_shared.start_listening().await {
            eprintln!("Node C error: {}", e);
        }
    });

    let node_d_shared = SharedNode::new({
        let n = node_d.lock().await;
        n.clone()
    });
    let node_d_handle = tokio::spawn(async move {
        println!("Node D listening on {}", addr_d);
        if let Err(e) = node_d_shared.start_listening().await {
            eprintln!("Node D error: {}", e);
        }
    });

    // Give nodes time to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Send test messages from A to C through different paths
    println!("\nSending messages from A to C...");

    // First message - should go through B
    let msg1 = b"Message 1: Through node B".to_vec();
    let packet1 = node_a.create_packet(addr_c, msg1).await?;
    node_a.send_packet(packet1, addr_b).await?;
    println!("Sent message 1 via node B");

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Second message - should go through D
    let msg2 = b"Message 2: Through node D".to_vec();
    let packet2 = node_a.create_packet(addr_c, msg2).await?;
    node_a.send_packet(packet2, addr_d).await?;
    println!("Sent message 2 via node D");

    // Wait to see messages arrive
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Print metrics
    print_node_metrics(&node_a, "Node A").await;

    // Clean shutdown
    node_b_handle.abort();
    node_c_handle.abort();
    node_d_handle.abort();

    println!("\nTest complete");
    Ok(())
}
