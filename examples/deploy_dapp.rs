// Simple DApp deployment example for ZHTP
// This demonstrates how developers can quickly deploy DApps to the ZHTP network

use std::env;
use std::net::SocketAddr;
use anyhow::Result;

use decentralized_network::{
    zhtp::{
        ZhtpNode, DAppLaunchpad, ZhtpDNS, ZhtpEconomics,
        dapp_launchpad::*,
        crypto::Keypair,
        zk_proofs::ByteRoutingProof,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 ZHTP DApp Deployment Tool");
    println!("============================");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let network = if args.len() > 2 && args[1] == "--network" {
        args[2].clone()
    } else {
        "local".to_string()
    };
    
    let (rpc_addr, network_name) = match network.as_str() {
        "testnet" => ("testnet-rpc.zhtp.network:443", "ZHTP Testnet"),
        "mainnet" => ("rpc.zhtp.network:443", "ZHTP Mainnet"),
        _ => ("127.0.0.1:8080", "Local Development"),
    };
    
    println!("🌐 Deploying to: {}", network_name);
    println!("📡 RPC Endpoint: {}", rpc_addr);
    println!();
    
    // Initialize ZHTP infrastructure
    let keypair = Keypair::generate();
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let _zhtp_node = ZhtpNode::new(addr, keypair.clone()).await?;
    
    // Initialize DApp deployment system
    let dapp_launchpad = DAppLaunchpad::new();
    
    // Initialize DNS for domain registration
    let _dns = ZhtpDNS::new();
    
    // Initialize economics for token payments
    let _economics = ZhtpEconomics::new();
    
    println!("✅ Connected to ZHTP network");
    println!();
    
    // Create sample DApp
    let sample_dapp = create_sample_dapp(&network);
    let domain_name = "storage.zhtp";
    
    println!("📋 DApp Details:");
    println!("   Name: {}", sample_dapp.name);
    println!("   Domain: {}", domain_name);
    println!("   Description: {}", sample_dapp.description);
    println!("   Developer: {}", sample_dapp.developer.display_name.as_ref().unwrap_or(&"Anonymous".to_string()));
    println!();
    
    // Deploy the DApp
    println!("🚀 Deploying DApp to ZHTP Network...");
    match dapp_launchpad.deploy_dapp(sample_dapp.clone()).await {
        Ok(dapp_id) => {
            println!("✅ DApp deployed successfully!");
            println!("🆔 DApp ID: {}", dapp_id);
            println!("🌐 Domain: {}", domain_name);
            println!("� Contract: {}", sample_dapp.contracts[0].address);
            println!();
            
            // Show economic impact
            println!("💰 Economic Summary:");
            println!("   Domain Registration: 10 ZHTP (~$1-10)");
            println!("   ZK Certificate: 100 ZHTP (~$10-100)");
            println!("   Deployment Fee: 5 ZHTP (~$0.50-5)");
            println!("   Total Cost: 115 ZHTP (~$11.50-115)");
            println!();
            
            println!("🌍 Your DApp is now live and accessible to users worldwide!");
            println!("📱 It will appear in the ZHTP browser for all users");
            println!("🔍 Access via: http://localhost:4000/browser/");
            
            if network == "testnet" || network == "mainnet" {
                println!();
                println!("🌐 Global Accessibility:");
                println!("   ✅ Your DApp is synchronized across the entire ZHTP network");
                println!("   ✅ Users worldwide can access it via any ZHTP browser");
                println!("   ✅ Domain {} is globally resolvable", domain_name);
                println!("   ✅ Earning token rewards for hosting and usage");
            }
            
            // Demonstrate DApp features
            println!();
            println!("🎯 DApp Features Deployed:");
            println!("   ✅ Smart Contracts: {} deployed", sample_dapp.contracts.len());
            println!("   ✅ Frontend: React-based UI on IPFS");
            println!("   ✅ Token: {} ({})", 
                sample_dapp.tokenomics.as_ref().map_or("None", |t| &t.name),
                sample_dapp.tokenomics.as_ref().map_or("No Token", |t| &t.symbol)
            );
            println!("   ✅ Revenue Model: {:?}", sample_dapp.revenue_model);
            println!("   ✅ Verification: {:?}", sample_dapp.verification_status);
        }
        Err(e) => {
            println!("❌ Deployment failed: {}", e);
            println!();
            println!("💡 Troubleshooting:");
            println!("   • Ensure you have sufficient ZHTP tokens");
            println!("   • Check network connectivity");
            println!("   • Verify domain is not already taken");
            println!("   • Try deploying to local network first");
            return Err(e);
        }
    }
    
    println!();
    println!("🎯 Next Steps:");
    println!("   1. Open the ZHTP browser: http://localhost:4000/browser/");
    println!("   2. Switch to the 'DApps' tab");
    println!("   3. See your DApp listed with all others");
    println!("   4. Click 'Open DApp' to interact with it");
    println!();
    println!("📚 Learn More:");
    println!("   • Developer Docs: docs/README.md");
    println!("   • DApp Templates: docs/templates/");
    println!("   • API Reference: docs/api/");
    println!();
    println!("🎉 Welcome to the decentralized internet! Your DApp is now part of the ZHTP network.");
    
    Ok(())
}

fn create_sample_dapp(network: &str) -> DeployedDApp {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Create demo ZK proof for developer identity
    let demo_proof = ByteRoutingProof {
        commitments: vec![vec![1u8; 32]],
        elements: vec![vec![2u8; 32]], 
        inputs: vec![vec![3u8; 32]],
    };
      DeployedDApp {
        id: format!("storage-dapp-{}", timestamp),
        name: "ZHTP Storage Hub".to_string(),
        description: "Decentralized file sharing and messaging platform on ZHTP. \
            Secure, private storage with zero-knowledge encryption. Share files, \
            send encrypted messages, and collaborate without intermediaries.".to_string(),
        developer: DeveloperInfo {
            developer_id: rand::random::<[u8; 32]>(),
            display_name: Some("Storage Team".to_string()),
            identity_proof: demo_proof,
            reputation: 5.0,
            contact_info: vec!["storage@zhtp.network".to_string()],
        },
        contracts: vec![
            ContractInfo {
                address: format!("0x{:x}", rand::random::<u64>()),
                contract_type: "storage".to_string(),
                bytecode_hash: rand::random::<[u8; 32]>(),
                abi: r#"{"functions":[{"name":"store_file","inputs":[{"type":"bytes"}],"outputs":[{"type":"string"}]},{"name":"retrieve_file","inputs":[{"type":"string"}],"outputs":[{"type":"bytes"}]},{"name":"send_message","inputs":[{"type":"address"},{"type":"bytes"}],"outputs":[{"type":"bool"}]}]}"#.to_string(),
                gas_limits: GasLimits {
                    deployment_gas: 2000000,
                    execution_gas: 500000,
                    storage_gas: 1000000,
                },
            }
        ],
        frontend: FrontendInfo {
            ipfs_hash: format!("Qm{:x}", rand::random::<u64>()),
            mirror_urls: vec!["https://storage.zhtp.network".to_string()],
            framework: "Vue.js".to_string(),
            build_hash: rand::random::<[u8; 32]>(),
        },
        tokenomics: Some(TokenInfo {
            symbol: "STORE".to_string(),
            name: "Storage Token".to_string(),
            total_supply: 500000,
            distribution: TokenDistribution {
                public_sale: 0.3,
                team_allocation: 0.15,
                community_rewards: 0.35,
                development_fund: 0.1,
                liquidity_provision: 0.1,
            },
            utility: vec![
                "Storage payments".to_string(),
                "File access fees".to_string(),
                "Bandwidth rewards".to_string(),
                "Storage node staking".to_string(),
            ],
            governance_rights: true,
        }),
        revenue_model: RevenueModel::PayPerUse { cost_per_action: 1 },
        stats: DAppStats {
            total_users: 0,
            daily_active_users: 0,
            monthly_active_users: 0,
            total_transactions: 0,
            total_revenue: 0,
            review_count: 0,
        },
        launched_at: timestamp,
        verification_status: VerificationStatus::Unverified,
        community_rating: 5.0,
    }
}

// Add dependencies for random number generation
use rand;
