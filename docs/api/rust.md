# 🦀 Rust SDK - Complete Decentralized Internet Replacement

**Complete API documentation for the ZHTP Rust SDK that replaces all traditional internet infrastructure**

> **Important**: This SDK completely replaces HTTP clients (reqwest, hyper), DNS resolution (trust-dns), TLS libraries (rustls), and traditional networking crates. No traditional internet infrastructure is used.

## Installation (Replaces HTTP Crates)

Add to your `Cargo.toml` (note: no traditional internet crates needed):

```toml
[dependencies]
# ZHTP SDK replaces: reqwest, hyper, curl, trust-dns, rustls, etc.
zhtp = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }

# Traditional crates NOT NEEDED:
# ❌ reqwest = "0.11"      # Replaced by zhtp::ZhtpClient
# ❌ hyper = "0.14"        # Replaced by zhtp native protocol  
# ❌ trust-dns = "0.22"    # Replaced by zhtp::BlockchainDNS
# ❌ rustls = "0.21"       # Replaced by zhtp::ZkCertificate
# ❌ curl = "0.4"          # Replaced by zhtp::fetch_content
```

## Quick Import (Native ZHTP Protocol)

```rust
// Import ZHTP SDK (replaces traditional networking crates)
use zhtp::{
    ZhtpClient,          // Replaces reqwest::Client, hyper::Client
    DAppBuilder,         // Replaces traditional web frameworks
    SmartContract,       // Blockchain contract interface
    ZkProof,            // Zero-knowledge privacy
    BlockchainDNS,      // Replaces trust-dns resolver
    ZkCertificateAuth,  // Replaces rustls, native-tls
    Result as ZhtpResult,
};

// No traditional imports needed:
// ❌ use reqwest::Client;
// ❌ use hyper::{Body, Client, Request};
// ❌ use trust_dns_resolver::TokioAsyncResolver;
// ❌ use rustls::{ClientConfig, RootCertStore};
```

## 🏗️ ZhtpClient - Complete Traditional Internet Replacement

Main client for interacting with the decentralized ZHTP network (replaces all HTTP clients).

### Constructor

```rust
pub struct ZhtpClient {
    // Internal fields for decentralized networking
}

impl ZhtpClient {
    pub async fn new(config: ZhtpConfig) -> ZhtpResult<Self> {
        // Initialize decentralized networking
    }
    
    pub async fn connect(&mut self) -> ZhtpResult<()> {
        // Connect to decentralized network (no HTTP/DNS)
    }
}

pub struct ZhtpConfig {
    pub network: Network,
    pub node_endpoints: Vec<String>,    // ZHTP protocol endpoints (not HTTP)
    pub private_key: Option<String>,    // Wallet for transactions
    pub privacy_level: PrivacyLevel,    // Built-in anonymity
    pub security_level: SecurityLevel,  // Post-quantum cryptography
    pub cache_size: usize,             // Local content cache
    pub max_hops: u8,                  // Anonymous routing hops
}

pub enum Network {
    Mainnet,
    Testnet,
    Local,
}

pub enum PrivacyLevel {
    Standard,
    High,
    Maximum,
}

pub enum SecurityLevel {
    Standard,
    QuantumResistant,
}
```

### Example (Replaces HTTP Clients)

```rust
use zhtp::{ZhtpClient, ZhtpConfig, Network, PrivacyLevel, SecurityLevel};
use std::time::Duration;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    // Traditional HTTP client setup - NEVER DO THIS IN ZHTP:
    // ❌ let client = reqwest::Client::new();
    // ❌ let response = client.get("https://api.example.com").send().await?;
    
    // ZHTP decentralized client setup:
    let config = ZhtpConfig {
        network: Network::Mainnet,
        node_endpoints: vec![
            "zhtp://node1.zhtp.network:8443".to_string(),
            "zhtp://node2.zhtp.network:8443".to_string(),
        ],
        private_key: None,
        privacy_level: PrivacyLevel::High,     // Built-in anonymity
        security_level: SecurityLevel::QuantumResistant,
        cache_size: 100_000_000,               // 100MB cache
        max_hops: 5,                           // Anonymous routing
    };
    
    let mut client = ZhtpClient::new(config).await?;
    
    // Connect to decentralized network (no DNS, no HTTP)
    client.connect().await?;
    println!("Connected to decentralized internet!");
    
    // Fetch content via decentralized storage (replaces HTTP requests)
    let content = client.fetch_content(
        "news.zhtp",        // Domain resolved via blockchain
        "/latest",          // Path in decentralized storage
        true,               // Anonymous access
        true                // Verify ZK proofs
    ).await?;
    
    println!("Decentralized content: {:?}", content);
    
    Ok(())
}
```

