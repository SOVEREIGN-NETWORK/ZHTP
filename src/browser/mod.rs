use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::{mpsc, Mutex};

use crate::{
    storage::{ContentId, ContentMetadata},
    zhtp::{Keypair, ZhtpNode},
};

/// Browser request type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserRequest {
    /// Get content by ID
    GetContent(ContentId),
    /// Search content by query
    Search(String),
    /// Connect to ZHTP node
    Connect(SocketAddr),
    /// Deploy contract
    DeployContract(Vec<u8>, String),
    /// Call contract method
    CallContract(String, String, Vec<Vec<u8>>),
    /// Create anonymous identity
    CreateIdentity,
    /// Create personhood proof
    CreatePersonhoodProof(Vec<u8>), // biometric data
    /// Register domain in ZHTP DNS
    RegisterDomain(String, Vec<SocketAddr>),
    /// Submit DAO proposal
    SubmitProposal(String, String), // title, description
    /// Vote on DAO proposal
    VoteOnProposal(String, bool), // proposal_id, vote
    /// Deploy DApp
    DeployDApp(String, Vec<u8>), // name, code
}

/// Browser response type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserResponse {
    /// Content data
    Content(Vec<u8>, ContentMetadata),
    /// Search results
    SearchResults(Vec<(ContentId, ContentMetadata)>),
    /// Connection status
    ConnectionStatus(bool),
    /// Contract response
    ContractResponse(Vec<u8>),
    /// Identity created
    IdentityCreated(String), // commitment
    /// Personhood proof created
    PersonhoodProofCreated(String), // proof_id
    /// Domain registered
    DomainRegistered(String), // domain
    /// Proposal submitted
    ProposalSubmitted(String), // proposal_id
    /// Vote recorded
    VoteRecorded(String), // vote_id
    /// DApp deployed
    DAppDeployed(String), // deployment_hash
    /// Error message
    Error(String),
}

/// ZHTP Browser interface
pub struct ZhtpBrowser {
    /// ZHTP node
    node: Arc<Mutex<ZhtpNode>>,
    /// Request channel
    request_tx: mpsc::Sender<BrowserRequest>,
    /// Response channel
    response_rx: mpsc::Receiver<BrowserResponse>,
    /// Storage manager reference
    storage: Option<Arc<crate::storage::ZhtpStorageManager>>,
    /// Active connections
    connections: Arc<Mutex<HashMap<SocketAddr, ContentMetadata>>>,
}

impl ZhtpBrowser {
    /// Create new browser instance
    pub async fn new(addr: SocketAddr) -> Result<Self> {
        // Create ZHTP node
        let node = ZhtpNode::new(addr, Keypair::generate()).await?;
        let node = Arc::new(Mutex::new(node));

        // Create channels
        let (request_tx, request_rx) = mpsc::channel(100);
        let (response_tx, response_rx) = mpsc::channel(100);

        // Create browser
        let browser = Self {
            node: node.clone(),
            request_tx,
            response_rx,
            storage: None,
            connections: Arc::new(Mutex::new(HashMap::new())),
        };

        // Spawn request handler
        let node_clone = node.clone();
        let response_tx_clone = response_tx.clone();
        tokio::spawn(async move {
            Self::handle_requests(node_clone, request_rx, response_tx_clone).await;
        });

        Ok(browser)
    }

