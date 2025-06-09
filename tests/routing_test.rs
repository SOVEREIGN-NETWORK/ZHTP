use anyhow::Result;
use decentralized_network::{
    zhtp::{Keypair, ZhtpNode},
    Arc, Mutex,
};
use std::net::SocketAddr;
use tokio;

#[tokio::test]
async fn test_multi_node_routing() -> Result<()> {
    // Create node addresses
    let addr_a: SocketAddr = "127.0.0.1:9701".parse()?;
    let addr_b: SocketAddr = "127.0.0.1:9702".parse()?;
    let addr_c: SocketAddr = "127.0.0.1:9703".parse()?;

    // Create and wrap nodes
    let node_a = Arc::new(Mutex::new(ZhtpNode::new(addr_a, Keypair::generate()).await?));
    let node_b = Arc::new(Mutex::new(ZhtpNode::new(addr_b, Keypair::generate()).await?));
    let node_c = Arc::new(Mutex::new(ZhtpNode::new(addr_c, Keypair::generate()).await?));

    // Start all nodes listening first
    let node_a_listen = node_a.clone();
    let node_b_listen = node_b.clone();
    let node_c_listen = node_c.clone();

    let listen_a = tokio::spawn(async move {
        ZhtpNode::start_listening_shared(node_a_listen).await
    });

    let listen_b = tokio::spawn(async move {
        ZhtpNode::start_listening_shared(node_b_listen).await
    });

    let listen_c = tokio::spawn(async move {
        ZhtpNode::start_listening_shared(node_c_listen).await
    });

    // Give nodes time to start and verify connections
    let mut retries = 0;
    let max_retries = 20;  // Increased retries
    while retries < max_retries {
        let a_ready = node_a.lock().await.check_ready().await;
        let b_ready = node_b.lock().await.check_ready().await;
        let c_ready = node_c.lock().await.check_ready().await;
        
        if a_ready && b_ready && c_ready {
            println!("All nodes ready");
            break;
        }
        println!("Waiting for nodes to be ready (attempt {}/{})", retries + 1, max_retries);
        
        retries += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        if retries == max_retries {
            return Err(anyhow::anyhow!("Nodes failed to become ready"));
        }
    }

    // Create and send test packet from A to C
    let test_payload = b"Hello through the route!".to_vec();
    let mut guard = node_a.lock().await;
    let packet = guard.create_packet(addr_c, test_payload).await?;
    println!("Sending packet from A to C through B...");
    guard.send_packet(packet, addr_b).await?;
    drop(guard);

    // Wait a bit for packet to be forwarded
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Clean shutdown
    listen_a.abort();
    listen_b.abort();
    listen_c.abort();

    Ok(())
}

#[tokio::test]
async fn test_route_failure_handling() -> Result<()> {
    // Create node addresses
    let addr_a: SocketAddr = "127.0.0.1:9801".parse()?;
    let addr_b: SocketAddr = "127.0.0.1:9802".parse()?;
    let addr_c: SocketAddr = "127.0.0.1:9803".parse()?;
    let addr_d: SocketAddr = "127.0.0.1:9804".parse()?;

    // Create nodes with shared state
    let node_a = Arc::new(Mutex::new(ZhtpNode::new(addr_a, Keypair::generate()).await?));
    let node_b = Arc::new(Mutex::new(ZhtpNode::new(addr_b, Keypair::generate()).await?));
    let node_c = Arc::new(Mutex::new(ZhtpNode::new(addr_c, Keypair::generate()).await?));
    let node_d = Arc::new(Mutex::new(ZhtpNode::new(addr_d, Keypair::generate()).await?));

    // Start listeners first
    let node_a_listen = node_a.clone();
    let node_b_listen = node_b.clone();
    let node_c_listen = node_c.clone();
    let node_d_listen = node_d.clone();

    let listen_a = tokio::spawn(async move {
        ZhtpNode::start_listening_shared(node_a_listen).await
    });

    let listen_b = tokio::spawn(async move {
        ZhtpNode::start_listening_shared(node_b_listen).await
    });

    let listen_c = tokio::spawn(async move {
        ZhtpNode::start_listening_shared(node_c_listen).await
    });

    let listen_d = tokio::spawn(async move {
        ZhtpNode::start_listening_shared(node_d_listen).await
    });

    // Give time for listeners to start and verify connections
    let mut retries = 0;
    let max_retries = 10;
    while retries < max_retries {
        let b_ready = node_b.lock().await.check_ready().await;
        let c_ready = node_c.lock().await.check_ready().await;
        let d_ready = node_d.lock().await.check_ready().await;
        
        if b_ready && c_ready && d_ready {
            println!("All nodes ready");
            break;
        }
        
        retries += 1;
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        if retries == max_retries {
            return Err(anyhow::anyhow!("Nodes failed to become ready"));
        }
    }

    // Send test packets
    {
        let mut guard = node_a.lock().await;
        for i in 1..=5 {
            let payload = format!("Test packet {}", i).into_bytes();
            let packet = guard.create_packet(addr_c, payload).await?;
            println!("Sending packet {} from A to C...", i);
            guard.send_packet(packet, addr_b).await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
        drop(guard);
    }

    // Clean shutdown
    listen_a.abort();
    listen_b.abort();
    listen_c.abort();
    listen_d.abort();

    Ok(())
}