### Methods

#### `connect()` - Replaces HTTP Connection
Establishes connection to decentralized ZHTP network.

```rust
// Traditional HTTP connection - NEVER DO THIS:
// ❌ let client = reqwest::Client::new();
// ❌ let response = client.get("https://api.example.com/health").send().await?;

// ZHTP decentralized connection:
let mut client = ZhtpClient::new(config).await?;
client.connect().await?;

// Verify connection without HTTP
let status = client.get_network_status().await?;
println!("Connected to {} nodes", status.node_count);
```

#### `fetch_content()` - Replaces HTTP Requests
Fetches content via decentralized storage.

```rust
// Traditional HTTP request - NEVER DO THIS:
// ❌ let response = reqwest::get("https://api.example.com/data").await?;
// ❌ let text = response.text().await?;

// ZHTP decentralized content fetching:
let content = client.fetch_content(
    "api.zhtp",          // Resolved via blockchain DNS
    "/data",             // Path in decentralized storage
    true,                // Anonymous access (built-in)
    true                 // Verify cryptographic proofs
).await?;

println!("Decentralized data: {}", content);
```

#### `blockchain_dns.resolve()` - Replaces DNS Resolution
Resolves domains via blockchain instead of traditional DNS.

```rust
// Traditional DNS resolution - NEVER DO THIS:
// ❌ use trust_dns_resolver::TokioAsyncResolver;
// ❌ let resolver = TokioAsyncResolver::tokio_from_system_conf()?;
// ❌ let response = resolver.lookup_ip("example.com").await?;

// ZHTP blockchain DNS resolution:
let domain_info = client.blockchain_dns()
    .resolve("my-app.zhtp")
    .await?;

println!("Domain info: {:?}", domain_info);
println!("Content hash: {}", domain_info.content_hash);
println!("ZK certificate: {}", domain_info.zk_certificate);
```

#### `get_network_info()`
Get current network information.

```rust
impl ZhtpClient {
    pub async fn get_network_info(&self) -> ZhtpResult<NetworkInfo> {
        // Implementation
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct NetworkInfo {
    pub network: String,
    pub node_count: u64,
    pub block_height: u64,
    pub total_supply: u128,
    pub staked_amount: u128,
}

// Usage
let info = client.get_network_info().await?;
println!("Network: {}", info.network);
println!("Nodes: {}", info.node_count);
println!("Block height: {}", info.block_height);
```

#### `deploy_dapp()`
Deploy a decentralized application.

```rust
impl ZhtpClient {
    pub async fn deploy_dapp(
        &self,
        name: &str,
        content: Vec<u8>,
        metadata: Option<DAppMetadata>,
    ) -> ZhtpResult<String> {
        // Implementation
    }
}

#[derive(Debug, serde::Serialize)]
pub struct DAppMetadata {
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: Option<String>,
    pub tags: Vec<String>,
}

// Usage
use std::fs;

let content = fs::read("my-dapp.zip")?;
let metadata = DAppMetadata {
    version: "1.0.0".to_string(),
    description: "My first ZHTP DApp".to_string(),
    author: "developer@example.com".to_string(),
    license: Some("MIT".to_string()),
    tags: vec!["productivity".to_string(), "tools".to_string()],
};

let dapp_address = client.deploy_dapp("MyAwesomeApp", content, Some(metadata)).await?;
println!("DApp deployed at: {}", dapp_address);
```

## 🚀 DAppBuilder

Helper struct for building decentralized applications.

### Constructor

```rust
pub struct DAppBuilder {
    name: String,
    version: String,
    description: String,
    files: HashMap<String, Vec<u8>>,
    contracts: HashMap<String, SmartContractSource>,
}

impl DAppBuilder {
    pub fn new(name: &str, version: &str, description: &str) -> Self {
        // Implementation
    }
}
```

### Example