    /// Handle browser requests
    async fn handle_requests(
        node: Arc<Mutex<ZhtpNode>>,
        mut request_rx: mpsc::Receiver<BrowserRequest>,
        response_tx: mpsc::Sender<BrowserResponse>,
    ) {
        while let Some(request) = request_rx.recv().await {
            let response = match request {
                BrowserRequest::GetContent(id) => {
                    let n = node.lock().await;
                    match n.get_content(&id.to_string()).await {
                        Ok((data, _)) => {
                            // Create basic metadata for response
                            let metadata = ContentMetadata {
                                id: id.clone(),
                                size: data.len() as u64,
                                content_type: "application/octet-stream".to_string(),
                                locations: vec![],
                                last_verified: 0,
                                tags: vec![],
                            };
                            BrowserResponse::Content(data, metadata)
                        },
                        Err(e) => BrowserResponse::Error(e.to_string()),
                    }
                }
                BrowserRequest::Search(query) => {
                    let n = node.lock().await;
                    match n.search_content(&query).await {
                        Ok(results) => {
                            let converted: Vec<(ContentId, ContentMetadata)> = results
                                .into_iter()
                                .filter(|(_, metadata)| {
                                    // Match against content_type and tags
                                    metadata.content_type.contains(&query) ||
                                    metadata.tags.iter().any(|tag| tag.contains(&query))
                                })
                                .map(|(id, metadata)| (ContentId::from(id), metadata))
                                .collect();
                            BrowserResponse::SearchResults(converted)
                        },
                        Err(e) => BrowserResponse::Error(e.to_string()),
                    }
                }
                BrowserRequest::Connect(addr) => {
                    let mut n = node.lock().await;
                    match n.connect(addr).await {
                        Ok(_) => BrowserResponse::ConnectionStatus(true),
                        Err(_) => BrowserResponse::ConnectionStatus(false),
                    }
                }
                BrowserRequest::CreateIdentity => {
                    // Simulate identity creation
                    let commitment = format!("identity_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
                    BrowserResponse::IdentityCreated(commitment)
                }
                BrowserRequest::CreatePersonhoodProof(_biometric_data) => {
                    // Simulate personhood proof creation
                    let proof_id = format!("proof_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
                    BrowserResponse::PersonhoodProofCreated(proof_id)
                }
                BrowserRequest::RegisterDomain(domain, _addresses) => {
                    // Simulate domain registration
                    BrowserResponse::DomainRegistered(domain)
                }
                BrowserRequest::SubmitProposal(_title, _description) => {
                    // Simulate proposal submission
                    let proposal_id = format!("proposal_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
                    BrowserResponse::ProposalSubmitted(proposal_id)
                }
                BrowserRequest::VoteOnProposal(_proposal_id, _vote) => {
                    // Simulate vote recording
                    let vote_id = format!("vote_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
                    BrowserResponse::VoteRecorded(vote_id)
                }
                BrowserRequest::DeployDApp(_name, _code) => {
                    // Simulate DApp deployment
                    let deployment_hash = format!("dapp_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
                    BrowserResponse::DAppDeployed(deployment_hash)
                }
                BrowserRequest::DeployContract(bytecode, interface) => {
                    let mut n = node.lock().await;
                    match n.deploy_contract(bytecode, interface).await {
                        Ok(response) => BrowserResponse::ContractResponse(response),
                        Err(e) => BrowserResponse::Error(e.to_string()),
                    }
                }
                BrowserRequest::CallContract(id, method, params) => {
                    let mut n = node.lock().await;
                    match n.call_contract(&id, &method, params).await {
                        Ok(response) => BrowserResponse::ContractResponse(response),
                        Err(e) => BrowserResponse::Error(e.to_string()),
                    }
                }
            };

            if response_tx.send(response).await.is_err() {
                break;
            }
        }
    }

    /// Send browser request
    async fn send_request(&mut self, request: BrowserRequest) -> Result<BrowserResponse> {
        self.request_tx.send(request).await?;
        Ok(self.response_rx.recv().await.ok_or_else(|| anyhow::anyhow!("No response received"))?)
    }

    /// Get content by ID
    pub async fn get_content(&mut self, id: &ContentId) -> Result<Vec<u8>> {
        // Try DHT first if available
        if let Some(storage) = &self.storage {
            if let Ok(Some(data)) = storage.get_content(&id.to_string()).await {
                return Ok(data);
            }
        }

        // Fall back to ZHTP
        match self.send_request(BrowserRequest::GetContent(id.clone())).await? {
            BrowserResponse::Content(data, _) => Ok(data),
            BrowserResponse::Error(e) => Err(anyhow::anyhow!(e)),
            _ => Err(anyhow::anyhow!("Unexpected response")),
        }
    }

    /// Search content
    pub async fn search(&mut self, query: String) -> Result<Vec<(ContentId, ContentMetadata)>> {
        // Try DHT search first if available
        if let Some(storage) = &self.storage {
            let results = storage.search_by_tag(&query).await;
            if !results.is_empty() {
                return Ok(results);
            }
            // Also try searching by content type
            let type_results = storage.search_by_type(&query).await;
            if !type_results.is_empty() {
                return Ok(type_results);
            }
        }

        // Fall back to ZHTP search
        match self.send_request(BrowserRequest::Search(query)).await? {
            BrowserResponse::SearchResults(results) => Ok(results),
            BrowserResponse::Error(e) => Err(anyhow::anyhow!(e)),
            _ => Err(anyhow::anyhow!("Unexpected response")),
        }
    }


    /// Connect to storage network
    pub async fn connect_storage(&mut self, storage: Arc<crate::storage::ZhtpStorageManager>) -> Result<()> {
        self.storage = Some(storage);
        Ok(())
    }

    /// Connect to ZHTP node
    pub async fn connect(&mut self, addr: SocketAddr) -> Result<bool> {
        match self.send_request(BrowserRequest::Connect(addr)).await? {
            BrowserResponse::ConnectionStatus(status) => Ok(status),
            BrowserResponse::Error(e) => Err(anyhow::anyhow!(e)),
            _ => Err(anyhow::anyhow!("Unexpected response")),
        }
    }

    /// Deploy contract
    pub async fn deploy_contract(&mut self, bytecode: Vec<u8>, interface: String) -> Result<Vec<u8>> {
        // Send deployment request with provided interface
        match self.send_request(BrowserRequest::DeployContract(bytecode, interface)).await? {
            BrowserResponse::ContractResponse(response) => Ok(response),
            BrowserResponse::Error(e) => Err(anyhow::anyhow!(e)),
            _ => Err(anyhow::anyhow!("Unexpected response")),
        }
    }

    /// Call contract method
    pub async fn call_contract(
        &mut self,
        id: String,
        method: String,
        params: Vec<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        match self.send_request(BrowserRequest::CallContract(id, method, params)).await? {
            BrowserResponse::ContractResponse(response) => Ok(response),
            BrowserResponse::Error(e) => Err(anyhow::anyhow!(e)),
            _ => Err(anyhow::anyhow!("Unexpected response")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[tokio::test]
    async fn test_browser_basic() -> Result<()> {
        let addr: SocketAddr = "127.0.0.1:8000".parse()?;
        let mut browser = ZhtpBrowser::new(addr).await?;

        // Test that browser was created successfully
        println!("✅ ZHTP Browser created successfully");

        // Test content search (should be empty initially)
        let results = browser.search("test".to_string()).await?;
        assert!(results.is_empty(), "No content should be indexed initially");
        
        println!("✅ Content search returns empty results as expected");
        Ok(())
    }
}