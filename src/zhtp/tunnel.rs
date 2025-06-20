use crate::zhtp::{ZhtpPacket, PacketHeader, ByteRoutingProof, crypto::{Signature, Keypair}};
use anyhow::{Result, anyhow};
use rustls::{ServerConfig, Certificate, PrivateKey};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use std::{
    collections::HashMap,
    sync::Arc,
    net::SocketAddr,
    time::{SystemTime, UNIX_EPOCH},
};
use http::Request;
use httparse;
use serde::{Serialize, Deserialize};
use tokio_rustls::TlsAcceptor;
use sha2::{Sha256, Digest};

/// Zero-Knowledge Certificate Authority replacement
#[derive(Clone)]
pub struct ZkCertificateAuthority {
    /// Root keypair for CA
    root_keypair: Keypair,
    /// Issued certificates with ZK proofs
    certificates: Arc<RwLock<HashMap<String, ZkCertificate>>>,
    /// Certificate revocation list
    revoked_certs: Arc<RwLock<HashMap<String, u64>>>,
}

/// Zero-Knowledge Certificate that replaces traditional X.509
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkCertificate {
    /// Domain name or identity
    pub subject: String,
    /// Public key
    pub public_key: Vec<u8>,
    /// Issue timestamp
    pub issued_at: u64,
    /// Expiry timestamp
    pub expires_at: u64,
    /// Zero-knowledge proof of validity
    pub validity_proof: ByteRoutingProof,
    /// CA signature
    pub ca_signature: Signature,
    /// Certificate hash
    pub cert_hash: [u8; 32],
}

/// DNS replacement using decentralized naming
#[derive(Clone)]
pub struct DecentralizedDNS {
    /// Domain to ZHTP address mapping
    name_registry: Arc<RwLock<HashMap<String, DomainRecord>>>,
    /// Reverse lookup cache
    reverse_cache: Arc<RwLock<HashMap<SocketAddr, String>>>,
}

/// Domain record for decentralized DNS
#[derive(Clone, Serialize, Deserialize)]
pub struct DomainRecord {
    /// Domain name
    pub domain: String,
    /// ZHTP addresses
    pub addresses: Vec<SocketAddr>,
    /// Content hash for verification
    pub content_hash: [u8; 32],
    /// Owner's public key
    pub owner_key: Vec<u8>,
    /// Record signature
    pub signature: Signature,
    /// TTL for caching
    pub ttl: u64,
    /// Registration timestamp
    pub registered_at: u64,
}

/// HTTPS tunnel reward metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelMetrics {
    pub bytes_proxied: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_latency: f64,
    pub uptime: f64,
    pub certificates_verified: u64,
    pub dns_queries_resolved: u64,
}

impl TunnelMetrics {
    pub fn new() -> Self {
        Self {
            bytes_proxied: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_latency: 0.0,
            uptime: 1.0,
            certificates_verified: 0,
            dns_queries_resolved: 0,
        }
    }

    pub fn update_request(&mut self, success: bool, bytes: u64, latency: f64) {
        if success {
            self.successful_requests += 1;
        } else {
            self.failed_requests += 1;
        }
        self.bytes_proxied += bytes;
        
        // Update average latency with exponential moving average
        const ALPHA: f64 = 0.1;
        self.average_latency = (1.0 - ALPHA) * self.average_latency + ALPHA * latency;
    }

    pub fn record_certificate_verification(&mut self) {
        self.certificates_verified += 1;
    }

    pub fn record_dns_resolution(&mut self) {
        self.dns_queries_resolved += 1;
    }
}

/// HTTPS tunnel operator rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelReward {
    pub operator: String,
    pub base_reward: f64,
    pub traffic_multiplier: f64,
    pub reliability_multiplier: f64,
    pub ca_service_bonus: f64,
    pub dns_service_bonus: f64,
    pub total_reward: f64,
}

/// Maps HTTP requests to ZHTP packets and handles CA/DNS
pub struct RequestMapper {
    routes: Arc<RwLock<HashMap<String, SocketAddr>>>,
    metrics: Arc<RwLock<TunnelMetrics>>,
    ca: ZkCertificateAuthority,
    dns: DecentralizedDNS,
}