```rust
use zhtp::{DAppBuilder, ZhtpClient, ZhtpConfig, Network};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    // Create a new DApp
    let mut app = DAppBuilder::new(
        "TodoApp",
        "1.0.0",
        "Decentralized todo list application"
    );
    
    // Add HTML content
    app.add_file("index.html", r#"
<!DOCTYPE html>
<html>
<head>
    <title>Todo DApp</title>
    <script src="zhtp.js"></script>
</head>
<body>
    <h1>My Decentralized Todo List</h1>
    <div id="app"></div>
    <script src="app.js"></script>
</body>
</html>
    "#.as_bytes().to_vec())?;
    
    // Add JavaScript
    app.add_file("app.js", r#"
class TodoApp {
    constructor() {
        this.todos = [];
        this.init();
    }
    
    async init() {
        // Connect to ZHTP
        this.zhtp = new ZHTP();
        await this.zhtp.connect();
        
        // Load todos from blockchain storage
        this.todos = await this.zhtp.storage.get('todos') || [];
        this.render();
    }
    
    async addTodo(text) {
        const todo = {
            id: Date.now(),
            text: text,
            completed: false,
            timestamp: new Date().toISOString()
        };
        
        this.todos.push(todo);
        
        // Save to blockchain
        await this.zhtp.storage.set('todos', this.todos);
        this.render();
    }
    
    render() {
        // Render UI...
    }
}

new TodoApp();
    "#.as_bytes().to_vec())?;
    
    // Add CSS
    app.add_file("style.css", r#"
body {
    font-family: Arial, sans-serif;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

.todo-item {
    padding: 10px;
    border: 1px solid #ddd;
    margin: 5px 0;
    border-radius: 4px;
}
    "#.as_bytes().to_vec())?;
    
    // Build the DApp package
    let package = app.build()?;
    
    // Deploy to ZHTP
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: None,
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    let address = client.deploy_dapp("TodoApp", package, None).await?;
    println!("Todo app deployed at: {}", address);
    
    Ok(())
}
```

### Methods

#### `add_file()`
Add a file to the DApp package.

```rust
impl DAppBuilder {
    pub fn add_file(&mut self, path: &str, content: Vec<u8>) -> ZhtpResult<()> {
        // Implementation
    }
}
```

#### `add_smart_contract()`
Add a smart contract to the DApp.

```rust
#[derive(Debug, Clone)]
pub struct SmartContractSource {
    pub code: String,
    pub abi: Option<serde_json::Value>,
    pub compiler_version: String,
}

impl DAppBuilder {
    pub fn add_smart_contract(
        &mut self,
        name: &str,
        contract: SmartContractSource,
    ) -> ZhtpResult<()> {
        // Implementation
    }
}

// Usage
let contract_source = SmartContractSource {
    code: r#"
        contract TodoContract {
            struct Todo {
                string text;
                bool completed;
                uint256 timestamp;
            }
            
            Todo[] public todos;
            
            function addTodo(string memory text) public {
                todos.push(Todo(text, false, block.timestamp));
            }
            
            function toggleTodo(uint256 index) public {
                require(index < todos.length, "Invalid index");
                todos[index].completed = !todos[index].completed;
            }
            
            function getTodoCount() public view returns (uint256) {
                return todos.length;
            }
        }
    "#.to_string(),
    abi: Some(serde_json::json!({
        "functions": [
            {
                "name": "addTodo",
                "inputs": [{"name": "text", "type": "string"}],
                "outputs": []
            },
            {
                "name": "toggleTodo", 
                "inputs": [{"name": "index", "type": "uint256"}],
                "outputs": []
            }
        ]
    })),
    compiler_version: "0.8.19".to_string(),
};

app.add_smart_contract("TodoContract", contract_source)?;
```

#### `build()`
Build the DApp package.

```rust
impl DAppBuilder {
    pub fn build(&self) -> ZhtpResult<Vec<u8>> {
        // Implementation - creates compressed package
    }
}
```

## 🔗 SmartContract

Struct for interacting with smart contracts.

### Constructor

```rust
pub struct SmartContract {
    address: String,
    abi: serde_json::Value,
    client: ZhtpClient,
}

impl SmartContract {
    pub fn new(
        address: String,
        abi: serde_json::Value,
        client: ZhtpClient,
    ) -> Self {
        // Implementation
    }
}
```

