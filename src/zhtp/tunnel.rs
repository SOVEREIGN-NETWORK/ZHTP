use crate::zhtp::{ZhtpPacket, PacketHeader, RoutingProof, ByteRoutingProof, crypto::Signature};
use anyhow::Result;
use rustls::ServerConfig;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};
use std::{
    collections::HashMap,
    sync::Arc,
    net::SocketAddr,
    io::{BufReader, Seek, SeekFrom},
};
use http::{Request, Response, StatusCode, Method};
use httparse;
use serde::{Serialize, Deserialize};
use tokio_rustls::TlsAcceptor;

/// HTTPS tunnel reward metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelMetrics {
    pub bytes_proxied: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_latency: f64,
    pub uptime: f64,
}

impl TunnelMetrics {
    pub fn new() -> Self {
        Self {
            bytes_proxied: 0,
            successful_requests: 0,
            failed_requests: 0,
            average_latency: 0.0,
            uptime: 1.0,
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
}

/// HTTPS tunnel operator rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelReward {
    pub operator: String,
    pub base_reward: f64,
    pub traffic_multiplier: f64,
    pub reliability_multiplier: f64,
    pub total_reward: f64,
}

/// Maps HTTP requests to ZHTP packets
#[derive(Debug)]
pub struct RequestMapper {
    routes: Arc<RwLock<HashMap<String, SocketAddr>>>,
    metrics: Arc<RwLock<TunnelMetrics>>,
}

impl Clone for RequestMapper {
    fn clone(&self) -> Self {
        Self {
            routes: Arc::clone(&self.routes),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

impl RequestMapper {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(TunnelMetrics::new())),
        }
    }

    pub async fn add_route(&self, path: String, target: SocketAddr) {
        let mut routes = self.routes.write().await;
        routes.insert(path, target);
    }

    pub async fn map_request(&self, req: Request<Vec<u8>>) -> Result<ZhtpPacket> {
        let routes = self.routes.read().await;
        let path = req.uri().path();
        
        let target = routes
            .get(path)
            .ok_or_else(|| anyhow::anyhow!("No route found for path"))?;

        let header = PacketHeader {
            id: rand::random(),
            source_addr: None,
            destination_commitment: [0; 32], // TODO: Calculate proper commitment
            ttl: 32,
            routing_metadata: Vec::new(),
        };

        // Create empty signature - would be properly signed in production
        let signature = Signature::empty();

        Ok(ZhtpPacket {
            header,
            payload: req.into_body(),
            key_package: None,
            routing_proof: ByteRoutingProof {
                commitments: vec![],
                elements: vec![],
                inputs: vec![],
            },
            signature,
        })
    }

