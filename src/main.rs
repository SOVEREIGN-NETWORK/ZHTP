use anyhow::Result;
use pqcrypto_traits::sign::PublicKey as PqPublicKey;
use decentralized_network::{
    zhtp::{
        Keypair, ZhtpNode, 
        zk_consensus::{ZkConsensus, ZkConsensusParams}, 
        zk_proofs::ByteRoutingProof,
        ZhtpDao, DAppLaunchpad,
    },
    Blockchain, Network, StorageManager, Transaction,
    storage::dht::DataChunk,
};
use std::io::{self, Write};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::timeout;
use log::{info, error};

const OPERATION_TIMEOUT: Duration = Duration::from_secs(10);

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
        metrics.average_latency()
    );
}

/// Helper function to setup a node with PQC capabilities
async fn setup_zkp_node(
    addr: SocketAddr,
    name: String,
    network: &mut Network,
    storage: &mut StorageManager,
    consensus: &ZkConsensus,
) -> Result<Arc<Mutex<ZhtpNode>>> {
    let node_name = name.clone();
    info!("Initializing {} at {} with PQ crypto", node_name, addr);
    
    // Generate post-quantum keypair
    let keypair = Keypair::generate();
    let public_key_bytes = PqPublicKey::as_bytes(&keypair.public).to_vec();
    let node = ZhtpNode::new(addr, keypair).await?;
    let node = Arc::new(Mutex::new(node));
    
    // Register with core systems
    network.add_node(&node_name, 1000.0);
    // Register node as validator with the ZK consensus
    let stake_proof = ByteRoutingProof {
        inputs: vec![vec![1u8; 32]],
        elements: vec![vec![0u8; 64]],
        commitments: vec![vec![0u8; 32]],
    };
    consensus.register_validator(
        node_name.clone(),
        1000.0,
        stake_proof,
        public_key_bytes
    ).await?;
    
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
    let consensus_params = ZkConsensusParams {
        min_stake: 500.0,
        max_validators: 100,
        round_timeout: 3600,
        min_votes: 3,
        slashing_penalty: 0.1,
        anonymity_set_size: 50,
    };
    let consensus = ZkConsensus::new(consensus_params);
    let mut storage = StorageManager::new();

    info!("Initializing core systems...");
    
    // Initialize DAO and DApp systems
    let dao = ZhtpDao::new();
    let _dapp_launchpad = DAppLaunchpad::new();
    
    info!("DAO and DApp launchpad initialized");
    
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

    // Establish connections (non-blocking)
    info!("\nEstablishing secure connections...");
    
    // Clone references for the connection task
    let node_1_conn = node_1.clone();
    let node_2_conn = node_2.clone();
    
    // Try to establish connections but don't fail if they timeout
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        if let Ok(mut n1) = node_1_conn.try_lock() {
            let _ = n1.connect(addr_2).await;
            let _ = n1.connect(addr_3).await;
        }
        
        if let Ok(mut n2) = node_2_conn.try_lock() {
            let _ = n2.connect(addr_3).await;
        }
        
        info!("Connection setup completed");
    });

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
        println!("10. DAO Governance");
        println!("11. DApp Launchpad");
        println!("12. UBI System");
        println!("13. Node Rewards");
        println!("14. Exit");

        print!("\nChoice (1-14): ");
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
            }            "10" => {
                println!("\n=== DAO Governance ===");
                println!("🏛️  ZHTP Decentralized Autonomous Organization");
                println!("✓ ZK-based voting with privacy-preserving identities");
                println!("✓ Universal Basic Income funded by transaction fees");
                println!("✓ Transparent treasury management");
                println!("✓ Node operator reward system");
                
                let stats = dao.get_dao_stats().await;
                println!("\n📊 Current Stats:");
                println!("   Registered voters: {}", stats.registered_voters);
                println!("   Active nodes: {}", stats.active_nodes);
                println!("   Treasury: {:.2} ZHTP", stats.total_treasury_balance);
                println!("   Monthly UBI distributed: {:.2} ZHTP", stats.monthly_ubi_distributed);
                
                println!("\n💡 Example governance features:");
                println!("   • Vote on protocol upgrades");
                println!("   • Adjust UBI amounts");
                println!("   • Fund public goods");
                println!("   • Set network parameters");
            }
            "11" => {
                println!("\n=== DApp Launchpad ===");
                println!("🚀 Zero-Knowledge Application Platform");
                println!("✓ One-click DApp deployment");
                println!("✓ Built-in token factory");
                println!("✓ Decentralized app store");
                println!("✓ Developer onboarding system");
                
                println!("\n🎯 Available Templates:");
                println!("   • DeFi protocols (DEX, lending, staking)");
                println!("   • Gaming applications");
                println!("   • Social networks");
                println!("   • Utility applications");
                
                println!("\n💰 Token Creation:");
                println!("   • Governance tokens");
                println!("   • Utility tokens");
                println!("   • NFT collections");
                println!("   • Community rewards");
                
                print!("\nWould you like to simulate deploying a DApp? (yes/no): ");
                io::stdout().flush().unwrap();
                let mut deploy = String::new();
                io::stdin().read_line(&mut deploy).unwrap();
                
                if deploy.trim().to_lowercase() == "yes" {
                    println!("🔧 Simulating DApp deployment...");
                    println!("✅ Smart contracts compiled");
                    println!("✅ Frontend deployed to IPFS");
                    println!("✅ Listed in DApp store");
                    println!("✅ Your DApp is now live on ZHTP!");
                    println!("   Access URL: https://your-dapp.zhtp");
                }
            }
            "12" => {
                println!("\n=== Universal Basic Income ===");
                println!("💰 ZK-Powered UBI System");
                println!("✓ Privacy-preserving identity verification");
                println!("✓ Sybil-resistant with ZK proofs");
                println!("✓ Funded by protocol transaction fees");
                println!("✓ Global, borderless access");
                
                println!("\n📋 Eligibility:");
                println!("   • Verified ZK identity");
                println!("   • 3+ months network participation");
                println!("   • Reputation score > 0.7");
                
                println!("\n💸 Current Distribution:");
                println!("   Monthly amount: 1,000 ZHTP (~$300)");
                println!("   Next distribution: 15 days");
                println!("   Registered recipients: 10,000+");
                
                println!("\n🌍 Social Impact:");
                println!("   • Healthcare fund: 10% of fees");
                println!("   • Education fund: 10% of fees");
                println!("   • Housing assistance: 5% of fees");
                println!("   • Emergency relief: 5% of fees");
            }
            "13" => {
                println!("\n=== Node Rewards & Onboarding ===");
                println!("⚡ Earn ZHTP by Running Network Infrastructure");
                println!("✓ Easy setup with automated onboarding");
                println!("✓ Multiple reward streams");
                println!("✓ Performance-based bonuses");
                println!("✓ Referral incentives");
                
                println!("\n💎 Reward Structure:");
                println!("   Base node reward: 500 ZHTP/month");
                println!("   Consensus participation: +200 ZHTP/month");
                println!("   Storage provision: +150 ZHTP/month");
                println!("   Traffic routing: +100 ZHTP/month");
                
                println!("\n🎯 Performance Bonuses:");
                println!("   99%+ uptime: +50% bonus");
                println!("   High bandwidth: +30% bonus");
                println!("   Low latency: +20% bonus");
                
                println!("\n👥 Onboarding Incentives:");
                println!("   New node bonus: 100 ZHTP");
                println!("   Referral reward: 50 ZHTP per referred node");
                println!("   Early adopter program: 2x rewards for first year");
                
                print!("\nInterested in becoming a node operator? (yes/no): ");
                io::stdout().flush().unwrap();
                let mut interested = String::new();
                io::stdin().read_line(&mut interested).unwrap();
                
                if interested.trim().to_lowercase() == "yes" {
                    println!("\n🚀 Node Setup Process:");
                    println!("1. Download ZHTP node software");
                    println!("2. Stake 1000 ZHTP tokens");
                    println!("3. Configure network settings");
                    println!("4. Start earning rewards!");
                    println!("\n📖 Full guide: https://docs.zhtp.network/nodes");
                }
            }
            "14" => {
                println!("\nExiting demo...");
                break;
            }
            _ => println!("Invalid choice")
        }
    }

    Ok(())
}
