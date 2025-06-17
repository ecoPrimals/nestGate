/*!
 * Network management and proxy configuration for the Port Manager
 */

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::errors::{Error, Result};
use crate::service::{ServiceInstance, ServiceType};

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Enable reverse proxy
    pub reverse_proxy_enabled: bool,
    
    /// Reverse proxy listen port
    pub reverse_proxy_port: u16,
    
    /// Enable SSL/TLS termination
    pub ssl_termination: bool,
    
    /// SSL certificate path
    pub ssl_cert_path: Option<String>,
    
    /// SSL private key path
    pub ssl_key_path: Option<String>,
    
    /// Enable HTTP to HTTPS redirect
    pub https_redirect: bool,
    
    /// Enable CORS
    pub cors_enabled: bool,
    
    /// Allowed origins for CORS
    pub cors_origins: Vec<String>,
    
    /// Enable rate limiting
    pub rate_limiting: bool,
    
    /// Rate limit per IP (requests per minute)
    pub rate_limit_rpm: u32,
    
    /// Enable load balancing
    pub load_balancing: bool,
    
    /// Load balancing strategy
    pub load_balancer_strategy: LoadBalancerStrategy,
    
    /// Network interfaces to bind to
    pub bind_interfaces: Vec<IpAddr>,
    
    /// Enable WebSocket proxying
    pub websocket_proxy: bool,
    
    /// Timeout settings
    pub timeouts: TimeoutConfig,
    
    /// Proxy type
    pub proxy_type: String,
    
    /// Auto-configure
    pub auto_configure: bool,
    
    /// Domain configuration
    pub domain_config: DomainConfig,
}

/// Load balancer strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancerStrategy {
    /// Round robin
    RoundRobin,
    
    /// Least connections
    LeastConnections,
    
    /// Weighted round robin
    WeightedRoundRobin,
    
    /// Health-based routing
    HealthBased,
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    /// Connection timeout in seconds
    pub connect_timeout: u64,
    
    /// Request timeout in seconds
    pub request_timeout: u64,
    
    /// Keep-alive timeout in seconds
    pub keepalive_timeout: u64,
    
    /// WebSocket timeout in seconds
    pub websocket_timeout: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            reverse_proxy_enabled: false,
            reverse_proxy_port: 8080,
            ssl_termination: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            https_redirect: false,
            cors_enabled: true,
            cors_origins: vec!["*".to_string()],
            rate_limiting: true,
            rate_limit_rpm: 1000,
            load_balancing: false,
            load_balancer_strategy: LoadBalancerStrategy::RoundRobin,
            bind_interfaces: vec!["0.0.0.0".parse().unwrap()],
            websocket_proxy: true,
            timeouts: TimeoutConfig {
                connect_timeout: 30,
                request_timeout: 300,
                keepalive_timeout: 60,
                websocket_timeout: 3600,
            },
            proxy_type: "nginx".to_string(),
            auto_configure: true,
            domain_config: DomainConfig {
                domain: String::new(),
                auto_ssl: false,
                ca_provider: String::new(),
                hsts_enabled: false,
                hsts_max_age: 31536000,
                subdomains: HashMap::new(),
            },
        }
    }
}

/// Proxy route configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRoute {
    /// Route path pattern
    pub path: String,
    
    /// Service ID to route to
    pub service_id: String,
    
    /// Target host
    pub target_host: String,
    
    /// Target port
    pub target_port: u16,
    
    /// Route priority (higher = more priority)
    pub priority: u32,
    
    /// Enable SSL for backend
    pub backend_ssl: bool,
    
    /// Health check configuration
    pub health_check: Option<ProxyHealthCheck>,
    
    /// Load balancer weight
    pub weight: u32,
    
    /// Custom headers to add
    pub headers: HashMap<String, String>,
    
    /// Enable caching
    pub cache_enabled: bool,
    
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    
    /// Enable route
    pub enabled: bool,
}

/// Proxy health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyHealthCheck {
    /// Health check path
    pub path: String,
    
    /// Expected status code
    pub expected_status: u16,
    
    /// Check interval in seconds
    pub interval: u64,
    
    /// Timeout in seconds
    pub timeout: u64,
    
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
    
    /// Healthy threshold
    pub healthy_threshold: u32,
}

/// Domain configuration for remote access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainConfig {
    /// Domain name
    pub domain: String,
    
    /// Enable automatic SSL certificate generation
    pub auto_ssl: bool,
    
    /// Certificate authority (e.g., "letsencrypt")
    pub ca_provider: String,
    
    /// Enable HSTS
    pub hsts_enabled: bool,
    
    /// HSTS max age in seconds
    pub hsts_max_age: u64,
    
    /// Subdomain mappings
    pub subdomains: HashMap<String, String>, // subdomain -> service_id
}