    pub async fn get_metrics(&self) -> TunnelMetrics {
        self.metrics.read().await.clone()
    }
}

/// HTTPS tunnel server
#[derive(Clone)]
pub struct HttpsTunnel {
    tls_config: Arc<ServerConfig>,
    pub mapper: RequestMapper,
    pub listener: Arc<TcpListener>,
}

impl HttpsTunnel {
    /// Wait until the tunnel is ready to accept connections
    pub async fn wait_ready(&self) -> Result<()> {
        for _ in 0..50 {
            if self.listener.local_addr().is_ok() {
                return Ok(());
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
        Err(anyhow::anyhow!("Tunnel failed to initialize"))
    }

    pub async fn new(addr: SocketAddr, cert_path: &str, key_path: &str) -> Result<Self> {
        // Load TLS certificate and key
        let cert_file = std::fs::File::open(cert_path)?;
        let mut key_file = std::fs::File::open(key_path)?;
        
        let certs = rustls_pemfile::certs(&mut BufReader::new(cert_file))?
            .into_iter()
            .map(rustls::Certificate)
            .collect();
            
        // Read key file with better error handling
        println!("Reading key file...");
        let mut reader = BufReader::new(key_file);
        let key_vec = match rustls_pemfile::pkcs8_private_keys(&mut reader) {
            Ok(mut keys) if !keys.is_empty() => keys.remove(0),
            Ok(_) => {
                // Try EC format if PKCS8 is empty
                reader.seek(std::io::SeekFrom::Start(0))?;
                let mut ec_keys = rustls_pemfile::ec_private_keys(&mut reader)?;
                if ec_keys.is_empty() {
                    return Err(anyhow::anyhow!("No valid private key found"));
                }
                ec_keys.remove(0)
            }
            Err(e) => {
                println!("PKCS8 parse failed: {:?}", e);
                // Try EC format on PKCS8 parse failure
                reader.seek(std::io::SeekFrom::Start(0))?;
                let mut ec_keys = rustls_pemfile::ec_private_keys(&mut reader)?;
                if ec_keys.is_empty() {
                    return Err(anyhow::anyhow!("No valid private key found"));
                }
                ec_keys.remove(0)
            }
        };
        
        let key = rustls::PrivateKey(key_vec);

        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)?;

        let listener = TcpListener::bind(addr).await?;

        Ok(Self {
            tls_config: Arc::new(config),
            mapper: RequestMapper::new(),
            listener: Arc::new(listener),
        })
    }

    pub async fn run(self: Arc<Self>) -> Result<()> {
        println!("HTTPS tunnel listening on {}", self.listener.local_addr()?);

        loop {
            match self.listener.accept().await {
                Ok((stream, peer_addr)) => {
                    let tunnel = self.clone();
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(
                            stream,
                            peer_addr,
                            tunnel.tls_config.clone(),
                            tunnel.mapper.clone()
                        ).await {
                            eprintln!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Accept error: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_connection(
        stream: TcpStream,
        peer_addr: SocketAddr,
        tls_config: Arc<ServerConfig>,
        mapper: RequestMapper,
    ) -> Result<()> {
        let acceptor = TlsAcceptor::from(tls_config);
        let mut tls_stream = acceptor.accept(stream).await?;

        // Buffer for reading HTTP request
        let mut buffer = Vec::new();
        let start_time = std::time::Instant::now();
        
        // Read the complete request
        loop {
            let mut chunk = [0; 8192];
            let n = tls_stream.read(&mut chunk).await?;
            if n == 0 { break; }
            buffer.extend_from_slice(&chunk[..n]);
        }

        // Parse headers
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        let _ = req.parse(&buffer)?;
        
        // Update metrics and handle request
        {
            let mut metrics = mapper.metrics.write().await;
            metrics.update_request(true, buffer.len() as u64, start_time.elapsed().as_millis() as f64);
        }

        // Build request from parsed data
        let uri = req.path
            .ok_or_else(|| anyhow::anyhow!("No path in request"))?
            .to_string();
            
        // Parse method string to proper HTTP method
        // Convert method string to http::Method
        let method_str = req.method
            .ok_or_else(|| anyhow::anyhow!("No method in request"))?;
            
        let method = match method_str {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            _ => Method::GET, // Default to GET for unknown methods
        };
            
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .body(buffer)?;

        // Convert to ZHTP packet and route it
        let zhtp_packet = mapper.map_request(request).await?;
        
        // Create response
        let response = Response::builder()
            .status(StatusCode::OK)
            .body("Request forwarded to ZHTP network".as_bytes().to_vec())?;
            
        // Write response
        let response_data = format!(
            "HTTP/1.1 {} {}\r\n\r\n{}",
            response.status().as_str(),
            response.status().canonical_reason().unwrap_or(""),
            String::from_utf8_lossy(response.body()),
        );
        
        tls_stream.write_all(response_data.as_bytes()).await?;
        tls_stream.flush().await?;
        
        // Update metrics with response bytes
        {
            let mut metrics = mapper.metrics.write().await;
            metrics.bytes_proxied += response_data.len() as u64;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_request_mapping() {
        let mapper = RequestMapper::new();
        mapper.add_route("/test".to_string(), "127.0.0.1:8000".parse().unwrap()).await;

        let req = Request::builder()
            .uri("/test")
            .body(vec![1, 2, 3])
            .unwrap();

        let packet = mapper.map_request(req).await.unwrap();
        assert_eq!(packet.payload, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_tunnel_metrics() {
        let mapper = RequestMapper::new();
        let metrics = Arc::clone(&mapper.metrics);

        {
            let mut m = metrics.write().await;
            m.update_request(true, 1000, 50.0);
            m.update_request(false, 500, 100.0);
        }

        let final_metrics = mapper.get_metrics().await;
        assert_eq!(final_metrics.successful_requests, 1);
        assert_eq!(final_metrics.failed_requests, 1);
        assert_eq!(final_metrics.bytes_proxied, 1500);
        assert!(final_metrics.average_latency > 0.0);
    }
}