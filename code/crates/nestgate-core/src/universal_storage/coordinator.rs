//! Storage Coordinator
//!
//! Routes storage requests to appropriate backends and handles distributed operations.

use super::types::*;
use crate::Result;
use std::sync::Arc;

/// Storage Coordinator - Routes requests to appropriate handlers
pub struct StorageCoordinator {
    /// Registry of storage backends
    _backend_registry: Arc<BackendRegistry>,
    /// Load balancer for backend selection
    _load_balancer: Arc<StorageLoadBalancer>,
    /// Consistency manager for distributed operations
    _consistency_manager: Arc<ConsistencyManager>,
    /// Transaction manager for atomic operations
    _transaction_manager: Arc<TransactionManager>,
}

impl StorageCoordinator {
    /// Create a new storage coordinator
    pub async fn new() -> Result<Self> {
        Ok(Self {
            _backend_registry: Arc::new(BackendRegistry::new()?),
            _load_balancer: Arc::new(StorageLoadBalancer::new()),
            _consistency_manager: Arc::new(ConsistencyManager::new()?),
            _transaction_manager: Arc::new(TransactionManager::new()?),
        })
    }

    /// Register a storage backend
    pub async fn register_backend(&self, backend: StorageBackend) -> Result<()> {
        self._backend_registry.register(backend)
    }

    /// Route a storage request to appropriate backend
    pub async fn route_request(&self, request: StorageRequest) -> Result<StorageResponse> {
        // Select appropriate backend
        let backend = self._load_balancer.select_backend(&request).await?;

        // Execute request
        backend.execute_request(request)
    }

    /// Coordinate multi-backend operations
    pub async fn coordinate_multi_backend(
        &self,
        operation: MultiBackendOperation,
    ) -> Result<OperationResult> {
        self._transaction_manager.execute_multi_backend(operation)
    }

    /// Ensure consistency across distributed operations
    pub async fn ensure_consistency(&self, data_id: &str) -> Result<ConsistencyStatus> {
        self._consistency_manager.check_consistency(data_id)
    }

    /// Manage distributed transactions
    pub async fn manage_transaction(
        &self,
        transaction: StorageTransaction,
    ) -> Result<TransactionResult> {
        self._transaction_manager.execute_transaction(transaction)
    }
}