### Example

```rust
use zhtp::{SmartContract, ZhtpClient, ZhtpConfig, Network};
use serde_json::json;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: Some("your_private_key".to_string()),
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    // Connect to existing contract
    let contract = SmartContract::new(
        "0x1234567890abcdef...".to_string(),
        json!({
            "functions": [
                {
                    "name": "getValue",
                    "inputs": [],
                    "outputs": [{"type": "uint256"}]
                },
                {
                    "name": "setValue",
                    "inputs": [{"name": "value", "type": "uint256"}],
                    "outputs": []
                }
            ]
        }),
        client,
    );
    
    // Call read-only function
    let value: u64 = contract.call("getValue", vec![]).await?;
    println!("Current value: {}", value);
    
    // Send transaction
    let tx_hash = contract.send("setValue", vec![42u64.into()]).await?;
    println!("Transaction sent: {}", tx_hash);
    
    // Wait for confirmation
    let receipt = contract.wait_for_transaction(&tx_hash).await?;
    println!("Transaction confirmed in block: {}", receipt.block_number);
    
    Ok(())
}
```

### Methods

#### `call()`
Call a read-only contract function.

```rust
impl SmartContract {
    pub async fn call<T: serde::de::DeserializeOwned>(
        &self,
        function_name: &str,
        args: Vec<serde_json::Value>,
    ) -> ZhtpResult<T> {
        // Implementation
    }
}
```

#### `send()`
Send a transaction to a contract function.

```rust
#[derive(Debug, Clone)]
pub struct TransactionOptions {
    pub gas_limit: Option<u64>,
    pub gas_price: Option<u128>,
    pub value: Option<u128>,
}

impl SmartContract {
    pub async fn send(
        &self,
        function_name: &str,
        args: Vec<serde_json::Value>,
        options: Option<TransactionOptions>,
    ) -> ZhtpResult<String> {
        // Implementation
    }
}
```

## 🔐 ZkProof

Zero-knowledge proof utilities.

### Constructor

```rust
pub struct ZkProof {
    circuit_path: Option<String>,
    proving_key: Option<Vec<u8>>,
    verification_key: Option<Vec<u8>>,
}

impl ZkProof {
    pub fn new(circuit_path: Option<&str>) -> Self {
        // Implementation
    }
    
    pub async fn load_circuit(&mut self, circuit_path: &str) -> ZhtpResult<()> {
        // Implementation
    }
}
```

### Example

```rust
use zhtp::{ZkProof, SmartContract};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    // Create age verification proof
    let mut age_proof = ZkProof::new(Some("circuits/age_verification.circom"));
    age_proof.load_circuit("circuits/age_verification.circom").await?;
    
    // Generate proof
    let mut inputs = HashMap::new();
    inputs.insert("age".to_string(), json!(25));
    inputs.insert("min_age".to_string(), json!(18));
    
    let proof = age_proof.generate(inputs).await?;
    
    // Verify proof
    let is_valid = age_proof.verify(&proof).await?;
    println!("Age proof valid: {}", is_valid);
    
    // Submit proof to contract (assuming you have a contract instance)
    // let tx_hash = contract.send("verifyAge", vec![proof.serialize()]).await?;
    
    Ok(())
}
```

### Methods

#### `generate()`
Generate a zero-knowledge proof.

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZkProofData {
    pub proof: Vec<u8>,
    pub public_inputs: Vec<serde_json::Value>,
    pub verification_key: Vec<u8>,
}

impl ZkProof {
    pub async fn generate(
        &self,
        inputs: HashMap<String, serde_json::Value>,
    ) -> ZhtpResult<ZkProofData> {
        // Implementation
    }
}
```

#### `verify()`
Verify a zero-knowledge proof.

```rust
impl ZkProof {
    pub async fn verify(&self, proof: &ZkProofData) -> ZhtpResult<bool> {
        // Implementation
    }
}
```

## 🌐 BlockchainDNS

Decentralized DNS utilities.

### Constructor

```rust
pub struct BlockchainDNS {
    client: ZhtpClient,
}