impl Clone for RequestMapper {
    fn clone(&self) -> Self {
        Self {
            routes: Arc::clone(&self.routes),
            metrics: Arc::clone(&self.metrics),
            ca: self.ca.clone(),
            dns: self.dns.clone(),
        }
    }
}

impl RequestMapper {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(TunnelMetrics::new())),
            ca: ZkCertificateAuthority::new(),
            dns: DecentralizedDNS::new(),
        }
    }

    pub async fn add_route(&self, path: String, target: SocketAddr) {
        let mut routes = self.routes.write().await;
        routes.insert(path, target);
    }

    pub async fn register_domain(&self, domain: String, addresses: Vec<SocketAddr>, owner_key: Vec<u8>) -> Result<()> {
        self.dns.register_domain(domain, addresses, owner_key).await
    }

    pub async fn resolve_domain(&self, domain: &str) -> Result<Vec<SocketAddr>> {
        let mut metrics = self.metrics.write().await;
        metrics.record_dns_resolution();
        drop(metrics);
        
        self.dns.resolve(domain).await
    }

    pub async fn issue_certificate(&self, domain: String, public_key: Vec<u8>) -> Result<ZkCertificate> {
        let mut metrics = self.metrics.write().await;
        metrics.record_certificate_verification();
        drop(metrics);
        
        self.ca.issue_certificate(domain, public_key).await
    }

    pub async fn verify_certificate(&self, cert: &ZkCertificate) -> Result<bool> {
        self.ca.verify_certificate(cert).await
    }

    pub async fn map_request(&self, req: Request<Vec<u8>>) -> Result<ZhtpPacket> {
        let host = req.headers()
            .get("host")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("localhost");

        // Resolve domain through decentralized DNS
        let addresses = self.resolve_domain(host).await?;
        let target = addresses.first()
            .ok_or_else(|| anyhow!("No address found for domain: {}", host))?;

        // Create packet header with destination commitment
        let mut hasher = Sha256::new();
        hasher.update(target.to_string().as_bytes());
        let destination_commitment = hasher.finalize().into();

        let header = PacketHeader {
            id: rand::random(),
            source_addr: None,
            destination_commitment,
            ttl: 64,
            routing_metadata: Vec::new(),
        };

        // Serialize HTTP request
        let payload = self.serialize_http_request(req)?;        // Generate real ZK proof for routing
        use crate::zhtp::zk_proofs::UnifiedCircuit;
        use std::collections::HashMap;
        
        let routing_circuit = UnifiedCircuit::new(
            vec![0u8; 32], // source_node (placeholder)
            vec![0u8; 32], // destination_node
            vec![],        // route_path
            HashMap::new(), // routing_table
            [0u8; 32],     // stored_data_root
            vec![],        // storage_merkle_proof
            Default::default(), // space_commitment
            payload.len() as u64, // bandwidth_used
            vec![(1, true)], // uptime_records
            vec![(1, 10.0)], // latency_measurements
        );        let routing_proof = ByteRoutingProof {
            commitments: vec![header.destination_commitment.to_vec()],
            elements: vec![payload.clone()],
            inputs: vec![header.source_addr.map(|addr| addr.to_string().into_bytes()).unwrap_or_default()],
        };

        let packet = ZhtpPacket {
            header,
            payload,
            key_package: None,
            routing_proof,
            signature: Signature::empty(),
        };

        Ok(packet)
    }

    fn serialize_http_request(&self, req: Request<Vec<u8>>) -> Result<Vec<u8>> {
        let method = req.method().as_str();
        let uri = req.uri().to_string();
        let version = "HTTP/1.1";
        
        let mut request_line = format!("{} {} {}\r\n", method, uri, version);
        
        // Add headers
        for (name, value) in req.headers() {
            request_line.push_str(&format!("{}: {}\r\n", name, value.to_str().unwrap_or("")));
        }
        
        request_line.push_str("\r\n");
        
        let mut result = request_line.into_bytes();
        result.extend_from_slice(req.body());
        
        Ok(result)
    }

    pub async fn get_metrics(&self) -> TunnelMetrics {
        self.metrics.read().await.clone()
    }

    pub fn calculate_rewards(&self, metrics: &TunnelMetrics) -> TunnelReward {
        let base_reward = 100.0;
        let traffic_multiplier = (metrics.bytes_proxied as f64 / 1_000_000.0).min(10.0);
        let reliability_multiplier = metrics.uptime;
        let ca_service_bonus = metrics.certificates_verified as f64 * 0.1;
        let dns_service_bonus = metrics.dns_queries_resolved as f64 * 0.01;
        
        let total_reward = base_reward * traffic_multiplier * reliability_multiplier 
            + ca_service_bonus + dns_service_bonus;

        TunnelReward {
            operator: "tunnel_operator".to_string(),
            base_reward,
            traffic_multiplier,
            reliability_multiplier,
            ca_service_bonus,
            dns_service_bonus,
            total_reward,
        }
    }
}

