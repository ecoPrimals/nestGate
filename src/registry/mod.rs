/// Start the service
/// 
/// # Errors
/// 
/// Returns an error if the service fails to start
pub async fn start(&self) -> Result<()> {
    self.service.write().await.start().await
        .map_err(|e| SongbirdError::service_error(&self.info.id, e.to_string()))?;
    Ok(())
}

/// Stop the service
/// 
/// # Errors
/// 
/// Returns an error if the service fails to stop
pub async fn stop(&self) -> Result<()> {
    self.service.write().await.stop().await
        .map_err(|e| SongbirdError::service_error(&self.info.id, e.to_string()))?;
    Ok(())
}

/// Perform health check on the service
/// 
/// # Errors
/// 
/// Returns an error if the health check fails or serialization fails
pub async fn health_check(&self) -> Result<serde_json::Value> {
    let health = self.service.read().await.health_check().await
        .map_err(|e| SongbirdError::health_check_failed(&self.info.id, e.to_string()))?;
    
    serde_json::to_value(health)
        .map_err(SongbirdError::Serialization)
}

/// Create a new service registry
/// 
/// # Errors
/// 
/// Returns an error if the registry fails to initialize
pub fn new() -> Result<Self> {
    Ok(Self {
        services: Arc::new(RwLock::new(HashMap::new())),
    })
}

/// Register a service with the registry
/// 
/// # Errors
/// 
/// Returns an error if the service registration fails
pub async fn register(&self, info: ServiceInfo) -> Result<()> {
    let mut services = self.services.write().await;
    if services.contains_key(&info.id) {
        return Err(SongbirdError::Service(format!("Service {} already registered", info.id)));
    }
    services.insert(info.id.clone(), info);
    Ok(())
}

/// Unregister a service from the registry
/// 
/// # Errors
/// 
/// Returns an error if the service is not found
pub async fn unregister(&self, service_id: &str) -> Result<()> {
    let mut services = self.services.write().await;
    if services.remove(service_id).is_none() {
        return Err(SongbirdError::Service(format!("Service {} not found", service_id)));
    }
    Ok(())
}

/// List all registered services
/// 
/// # Errors
/// 
/// Returns an error if the registry cannot be accessed
pub async fn list_services(&self) -> Result<Vec<ServiceInfo>> {
    Ok(self.services.read().await.values().cloned().collect())
}

/// Get information about a specific service
/// 
/// # Errors
/// 
/// Returns an error if the registry cannot be accessed
pub async fn get_service(&self, service_id: &str) -> Result<Option<ServiceInfo>> {
    Ok(self.services.read().await.get(service_id).cloned())
} 