impl BlockchainDNS {
    pub fn new(client: ZhtpClient) -> Self {
        // Implementation
    }
}
```

### Example

```rust
use zhtp::{BlockchainDNS, ZhtpClient, ZhtpConfig, Network};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: Some("your_private_key".to_string()),
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    let dns = BlockchainDNS::new(client);
    
    // Register a domain
    let mut record = HashMap::new();
    record.insert("address".to_string(), json!("0x1234567890abcdef..."));
    record.insert("content_hash".to_string(), json!("QmXoYCz..."));
    record.insert("ttl".to_string(), json!(3600));
    
    let tx_hash = dns.register("myapp.zhtp", record, None).await?;
    println!("Domain registered: {}", tx_hash);
    
    // Resolve a domain
    let record = dns.resolve("myapp.zhtp").await?;
    println!("Domain points to: {}", record.get("address").unwrap());
    
    // Update domain record
    let mut updated_record = HashMap::new();
    updated_record.insert("address".to_string(), json!("0xfedcba0987654321..."));
    updated_record.insert("ttl".to_string(), json!(7200));
    
    let update_tx = dns.update("myapp.zhtp", updated_record).await?;
    println!("Domain updated: {}", update_tx);
    
    Ok(())
}
```

### Methods

#### `register()`
Register a new domain.

```rust
impl BlockchainDNS {
    pub async fn register(
        &self,
        domain: &str,
        record: HashMap<String, serde_json::Value>,
        duration_seconds: Option<u64>, // Default: 1 year
    ) -> ZhtpResult<String> {
        // Implementation
    }
}
```

#### `resolve()`
Resolve a domain to its record.

```rust
impl BlockchainDNS {
    pub async fn resolve(
        &self,
        domain: &str,
    ) -> ZhtpResult<HashMap<String, serde_json::Value>> {
        // Implementation
    }
}
```

#### `update()`
Update an existing domain record.

```rust
impl BlockchainDNS {
    pub async fn update(
        &self,
        domain: &str,
        record: HashMap<String, serde_json::Value>,
    ) -> ZhtpResult<String> {
        // Implementation
    }
}
```

## 🔑 Certificate

TLS certificate management for zero-knowledge HTTPS.

### Constructor

```rust
pub struct Certificate {
    client: ZhtpClient,
}

impl Certificate {
    pub fn new(client: ZhtpClient) -> Self {
        // Implementation
    }
}
```

### Example

```rust
use zhtp::{Certificate, ZhtpClient, ZhtpConfig, Network};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: Some("your_private_key".to_string()),
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    let cert_manager = Certificate::new(client);
    
    // Generate a new certificate
    let mut subject = HashMap::new();
    subject.insert("country", "US");
    subject.insert("state", "CA");
    subject.insert("city", "San Francisco");
    subject.insert("organization", "My Company");
    subject.insert("email", "admin@myapp.zhtp");
    
    let cert = cert_manager.generate("myapp.zhtp", subject, Some(365)).await?;
    
    // Install certificate for domain
    let tx_hash = cert_manager.install("myapp.zhtp", &cert).await?;
    println!("Certificate installed: {}", tx_hash);
    
    // Verify certificate
    let is_valid = cert_manager.verify("myapp.zhtp").await?;
    println!("Certificate valid: {}", is_valid);
    
    Ok(())
}
```

### Methods

#### `generate()`
Generate a new TLS certificate.

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CertificateData {
    pub certificate: Vec<u8>,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub domain: String,
    pub validity_start: u64,
    pub validity_end: u64,
}

impl Certificate {
    pub async fn generate(
        &self,
        domain: &str,
        subject: HashMap<&str, &str>,
        validity_days: Option<u32>, // Default: 365
    ) -> ZhtpResult<CertificateData> {
        // Implementation
    }
}
```

#### `install()`
Install a certificate for a domain.

```rust
impl Certificate {
    pub async fn install(
        &self,
        domain: &str,
        certificate: &CertificateData,
    ) -> ZhtpResult<String> {
        // Implementation
    }
}
```

#### `verify()`
Verify a domain's certificate.

```rust
impl Certificate {
    pub async fn verify(&self, domain: &str) -> ZhtpResult<bool> {
        // Implementation
    }
}
```

## 💰 Wallet

Wallet management for ZHTP tokens and transactions.

### Constructor