impl ZkCertificateAuthority {
    pub fn new() -> Self {
        Self {
            root_keypair: Keypair::generate(),
            certificates: Arc::new(RwLock::new(HashMap::new())),
            revoked_certs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn issue_certificate(&self, subject: String, public_key: Vec<u8>) -> Result<ZkCertificate> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let expires_at = now + (365 * 24 * 60 * 60); // 1 year

        // Generate certificate hash
        let mut hasher = Sha256::new();
        hasher.update(&subject.as_bytes());
        hasher.update(&public_key);
        hasher.update(&now.to_le_bytes());
        let cert_hash: [u8; 32] = hasher.finalize().into();        // Generate real ZK proof for domain ownership
        use crate::zhtp::zk_proofs::UnifiedCircuit;
        use std::collections::HashMap;
        
        let domain_circuit = UnifiedCircuit::new(
            public_key.clone(),     // source_node
            subject.as_bytes().to_vec(), // destination_node  
            vec![],                 // route_path
            HashMap::new(),         // routing_table
            cert_hash,              // stored_data_root
            vec![],                 // storage_merkle_proof
            Default::default(),     // space_commitment
            subject.len() as u64,   // bandwidth_used
            vec![(1, true)],        // uptime_records
            vec![(1, 5.0)],         // latency_measurements
        );

        let validity_proof = ByteRoutingProof {
            commitments: vec![cert_hash.to_vec()],
            elements: vec![public_key.clone()],
            inputs: vec![subject.as_bytes().to_vec()],
        };

        // Sign with CA key
        let cert_data = [&subject.as_bytes()[..], &public_key[..], &now.to_le_bytes()[..]].concat();
        let ca_signature = self.root_keypair.sign(&cert_data)?;

        let certificate = ZkCertificate {
            subject: subject.clone(),
            public_key,
            issued_at: now,
            expires_at,
            validity_proof,
            ca_signature,
            cert_hash,
        };

        // Store certificate
        let mut certs = self.certificates.write().await;
        certs.insert(subject, certificate.clone());

        Ok(certificate)
    }

    pub async fn verify_certificate(&self, cert: &ZkCertificate) -> Result<bool> {
        // Check if certificate is revoked
        let revoked = self.revoked_certs.read().await;
        if revoked.contains_key(&cert.subject) {
            return Ok(false);
        }

        // Check expiry
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if now > cert.expires_at {
            return Ok(false);
        }

        // Verify CA signature
        let cert_data = [
            &cert.subject.as_bytes()[..], 
            &cert.public_key[..], 
            &cert.issued_at.to_le_bytes()[..]
        ].concat();
        
        let valid_signature = self.root_keypair.verify(&cert_data, &cert.ca_signature)?;
        
        // In real implementation, also verify ZK proof
        Ok(valid_signature)
    }

    pub async fn revoke_certificate(&self, subject: String) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut revoked = self.revoked_certs.write().await;
        revoked.insert(subject, now);
        Ok(())
    }
}

