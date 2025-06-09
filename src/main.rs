use anyhow::Result;
use decentralized_network::{
    zhtp::{Keypair, ZhtpNode},
    Blockchain, ConsensusManager, Network, StorageManager, Transaction,
    storage::dht::DataChunk,
};
use std::io::{self, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use log::{info, error};

const OPERATION_TIMEOUT: Duration = Duration::from_secs(2);

/// Helper function to print node metrics
async fn print_node_metrics(node: &ZhtpNode, name: &str) {
    println!("\n=== {} Quick Status ===", name);
    println!("Key Status: {}", if node.get_key_status().needs_rotation {
        "Rotation needed"
    } else {
        "Valid"
    });

    let metrics = node.get_routing_metrics();
    let success_rate = if metrics.packets_routed > 0 {
        (metrics.delivery_success as f64 / metrics.packets_routed as f64) * 100.0
    } else {
        100.0
    };
    println!("Network metrics - Success Rate: {:.1}%, Avg Latency: {:.2}ms",
        success_rate,
        metrics.average_latency
    );
}

/// Helper function to setup a node with PQC capabilities
async fn setup_zkp_node(
    addr: SocketAddr,
    name: String,
    network: &mut Network,
    storage: &mut StorageManager,
    consensus: &ConsensusManager,
) -> Result<Arc<Mutex<ZhtpNode>>> {
    let node_name = name.clone();
    info!("Initializing {} at {} with PQ crypto", node_name, addr);
    
    // Generate post-quantum keypair
    let keypair = Keypair::generate();
    let node = ZhtpNode::new(addr, keypair).await?;
    let node = Arc::new(Mutex::new(node));
    
    // Register with core systems
    network.add_node(&node_name, 1000.0);
    consensus.register_node(node_name.clone(), 1000.0).await;
    
    // Initialize storage and wait for routing setup
    info!("Registering {} with storage system", node_name);
    if !storage.register_node(node_name.clone(), 1_000_000).await {
        anyhow::bail!("Failed to register storage node");
    }
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Start listening with longer timeout
    let node_listen = node.clone();
    let listen_name = node_name.clone();
    tokio::spawn(async move {
        info!("{} online", listen_name);
        if let Err(e) = ZhtpNode::start_listening_shared(node_listen).await {
            error!("{} listener error: {}", listen_name, e);
        }
    });

    // Start key rotation checker
    let node_rotation = node.clone();
    tokio::spawn(async move {
        ZhtpNode::init_key_rotation(node_rotation).await;
    });

    // Longer delay to ensure node is fully initialized
    tokio::time::sleep(Duration::from_secs(2)).await;
    info!("{} setup complete", node_name);
    Ok(node)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging first
    env_logger::init();
    
    println!("=== Decentralized Network Demo ===\n");
    // Initialize core components
    let mut network = Network::new();
    let blockchain = Blockchain::new(100.0);
    let consensus = ConsensusManager::new(500.0, 3600);
    let mut storage = StorageManager::new();

    info!("Initializing core systems...");
    
    // Ensure core systems are ready
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Set up network addresses
    let addr_1: SocketAddr = "127.0.0.1:9001".parse()?;
    let addr_2: SocketAddr = "127.0.0.1:9002".parse()?;
    let addr_3: SocketAddr = "127.0.0.1:9003".parse()?;

    // Initialize nodes sequentially with post-quantum cryptography
    info!("\nInitializing nodes with PQ crypto...");
    
    // Initialize nodes one at a time to ensure proper setup
    let node_1 = setup_zkp_node(
        addr_1,
        String::from("node1"),
        &mut network,
        &mut storage,
        &consensus,
    ).await?;
    
    info!("Node 1 initialized, waiting for readiness...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let node_2 = setup_zkp_node(
        addr_2,
        String::from("node2"),
        &mut network,
        &mut storage,
        &consensus,
    ).await?;

    info!("Node 2 initialized, waiting for readiness...");
    tokio::time::sleep(Duration::from_secs(1)).await;

    let node_3 = setup_zkp_node(
        addr_3,
        String::from("node3"),
        &mut network,
        &mut storage,
        &consensus,
    ).await?;

    info!("All nodes initialized");
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Quick genesis setup
    info!("\nInitializing blockchain...");
    let mut genesis_tx = Transaction::new("network".to_string(), "node1".to_string(), 1000.0);
    genesis_tx.sign("network");
    blockchain.add_transaction(genesis_tx).await;
    blockchain.create_block("genesis", 1.0, None).await;

    // Initial fund distribution
    info!("Initial fund distribution...");
    let mut tx1 = Transaction::new("node1".to_string(), "node2".to_string(), 300.0);
    tx1.sign("node1");
    blockchain.add_transaction(tx1).await;

    let mut tx2 = Transaction::new("node1".to_string(), "node3".to_string(), 300.0);
    tx2.sign("node1");
    blockchain.add_transaction(tx2).await;
    blockchain.create_block("node1", 1.0, None).await;

    // Establish connections
    info!("\nEstablishing secure connections...");
    let connect_result = timeout(OPERATION_TIMEOUT, async {
        let mut n1 = node_1.lock().await;
        if let Err(e) = n1.connect(addr_2).await {
            error!("Failed to connect to node2: {}", e);
        }
        if let Err(e) = n1.connect(addr_3).await {
            error!("Failed to connect to node3: {}", e);
        }
        
        let mut n2 = node_2.lock().await;
        if let Err(e) = n2.connect(addr_3).await {
            error!("Failed to connect node2 to node3: {}", e);
        }
    }).await;

    if let Err(e) = connect_result {
        error!("Connection setup timed out: {}", e);
    }

    info!("\nNetwork ready!");
    println!("Starting demo mode...");

    loop {
        println!("\n=== Demo Menu ===");
        println!("1. Send encrypted message");
        println!("2. Store data with PQ signatures");
        println!("3. Store content with addressing");
        println!("4. Search content");
        println!("5. Register/discover service");
        println!("6. View popular content");
        println!("7. Make transaction");
        println!("8. View node status");
        println!("9. Force key rotation");
        println!("10. Exit");

        print!("\nChoice (1-10): ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("\nSending encrypted message...");
                let msg = b"Test message with PQ encryption".to_vec();
                
                let result = timeout(OPERATION_TIMEOUT, async {
                    let n1 = node_1.lock().await;
                    let packet = n1.create_packet(addr_3, msg).await?;
                    n1.send_packet(packet, addr_2).await
                }).await;

                match result {
                    Ok(Ok(_)) => {
                        println!("Message sent successfully!");
                        let n1 = node_1.lock().await;
                        print_node_metrics(&n1, "Node 1").await;
                    }
                    Ok(Err(e)) => error!("Send error: {}", e),
                    Err(_) => error!("Send operation timed out"),
                }
            }
            "2" => {
                println!("\nStoring data with PQ signatures...");
                let test_data = b"Test data with quantum resistance".to_vec();
                let chunk = DataChunk::new(test_data, "node1".to_string(), 2);
                
                // Ensure all nodes are registered
                println!("Registering storage nodes...");
                for node_id in ["node1", "node2", "node3"].iter() {
                    storage.register_node(node_id.to_string(), 1_000_000).await;
                }
                
                let mut stored = false;
                let store_future = async {
                    for node_id in ["node2", "node3"].iter() {
                        if storage.store_chunk(chunk.clone(), node_id).await {
                            println!("✓ Stored on {}", node_id);
                            stored = true;
                        }
                    }
                    stored
                };
                
                match timeout(Duration::from_secs(5), store_future).await {
                    Ok(result) => {
                        if result {
                            let mut store_tx = Transaction::new("network".to_string(), "node1".to_string(), 0.0);
                            store_tx.sign("network");
                            blockchain.add_transaction(store_tx).await;
                            blockchain.create_block("node1", 1.0, None).await;
                            println!("Storage operation completed with PQ signatures");
                        } else {
                            error!("Storage operation failed");
                        }
                    }
                    Err(_) => error!("Storage operation timed out"),
                }
            }
            "3" => {
                println!("\nStoring content with addressing...");
                print!("Enter content (text): ");
                io::stdout().flush().unwrap();
                
                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();
                let content = content.trim().as_bytes().to_vec();

                // Store on node1 initially
                println!("Storing content on node1...");
                // Ask for tags
                print!("Enter tags (comma-separated, press enter for none): ");
                io::stdout().flush().unwrap();
                let mut tags_input = String::new();
                io::stdin().read_line(&mut tags_input).unwrap();
                
                let tags = if tags_input.trim().is_empty() {
                    None
                } else {
                    Some(tags_input.trim()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect())
                };

                let content_id = storage.store_content(
                    content.clone(),
                    "text/plain".to_string(),
                    "node1",
                    tags,
                ).await.unwrap();

                println!("\nContent stored successfully!");
                println!("Content ID: {:?}", content_id);

                // Try to retrieve the content
                println!("\nRetrieving content...");
                if let Some((metadata, data)) = storage.find_content(&content_id).await {
                    println!("Retrieved successfully:");
                    println!("Type: {}", metadata.content_type);
                    println!("Size: {} bytes", metadata.size);
                    println!("Content: {}", String::from_utf8_lossy(&data));
                } else {
                    println!("Failed to retrieve content");
                }
            }
            "4" => {
                println!("\nContent Search Options:");
                println!("1. Search by type");
                println!("2. Search by size");
                println!("3. Search by tag");
                
                print!("Select search option (1-3): ");
                io::stdout().flush().unwrap();
                
                let mut search_choice = String::new();
                io::stdin().read_line(&mut search_choice).unwrap();
                
                match search_choice.trim() {
                    "1" => {
                        print!("Enter content type to search (e.g., text/plain): ");
                        io::stdout().flush().unwrap();
                        let mut content_type = String::new();
                        io::stdin().read_line(&mut content_type).unwrap();
                        
                        println!("\nSearching for content type: {}", content_type.trim());
                        let results = storage.search_content_by_type(content_type.trim()).await;
                        
                        if results.is_empty() {
                            println!("No content found of type: {}", content_type.trim());
                        } else {
                            println!("\nFound {} results:", results.len());
                            for (id, locations) in results {
                                println!("Content ID: {:?}", id);
                                println!("Type: {}", locations.content_type);
                                println!("Size: {} bytes", locations.size);
                                println!("Available on {} nodes", locations.locations.len());
                            }
                        }
                    }
                    "2" => {
                        println!("Enter size range to search (in KB)");
                        print!("Minimum size: ");
                        io::stdout().flush().unwrap();
                        let mut min_size = String::new();
                        io::stdin().read_line(&mut min_size).unwrap();
                        
                        print!("Maximum size: ");
                        io::stdout().flush().unwrap();
                        let mut max_size = String::new();
                        io::stdin().read_line(&mut max_size).unwrap();
                        
                        let min_kb = min_size.trim().parse().unwrap_or(0);
                        let max_kb = max_size.trim().parse().unwrap_or(u64::MAX);
                        
                        println!("\nSearching for content between {}KB and {}KB", min_kb, max_kb);
                        let results = storage.search_content_by_size(min_kb, max_kb).await;
                        
                        if results.is_empty() {
                            println!("No content found in size range");
                        } else {
                            println!("\nFound {} results:", results.len());
                            for (id, metadata) in results {
                                println!("Content ID: {:?}", id);
                                println!("Type: {}", metadata.content_type);
                                println!("Size: {} bytes", metadata.size);
                                println!("Available on {} nodes", metadata.locations.len());
                            }
                        }
                    }
                    "3" => {
                        print!("Enter tag to search: ");
                        io::stdout().flush().unwrap();
                        let mut tag = String::new();
                        io::stdin().read_line(&mut tag).unwrap();
                        
                        println!("\nSearching for content with tag: {}", tag.trim());
                        let results = storage.search_content_by_tag(tag.trim()).await;
                        
                        if results.is_empty() {
                            println!("No content found with tag: {}", tag.trim());
                        } else {
                            println!("\nFound {} results:", results.len());
                            for (id, metadata) in results {
                                println!("Content ID: {:?}", id);
                                println!("Type: {}", metadata.content_type);
                                println!("Size: {} bytes", metadata.size);
                                println!("Available on {} nodes", metadata.locations.len());
                            }
                        }
                    }
                    _ => println!("Invalid search option")
                }
            }
            "5" => {
                println!("\nService Registry Options:");
                println!("1. Register new service");
                println!("2. Discover services");
                
                print!("Select option (1-2): ");
                io::stdout().flush().unwrap();
                
                let mut service_choice = String::new();
                io::stdin().read_line(&mut service_choice).unwrap();
                
                match service_choice.trim() {
                    "1" => {
                        println!("\nRegistering test service...");
                        let service = storage.create_test_service("node1", "test_service").await;
                        
                        if storage.register_service(service.clone()).await.is_ok() {
                            println!("Service registered successfully!");
                            println!("Service ID: {:?}", service.id);
                            println!("Type: {:?}", service.service_type);
                            println!("Provider: {:?}", service.provider);
                        } else {
                            println!("Failed to register service");
                        }
                    }
                    "2" => {
                        println!("\nDiscovering services...");
                        let services = storage.list_services().await;
                        
                        if services.is_empty() {
                            println!("No services found");
                        } else {
                            println!("\nRegistered Services:");
                            for (service_type, service_list) in services {
                                println!("\nType: {:?}", service_type);
                                for service in service_list {
                                    println!("  ID: {:?}", service.id);
                                    println!("  Provider: {:?}", service.provider);
                                    println!("  Endpoint: {}", service.endpoint);
                                }
                            }
                        }
                    }
                    _ => println!("Invalid option")
                }
            }
            "6" => {
                println!("\nViewing popular content...");
                print!("Enter minimum access count: ");
                io::stdout().flush().unwrap();
                
                let mut min_access = String::new();
                io::stdin().read_line(&mut min_access).unwrap();
                let min_count = min_access.trim().parse().unwrap_or(100);
                
                let popular = storage.get_popular_content(min_count).await;
                
                if popular.is_empty() {
                    println!("No popular content found with {} or more accesses", min_count);
                } else {
                    println!("\nPopular Content:");
                    for (id, metadata) in popular {
                        println!("\nContent ID: {:?}", id);
                        println!("Type: {}", metadata.content_type);
                        println!("Size: {} bytes", metadata.size);
                        println!("Locations: {} nodes", metadata.locations.len());
                    }
                }
            }
            "7" => {
                println!("\nCreating signed transaction...");
                let mut tx = Transaction::new("node1".to_string(), "node2".to_string(), 50.0);
                tx.sign("node1");
                if blockchain.add_transaction(tx).await {
                    blockchain.create_block("node1", 1.0, None).await;
                    println!("\n=== Blockchain Status ===");
                    println!("Getting chain info...");
                    let block = blockchain.get_latest_block().await;
                    println!("Latest block index: {}", block.index);
                    println!("\nNode Balances:");
                    for node in &["node1", "node2", "node3"] {
                        let balance = blockchain.get_balance(node).await;
                        println!("  {}: {:.2}", node, balance);
                    }
                } else {
                    println!("Transaction failed - insufficient balance");
                }
            }
            "8" => {
                println!("\n=== System Status ===");
                {
                    let n1 = node_1.lock().await;
                    print_node_metrics(&n1, "Node 1").await;
                }
                {
                    let n2 = node_2.lock().await;
                    print_node_metrics(&n2, "Node 2").await;
                }
                {
                    let n3 = node_3.lock().await;
                    print_node_metrics(&n3, "Node 3").await;
                }
            }
            "9" => {
                println!("\nForcing key rotation...");
                let mut success = false;
                
                {
                    let mut n1 = node_1.lock().await;
                    n1.force_immediate_rotation();
                    if let Ok(()) = n1.rotate_keys() {
                        success = true;
                    }
                }

                if success {
                    println!("Keys rotated successfully!");
                    let n1 = node_1.lock().await;
                    print_node_metrics(&n1, "Node 1").await;
                } else {
                    error!("Key rotation failed");
                }
            }
            "10" => {
                println!("\nExiting demo...");
                break;
            }
            _ => println!("Invalid choice")
        }
    }

    Ok(())
}