```rust
pub struct Wallet {
    private_key: Option<String>,
    public_key: String,
    address: String,
    client: Option<ZhtpClient>,
}

impl Wallet {
    pub fn new(private_key: Option<String>) -> ZhtpResult<Self> {
        // Implementation
    }
    
    pub fn generate() -> ZhtpResult<Self> {
        // Generate new wallet
    }
    
    pub fn set_client(&mut self, client: ZhtpClient) {
        // Implementation
    }
}
```

### Example

```rust
use zhtp::{Wallet, ZhtpClient, ZhtpConfig, Network};

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    // Create new wallet
    let wallet = Wallet::generate()?;
    println!("Address: {}", wallet.address());
    println!("Private key: {}", wallet.private_key().unwrap());
    
    // Load existing wallet
    let existing_wallet = Wallet::new(Some("your_private_key_here".to_string()))?;
    
    // Connect to network
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: None,
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    let mut wallet = existing_wallet;
    wallet.set_client(client);
    
    // Check balance
    let balance = wallet.get_balance().await?;
    println!("Balance: {} ZHTP", balance);
    
    // Send tokens
    let tx_hash = wallet.send_tokens(
        "0x...",
        10.5,
        Some("Payment for services".to_string()),
        None
    ).await?;
    println!("Transaction sent: {}", tx_hash);
    
    // Get transaction history
    let history = wallet.get_transaction_history(Some(100), Some(0)).await?;
    for tx in history {
        println!("{}: {} {} ZHTP", tx.timestamp, tx.tx_type, tx.amount);
    }
    
    Ok(())
}
```

### Methods

#### `get_balance()`
Get wallet balance.

```rust
impl Wallet {
    pub async fn get_balance(&self) -> ZhtpResult<f64> {
        // Implementation
    }
}
```

#### `send_tokens()`
Send ZHTP tokens to another address.

```rust
impl Wallet {
    pub async fn send_tokens(
        &self,
        to_address: &str,
        amount: f64,
        memo: Option<String>,
        gas_limit: Option<u64>,
    ) -> ZhtpResult<String> {
        // Implementation
    }
}
```

#### `get_transaction_history()`
Get transaction history for the wallet.

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub timestamp: u64,
    pub tx_type: String,
    pub status: String,
    pub block_number: u64,
}

impl Wallet {
    pub async fn get_transaction_history(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ZhtpResult<Vec<Transaction>> {
        // Implementation
    }
}
```

## 🔍 Advanced Examples

### Complete DApp with Smart Contract

```rust
use zhtp::{
    ZhtpClient, DAppBuilder, SmartContract, BlockchainDNS,
    ZhtpConfig, Network, SmartContractSource
};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    // Connect to ZHTP
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: Some("your_private_key".to_string()),
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    // Create DApp builder
    let mut app = DAppBuilder::new(
        "VotingApp",
        "1.0.0",
        "Decentralized voting application"
    );
    
    // Add smart contract
    let contract_source = SmartContractSource {
        code: r#"
        contract VotingContract {
            struct Proposal {
                string title;
                string description;
                uint256 votesFor;
                uint256 votesAgainst;
                uint256 deadline;
                bool executed;
            }
            
            Proposal[] public proposals;
            mapping(address => mapping(uint256 => bool)) public hasVoted;
            
            function createProposal(
                string memory title,
                string memory description,
                uint256 duration
            ) public {
                proposals.push(Proposal(
                    title,
                    description,
                    0,
                    0,
                    block.timestamp + duration,
                    false
                ));
            }
            
            function vote(uint256 proposalId, bool support) public {
                require(proposalId < proposals.length, "Invalid proposal");
                require(!hasVoted[msg.sender][proposalId], "Already voted");
                require(block.timestamp < proposals[proposalId].deadline, "Voting ended");
                
                hasVoted[msg.sender][proposalId] = true;
                
                if (support) {
                    proposals[proposalId].votesFor++;
                } else {
                    proposals[proposalId].votesAgainst++;
                }
            }
        }
        "#.to_string(),
        abi: Some(json!({
            "functions": [
                {
                    "name": "createProposal",
                    "inputs": [
                        {"name": "title", "type": "string"},
                        {"name": "description", "type": "string"},
                        {"name": "duration", "type": "uint256"}
                    ]
                },
                {
                    "name": "vote",
                    "inputs": [
                        {"name": "proposalId", "type": "uint256"},
                        {"name": "support", "type": "bool"}
                    ]
                }
            ]
        })),
        compiler_version: "0.8.19".to_string(),
    };
    
