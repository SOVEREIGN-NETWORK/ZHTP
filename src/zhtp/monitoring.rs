// Real-time system monitoring and health dashboard for ZHTP protocol
// Provides comprehensive metrics for production deployment

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use anyhow::Result;

/// Comprehensive system health metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Network health indicators
    pub network: NetworkHealth,
    /// Economic system status
    pub economics: EconomicHealth,
    /// Consensus performance
    pub consensus: ConsensusHealth,
    /// Storage system status
    pub storage: StorageHealth,
    /// Security metrics
    pub security: SecurityHealth,
    /// Performance indicators
    pub performance: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkHealth {
    /// Active validators count
    pub active_validators: u64,
    /// Network uptime percentage
    pub uptime_percentage: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Packets processed per second
    pub packets_per_second: u64,
    /// Node count by type
    pub node_counts: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicHealth {
    /// Total supply in circulation
    pub total_supply: u64,
    /// Tokens burned per hour
    pub burn_rate: u64,
    /// Average validator APR
    pub validator_apr: f64,
    /// Fee market health (0-100)
    pub fee_market_health: f64,
    /// Market capitalization estimate
    pub market_cap_usd: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusHealth {
    /// Blocks finalized per hour
    pub blocks_per_hour: u64,
    /// Average finalization time (seconds)
    pub avg_finalization_time: f64,
    /// Validator participation rate
    pub participation_rate: f64,
    /// Slashing events in last 24h
    pub slashing_events_24h: u64,
    /// Current epoch
    pub current_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealth {
    /// Total storage capacity (GB)
    pub total_capacity_gb: u64,
    /// Used storage percentage
    pub utilization_percentage: f64,
    /// Replication factor
    pub avg_replication_factor: f64,
    /// Failed retrievals in last hour
    pub failed_retrievals_1h: u64,
    /// Data integrity score (0-100)
    pub data_integrity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHealth {
    /// ZK proof verification rate
    pub zk_proof_success_rate: f64,
    /// Certificate issuance per hour
    pub cert_issuance_per_hour: u64,
    /// Failed authentication attempts
    pub failed_auth_attempts_1h: u64,
    /// DNS poisoning attempts blocked
    pub dns_attacks_blocked_24h: u64,
    /// Overall security score (0-100)
    pub security_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk I/O rate (MB/s)
    pub disk_io_mbps: f64,
    /// Network bandwidth usage (Mbps)
    pub network_bandwidth_mbps: f64,
    /// Transaction throughput (TPS)
    pub transaction_throughput: u64,
}

/// Real-time monitoring system for ZHTP protocol
pub struct ZhtpMonitor {
    /// Current system metrics
    metrics: Arc<RwLock<SystemMetrics>>,
    /// Historical data for trending
    history: Arc<RwLock<Vec<(u64, SystemMetrics)>>>, // (timestamp, metrics)
    /// Alert thresholds
    thresholds: AlertThresholds,
    /// Active alerts
    alerts: Arc<RwLock<Vec<SystemAlert>>>,
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub min_validator_count: u64,
    pub max_latency_ms: f64,
    pub min_uptime_percentage: f64,
    pub max_cpu_usage: f64,
    pub max_memory_usage: f64,
    pub min_security_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAlert {
    pub id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: u64,
    pub component: String,
    pub resolved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            min_validator_count: 3,
            max_latency_ms: 500.0,
            min_uptime_percentage: 99.0,
            max_cpu_usage: 80.0,
            max_memory_usage: 85.0,
            min_security_score: 95.0,
        }
    }
}

impl ZhtpMonitor {
    /// Create new monitoring system
    pub fn new() -> Self {
        let initial_metrics = SystemMetrics::default();
        
        Self {
            metrics: Arc::new(RwLock::new(initial_metrics)),
            history: Arc::new(RwLock::new(Vec::new())),
            thresholds: AlertThresholds::default(),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Update system metrics
    pub async fn update_metrics(&self, new_metrics: SystemMetrics) -> Result<()> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Update current metrics
        {
            let mut metrics = self.metrics.write().await;
            *metrics = new_metrics.clone();
        }

        // Add to history (keep last 1000 entries)
        {
            let mut history = self.history.write().await;
            history.push((timestamp, new_metrics.clone()));
            if history.len() > 1000 {
                history.remove(0);
            }
        }

        // Check for alerts
        self.check_alerts(&new_metrics).await?;

        Ok(())
    }

    /// Check system health and generate alerts
    pub async fn check_alerts(&self, metrics: &SystemMetrics) -> Result<()> {
        let mut new_alerts = Vec::new();

        // Network health checks
        if metrics.network.active_validators < self.thresholds.min_validator_count {
            new_alerts.push(SystemAlert {
                id: format!("validator_count_{}", metrics.network.active_validators),
                severity: AlertSeverity::Critical,
                message: format!("Low validator count: {} (minimum: {})", 
                    metrics.network.active_validators, self.thresholds.min_validator_count),
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                component: "consensus".to_string(),
                resolved: false,
            });
        }

        if metrics.network.avg_latency_ms > self.thresholds.max_latency_ms {
            new_alerts.push(SystemAlert {
                id: format!("high_latency_{}", metrics.network.avg_latency_ms as u64),
                severity: AlertSeverity::Warning,
                message: format!("High network latency: {:.2}ms (threshold: {:.2}ms)", 
                    metrics.network.avg_latency_ms, self.thresholds.max_latency_ms),
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                component: "network".to_string(),
                resolved: false,
            });
        }

        if metrics.network.uptime_percentage < self.thresholds.min_uptime_percentage {
            new_alerts.push(SystemAlert {
                id: format!("low_uptime_{}", (metrics.network.uptime_percentage * 100.0) as u64),
                severity: AlertSeverity::Critical,
                message: format!("Low network uptime: {:.2}% (minimum: {:.2}%)", 
                    metrics.network.uptime_percentage, self.thresholds.min_uptime_percentage),
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                component: "network".to_string(),
                resolved: false,
            });
        }

        // Performance checks
        if metrics.performance.cpu_usage > self.thresholds.max_cpu_usage {
            new_alerts.push(SystemAlert {
                id: format!("high_cpu_{}", metrics.performance.cpu_usage as u64),
                severity: AlertSeverity::Warning,
                message: format!("High CPU usage: {:.1}% (threshold: {:.1}%)", 
                    metrics.performance.cpu_usage, self.thresholds.max_cpu_usage),
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                component: "performance".to_string(),
                resolved: false,
            });
        }

        if metrics.performance.memory_usage > self.thresholds.max_memory_usage {
            new_alerts.push(SystemAlert {
                id: format!("high_memory_{}", metrics.performance.memory_usage as u64),
                severity: AlertSeverity::Warning,
                message: format!("High memory usage: {:.1}% (threshold: {:.1}%)", 
                    metrics.performance.memory_usage, self.thresholds.max_memory_usage),
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                component: "performance".to_string(),
                resolved: false,
            });
        }

        // Security checks
        if metrics.security.security_score < self.thresholds.min_security_score {
            new_alerts.push(SystemAlert {
                id: format!("low_security_{}", metrics.security.security_score as u64),
                severity: AlertSeverity::Critical,
                message: format!("Low security score: {:.1} (minimum: {:.1})", 
                    metrics.security.security_score, self.thresholds.min_security_score),
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                component: "security".to_string(),
                resolved: false,
            });
        }

        // Add new alerts
        if !new_alerts.is_empty() {
            let mut alerts = self.alerts.write().await;
            alerts.extend(new_alerts);
        }

        Ok(())
    }

    /// Get current system status
    pub async fn get_system_status(&self) -> Result<SystemMetrics> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// Get active alerts
    pub async fn get_active_alerts(&self) -> Result<Vec<SystemAlert>> {
        let alerts = self.alerts.read().await;
        Ok(alerts.iter().filter(|a| !a.resolved).cloned().collect())
    }

    /// Get system health score (0-100)
    pub async fn get_health_score(&self) -> Result<f64> {
        let metrics = self.metrics.read().await;
        
        // Weighted health calculation
        let network_score = if metrics.network.uptime_percentage > 99.0 { 100.0 } else { metrics.network.uptime_percentage };
        let performance_score = 100.0 - (metrics.performance.cpu_usage + metrics.performance.memory_usage) / 2.0;
        let security_score = metrics.security.security_score;
        let consensus_score = if metrics.consensus.participation_rate > 0.9 { 100.0 } else { metrics.consensus.participation_rate * 100.0 };
        
        let overall_score = network_score * 0.3 + performance_score * 0.2 + security_score * 0.3 + consensus_score * 0.2;
        
        Ok(overall_score.max(0.0).min(100.0))
    }

    /// Generate comprehensive status report
    pub async fn generate_status_report(&self) -> Result<String> {
        let metrics = self.metrics.read().await;
        let alerts = self.get_active_alerts().await?;
        let health_score = self.get_health_score().await?;

        let report = format!(r#"
🌐 ZHTP Protocol System Status Report
=====================================

🏥 Overall Health Score: {:.1}%
{}

📊 Network Status:
- Active Validators: {}
- Network Uptime: {:.2}%
- Average Latency: {:.2}ms
- Packets/Second: {}

💰 Economic Health:
- Total Supply: {} ZHTP
- Burn Rate: {} ZHTP/hour
- Validator APR: {:.2}%
- Market Cap: ${} USD

🔐 Consensus Performance:
- Blocks/Hour: {}
- Finalization Time: {:.2}s
- Participation Rate: {:.1}%
- Current Epoch: {}

💾 Storage Health:
- Capacity: {} GB
- Utilization: {:.1}%
- Replication Factor: {:.1}x
- Data Integrity: {:.1}%

🛡️ Security Status:
- ZK Proof Success Rate: {:.1}%
- Certificates/Hour: {}
- Security Score: {:.1}%

⚡ Performance Metrics:
- CPU Usage: {:.1}%
- Memory Usage: {:.1}%
- Disk I/O: {:.1} MB/s
- Network Bandwidth: {:.1} Mbps
- Transaction Throughput: {} TPS

🚨 Active Alerts: {}
"#,
            health_score,
            if health_score > 95.0 { "🟢 EXCELLENT" } 
            else if health_score > 85.0 { "🟡 GOOD" }
            else if health_score > 70.0 { "🟠 WARNING" }
            else { "🔴 CRITICAL" },
            
            metrics.network.active_validators,
            metrics.network.uptime_percentage,
            metrics.network.avg_latency_ms,
            metrics.network.packets_per_second,
            
            metrics.economics.total_supply,
            metrics.economics.burn_rate,
            metrics.economics.validator_apr,
            metrics.economics.market_cap_usd,
            
            metrics.consensus.blocks_per_hour,
            metrics.consensus.avg_finalization_time,
            metrics.consensus.participation_rate * 100.0,
            metrics.consensus.current_epoch,
            
            metrics.storage.total_capacity_gb,
            metrics.storage.utilization_percentage,
            metrics.storage.avg_replication_factor,
            metrics.storage.data_integrity_score,
            
            metrics.security.zk_proof_success_rate,
            metrics.security.cert_issuance_per_hour,
            metrics.security.security_score,
            
            metrics.performance.cpu_usage,
            metrics.performance.memory_usage,
            metrics.performance.disk_io_mbps,
            metrics.performance.network_bandwidth_mbps,
            metrics.performance.transaction_throughput,
            
            alerts.len()
        );

        if !alerts.is_empty() {
            let alert_details = alerts.iter()
                .map(|alert| format!("  {} [{}] {}", 
                    match alert.severity {
                        AlertSeverity::Emergency => "🚨",
                        AlertSeverity::Critical => "🔴",
                        AlertSeverity::Warning => "🟡",
                        AlertSeverity::Info => "🔵",
                    },
                    alert.component.to_uppercase(),
                    alert.message
                ))
                .collect::<Vec<_>>()
                .join("\n");
            
            return Ok(format!("{}\n\nAlert Details:\n{}", report, alert_details));
        }

        Ok(report)
    }

    /// Resolve an alert
    pub async fn resolve_alert(&self, alert_id: &str) -> Result<()> {
        let mut alerts = self.alerts.write().await;
        if let Some(alert) = alerts.iter_mut().find(|a| a.id == alert_id) {
            alert.resolved = true;
        }
        Ok(())
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            network: NetworkHealth {
                active_validators: 0,
                uptime_percentage: 100.0,
                avg_latency_ms: 50.0,
                packets_per_second: 0,
                node_counts: HashMap::new(),
            },
            economics: EconomicHealth {
                total_supply: 21_000_000,
                burn_rate: 0,
                validator_apr: 8.0,
                fee_market_health: 100.0,
                market_cap_usd: 0,
            },
            consensus: ConsensusHealth {
                blocks_per_hour: 360,
                avg_finalization_time: 10.0,
                participation_rate: 1.0,
                slashing_events_24h: 0,
                current_epoch: 1,
            },
            storage: StorageHealth {
                total_capacity_gb: 0,
                utilization_percentage: 0.0,
                avg_replication_factor: 3.0,
                failed_retrievals_1h: 0,
                data_integrity_score: 100.0,
            },
            security: SecurityHealth {
                zk_proof_success_rate: 100.0,
                cert_issuance_per_hour: 0,
                failed_auth_attempts_1h: 0,
                dns_attacks_blocked_24h: 0,
                security_score: 100.0,
            },
            performance: PerformanceMetrics {
                cpu_usage: 10.0,
                memory_usage: 15.0,
                disk_io_mbps: 5.0,
                network_bandwidth_mbps: 10.0,
                transaction_throughput: 100,
            },
        }
    }
}

/// Utility for collecting system metrics from various sources
pub struct MetricsCollector;

impl MetricsCollector {
    /// Collect metrics from system components
    pub async fn collect_metrics() -> Result<SystemMetrics> {
        // In a real implementation, this would collect from:
        // - System resources (CPU, memory, disk, network)
        // - ZHTP consensus layer
        // - Economic system
        // - Storage DHT
        // - Security subsystems
        
        // For now, return mock data with realistic values
        let mut node_counts = HashMap::new();
        node_counts.insert("validators".to_string(), 5);
        node_counts.insert("routers".to_string(), 15);
        node_counts.insert("storage".to_string(), 25);
        node_counts.insert("dns".to_string(), 3);
        node_counts.insert("ca".to_string(), 3);

        Ok(SystemMetrics {
            network: NetworkHealth {
                active_validators: 5,
                uptime_percentage: 99.8,
                avg_latency_ms: 45.2,
                packets_per_second: 2500,
                node_counts,
            },
            economics: EconomicHealth {
                total_supply: 20_850_000,
                burn_rate: 150,
                validator_apr: 8.5,
                fee_market_health: 95.2,
                market_cap_usd: 31_275_000,
            },
            consensus: ConsensusHealth {
                blocks_per_hour: 354,
                avg_finalization_time: 8.7,
                participation_rate: 0.96,
                slashing_events_24h: 0,
                current_epoch: 127,
            },
            storage: StorageHealth {
                total_capacity_gb: 15_680,
                utilization_percentage: 67.3,
                avg_replication_factor: 3.2,
                failed_retrievals_1h: 2,
                data_integrity_score: 99.8,
            },
            security: SecurityHealth {
                zk_proof_success_rate: 99.9,
                cert_issuance_per_hour: 23,
                failed_auth_attempts_1h: 8,
                dns_attacks_blocked_24h: 12,
                security_score: 98.7,
            },
            performance: PerformanceMetrics {
                cpu_usage: 35.2,
                memory_usage: 42.1,
                disk_io_mbps: 125.7,
                network_bandwidth_mbps: 847.3,
                transaction_throughput: 1250,
            },
        })
    }
}
