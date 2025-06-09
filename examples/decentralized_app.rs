use anyhow::Result;
use decentralized_network::{
    browser::ZhtpBrowser,
    contracts::{ContractExecutor, ContractInterface}
};
use std::net::SocketAddr;
use std::time::Duration;
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== ZHTP Decentralized App Demo ===");

    // Initialize browser
    let browser_addr: SocketAddr = "127.0.0.1:9100".parse()?;
    println!("Starting ZHTP browser on {}", browser_addr);
    let mut browser = ZhtpBrowser::new(browser_addr).await?;

    // Initialize contract executor
    let mut executor = ContractExecutor::new();

    // Deploy example token contract
    println!("\nDeploying token contract...");
    let wasm_path = "./target/wasm32-unknown-unknown/release/zhtp_contracts.wasm";
    let token_bytecode = fs::read(wasm_path)?;
    let interface_path = "./contracts/token.json";
    let token_interface: ContractInterface = serde_json::from_str(&fs::read_to_string(interface_path)?)?;

    executor.deploy_contract(
        "token".to_string(),
        "owner".to_string(),
        token_bytecode,
        token_interface,
    ).await?;

    // Connect to a network node
    let node_addr: SocketAddr = "127.0.0.1:9000".parse()?;
    println!("\nConnecting to ZHTP node at {}", node_addr);
    match browser.connect(node_addr).await {
        Ok(true) => println!("✓ Connected successfully"),
        Ok(false) => println!("✗ Connection failed"),
        Err(e) => println!("✗ Connection error: {}", e),
    }

    // Search for content
    println!("\nSearching network content...");
    let results = browser.search("test".to_string()).await?;
    println!("Found {} results", results.len());
    for (id, metadata) in results {
        println!("- Content {}: {}", id, metadata.content_type);
    }

    // Interact with token contract
    println!("\nTesting token contract...");
    let transfer_params = vec![
        "recipient".as_bytes().to_vec(),
        "100".as_bytes().to_vec(),
    ];
    
    match browser.call_contract(
        "token".to_string(),
        "transfer".to_string(),
        transfer_params,
    ).await {
        Ok(response) => {
            let success = response[0] == 1;
            println!("Token transfer {}", if success { "successful" } else { "failed" });
        }
        Err(e) => println!("Token transfer failed: {}", e),
    }

    // Keep browser running
    println!("\nBrowser running. Press Ctrl+C to exit.");
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}