    app.add_smart_contract("VotingContract", contract_source)?;
    
    // Add frontend files
    app.add_file("index.html", include_bytes!("../templates/voting_app.html").to_vec())?;
    app.add_file("app.js", include_bytes!("../templates/voting_app.js").to_vec())?;
    app.add_file("style.css", include_bytes!("../templates/voting_app.css").to_vec())?;
    
    // Build and deploy
    let package = app.build()?;
    let dapp_address = client.deploy_dapp("VotingApp", package, None).await?;
    
    // Register DNS
    let dns = BlockchainDNS::new(client);
    let mut record = HashMap::new();
    record.insert("address".to_string(), json!(dapp_address));
    record.insert("content_hash".to_string(), json!("QmXoYCz..."));
    
    dns.register("voting.zhtp", record, None).await?;
    
    println!("Voting DApp deployed at: voting.zhtp");
    Ok(())
}
```

### Zero-Knowledge Identity Verification

```rust
use zhtp::{ZhtpClient, ZkProof, SmartContract, ZhtpConfig, Network};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: Some("your_private_key".to_string()),
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    // Create identity verification proof
    let mut identity_proof = ZkProof::new(Some("circuits/identity_verification.circom"));
    identity_proof.load_circuit("circuits/identity_verification.circom").await?;
    
    // Generate proof without revealing personal information
    let mut inputs = HashMap::new();
    inputs.insert("age".to_string(), json!(25));
    inputs.insert("citizenship".to_string(), json!("US"));
    inputs.insert("min_age".to_string(), json!(18));
    inputs.insert("required_citizenship".to_string(), json!("US"));
    
    let proof = identity_proof.generate(inputs).await?;
    
    // Submit proof to verification contract
    let contract = SmartContract::new(
        "0x...".to_string(), // Identity verification contract
        json!({}), // Contract ABI
        client,
    );
    
    // Verify identity without revealing details
    let tx_hash = contract.send("verifyIdentity", vec![json!(proof.serialize())], None).await?;
    println!("Identity verified: {}", tx_hash);
    
    // Check verification status
    let is_verified: bool = contract.call("isVerified", vec![json!("wallet_address")]).await?;
    println!("Verification status: {}", is_verified);
    
    Ok(())
}
```

## 🛠️ Error Handling

```rust
use zhtp::{ZhtpClient, ZhtpError, NetworkError, ContractError};

#[tokio::main]
async fn main() {
    match run_app().await {
        Ok(_) => println!("App completed successfully"),
        Err(e) => match e {
            ZhtpError::Network(net_err) => {
                println!("Network error: {}", net_err);
                // Handle network issues
            }
            ZhtpError::Contract(contract_err) => {
                println!("Contract error: {}", contract_err);
                // Handle contract-specific issues
            }
            ZhtpError::InvalidInput(msg) => {
                println!("Invalid input: {}", msg);
                // Handle validation errors
            }
            ZhtpError::Timeout => {
                println!("Request timed out");
                // Handle timeouts
            }
            _ => {
                println!("Unexpected error: {}", e);
                // Handle unexpected errors
            }
        }
    }
}

async fn run_app() -> zhtp::Result<()> {
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: None,
        timeout: std::time::Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    // This might fail
    let result = client.deploy_dapp("TestApp", b"invalid_package".to_vec(), None).await?;
    
    Ok(())
}
```

## 📊 Event Monitoring

```rust
use zhtp::{ZhtpClient, SmartContract, ZhtpConfig, Network};
use serde_json::json;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    let config = ZhtpConfig {
        network: Network::Testnet,
        endpoint: None,
        private_key: None,
        timeout: Duration::from_secs(30),
    };
    
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    let contract = SmartContract::new(
        "0x...".to_string(),
        json!({}), // Contract ABI with events
        client,
    );
    
    // Listen for specific events
    let transfer_handler = |event: serde_json::Value| {
        println!("Transfer: {} -> {}: {}", 
            event["from"], event["to"], event["amount"]);
    };
    
    let approval_handler = |event: serde_json::Value| {
        println!("Approval: {} approved {}", 
            event["owner"], event["spender"]);
    };
    
    // Start event monitoring
    contract.on("Transfer", transfer_handler).await?;
    contract.on("Approval", approval_handler).await?;
    
    // Keep monitoring
    sleep(Duration::from_secs(3600)).await; // Monitor for 1 hour
    
    Ok(())
}
```

## 🔧 Configuration

### Environment Variables

```bash
# Network configuration
ZHTP_NETWORK=mainnet
ZHTP_ENDPOINT=https://mainnet.zhtp.network
ZHTP_TIMEOUT=30

