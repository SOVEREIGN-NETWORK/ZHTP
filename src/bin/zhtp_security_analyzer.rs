use std::net::UdpSocket;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;

/// ZHTP Packet Security Analyzer
/// Monitors network traffic and validates security properties
pub struct ZhtpSecurityAnalyzer {
    capture_socket: UdpSocket,
    analysis_results: SecurityAnalysisResults,
    start_time: SystemTime,
}

#[derive(Debug, Default)]
pub struct SecurityAnalysisResults {
    pub total_packets: u64,
    pub encrypted_packets: u64,
    pub signed_packets: u64,
    pub proof_packets: u64,
    pub handshake_packets: u64,
    pub malformed_packets: u64,
    pub packet_sizes: Vec<usize>,
    pub encryption_methods: HashMap<String, u64>,
    pub security_warnings: Vec<String>,
    pub timing_analysis: Vec<(SystemTime, String)>,
}

impl ZhtpSecurityAnalyzer {
    pub fn new(monitor_port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", monitor_port))?;
        socket.set_read_timeout(Some(Duration::from_millis(100)))?;
        
        Ok(ZhtpSecurityAnalyzer {
            capture_socket: socket,
            analysis_results: SecurityAnalysisResults::default(),
            start_time: SystemTime::now(),
        })
    }
    
    pub fn start_monitoring(&mut self, duration_seconds: u64) {
        println!("=== ZHTP Security Packet Analysis ===");
        println!("Monitoring for {} seconds...", duration_seconds);
        
        let end_time = SystemTime::now() + Duration::from_secs(duration_seconds);
        let mut buffer = [0u8; 65536];
        
        while SystemTime::now() < end_time {
            match self.capture_socket.recv_from(&mut buffer) {
                Ok((size, addr)) => {
                    self.analyze_packet(&buffer[..size], addr);
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Timeout, continue monitoring
                    continue;
                }
                Err(e) => {
                    eprintln!("Error receiving packet: {}", e);
                }
            }
        }
        
        self.generate_security_report();
    }
    
    fn analyze_packet(&mut self, data: &[u8], addr: std::net::SocketAddr) {
        self.analysis_results.total_packets += 1;
        self.analysis_results.packet_sizes.push(data.len());
        
        // Record timing
        self.analysis_results.timing_analysis.push((
            SystemTime::now(),
            format!("Packet from {} size {}", addr, data.len())
        ));
        
        // Try to parse as ZHTP packet
        if let Ok(packet_info) = self.parse_zhtp_packet(data) {
            self.validate_security_properties(&packet_info);
        } else {
            self.analysis_results.malformed_packets += 1;
            self.analysis_results.security_warnings.push(
                format!("Malformed packet from {} at {:?}", addr, SystemTime::now())
            );
        }
    }
    
    fn parse_zhtp_packet(&self, data: &[u8]) -> Result<PacketInfo, Box<dyn std::error::Error>> {
        // Check for ZHTP packet structure
        if data.len() < 64 {
            return Err("Packet too small for ZHTP".into());
        }
        
        // Look for JSON structure (ZHTP packets are serialized)
        if let Ok(json_str) = std::str::from_utf8(data) {
            if json_str.contains("header") && json_str.contains("payload") {
                return Ok(PacketInfo {
                    has_header: true,
                    has_payload: true,
                    has_signature: json_str.contains("signature"),
                    has_routing_proof: json_str.contains("routing_proof"),
                    has_key_package: json_str.contains("key_package"),
                    payload_size: data.len(),
                    is_encrypted: self.detect_encryption(data),
                    packet_type: self.classify_packet_type(json_str),
                });
            }
        }
        
        // Check for binary ZHTP indicators
        Ok(PacketInfo {
            has_header: data.len() > 32,
            has_payload: data.len() > 64,
            has_signature: data.len() > 128, // Rough heuristic
            has_routing_proof: false,
            has_key_package: false,
            payload_size: data.len(),
            is_encrypted: self.detect_encryption(data),
            packet_type: "unknown".to_string(),
        })
    }
    
    fn detect_encryption(&self, data: &[u8]) -> bool {
        // High entropy indicates encryption
        let entropy = self.calculate_entropy(data);
        
        // Check for encryption indicators
        let has_high_entropy = entropy > 7.5; // High entropy threshold
        let has_crypto_patterns = data.windows(4).any(|w| {
            // Look for common crypto library signatures
            w == b"AES\x00" || w == b"RSA\x00" || w == b"ECC\x00"
        });
        
        has_high_entropy || has_crypto_patterns
    }
    
    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        let mut freq = [0u32; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }
        
        let len = data.len() as f64;
        let mut entropy = 0.0;
        
        for &count in &freq {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        entropy
    }
    
    fn classify_packet_type(&self, json_str: &str) -> String {
        if json_str.contains("discovery") {
            "discovery".to_string()
        } else if json_str.contains("handshake") {
            "handshake".to_string()
        } else if json_str.contains("transaction") {
            "transaction".to_string()
        } else if json_str.contains("consensus") {
            "consensus".to_string()
        } else {
            "data".to_string()
        }
    }
    
    fn validate_security_properties(&mut self, packet: &PacketInfo) {
        // Check for encrypted payloads
        if packet.is_encrypted {
            self.analysis_results.encrypted_packets += 1;
        } else if packet.payload_size > 100 {
            self.analysis_results.security_warnings.push(
                "Large unencrypted payload detected".to_string()
            );
        }
        
        // Check for signatures
        if packet.has_signature {
            self.analysis_results.signed_packets += 1;
        } else {
            self.analysis_results.security_warnings.push(
                "Unsigned packet detected".to_string()
            );
        }
        
        // Check for routing proofs
        if packet.has_routing_proof {
            self.analysis_results.proof_packets += 1;
        }
        
        // Check for handshake patterns
        if packet.packet_type == "handshake" {
            self.analysis_results.handshake_packets += 1;
        }
        
        // Update encryption method tracking
        let encryption_type = if packet.is_encrypted {
            "encrypted"
        } else {
            "plaintext"
        };
        
        *self.analysis_results.encryption_methods
            .entry(encryption_type.to_string())
            .or_insert(0) += 1;
    }
    