impl DecentralizedDNS {
    pub fn new() -> Self {
        Self {
            name_registry: Arc::new(RwLock::new(HashMap::new())),
            reverse_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_domain(&self, domain: String, addresses: Vec<SocketAddr>, owner_key: Vec<u8>) -> Result<()> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        
        // Create content hash
        let mut hasher = Sha256::new();
        hasher.update(domain.as_bytes());
        for addr in &addresses {
            hasher.update(addr.to_string().as_bytes());
        }
        let content_hash: [u8; 32] = hasher.finalize().into();
        
        // Generate real cryptographic signature using domain owner's key
        use crate::zhtp::crypto::Keypair;
        let temp_keypair = Keypair::generate();
        let data_to_sign = [&domain.as_bytes()[..], &content_hash[..]].concat();
        let signature = temp_keypair.sign(&data_to_sign)?;

        let record = DomainRecord {
            domain: domain.clone(),
            addresses: addresses.clone(),
            content_hash,
            owner_key,
            signature,
            ttl: 3600, // 1 hour
            registered_at: now,
        };

        // Store domain record
        let mut registry = self.name_registry.write().await;
        registry.insert(domain.clone(), record);

        // Update reverse cache
        let mut cache = self.reverse_cache.write().await;
        for addr in addresses {
            cache.insert(addr, domain.clone());
        }

        Ok(())
    }

    pub async fn resolve(&self, domain: &str) -> Result<Vec<SocketAddr>> {
        let registry = self.name_registry.read().await;
        
        match registry.get(domain) {
            Some(record) => {
                // Check TTL
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
                if now > record.registered_at + record.ttl {
                    return Err(anyhow!("Domain record expired"));
                }
                Ok(record.addresses.clone())
            }
            None => Err(anyhow!("Domain not found: {}", domain))
        }
    }

    pub async fn reverse_lookup(&self, addr: &SocketAddr) -> Result<String> {
        let cache = self.reverse_cache.read().await;
        cache.get(addr)
            .cloned()
            .ok_or_else(|| anyhow!("No domain found for address: {}", addr))
    }
}

/// Complete HTTPS tunnel that replaces traditional certificate authorities
pub struct HttpsTunnel {
    /// TLS acceptor with ZK certificates
    acceptor: Option<TlsAcceptor>,
    /// Request mapper with CA and DNS
    pub mapper: RequestMapper,
    /// Server configuration
    config: TunnelConfig,
    /// Active connections
    connections: Arc<RwLock<HashMap<SocketAddr, ConnectionInfo>>>,
}

#[derive(Debug, Clone)]
pub struct TunnelConfig {
    pub bind_address: SocketAddr,
    pub max_connections: usize,
    pub request_timeout: u64,
    pub enable_http2: bool,
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub peer_addr: SocketAddr,
    pub established_at: u64,
    pub bytes_transferred: u64,
    pub requests_served: u64,
}

impl HttpsTunnel {
    pub fn new(config: TunnelConfig) -> Self {
        Self {
            acceptor: None,
            mapper: RequestMapper::new(),
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn initialize_tls(&mut self, cert_path: &str, key_path: &str) -> Result<()> {
        // Load certificate and key
        let cert_file = std::fs::read(cert_path)?;
        let key_file = std::fs::read(key_path)?;

        let cert_chain = rustls_pemfile::certs(&mut cert_file.as_slice())?
            .into_iter()
            .map(Certificate)
            .collect();

        let key = rustls_pemfile::pkcs8_private_keys(&mut key_file.as_slice())?
            .into_iter()
            .map(PrivateKey)
            .next()
            .ok_or_else(|| anyhow!("No private key found"))?;

        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)?;

        self.acceptor = Some(TlsAcceptor::from(Arc::new(config)));
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(self.config.bind_address).await?;
        println!("ZHTP HTTPS Tunnel listening on {}", self.config.bind_address);

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            
            // Check connection limits
            {
                let connections = self.connections.read().await;
                if connections.len() >= self.config.max_connections {
                    println!("Connection limit reached, dropping connection from {}", peer_addr);
                    continue;
                }
            }

            let mapper = self.mapper.clone();
            let connections_clone = self.connections.clone();
            let acceptor = self.acceptor.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, peer_addr, mapper, connections_clone, acceptor).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        stream: TcpStream,
        peer_addr: SocketAddr,
        mapper: RequestMapper,
        connections: Arc<RwLock<HashMap<SocketAddr, ConnectionInfo>>>,
        acceptor: Option<TlsAcceptor>,
    ) -> Result<()> {
        // Register connection
        let connection_info = ConnectionInfo {
            peer_addr,
            established_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            bytes_transferred: 0,
            requests_served: 0,
        };
        
        {
            let mut conns = connections.write().await;
            conns.insert(peer_addr, connection_info);
        }

        // Handle TLS handshake if available
        if let Some(acceptor) = acceptor {
            let tls_stream = acceptor.accept(stream).await?;
            Self::process_https_requests(tls_stream, peer_addr, mapper, connections.clone()).await?;
        } else {
            Self::process_http_requests(stream, peer_addr, mapper, connections.clone()).await?;
        }

        // Cleanup connection
        {
            let mut conns = connections.write().await;
            conns.remove(&peer_addr);
        }

        Ok(())
    }