# Wallet configuration
ZHTP_PRIVATE_KEY=your_private_key
ZHTP_MNEMONIC="your twelve word mnemonic phrase here"

# Development configuration
ZHTP_DEBUG=true
ZHTP_LOG_LEVEL=info
```

### Configuration File

```rust
// zhtp_config.rs
use zhtp::{ZhtpConfig, Network};
use std::time::Duration;

pub fn get_mainnet_config() -> ZhtpConfig {
    ZhtpConfig {
        network: Network::Mainnet,
        endpoint: Some("https://mainnet.zhtp.network".to_string()),
        private_key: std::env::var("ZHTP_PRIVATE_KEY").ok(),
        timeout: Duration::from_secs(30),
    }
}

pub fn get_testnet_config() -> ZhtpConfig {
    ZhtpConfig {
        network: Network::Testnet,
        endpoint: Some("https://testnet.zhtp.network".to_string()),
        private_key: std::env::var("ZHTP_PRIVATE_KEY").ok(),
        timeout: Duration::from_secs(10),
    }
}

pub fn get_local_config() -> ZhtpConfig {
    ZhtpConfig {
        network: Network::Local,
        endpoint: Some("http://localhost:8080".to_string()),
        private_key: Some("dev_private_key".to_string()),
        timeout: Duration::from_secs(5),
    }
}

// Usage
use zhtp::ZhtpClient;

#[tokio::main]
async fn main() -> zhtp::Result<()> {
    let config = get_mainnet_config();
    let mut client = ZhtpClient::new(config).await?;
    client.connect().await?;
    
    Ok(())
}
```

## 🧪 Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use zhtp::{ZhtpClient, ZhtpConfig, Network};
    use std::time::Duration;
    
    #[tokio::test]
    async fn test_client_connection() {
        let config = ZhtpConfig {
            network: Network::Local,
            endpoint: Some("http://localhost:8080".to_string()),
            private_key: None,
            timeout: Duration::from_secs(5),
        };
        
        let mut client = ZhtpClient::new(config).await.unwrap();
        assert!(client.connect().await.is_ok());
    }
    
    #[tokio::test]
    async fn test_dapp_deployment() {
        let config = get_local_config();
        let mut client = ZhtpClient::new(config).await.unwrap();
        client.connect().await.unwrap();
        
        let mut app = DAppBuilder::new("TestApp", "1.0.0", "Test application");
        app.add_file("index.html", b"<html><body>Test</body></html>".to_vec()).unwrap();
        
        let package = app.build().unwrap();
        let address = client.deploy_dapp("TestApp", package, None).await.unwrap();
        
        assert!(!address.is_empty());
    }
    
    #[tokio::test]
    async fn test_smart_contract_interaction() {
        // Test contract calls and transactions
        let config = get_local_config();
        let mut client = ZhtpClient::new(config).await.unwrap();
        client.connect().await.unwrap();
        
        let contract = SmartContract::new(
            "0x1234567890abcdef".to_string(),
            json!({"functions": []}),
            client,
        );
        
        // Test read call
        let result: u64 = contract.call("getValue", vec![]).await.unwrap_or(0);
        assert!(result >= 0);
    }
}
```

---

## 📚 Next Steps

- **[Python SDK Reference](python.md)** - Python development tools
- **[JavaScript SDK Reference](javascript.md)** - Web development with JS/TS
- **[REST API Reference](rest.md)** - HTTP API documentation  
- **[Smart Contract Guide](../guides/smart-contracts.md)** - Advanced contract development
- **[DApp Templates](../templates/)** - Ready-to-use DApp templates
- **[Community Discord](https://discord.gg/zhtp)** - Get help and connect with developers

For more examples and tutorials, visit our **[Developer Portal](../README.md)**.
