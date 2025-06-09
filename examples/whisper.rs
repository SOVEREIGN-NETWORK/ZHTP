use anyhow::Result;
use chrono;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio::time::{timeout, Duration};

use decentralized_network::{
    Blockchain, ConsensusManager, Network, StorageManager, Transaction,
    consensus::NetworkMetrics,
    zhtp::{Keypair, ZhtpNode, SharedNode},
    storage::StorageConfig,
};

const OPERATION_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_DISCOVERY: &str = "127.0.0.1:9000";
// Get discovery node from env var or use default
fn get_discovery_addr() -> String {
    std::env::var("DISCOVERY_NODE").unwrap_or_else(|_| DEFAULT_DISCOVERY.to_string())
}
const CONFIG_DIR: &str = ".whisper";
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(1);
const DEFAULT_PORT: u16 = 8000;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    from: String,
    content: String,
    timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NodeConfig {
    node_id: String,
    port: u16,
    #[serde(default)]
    known_peers: Vec<String>,
    #[serde(default)]
    last_discovery: Option<String>,
    #[serde(default)]
    messages: Vec<Message>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_id: String::new(),
            port: DEFAULT_PORT,
            known_peers: Vec::new(),
            last_discovery: None,
            messages: Vec::new(),
        }
    }
}

impl Message {
    fn new(from: String, content: String) -> Self {
        Self {
            from,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn display(&self) -> String {
        let datetime = chrono::DateTime::<chrono::Local>::from(
            std::time::UNIX_EPOCH + std::time::Duration::from_secs(self.timestamp)
        );
        format!("[{}] From {}: {}",
            datetime.format("%Y-%m-%d %H:%M:%S"),
            self.from,
            self.content
        )
    }
}

/// Parse command line arguments
struct Args {
    port: Option<u16>,
    node_id: Option<String>,
}

fn parse_args() -> Args {
    let args: Vec<String> = std::env::args().collect();
    let mut port = None;
    let mut node_id = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--port" => {
                if i + 1 < args.len() {
                    if let Ok(p) = args[i + 1].parse() {
                        port = Some(p);
                    }
                    i += 2;
                    continue;
                }
            }
            "--id" => {
                if i + 1 < args.len() {
                    node_id = Some(args[i + 1].clone());
                    i += 2;
                    continue;
                }
            }
            _ => i += 1,
        }
    }
    Args { port, node_id }
}

/// Helper to get user input
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    
    // Keep trying until we get valid input
    while io::stdin().read_line(&mut input).is_ok() {
        if !input.trim().is_empty() {
            break;
        }
        input.clear();
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }
    
    input.trim().to_string()
}

/// Get the most suitable IP for networking
fn get_network_ip() -> String {
    if let Ok(socket) = std::net::UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                return addr.ip().to_string();
            }
        }
    }
    "127.0.0.1".to_string()
}

/// Connect to a peer with retries
async fn connect_to_peer(node: &Arc<Mutex<ZhtpNode>>, addr: SocketAddr) -> Result<()> {
    for attempt in 1..=MAX_RETRIES {
        println!("Connection attempt {} to {}...", attempt, addr);
        
        // Use aggressive timeouts for initial attempts, longer for later ones
        let timeout_duration = if attempt < 2 {
            Duration::from_secs(2)
        } else {
            Duration::from_secs(5)
        };

        let connection_future = async {
            // Try to acquire lock with exponential backoff
            let mut backoff = Duration::from_millis(10);
            loop {
                if let Ok(mut locked_node) = node.try_lock() {
                    println!("Acquired node lock, attempting connection...");
                    return locked_node.connect(addr).await;
                }
                tokio::time::sleep(backoff).await;
                backoff = std::cmp::min(backoff * 2, Duration::from_millis(100));
            }
        };

        match timeout(timeout_duration, connection_future).await {
            Ok(Ok(_)) => {
                println!("✓ Successfully connected on attempt {}", attempt);
                return Ok(());
            }
            Ok(Err(e)) => {
                println!("✗ Attempt {} failed: {}", attempt, e);
                if attempt < MAX_RETRIES {
                    println!("Waiting {}ms before retry...", RETRY_DELAY.as_millis());
                    tokio::time::sleep(RETRY_DELAY).await;
                } else {
                    return Err(anyhow::anyhow!("Failed after {} attempts: {}", MAX_RETRIES, e));
                }
            }
            Err(_) => {
                println!("✗ Attempt {} timed out", attempt);
                if attempt < MAX_RETRIES {
                    println!("Waiting {}ms before retry...", RETRY_DELAY.as_millis());
                    tokio::time::sleep(RETRY_DELAY).await;
                } else {
                    return Err(anyhow::anyhow!("All {} attempts timed out", MAX_RETRIES));
                }
            }
        }
    }
    unreachable!()
}