    fn generate_security_report(&self) {
        println!("\n=== ZHTP Security Analysis Report ===");
        println!("Analysis Duration: {:?}", self.start_time.elapsed().unwrap_or_default());
        println!();
        
        println!("=== Packet Statistics ===");
        println!("Total Packets: {}", self.analysis_results.total_packets);
        println!("Encrypted Packets: {} ({:.1}%)", 
                self.analysis_results.encrypted_packets,
                (self.analysis_results.encrypted_packets as f64 / self.analysis_results.total_packets as f64) * 100.0
        );
        println!("Signed Packets: {} ({:.1}%)", 
                self.analysis_results.signed_packets,
                (self.analysis_results.signed_packets as f64 / self.analysis_results.total_packets as f64) * 100.0
        );
        println!("Proof Packets: {}", self.analysis_results.proof_packets);
        println!("Handshake Packets: {}", self.analysis_results.handshake_packets);
        println!("Malformed Packets: {}", self.analysis_results.malformed_packets);
        println!();
        
        println!("=== Packet Size Analysis ===");
        if !self.analysis_results.packet_sizes.is_empty() {
            let avg_size = self.analysis_results.packet_sizes.iter().sum::<usize>() as f64 / self.analysis_results.packet_sizes.len() as f64;
            let max_size = *self.analysis_results.packet_sizes.iter().max().unwrap_or(&0);
            let min_size = *self.analysis_results.packet_sizes.iter().min().unwrap_or(&0);
            
            println!("Average Packet Size: {:.1} bytes", avg_size);
            println!("Max Packet Size: {} bytes", max_size);
            println!("Min Packet Size: {} bytes", min_size);
        }
        println!();
        
        println!("=== Encryption Methods ===");
        for (method, count) in &self.analysis_results.encryption_methods {
            println!("{}: {} packets", method, count);
        }
        println!();
        
        println!("=== Security Warnings ===");
        if self.analysis_results.security_warnings.is_empty() {
            println!("✅ No security warnings detected");
        } else {
            for warning in &self.analysis_results.security_warnings {
                println!("⚠️  {}", warning);
            }
        }
        println!();
        
        println!("=== Security Assessment ===");
        let encryption_rate = (self.analysis_results.encrypted_packets as f64 / self.analysis_results.total_packets as f64) * 100.0;
        let signature_rate = (self.analysis_results.signed_packets as f64 / self.analysis_results.total_packets as f64) * 100.0;
        
        if encryption_rate > 80.0 {
            println!("✅ High encryption rate ({:.1}%)", encryption_rate);
        } else {
            println!("⚠️  Low encryption rate ({:.1}%)", encryption_rate);
        }
        
        if signature_rate > 90.0 {
            println!("✅ High signature rate ({:.1}%)", signature_rate);
        } else {
            println!("⚠️  Low signature rate ({:.1}%)", signature_rate);
        }
        
        if self.analysis_results.proof_packets > 0 {
            println!("✅ ZK proofs detected ({} packets)", self.analysis_results.proof_packets);
        } else {
            println!("⚠️  No ZK proofs detected");
        }
        
        if self.analysis_results.malformed_packets == 0 {
            println!("✅ No malformed packets");
        } else {
            println!("⚠️  {} malformed packets detected", self.analysis_results.malformed_packets);
        }
        
        println!("\n=== Overall Security Score ===");
        let security_score = self.calculate_security_score(encryption_rate, signature_rate);
        println!("Security Score: {:.1}/10", security_score);
        
        if security_score >= 8.0 {
            println!("🛡️  EXCELLENT - Network security is strong");
        } else if security_score >= 6.0 {
            println!("⚠️  GOOD - Some security improvements recommended");
        } else {
            println!("🚨 POOR - Significant security issues detected");
        }
    }
    
    fn calculate_security_score(&self, encryption_rate: f64, signature_rate: f64) -> f64 {
        let mut score = 0.0;
        
        // Encryption weight (40%)
        score += (encryption_rate / 100.0) * 4.0;
        
        // Signature weight (30%)
        score += (signature_rate / 100.0) * 3.0;
        
        // Proof validation weight (20%)
        if self.analysis_results.proof_packets > 0 {
            score += 2.0;
        }
        
        // No malformed packets weight (10%)
        if self.analysis_results.malformed_packets == 0 {
            score += 1.0;
        }
        
        score
    }
}

#[derive(Debug)]
struct PacketInfo {
    has_header: bool,
    has_payload: bool,
    has_signature: bool,
    has_routing_proof: bool,
    has_key_package: bool,
    payload_size: usize,
    is_encrypted: bool,
    packet_type: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let monitor_port = if args.len() > 1 {
        args[1].parse().unwrap_or(9000)
    } else {
        9000
    };
    
    let duration = if args.len() > 2 {
        args[2].parse().unwrap_or(60)
    } else {
        60
    };
    
    match ZhtpSecurityAnalyzer::new(monitor_port) {
        Ok(mut analyzer) => {
            analyzer.start_monitoring(duration);
        }
        Err(e) => {
            eprintln!("Failed to start security analyzer: {}", e);
            std::process::exit(1);
        }
    }
}
