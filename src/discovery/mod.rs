impl StaticServiceDiscovery {
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }
} 