/// SSL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    /// Enable SSL
    pub enabled: bool,
    
    /// SSL certificate path
    pub cert_path: String,
    
    /// SSL private key path
    pub key_path: String,
}

/// Network manager
#[derive(Clone)]
pub struct NetworkManager {
    /// Configuration
    config: NetworkConfig,
    
    /// Proxy routes
    routes: Arc<RwLock<HashMap<String, ProxyRoute>>>,
    
    /// Domain configurations
    domains: Arc<RwLock<HashMap<String, DomainConfig>>>,
    
    /// Service instances for routing
    services: Arc<RwLock<HashMap<String, ServiceInstance>>>,
    
    /// Connection statistics
    connection_stats: Arc<RwLock<ConnectionStats>>,
}

/// Connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    /// Total connections
    pub total_connections: u64,
    
    /// Active connections
    pub active_connections: u64,
    
    /// Failed connections
    pub failed_connections: u64,
    
    /// Bytes transferred
    pub bytes_transferred: u64,
    
    /// Average response time in milliseconds
    pub avg_response_time: f64,
}

impl Default for ConnectionStats {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            failed_connections: 0,
            bytes_transferred: 0,
            avg_response_time: 0.0,
        }
    }
}

impl NetworkManager {
    /// Create a new network manager
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            routes: Arc::new(RwLock::new(HashMap::new())),
            domains: Arc::new(RwLock::new(HashMap::new())),
            services: Arc::new(RwLock::new(HashMap::new())),
            connection_stats: Arc::new(RwLock::new(ConnectionStats::default())),
        }
    }
    
    /// Initialize the network manager
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing network manager");
        tracing::info!("Proxy type: {}", self.config.proxy_type);
        tracing::info!("Auto configure: {}", self.config.auto_configure);
        tracing::info!("Domain config enabled: {}", self.config.domain_config.auto_ssl);
        
        if self.config.reverse_proxy_enabled {
            tracing::info!("Reverse proxy enabled on port {}", self.config.reverse_proxy_port);
            self.start_reverse_proxy().await?;
        }
        
        if self.config.ssl_termination {
            tracing::info!("SSL termination enabled");
            self.configure_ssl().await?;
        }
        
        Ok(())
    }
    
    /// Start the reverse proxy server
    async fn start_reverse_proxy(&self) -> Result<()> {
        // In a real implementation, this would start the actual reverse proxy server
        // For now, we'll just log that it's starting
        tracing::info!("Starting reverse proxy server on port {}", self.config.reverse_proxy_port);
        
        // TODO: Implement actual reverse proxy using hyper or similar
        
        Ok(())
    }
    
    /// Configure SSL/TLS
    async fn configure_ssl(&self) -> Result<()> {
        if let (Some(cert_path), Some(key_path)) = (&self.config.ssl_cert_path, &self.config.ssl_key_path) {
            tracing::info!("Configuring SSL with cert: {}, key: {}", cert_path, key_path);
            
            // TODO: Implement SSL configuration
            
        } else {
            return Err(Error::Config("SSL enabled but certificate paths not configured".to_string()));
        }
        
        Ok(())
    }
    
    /// Add a proxy route
    pub async fn add_route(&self, route_id: String, route: ProxyRoute) -> Result<()> {
        let mut routes = self.routes.write().await;
        routes.insert(route_id.clone(), route);
        
        tracing::info!("Added proxy route: {}", route_id);
        Ok(())
    }
    
    /// Remove a proxy route
    pub async fn remove_route(&self, route_id: &str) -> Result<()> {
        let mut routes = self.routes.write().await;
        if routes.remove(route_id).is_some() {
            tracing::info!("Removed proxy route: {}", route_id);
            Ok(())
        } else {
            Err(Error::Api(format!("Route not found: {}", route_id)))
        }
    }
    
    /// Update service registry for routing
    pub async fn update_services(&self, services: Vec<ServiceInstance>) -> Result<()> {
        let mut service_map = self.services.write().await;
        service_map.clear();
        
        for service in services {
            service_map.insert(service.definition.id.clone(), service);
        }
        
        // Auto-configure routes for services if enabled
        self.auto_configure_routes().await?;
        
        Ok(())
    }
    
    /// Auto-configure routes based on services
    async fn auto_configure_routes(&self) -> Result<()> {
        let services = self.services.read().await;
        let mut routes = self.routes.write().await;
        
        for (service_id, service) in services.iter() {
            // Skip if route already exists
            if routes.contains_key(service_id) {
                continue;
            }
            
            // Only create routes for web services with allocated ports
            if let Some(port) = service.port {
                let route_path = match service.definition.service_type {
                    ServiceType::UI => "/",
                    ServiceType::API => "/api",
                    ServiceType::WebSocket => "/ws",
                    _ => continue,
                };
                
                let route = ProxyRoute {
                    path: route_path.to_string(),
                    service_id: service_id.clone(),
                    target_host: "localhost".to_string(),
                    target_port: port,
                    priority: 100,
                    backend_ssl: false,
                    health_check: Some(ProxyHealthCheck {
                        path: "/health".to_string(),
                        expected_status: 200,
                        interval: 30,
                        timeout: 5,
                        unhealthy_threshold: 3,
                        healthy_threshold: 2,
                    }),
                    weight: 100,
                    headers: HashMap::new(),
                    cache_enabled: service.definition.service_type == ServiceType::UI,
                    cache_ttl: 300, // 5 minutes
                    enabled: true,
                };
                
                routes.insert(service_id.clone(), route);
                tracing::info!("Auto-configured route for service: {}", service_id);
            }
        }
        
        Ok(())
    }
    
    /// Add domain configuration
    pub async fn add_domain(&self, domain: String, config: DomainConfig) -> Result<()> {
        let mut domains = self.domains.write().await;
        domains.insert(domain.clone(), config);
        
        tracing::info!("Added domain configuration: {}", domain);
        Ok(())
    }
    
    /// Remove domain configuration
    pub async fn remove_domain(&self, domain: &str) -> Result<()> {
        let mut domains = self.domains.write().await;
        if domains.remove(domain).is_some() {
            tracing::info!("Removed domain configuration: {}", domain);
            Ok(())
        } else {
            Err(Error::Api(format!("Domain not found: {}", domain)))
        }
    }
    
    /// Get all routes
    pub async fn get_routes(&self) -> HashMap<String, ProxyRoute> {
        let routes = self.routes.read().await;
        routes.clone()
    }
    
    /// Get all domains
    pub async fn get_domains(&self) -> HashMap<String, DomainConfig> {
        let domains = self.domains.read().await;
        domains.clone()
    }
    
    /// Get connection statistics
    pub async fn get_connection_stats(&self) -> ConnectionStats {
        let stats = self.connection_stats.read().await;
        stats.clone()
    }
    
    /// Generate proxy configuration (for nginx, HAProxy, etc.)
    pub async fn generate_proxy_config(&self, proxy_type: ProxyType) -> Result<String> {
        let routes = self.routes.read().await;
        let domains = self.domains.read().await;
        
        match proxy_type {
            ProxyType::Nginx => self.generate_nginx_config(&routes, &domains).await,
            ProxyType::HAProxy => self.generate_haproxy_config(&routes, &domains).await,
            ProxyType::Traefik => self.generate_traefik_config(&routes, &domains).await,
        }
    }
    
    /// Generate nginx configuration
    async fn generate_nginx_config(
        &self,
        routes: &HashMap<String, ProxyRoute>,
        _domains: &HashMap<String, DomainConfig>,
    ) -> Result<String> {
        let mut config = String::new();
        
        // Generate upstream blocks
        config.push_str("# Generated by NestGate Port Manager\n\n");
        
        for (route_id, route) in routes {
            config.push_str(&format!(
                "upstream {} {{\n",
                route_id.replace('-', "_")
            ));
            config.push_str(&format!(
                "    server {}:{};\n",
                route.target_host, route.target_port
            ));
            config.push_str("}\n\n");
        }
        
        // Generate server blocks
        config.push_str("server {\n");
        config.push_str(&format!("    listen {};\n", self.config.reverse_proxy_port));
        
        if self.config.ssl_termination {
            config.push_str(&format!("    listen {} ssl;\n", self.config.reverse_proxy_port + 363)); // 443 equivalent
        }
        
        config.push_str("    server_name _;\n\n");
        
        // Add locations for each route
        for (route_id, route) in routes {
            config.push_str(&format!("    location {} {{\n", route.path));
            config.push_str(&format!(
                "        proxy_pass http://{};\n",
                route_id.replace('-', "_")
            ));
            config.push_str("        proxy_set_header Host $host;\n");
            config.push_str("        proxy_set_header X-Real-IP $remote_addr;\n");
            config.push_str("        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;\n");
            config.push_str("        proxy_set_header X-Forwarded-Proto $scheme;\n");
            
            if route.service_id.contains("websocket") || route.path.contains("ws") {
                config.push_str("        proxy_http_version 1.1;\n");
                config.push_str("        proxy_set_header Upgrade $http_upgrade;\n");
                config.push_str("        proxy_set_header Connection \"upgrade\";\n");
            }
            
            config.push_str("    }\n\n");
        }
        
        config.push_str("}\n");
        
        Ok(config)
    }
    
    /// Generate HAProxy configuration
    async fn generate_haproxy_config(
        &self,
        routes: &HashMap<String, ProxyRoute>,
        _domains: &HashMap<String, DomainConfig>,
    ) -> Result<String> {
        let mut config = String::new();
        
        config.push_str("# Generated by NestGate Port Manager\n");
        config.push_str("global\n");
        config.push_str("    daemon\n");
        config.push_str("    maxconn 4096\n\n");
        
        config.push_str("defaults\n");
        config.push_str("    mode http\n");
        config.push_str("    timeout connect 5000ms\n");
        config.push_str("    timeout client 50000ms\n");
        config.push_str("    timeout server 50000ms\n\n");
        
        config.push_str("frontend web_frontend\n");
        config.push_str(&format!("    bind *:{}\n", self.config.reverse_proxy_port));
        
        for (route_id, route) in routes {
            config.push_str(&format!(
                "    acl is_{} path_beg {}\n",
                route_id.replace('-', "_"),
                route.path
            ));
            config.push_str(&format!(
                "    use_backend {} if is_{}\n",
                route_id.replace('-', "_"),
                route_id.replace('-', "_")
            ));
        }
        
        config.push_str("\n");
        
        for (route_id, route) in routes {
            config.push_str(&format!("backend {}\n", route_id.replace('-', "_")));
            config.push_str("    balance roundrobin\n");
            config.push_str(&format!(
                "    server {} {}:{} check\n",
                route_id.replace('-', "_"),
                route.target_host,
                route.target_port
            ));
            config.push_str("\n");
        }
        
        Ok(config)
    }
    
    /// Generate Traefik configuration
    async fn generate_traefik_config(
        &self,
        routes: &HashMap<String, ProxyRoute>,
        _domains: &HashMap<String, DomainConfig>,
    ) -> Result<String> {
        // Generate Traefik dynamic configuration in YAML format
        let mut config = String::new();
        
        config.push_str("# Generated by NestGate Port Manager\n");
        config.push_str("http:\n");
        config.push_str("  routers:\n");
        
        for (route_id, route) in routes {
            config.push_str(&format!("    {}:\n", route_id.replace('-', "_")));
            config.push_str(&format!("      rule: \"PathPrefix(`{}`)\"\n", route.path));
            config.push_str(&format!("      service: {}\n", route_id.replace('-', "_")));
            
            if self.config.ssl_termination {
                config.push_str("      tls: {}\n");
            }
            
            config.push_str("\n");
        }
        
        config.push_str("  services:\n");
        
        for (route_id, route) in routes {
            config.push_str(&format!("    {}:\n", route_id.replace('-', "_")));
            config.push_str("      loadBalancer:\n");
            config.push_str("        servers:\n");
            config.push_str(&format!(
                "          - url: \"http://{}:{}\"\n",
                route.target_host, route.target_port
            ));
            config.push_str("\n");
        }
        
        Ok(config)
    }
    
    /// Enable remote access for a domain
    pub async fn enable_remote_access(&self, domain: String, services: Vec<String>) -> Result<()> {
        let domain_config = DomainConfig {
            domain: domain.clone(),
            auto_ssl: true,
            ca_provider: "letsencrypt".to_string(),
            hsts_enabled: true,
            hsts_max_age: 31536000, // 1 year
            subdomains: services.into_iter()
                .enumerate()
                .map(|(i, service)| (format!("service{}", i), service))
                .collect(),
        };
        
        self.add_domain(domain, domain_config).await?;
        
        Ok(())
    }
    
    /// Configure LAN access
    pub async fn configure_lan_access(&self, network_interface: IpAddr) -> Result<()> {
        // Add the network interface to the bind list
        // In a real implementation, this would reconfigure the reverse proxy
        tracing::info!("Configuring LAN access on interface: {}", network_interface);
        
        Ok(())
    }
}

/// Proxy type for configuration generation
#[derive(Debug, Clone)]
pub enum ProxyType {
    /// Nginx
    Nginx,
    
    /// HAProxy
    HAProxy,
    
    /// Traefik
    Traefik,
} 