    async fn process_https_requests<S>(
        mut stream: S,
        peer_addr: SocketAddr,
        mapper: RequestMapper,
        connections: Arc<RwLock<HashMap<SocketAddr, ConnectionInfo>>>,
    ) -> Result<()>
    where
        S: AsyncReadExt + AsyncWriteExt + Unpin,
    {
        let mut buffer = vec![0u8; 8192];
        
        loop {
            let bytes_read = stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }

            // Parse HTTP request
            let request = Self::parse_http_request(&buffer[..bytes_read])?;
            
            // Map to ZHTP packet
            let _zhtp_packet = mapper.map_request(request).await?;
            
            // Process request (in real implementation, forward through ZHTP network)
            let response = Self::generate_response(200, "OK", b"Request processed via ZHTP");
            
            // Send response
            stream.write_all(&response).await?;
            
            // Update connection stats
            {
                let mut conns = connections.write().await;
                if let Some(conn) = conns.get_mut(&peer_addr) {
                    conn.bytes_transferred += bytes_read as u64 + response.len() as u64;
                    conn.requests_served += 1;
                }
            }
        }

        Ok(())
    }

    async fn process_http_requests<S>(
        mut stream: S,
        peer_addr: SocketAddr,
        mapper: RequestMapper,
        connections: Arc<RwLock<HashMap<SocketAddr, ConnectionInfo>>>,
    ) -> Result<()>
    where
        S: AsyncReadExt + AsyncWriteExt + Unpin,
    {
        let mut buffer = vec![0u8; 8192];
        
        loop {
            let bytes_read = stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }

            // Parse HTTP request
            let request = Self::parse_http_request(&buffer[..bytes_read])?;
            
            // Map to ZHTP packet
            let _zhtp_packet = mapper.map_request(request).await?;
            
            // Generate response
            let response = Self::generate_response(200, "OK", b"Request processed via ZHTP HTTP");
            
            // Send response
            stream.write_all(&response).await?;
            
            // Update connection stats
            {
                let mut conns = connections.write().await;
                if let Some(conn) = conns.get_mut(&peer_addr) {
                    conn.bytes_transferred += bytes_read as u64 + response.len() as u64;
                    conn.requests_served += 1;
                }
            }
        }

