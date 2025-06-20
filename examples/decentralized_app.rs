// Decentralized App Example for ZHTP
// This demonstrates a simple decentralized application running on ZHTP

use std::env;
use std::net::SocketAddr;
use anyhow::Result;
use rand;
use chrono;

use decentralized_network::{
    zhtp::{
        ZhtpNode, DAppLaunchpad, ZhtpDNS,
        dapp_launchpad::*,
        crypto::Keypair,
        zk_proofs::ByteRoutingProof,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔗 ZHTP Decentralized App Demo");
    println!("==============================");
    
    // Initialize environment
    env_logger::init();
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let app_name = if args.len() > 1 {
        args[1].clone()
    } else {
        "demo-app".to_string()
    };
    
    println!("Starting decentralized app: {}", app_name);
    
    // Create ZHTP node for the app
    let keypair = Keypair::generate();
    let node_addr: SocketAddr = "127.0.0.1:8080".parse()?;
    
    println!("📡 Initializing ZHTP node at {}", node_addr);
    let _node = ZhtpNode::new(node_addr, keypair.clone()).await?;
    
    // Initialize DApp launchpad
    println!("Setting up DApp infrastructure...");
    let launchpad = DAppLaunchpad::new();
    
    // Create a simple DApp configuration using DeployedDApp structure
    let dapp_config = DeployedDApp {
        id: format!("dapp-{}", rand::random::<u32>()),
        name: app_name.clone(),
        description: "A simple decentralized application on ZHTP".to_string(),
        developer: DeveloperInfo {
            developer_id: rand::random::<[u8; 32]>(),
            display_name: Some("Demo Developer".to_string()),
            identity_proof: ByteRoutingProof {
                commitments: vec![],
                elements: vec![],
                inputs: vec![],
            },
            reputation: 5.0,
            contact_info: vec!["demo@zhtp.dev".to_string()],
        },
        contracts: vec![],
        frontend: FrontendInfo {
            ipfs_hash: "QmExampleHash123".to_string(),
            mirror_urls: vec![],
            framework: "React".to_string(),
            build_hash: rand::random::<[u8; 32]>(),
        },
        tokenomics: None,
        revenue_model: RevenueModel::Free,
        stats: DAppStats {
            total_users: 0,
            daily_active_users: 0,
            monthly_active_users: 0,
            total_transactions: 0,
            total_revenue: 0,
            review_count: 0,
        },
        launched_at: chrono::Utc::now().timestamp() as u64,
        verification_status: VerificationStatus::Unverified,
        community_rating: 0.0,
    };
    
    println!("📦 Deploying DApp: {}", dapp_config.name);
    
    // Deploy the DApp
    match launchpad.deploy_dapp(dapp_config.clone()).await {
        Ok(deployment_id) => {
            println!("DApp deployed successfully!");
            println!("   Deployment ID: {}", deployment_id);
            println!("   App Name: {}", dapp_config.name);
            println!("   Description: {}", dapp_config.description);
        }
        Err(e) => {
            println!("❌ Failed to deploy DApp: {}", e);
            return Err(e);
        }
    }
    
    // Register with ZHTP DNS
    println!("Registering with ZHTP DNS...");
    let dns = ZhtpDNS::new();
    let app_address = format!("{}.zhtp", app_name);
    
    match dns.register_domain(
        app_address.clone(),
        vec![node_addr],
        &keypair,
        rand::random::<[u8; 32]>(),
    ).await {
        Ok(_) => println!("DNS registration successful: {}", app_address),
        Err(e) => println!("⚠️  DNS registration failed: {}", e),
    }
    
    // Start the application server
    println!("Decentralized app server ready!");
    println!("   Access your app at: http://{}", node_addr);
    println!("   ZHTP address: {}", app_address);
    println!("   Press Ctrl+C to stop");
    
    // Keep the application running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        // Check if we should shutdown
        if let Ok(_) = tokio::time::timeout(
            tokio::time::Duration::from_millis(100),
            tokio::signal::ctrl_c()
        ).await {
            println!("\n🛑 Shutting down decentralized app...");
            break;
        }
    }
    
    println!("👋 Decentralized app stopped");
    Ok(())
}
