// Advanced EcoPrimal functionality
//
// Advanced features for ecosystem integration through the universal adapter.

use super::errors::*;
use super::implementation::*;
use super::traits::*;
use nestgate_core::universal_adapter::UniversalAdapter;
use tracing::{info, warn};

impl AdvancedEcoPrimal for NestGateEcoPrimal {
    fn advanced_operation(&self) -> impl Future<Output = Result<(), PrimalError>> + Send {
        async move {
            info!("Performing advanced NestGate ecosystem operation");
            
            // Create universal adapter for ecosystem integration
            let mut adapter = UniversalAdapter::new(
                std::env::var("NESTGATE_ECOSYSTEM_ENDPOINT")
                    .unwrap_or_else(|_| {
                        use nestgate_core::constants::hardcoding::{addresses, ports};
                        format!("http://{}:{}", addresses::LOCALHOST_NAME, ports::HTTP_DEFAULT)
                    })
            );
            
            // Discover available capabilities
            match adapter.discover_capabilities().await {
                Ok(capabilities) => {
                    info!("Discovered {} ecosystem capabilities", capabilities.len());
                    
                    // Register our storage capabilities with the ecosystem
                    for capability in capabilities {
                        info!("Available capability: {} from {}", capability.category, capability.provider);
                    }
                    
                    Ok(())
                }
                Err(e) => {
                    warn!("Failed to perform advanced ecosystem operation: {}", e);
                    Err(PrimalError::EcosystemIntegration(
                        format!("Advanced operation failed: {e}")
                    ))
                }
            }
        }
    }
}
