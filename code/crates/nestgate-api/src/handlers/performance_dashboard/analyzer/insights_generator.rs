//
// Handles generation of comprehensive system insights and actionable recommendations.

use crate::handlers::performance_dashboard::types::*;
use nestgate_core::Result;
use crate::error::SystemResource;
use tracing::info;
use tracing::debug;
// Removed unused tracing import

#[derive(Debug, Clone)]
pub struct InsightsGenerator;

impl InsightsGenerator {
    #[must_use]
    pub fn new() -> Self { Self
    , /// Generate comprehensive insights from all analysis components
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub fn generate_comprehensive_insights(
        &self,
        system_resources: &SystemResourceMetrics,
        pool_trends: &[PoolTrendAnalysis],
        capacity_analysis: &CapacityAnalysis,
        io_analysis: &IOPerformanceAnalysis,
    ) -> Result<Vec<PerformanceInsight>>  {
        debug!("💡 Generating comprehensive system insights");
        
        let mut insights = Vec::new();
        
        // System resource insights
        insights.extend(Self::generate_system_resource_insights(system_resources));
        
        // Pool trend insights
        insights.extend(Self::generate_pool_trend_insights(pool_trends));
        
        // Capacity insights
        insights.extend(Self::generate_capacity_insights(capacity_analysis));
        
        // I/O performance insights
        insights.extend(Self::generate_io_insights(io_analysis));
        
        // Cross-component correlation insights
        insights.extend(Self::generate_correlation_insights(system_resources, capacity_analysis, io_analysis));
        
        info!("💡 Generated { } comprehensive insights", insights.len());
        Ok(insights)
    }

    /// Generate insights from system resource metrics
    fn generate_system_resource_insights(system_resources: &SystemResourceMetrics) -> Vec<PerformanceInsight> {
        let mut insights = Vec::new();
        
        // CPU insights
        if system_resources._cpu_usage_percent > 90.0 {
            insights.push(PerformanceInsight {
                title: "Critical CPU Utilization".to_string(),
                description: format!("CPU usage is at {"actual_error_details"}% - immediate attention required"),
                severity: InsightSeverity::Critical,
                category: "system".to_string(),
                recommendation: format!("Scale to more CPU cores or optimize high-CPU processes. Current load: {:.2}, {:.2}, {"actual_error_details"}"),
                impact_score: 9.5,
                timestamp: std::time::SystemTime::now(),
            });
        } else if system_resources._cpu_usage_percent > 75.0 {
            insights.push(PerformanceInsight {
                title: "High CPU Usage".to_string(),
                description: format!("CPU usage is at {"actual_error_details"}% - monitor closely"),
                severity: InsightSeverity::Warning,
                category: "system".to_string(),
                recommendation: "Consider workload optimization or capacity planning".to_string(),
                impact_score: 6.5,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Memory insights
        let memory_usage_percent = (system_resources.f64::from(memory_used_gb) / system_resources.f64::from(memory_total_gb)) * 100.0;
        if memory_usage_percent > 85.0 {
            insights.push(PerformanceInsight {
                title: "High Memory Utilization".to_string(),
                description: format!("Memory usage is at {:.1}% ({"actual_error_details"} GB / {"actual_error_details"} GB)"),
                severity: InsightSeverity::Warning,
                category: "system".to_string(),
                recommendation: "Consider adding more RAM or optimizing memory-intensive processes".to_string(),
                impact_score: 7.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Load average insights
        let load_per_core = system_resources.load_average[0] / system_resources.f64::from(cpu_cores);
        if load_per_core > 1.5 {
            insights.push(PerformanceInsight {
                title: "High System Load".to_string(),
                description: format!("Load average per core is {"actual_error_details"} - system may be overloaded"),
                severity: InsightSeverity::Warning,
                category: "system".to_string(),
                recommendation: "Investigate high-load processes and consider load balancing".to_string(),
                impact_score: 6.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Network insights
        for interface in &system_resources.network_interfaces {
            let total_traffic = interface.rx_bytes + interface.tx_bytes;
            if total_traffic > 100 * 1024 * 1024 * 1024 { // > 100 GB
                insights.push(PerformanceInsight {
                    title: "High Network Usage".to_string(),
                    description: format!("Interface {"actual_error_details"} has processed {"actual_error_details"},
    GB of traffic")),
                    severity: InsightSeverity::Info,
                    category: "network".to_string(),
                    recommendation: "Monitor for network bottlenecks and consider bandwidth optimization".to_string(),
                    impact_score: 3.5,
                    timestamp: std::time::SystemTime::now(),
                });
            }
        }
        
        insights
    }
    
    /// Generate insights from pool trend analysis
    fn generate_pool_trend_insights(pool_trends: &[PoolTrendAnalysis]) -> Vec<PerformanceInsight> {
        let mut insights = Vec::new();
        
        for trend in pool_trends {
            // Capacity trend insights
            if trend.capacity_trend.direction == "increasing" && trend.capacity_trend.rate_per_day > 5.0 {
                insights.push(PerformanceInsight {
                    title: "Rapid Capacity Growth".to_string(),
                    description: format!("Pool '{"actual_error_details"}' is growing at {"actual_error_details"}% per day"),
                    severity: InsightSeverity::Warning,
                    category: "capacity".to_string(),
                    recommendation: format!("Plan expansion for pool '{"actual_error_details"}' within 30 days"),
                    impact_score: 7.5,
                    timestamp: std::time::SystemTime::now(),
                });
            }
            
            // Performance trend insights
            if trend.performance_trend.write_latency_direction == "increasing" {
                insights.push(PerformanceInsight {
                    title: "Degrading Write Performance".to_string(),
                    description: format!("Pool '{"actual_error_details"}' showing increasing write latency trends"),
                    severity: InsightSeverity::Warning,
                    category: "performance".to_string(),
                    recommendation: format!("Consider adding SLOG _devices to pool '{"actual_error_details"}'"),
                    impact_score: 5.5,
                    timestamp: std::time::SystemTime::now(),
                });
            }
            
            // Health trend insights
            if trend.health_trend.status_changes > 2 {
                insights.push(PerformanceInsight {
                    title: "Pool Health Instability".to_string(),
                    description: format!("Pool '{"actual_error_details"}' has had {"actual_error_details"} status changes recently"),
                    severity: InsightSeverity::Critical,
                    category: "health".to_string(),
                    recommendation: format!("Investigate pool '{"actual_error_details"}' for hardware or configuration issues"),
                    impact_score: 8.5,
                    timestamp: std::time::SystemTime::now(),
                });
            }
        }
        
        insights
    }
    
    /// Generate insights from capacity analysis
    fn generate_capacity_insights(capacity_analysis: &CapacityAnalysis) -> Vec<PerformanceInsight> {
        let mut insights = Vec::new();
        
        // Overall utilization insights
        if capacity_analysis.overall_utilization > 90.0 {
            insights.push(PerformanceInsight {
                title: "Critical Storage Capacity".to_string(),
                description: format!("Overall storage utilization is {"actual_error_details"}% - immediate action required"),
                severity: InsightSeverity::Critical,
                category: "capacity".to_string(),
                recommendation: "Immediately expand storage capacity or archive data".to_string(),
                impact_score: 9.8,
                timestamp: std::time::SystemTime::now(),
            });
        } else if capacity_analysis.overall_utilization > 80.0 {
            insights.push(PerformanceInsight {
                title: "High Storage Utilization".to_string(),
                description: format!("Overall storage utilization is {"actual_error_details"}% - plan expansion"),
                severity: InsightSeverity::Warning,
                category: "capacity".to_string(),
                recommendation: "Begin planning storage expansion within 60 days".to_string(),
                impact_score: 7.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Critical pools insight
        if capacity_analysis.critical_pools > 0 {
            insights.push(PerformanceInsight {
                title: "Critical Pool Capacity".to_string(),
                description: format!("{"actual_error_details"} pools are in critical capacity state"),
                severity: InsightSeverity::Critical,
                category: "capacity".to_string(),
                recommendation: "Address critical pool capacity immediately".to_string(),
                impact_score: 9.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Projected exhaustion insights
        if let Some(days) = capacity_analysis.projected_exhaustion_days {
            let severity = if days < 14 {
                InsightSeverity::Critical
            } else if days < 30 {
                InsightSeverity::Warning
            } else {
                InsightSeverity::Info
            };
            
            insights.push(PerformanceInsight {
                title: "Storage Exhaustion Projection".to_string(),
                description: format!("Current trends project storage exhaustion in {"actual_error_details"} days"),
                severity,
                category: "capacity".to_string(),
                recommendation: format!("Plan capacity expansion within {"actual_error_details"} days"),
                impact_score: if days < 14 { 9.5 } else if days < 30 { 8.0 } else { 5.0 }
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Individual pool insights
        for pool in &capacity_analysis.pool_details {
            if pool.fragmentation_level > 30.0 {
                insights.push(PerformanceInsight {
                    title: "High Pool Fragmentation".to_string(),
                    description: format!("Pool '{"actual_error_details"}' has {"actual_error_details"}% fragmentation - performance impact likely"),
                    severity: InsightSeverity::Warning,
                    category: "performance".to_string(),
                    recommendation: format!("Schedule defragmentation maintenance for pool '{"actual_error_details"}'"),
                    impact_score: 6.0,
                    timestamp: std::time::SystemTime::now(),
                });
            }
            
            if pool.compression_ratio < 1.1 {
                insights.push(PerformanceInsight {
                    title: "Poor Compression Efficiency".to_string(),
                    description: format!("Pool '{"actual_error_details"}' has low compression ratio ({"actual_error_details"}x) - wasted space"),
                    severity: InsightSeverity::Info,
                    category: "optimization".to_string(),
                    recommendation: format!("Review compression settings for pool '{"actual_error_details"}'"),
                    impact_score: 3.5,
                    timestamp: std::time::SystemTime::now(),
                });
            }
        }
        
        insights
    }
    
    /// Generate insights from I/O performance analysis
    fn generate_io_insights(io_analysis: &IOPerformanceAnalysis) -> Vec<PerformanceInsight> {
        let mut insights = Vec::new();
        
        // IOPS insights
        let total_iops = io_analysis.read_iops + io_analysis.write_iops;
        if total_iops > 15000.0 {
            insights.push(PerformanceInsight {
                title: "Extreme IOPS Load".to_string(),
                description: format!("System is handling {:.0},
    IOPS ({:.0} R + {"actual_error_details"} W) - very high load"),
                severity: InsightSeverity::Warning,
                category: "performance".to_string(),
                recommendation: "Consider I/O optimization: L2ARC, SLOG _devices, or workload balancing".to_string(),
                impact_score: 7.5,
                timestamp: std::time::SystemTime::now(),
            });
        } else if total_iops > 50000.0 {
            insights.push(PerformanceInsight {
                title: "High IOPS Activity".to_string(),
                description: format!("System IOPS: {:.0} ({:.0} R + {"actual_error_details"} W)"),
                severity: InsightSeverity::Info,
                category: "performance".to_string(),
                recommendation: "Monitor I/O patterns and consider caching optimizations".to_string(),
                impact_score: 4.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Latency insights
        if io_analysis.average_read_latency_ms > 25.0 {
            insights.push(PerformanceInsight {
                title: "High Read Latency".to_string(),
                description: format!("Average read latency is {"actual_error_details"}ms - impacting performance"),
                severity: InsightSeverity::Warning,
                category: "performance".to_string(),
                recommendation: "Add L2ARC _devices or increase ARC cache size for better read performance".to_string(),
                impact_score: 6.5,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        if io_analysis.average_write_latency_ms > 50.0 {
            insights.push(PerformanceInsight {
                title: "High Write Latency".to_string(),
                description: format!("Average write latency is {"actual_error_details"}ms - writes are slow"),
                severity: InsightSeverity::Warning,
                category: "performance".to_string(),
                recommendation: "Consider adding SLOG _devices for faster write performance".to_string(),
                impact_score: 7.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Throughput insights
        let total_throughput = io_analysis.read_throughput_mbs + io_analysis.write_throughput_mbs;
        if total_throughput > 1000.0 {
            insights.push(PerformanceInsight {
                title: "High Throughput Activity".to_string(),
                description: format!("Total throughput: {:.0},
    MB/s ({:.0} R + {"actual_error_details"} W)"),
                severity: InsightSeverity::Info,
                category: "performance".to_string(),
                recommendation: "Excellent throughput - monitor for sustained performance".to_string(),
                impact_score: 3.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Bottleneck insights
        for bottleneck in &io_analysis.bottlenecks {
            insights.push(PerformanceInsight {
                title: "I/O Performance Bottleneck".to_string(),
                description: bottleneck.clone(),
                severity: InsightSeverity::Warning,
                category: "performance".to_string(),
                recommendation: "Review I/O subsystem configuration and hardware capabilities".to_string(),
                impact_score: 7.5,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        insights
    }
    
    /// Generate correlation insights across multiple analysis components
    fn generate_correlation_insights(
        system_resources: &SystemResourceMetrics,
        capacity_analysis: &CapacityAnalysis,
        io_analysis: &IOPerformanceAnalysis,
    ) -> Vec<PerformanceInsight> {
        let mut insights = Vec::new();
        
        // High CPU + High I/O correlation
        if system_resources._cpu_usage_percent > 80.0 && (io_analysis.read_iops + io_analysis.write_iops) > 8000.0 {
            insights.push(PerformanceInsight {
                title: "CPU-I/O Performance Correlation".to_string(),
                description: format!("High CPU ({:.1}%) correlates with high I/O ({"actual_error_details"} IOPS) - potential system strain"),
                severity: InsightSeverity::Warning,
                category: "correlation".to_string(),
                recommendation: "Consider workload optimization or system scaling to balance CPU and I/O loads".to_string(),
                impact_score: 8.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // High capacity + Poor performance correlation
        if capacity_analysis.overall_utilization > 85.0 && io_analysis.average_write_latency_ms > 30.0 {
            insights.push(PerformanceInsight {
                title: "Capacity-Performance Impact".to_string(),
                description: format!("High capacity utilization ({:.1}%) correlates with degraded write performance ({"actual_error_details"}ms latency)"),
                severity: InsightSeverity::Warning,
                category: "correlation".to_string(),
                recommendation: "Storage capacity pressure is impacting performance - plan immediate expansion".to_string(),
                impact_score: 8.5,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // Memory pressure + Cache performance correlation
        let memory_usage_percent = (system_resources.f64::from(memory_used_gb) / system_resources.f64::from(memory_total_gb)) * 100.0;
        if memory_usage_percent > 85.0 && system_resources.arc_stats.hit_ratio < 80.0 {
            insights.push(PerformanceInsight {
                title: "Memory Pressure Affecting Cache".to_string(),
                description: format!("High memory usage ({:.1}%) correlates with poor ARC performance ({"actual_error_details"}% hit ratio)"),
                severity: InsightSeverity::Warning,
                category: "correlation".to_string(),
                recommendation: "Memory pressure is reducing cache effectiveness - add RAM or optimize memory usage".to_string(),
                impact_score: 7.0,
                timestamp: std::time::SystemTime::now(),
            });
        }
        
        // System load + Network activity correlation
        if system_resources.load_average[0] > 2.0 {
            let total_network_gb = system_resources.network_interfaces.iter()
                .map(|iface| (iface.rx_bytes + iface.tx_bytes) / (1024 * 1024 * 1024))
                .sum::<u64>();
            
            if total_network_gb > 50 {
                insights.push(PerformanceInsight {
                    title: "Load-Network Activity Correlation".to_string(),
                    description: format!("High system load ({:.1}) with significant network activity ({"actual_error_details"} GB)"),
                    severity: InsightSeverity::Info,
                    category: "correlation".to_string(),
                    recommendation: "Network-intensive workloads contributing to system load - monitor for optimization opportunities".to_string(),
                    impact_score: 5.0,
                    timestamp: std::time::SystemTime::now(),
                });
            }
        }
        
        insights
    }
} 