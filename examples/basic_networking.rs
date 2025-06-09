use decentralized_network::{ZhtpNode, Keypair, SharedNode};
use std::net::SocketAddr;
use tokio;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Create two nodes - one sender and one receiver
    let sender_addr: SocketAddr = "127.0.0.1:8081".parse()?;
    let receiver_addr: SocketAddr = "127.0.0.1:8082".parse()?;

    // Use same keypair for this example to demonstrate packet verification
    let shared_keypair = Keypair::generate();
    
    let sender = ZhtpNode::new(sender_addr, shared_keypair.clone()).await?;
    
    println!("Created sender node at {}", sender_addr);
    println!("Created receiver node at {}", receiver_addr);

    // Create and start receiver in a separate task
    let receiver_handle = tokio::spawn(async move {
        let receiver = ZhtpNode::new(receiver_addr, shared_keypair)
            .await
            .expect("Failed to create receiver");
        let shared_receiver = SharedNode::new(receiver.clone());
        println!("Starting receiver...");
        if let Err(e) = shared_receiver.start_listening().await {
            eprintln!("Receiver error: {}", e);
        }
    });

    // Give the receiver a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Create and send a test packet
    let test_payload = b"Hello, ZHTP!".to_vec();
    let packet = sender.create_packet(receiver_addr, test_payload).await?;
    println!("Created test packet");

    // Send packet
    sender.send_packet(packet, receiver_addr).await?;
    println!("Sent packet to receiver");

    // Wait a bit to see the received packet
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Clean shutdown (in practice you'd want proper shutdown handling)
    receiver_handle.abort();

    Ok(())
}