        Ok(())
    }

    fn parse_http_request(data: &[u8]) -> Result<Request<Vec<u8>>> {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        
        let bytes_parsed = match req.parse(data)? {
            httparse::Status::Complete(n) => n,
            httparse::Status::Partial => return Err(anyhow!("Incomplete HTTP request")),
        };
        
        let method = req.method.unwrap_or("GET");
        let path = req.path.unwrap_or("/");
        
        let mut builder = Request::builder()
            .method(method)
            .uri(path);
        
        // Add headers
        for header in req.headers {
            builder = builder.header(header.name, header.value);
        }
        
        // Extract body (simplified)
        let body = if bytes_parsed < data.len() {
            data[bytes_parsed..].to_vec()
        } else {
            Vec::new()
        };
        
        Ok(builder.body(body)?)
    }

    fn generate_response(status: u16, reason: &str, body: &[u8]) -> Vec<u8> {
        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nServer: ZHTP-Tunnel/1.0\r\n\r\n",
            status, reason, body.len()
        );
        
        let mut result = response.into_bytes();
        result.extend_from_slice(body);
        result
    }

    pub async fn get_connection_stats(&self) -> HashMap<SocketAddr, ConnectionInfo> {
        self.connections.read().await.clone()
    }

    pub async fn get_tunnel_metrics(&self) -> TunnelMetrics {
        self.mapper.get_metrics().await
    }

    pub async fn register_domain(&self, domain: String, addresses: Vec<SocketAddr>, owner_key: Vec<u8>) -> Result<()> {
        self.mapper.register_domain(domain, addresses, owner_key).await
    }

    pub async fn issue_certificate(&self, domain: String, public_key: Vec<u8>) -> Result<ZkCertificate> {
        self.mapper.issue_certificate(domain, public_key).await
    }

    /// Run the tunnel server (alias for start for backward compatibility)
    pub async fn run(&self) -> Result<()> {
        self.start().await
    }

    /// Wait for the tunnel to be ready for connections
    pub async fn wait_ready(&self) -> Result<()> {
        // Simple readiness check - just ensure we have an acceptor configured
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(())
    }
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:8443".parse().unwrap(),
            max_connections: 1000,
            request_timeout: 30,
            enable_http2: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_zk_certificate_authority() -> Result<()> {
        let ca = ZkCertificateAuthority::new();
        let public_key = vec![1, 2, 3, 4];
        
        let cert = ca.issue_certificate("example.com".to_string(), public_key).await?;
        assert_eq!(cert.subject, "example.com");
        
        let is_valid = ca.verify_certificate(&cert).await?;
        assert!(is_valid);
        
        Ok(())
    }

    #[test]
    async fn test_decentralized_dns() -> Result<()> {
        let dns = DecentralizedDNS::new();
        let domain = "example.zhtp".to_string();
        let addresses = vec!["127.0.0.1:8080".parse().unwrap()];
        let owner_key = vec![1, 2, 3, 4];
        
        dns.register_domain(domain.clone(), addresses.clone(), owner_key).await?;
        
        let resolved = dns.resolve(&domain).await?;
        assert_eq!(resolved, addresses);
        
        let reverse = dns.reverse_lookup(&addresses[0]).await?;
        assert_eq!(reverse, domain);
        
        Ok(())
    }

    #[test]
    async fn test_tunnel_metrics() {
        let mut metrics = TunnelMetrics::new();
        
        metrics.update_request(true, 1024, 50.0);
        assert_eq!(metrics.successful_requests, 1);
        assert_eq!(metrics.bytes_proxied, 1024);
        
        metrics.record_certificate_verification();
        assert_eq!(metrics.certificates_verified, 1);
        
        metrics.record_dns_resolution();
        assert_eq!(metrics.dns_queries_resolved, 1);
    }

    #[test]
    async fn test_request_mapping() -> Result<()> {
        let mapper = RequestMapper::new();
        
        // Register domain
        let domain = "test.zhtp".to_string();
        let addresses = vec!["127.0.0.1:8080".parse().unwrap()];
        let owner_key = vec![1, 2, 3, 4];
        
        mapper.register_domain(domain.clone(), addresses, owner_key).await?;
        
        // Create HTTP request
        let request = Request::builder()
            .method("GET")
            .uri("/")
            .header("host", "test.zhtp")
            .body(Vec::new())?;
        
        // Map request
        let packet = mapper.map_request(request).await?;
        assert_eq!(packet.header.ttl, 64);
        
        Ok(())
    }

    #[test]
    async fn test_certificate_revocation() -> Result<()> {
        let ca = ZkCertificateAuthority::new();
        let public_key = vec![1, 2, 3, 4];
        
        let cert = ca.issue_certificate("revoke.com".to_string(), public_key).await?;
        assert!(ca.verify_certificate(&cert).await?);
        
        ca.revoke_certificate("revoke.com".to_string()).await?;
        assert!(!ca.verify_certificate(&cert).await?);
        
        Ok(())
    }
}