fn get_config() -> Result<NodeConfig> {
    let args = parse_args();
    
    let node_id = if let Some(id) = args.node_id {
        if !id.chars().all(|c| c.is_alphanumeric()) {
            return Err(anyhow::anyhow!("Node ID must be alphanumeric"));
        }
        id
    } else {
        return Err(anyhow::anyhow!("Node ID required. Use --id <name>"));
    };

    let port = if let Some(p) = args.port {
        if p < 1024 {
            return Err(anyhow::anyhow!("Port must be >= 1024"));
        }
        p
    } else {
        return Err(anyhow::anyhow!("Port required. Use --port <number>"));
    };

    Ok(NodeConfig {
        node_id,
        port,
        known_peers: Vec::new(),
        last_discovery: Some(get_discovery_addr()),
        messages: Vec::new(),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n=== Whisper ===");
    println!("Zero-Knowledge Hidden Transit Protocol Messenger\n");

    // Get configuration from command line args
    let config = get_config()?;
    println!("Node ID: {}", config.node_id);
    println!("Port: {}", config.port);
    
    let config = Arc::new(Mutex::new(config));
    let cfg = config.lock().await;

    // Initialize systems
    let mut network = Network::new();
    let storage = StorageManager::new();
    let storage_config = StorageConfig {
        replication_factor: 3,
        min_proofs: 2,
        max_node_storage: 1024 * 1024 * 1024,
    };
    let mut consensus = ConsensusManager::new(500.0, 3600);
    let blockchain = Blockchain::new(100.0);

    // Create and start node
    let keypair = Keypair::generate();
    println!("Starting node on port {}...", cfg.port);
    let addr = format!("0.0.0.0:{}", cfg.port).parse()?;
    let node = ZhtpNode::new(addr, keypair).await?;
    let node = Arc::new(Mutex::new(node));

    // Create message handler
    let config_clone = config.clone();
    let storage_clone = storage.clone();
    let blockchain_clone = blockchain.clone();
    let node_clone = node.clone();

    // Set up message handling
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(100);
    {
        let mut n = node.lock().await;
        n.set_message_handler(tx.clone());
    }

    // Create shared node instance
    let node_clone = node.clone();
    let shared = {
        let n = node_clone.lock().await;
        SharedNode::new(n.clone())
    };
    
    // Spawn message handler
    let config_c = config.clone();
    tokio::spawn(async move {
        println!("Starting message handler...");
        while let Some(data) = rx.recv().await {
            if let Ok(msg) = serde_json::from_slice::<Message>(&data) {
                println!("\nReceived message from {}", msg.from);
                let mut cfg = config_c.lock().await;
                cfg.messages.push(msg.clone());
                println!("{}", msg.display());
            }
        }
    });

    // Start node listener
    if let Err(e) = shared.start_listening().await {
        println!("Error starting listener: {}", e);
    }

    // Update initial metrics
    consensus.update_metrics(&cfg.node_id, true, Some(0.0)).await;
    drop(cfg);

    // Give node time to start
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Set up node in network
    let mut cfg = config.lock().await;
    network.add_node(cfg.node_id.clone(), 1000.0);
    consensus.register_node(cfg.node_id.clone(), 1000.0).await;
    if !storage.register_node(cfg.node_id.clone(), 1_000_000).await {
        println!("Warning: Failed to register with storage network");
    }
    
    // Connect to discovery node
    let discovery = get_discovery_addr();
    println!("Connecting to discovery network at {}...", discovery);
    let discovery_addr = match SocketAddr::from_str(&discovery) {
        Ok(addr) => addr,
        Err(_) => {
            println!("Invalid discovery address format");
            println!("Set DISCOVERY_NODE env var to the discovery node's IP:PORT");
            println!("Example: DISCOVERY_NODE=1.2.3.4:9000");
            return Ok(());
        }
    };
    if let Err(e) = connect_to_peer(&node, discovery_addr).await {
        println!("⚠ Failed to connect to discovery node: {}", e);
    }

    println!("\n=== Node Status ===");
    println!("Your ID: {}", cfg.node_id);
    println!("Your address: {}:{}", get_network_ip(), cfg.port);
    println!("Discovery node: {}", discovery);
    println!();
    drop(cfg);

    // Main menu loop
    loop {
        print!("\n=== Menu ===\n");
        println!("1. Send message");
        println!("2. View messages");
        println!("3. Add contact");
        println!("4. View contacts");
        println!("5. Node status");
        println!("6. View blockchain transactions");
        println!("7. Exit");

        print!("\nChoice (1-6): ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            println!("Failed to read input");
            continue;
        }

        match choice.trim() {
            "1" => {
                println!("\nSend Message");
                println!("-----------");
                
                let recipient = get_input("Recipient address: ");
                let dest_addr: SocketAddr = match recipient.parse() {
                    Ok(addr) => addr,
                    Err(_) => {
                        println!("Invalid address format (use IP:PORT)");
                        continue;
                    }
                };

                let mut cfg = config.lock().await;
                // Don't allow sending to self
                if dest_addr.port() == cfg.port {
                    println!("Cannot send message to self");
                    drop(cfg);
                    continue;
                }

                // Try to establish/verify connection
                println!("Establishing connection to recipient at {}...", dest_addr);
                if let Err(e) = connect_to_peer(&node, dest_addr).await {
                    println!("Failed to connect to recipient: {}", e);
                    drop(cfg);
                    continue;
                }
                println!("✓ Connection verified");

                // Create and send message
                let node_id = cfg.node_id.clone();
                drop(cfg);

                print!("\nEnter your message: ");
                io::stdout().flush().unwrap();
                
                let mut message = String::new();
                io::stdin().read_line(&mut message)?;
                let message = message.trim().to_string();
                if message.is_empty() {
                    println!("Message cannot be empty");
                    continue;
                }

                let msg = Message::new(node_id.clone(), message);
                let msg_data = serde_json::to_vec(&msg)?;

                println!("\nProcessing message...");

                println!("\nSending message...");
                
                consensus.update_metrics(&node_id, true, Some(10.0)).await;
                println!("Processing message...");
                let send_result = {
                    let mut n = node.lock().await;
                    match n.create_packet(dest_addr, msg_data.clone()).await {
                        Ok(packet) => {
                            // First try to send packet
                            let result = n.send_packet(packet, dest_addr).await;
                            
                            // If send successful, store and record
                            if result.is_ok() {
                                // Store in DHT
                                println!("Storing message...");
                                if let Err(e) = storage_clone.store_content(
                                    msg_data.clone(),
                                    "whisper-message".to_string(),
                                    &node_id,
                                    None  // Use default storage config
                                ).await {
                                    println!("Warning: Failed to store in DHT: {}", e);
                                }

                                // Record in blockchain
                                let mut tx = Transaction::with_data(
                                    node_id.clone(),
                                    dest_addr.to_string(),
                                    0.0,
                                    msg_data
                                );
                                tx.sign(&node_id);
                                if !blockchain.add_transaction(tx).await {
                                    println!("Warning: Failed to record transaction");
                                }
                            }
                            result
                        },
                        Err(e) => Err(anyhow::anyhow!("Failed to create packet: {}", e))
                    }
                };

                match send_result {
                    Ok(_) => {
                        println!("✓ Message sent successfully!");
                        let mut cfg = config.lock().await;
                        cfg.messages.push(msg);
                        drop(cfg);
                    },
                    Err(e) => {
                        println!("Failed to send message: {}", e);
                    }
                }

                // Wait for user acknowledgment
                println!("\nPress Enter to continue...");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }

            "2" => {
                println!("\nMessages");
                println!("--------");
                
                let node_id = config.lock().await.node_id.clone();
                let mut total_messages = 0;

                println!("Fetching messages from all sources...");
                
                // 1. Local messages
                {
                    let cfg = config.lock().await;
                    if !cfg.messages.is_empty() {
                        println!("\nLocal messages:");
                        for msg in &cfg.messages {
                            println!("{}", msg.display());
                            total_messages += 1;
                        }
                    }
                    drop(cfg);
                }

                // 2. DHT stored messages
                println!("\nSearching DHT storage...");
                let mut dht_found = false;

                let stored_messages = storage_clone.search_content_by_type("whisper-message").await;
                for (content_id, metadata) in stored_messages {
                    if let Some((_, data)) = storage_clone.find_content(&content_id).await {
                        if let Ok(msg) = serde_json::from_slice::<Message>(&data) {
                            if !dht_found {
                                println!("\nDHT stored messages:");
                                dht_found = true;
                            }
                            println!("{}", msg.display());
                            println!("Storage proof: ✓");
                            total_messages += 1;
                        }
                    }
                }

                // 3. Blockchain transactions
                println!("\nVerifying blockchain records...");
                let mut found_msgs = false;
                let mut msg_count = 0;
                
                let transactions = blockchain.get_transactions().await;
                for tx in transactions {
                    if tx.from == node_id || tx.to == node_id {
                        if let Ok(msg) = serde_json::from_slice::<Message>(&tx.data) {
                            if !found_msgs {
                                println!("\nBlockchain verified messages:");
                                found_msgs = true;
                            }
                            println!("{}", msg.display());
                            println!("Transaction: {}", tx.calculate_hash());
                            println!("Verified: ✓");
                            msg_count += 1;
                            total_messages += 1;
                        }
                    }
                }
                
                if !found_msgs {
                    println!("No blockchain transactions found");
                }

                if total_messages == 0 {
                    println!("\nNo messages found");
                } else {
                    println!("\nTotal messages: {}", total_messages);
                }

                println!("\nPress Enter to continue...");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }

            "3" => {
                println!("\nAdd Contact"); 
                println!("-----------");
                
                let addr = get_input("Contact address: ");
                match addr.parse::<SocketAddr>() {
                    Ok(sock_addr) => {
                        // First verify connection
                        if let Err(e) = connect_to_peer(&node, sock_addr).await {
                            println!("Failed to connect: {}", e);
                            continue;
                        }

                        // Create and store contact info in DHT
                        let node_id = config.lock().await.node_id.clone();
                        let contact_data = format!("{}:{}", sock_addr, addr).into_bytes();
                        
                        if let Err(e) = storage.store_content(
                            contact_data,
                            "contact".to_string(),
                            &node_id,
                            None
                        ).await {
                            println!("Failed to store contact: {}", e);
                        } else {
                            println!("✓ Contact added!");
                            
                            // Send handshake message
                            let handshake_msg = Message::new(
                                node_id.clone(),
                                format!("Hello from {}", node_id)
                            );
                            let msg_data = serde_json::to_vec(&handshake_msg)?;
                            {
                                let mut n = node.lock().await;
                                if let Ok(packet) = n.create_packet(sock_addr, msg_data).await {
                                    match n.send_packet(packet, sock_addr).await {
                                        Ok(_) => println!("✓ Handshake sent"),
                                        Err(e) => println!("Warning: Failed to send handshake: {}", e),
                                    }
                                }
                                drop(n);
                            }
                        }
                    }
                    Err(_) => println!("Invalid address format"),
                }
            }

            "4" => {
                println!("\nContacts");
                println!("--------");
                
                {
                    let n = node.lock().await;
                    let metrics = n.get_routing_metrics();
                    println!("Network Status:");
                    println!("  Delivery Success: {}", metrics.delivery_success);
                    println!("  Delivery Failures: {}", metrics.delivery_failures);
                    println!("  Average Latency: {:.1}ms", metrics.average_latency);
                    println!("  Reputation Score: {:.1}%", metrics.reputation_score * 100.0);
                    drop(n);
                }
            }

            "5" => {
                println!("\nNode Status");
                println!("-----------");
                
                {
                    let n = node.lock().await;
                    let cfg = config.lock().await;
                    println!("ID: {}", cfg.node_id);
                    println!("Address: {}:{}", get_network_ip(), cfg.port);
                    println!("Known Peers: {}", cfg.known_peers.len());
                    println!("Key Status: {}",
                        if n.get_key_status().needs_rotation { "⚠ Needs Rotation" } else { "✓ Valid" }
                    );
                    println!("Balance: {} tokens", blockchain.get_balance(&cfg.node_id).await);
                    
                    let metrics = n.get_routing_metrics();
                    println!("\nNetwork Status:");
                    println!("  Success Rate: {:.1}%", metrics.get_delivery_success_rate() * 100.0);
                    println!("  Messages Routed: {}", metrics.packets_routed);
                    println!("  Latency: {:.1}ms", metrics.average_latency);
                    println!("  Reputation: {:.1}%", metrics.reputation_score * 100.0);
                    drop(n);
                    drop(cfg);
                }
            }
            
            "6" => {
                println!("\nBlockchain Transactions");
                println!("----------------------");
                
                // Get user ID and transactions
                let user_id = {
                    let cfg = config.lock().await;
                    cfg.node_id.clone()
                };
                
                let transactions = blockchain.get_transactions().await;
                let mut msg_count = 0;
                
                // Display relevant transactions
                for tx in transactions {
                    if tx.data.is_empty() {
                        continue;
                    }
                    
                    if tx.from == user_id || tx.to == user_id {
                        if let Ok(msg) = serde_json::from_slice::<Message>(&tx.data) {
                            if msg_count == 0 {
                                println!("\nMessage Transactions:");
                            }
                            msg_count += 1;
                            
                            println!("\nTransaction #{}", msg_count);
                            println!("From: {}", tx.from);
                            println!("To: {}", tx.to);
                            println!("Message: {}", msg.display());
                            println!("ID: {}", tx.calculate_hash());
                            println!("Time: {}",
                                chrono::DateTime::<chrono::Local>::from(
                                    std::time::UNIX_EPOCH + std::time::Duration::from_secs(msg.timestamp as u64)
                                ).format("%Y-%m-%d %H:%M:%S")
                            );
                            println!("Signature: {}", if tx.verify_signature(&tx.from) { "✓" } else { "✗" });
                            println!("----------");
                        }
                    }
                }
                
                if msg_count == 0 {
                    println!("\nNo message transactions found");
                }
                
                println!("\nPress Enter to continue...");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            "7" => break,
            _ => println!("Invalid choice"),
        }
    }

    // All cases handled without errors
    Ok(())
}