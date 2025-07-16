//! Universal Model API Demo
//!
//! This example demonstrates the universal, agnostic model API system that can
//! integrate with HuggingFace, OpenAI, Claude, local models, and any future providers.

use anyhow::Result;
use nestgate_core::universal_model_api::{
    InferenceInput, InferenceRequest, ModelCapability, ModelConfiguration, ModelHandle, ModelInfo,
    ModelLoadRequest, ModelParameters, ModelPrecision, OptimizationLevel, OptimizationSettings,
    ProviderConfig, RegistryConfig, ResourceRequirements, UniversalModelProviderFactory,
    UniversalModelRegistry,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Universal Model API Demo Starting");
    info!("📖 This demo shows how to integrate with ANY model provider universally");

    // Initialize the universal model registry
    let registry = Arc::new(UniversalModelRegistry::new(RegistryConfig {
        max_providers: 50,
        enable_caching: true,
        cache_size_mb: 2048,
        enable_health_monitoring: true,
        min_healthy_providers: 0,
        version: "1.0.0".to_string(),
    }));

    // Demo 1: Register multiple providers
    register_multiple_providers(&registry).await?;

    // Demo 2: List and discover models
    list_and_discover_models(&registry).await?;

    // Demo 3: Find models by capability
    find_models_by_capability(&registry).await?;

    // Demo 4: Load and configure models
    load_and_configure_models(&registry).await?;

    // Demo 5: Universal inference
    universal_inference_demo(&registry).await?;

    // Demo 6: Registry health and metrics
    registry_health_demo(&registry).await?;

    info!("✅ Universal Model API Demo Completed Successfully");
    Ok(())
}

async fn register_multiple_providers(registry: &Arc<UniversalModelRegistry>) -> Result<()> {
    info!("🔌 Demo 1: Registering Multiple Model Providers");

    // HuggingFace provider configuration
    let huggingface_config = ProviderConfig {
        provider_type: "huggingface".to_string(),
        base_url: Some("https://api-inference.huggingface.co".to_string()),
        api_key: std::env::var("HUGGINGFACE_API_KEY").ok(),
        timeout_seconds: 30,
        max_retries: 3,
        custom_headers: None,
        custom_config: HashMap::new(),
        max_tokens: Some(2048),
        enable_streaming: true,
        model_filter: vec![],
        supports_embeddings: true,
        model_path: Some("/tmp/huggingface_cache".to_string()),
    };

    // OpenAI provider configuration
    let openai_config = ProviderConfig {
        provider_type: "openai".to_string(),
        base_url: Some("https://api.openai.com/v1".to_string()),
        api_key: std::env::var("OPENAI_API_KEY").ok(),
        timeout_seconds: 60,
        max_retries: 3,
        custom_headers: None,
        custom_config: {
            let mut config = HashMap::new();
            if let Ok(org_id) = std::env::var("OPENAI_ORG_ID") {
                config.insert("organization".to_string(), json!(org_id));
            }
            config
        },
        max_tokens: Some(4096),
        enable_streaming: true,
        model_filter: vec![],
        supports_embeddings: true,
        model_path: None,
    };

    // Claude provider configuration
    let claude_config = ProviderConfig {
        provider_type: "claude".to_string(),
        base_url: Some("https://api.anthropic.com/v1".to_string()),
        api_key: std::env::var("CLAUDE_API_KEY").ok(),
        timeout_seconds: 60,
        max_retries: 3,
        custom_headers: None,
        custom_config: HashMap::new(),
        max_tokens: Some(4096),
        enable_streaming: true,
        model_filter: vec![],
        supports_embeddings: false,
        model_path: None,
    };

    // Local model provider configuration
    let local_config = ProviderConfig {
        provider_type: "local".to_string(),
        base_url: Some("http://localhost:8080".to_string()),
        api_key: None,
        timeout_seconds: 120,
        max_retries: 1,
        custom_headers: None,
        custom_config: {
            let mut config = HashMap::new();
            config.insert("gpu_layers".to_string(), json!(32));
            config.insert("context_length".to_string(), json!(4096));
            config
        },
        max_tokens: Some(4096),
        enable_streaming: true,
        model_filter: vec![],
        supports_embeddings: true,
        model_path: Some("/models".to_string()),
    };

    // Ollama provider configuration
    let ollama_config = ProviderConfig {
        provider_type: "ollama".to_string(),
        base_url: Some("http://localhost:11434".to_string()),
        api_key: None,
        timeout_seconds: 60,
        max_retries: 2,
        custom_headers: None,
        custom_config: HashMap::new(),
        max_tokens: Some(4096),
        enable_streaming: true,
        model_filter: vec![],
        supports_embeddings: true,
        model_path: None,
    };

    // Register all providers
    let configs = vec![
        ("HuggingFace", huggingface_config),
        ("OpenAI", openai_config),
        ("Claude", claude_config),
        ("Local", local_config),
        ("Ollama", ollama_config),
    ];

    for (name, config) in configs {
        info!("  📝 Registering {} provider...", name);
        match UniversalModelProviderFactory::create_provider(config) {
            Ok(provider) => match registry.register_provider(provider).await {
                Ok(_) => info!("  ✅ {} provider registered successfully", name),
                Err(e) => warn!("  ⚠️  Failed to register {} provider: {}", name, e),
            },
            Err(e) => warn!("  ⚠️  Failed to create {} provider: {}", name, e),
        }
    }

    let providers = registry.list_providers().await;
    info!("  📊 Total providers registered: {}", providers.len());
    for provider in providers {
        info!("    - {}", provider);
    }

    Ok(())
}

async fn list_and_discover_models(registry: &Arc<UniversalModelRegistry>) -> Result<()> {
    info!("🔍 Demo 2: Listing and Discovering Models");

    match registry.list_all_models().await {
        Ok(models) => {
            info!("  📋 Found {} models across all providers", models.len());

            // Group models by provider
            let mut provider_models: HashMap<String, Vec<&ModelInfo>> = HashMap::new();
            for model in &models {
                provider_models
                    .entry(model.provider.clone())
                    .or_default()
                    .push(model);
            }

            for (provider, provider_models) in provider_models {
                info!("  🏢 Provider: {}", provider);
                for model in provider_models {
                    info!("    📦 Model: {} ({})", model.name, model.id);
                    info!("      Type: {:?}", model.model_type);
                    info!("      Capabilities: {} total", model.capabilities.len());
                    if let Some(pricing) = &model.pricing {
                        info!(
                            "      Pricing: {} {:?}",
                            pricing.currency, pricing.billing_model
                        );
                    }
                }
            }
        }
        Err(e) => warn!("  ❌ Failed to list models: {}", e),
    }

    Ok(())
}

async fn find_models_by_capability(registry: &Arc<UniversalModelRegistry>) -> Result<()> {
    info!("🎯 Demo 3: Finding Models by Capability");

    // Example capabilities to search for
    let capabilities = vec![
        ModelCapability::TextGeneration {
            max_tokens: Some(4096),
            supports_streaming: true,
            supports_system_prompts: true,
            supports_function_calling: false,
        },
        ModelCapability::CodeGeneration {
            supported_languages: vec!["rust".to_string(), "python".to_string()],
            supports_completion: true,
            supports_explanation: true,
            supports_debugging: false,
        },
        ModelCapability::ImageUnderstanding {
            supported_formats: vec!["jpeg".to_string(), "png".to_string()],
            max_resolution: Some((1024, 1024)),
            supports_ocr: true,
            supports_description: true,
        },
    ];

    for capability in capabilities {
        info!("  🔍 Searching for capability: {:?}", capability);
        match registry.find_models_by_capability(&capability).await {
            Ok(models) => {
                info!("    📊 Found {} models with this capability", models.len());
                for model in models {
                    info!("      - {} ({})", model.name, model.provider);
                }
            }
            Err(e) => warn!("    ❌ Failed to search for capability: {}", e),
        }
    }

    Ok(())
}

async fn load_and_configure_models(registry: &Arc<UniversalModelRegistry>) -> Result<()> {
    info!("🚀 Demo 4: Loading and Configuring Models");

    // Example model configurations
    let model_configs = vec![
        (
            "gpt-3.5-turbo",
            ModelConfiguration {
                parameters: ModelParameters {
                    max_tokens: Some(1000),
                    temperature: Some(0.7),
                    top_p: Some(0.9),
                    frequency_penalty: Some(0.0),
                    presence_penalty: Some(0.0),
                    stop_sequences: None,
                    system_prompt: None,
                    custom_parameters: HashMap::new(),
                },
                context_length: Some(4096),
                batch_size: Some(1),
                precision: Some(ModelPrecision::Float32),
                custom_config: HashMap::new(),
            },
        ),
        (
            "claude-3-sonnet",
            ModelConfiguration {
                parameters: ModelParameters {
                    max_tokens: Some(2000),
                    temperature: Some(0.5),
                    top_p: Some(0.95),
                    frequency_penalty: None,
                    presence_penalty: None,
                    stop_sequences: Some(vec!["Human:".to_string()]),
                    system_prompt: None,
                    custom_parameters: HashMap::new(),
                },
                context_length: Some(8192),
                batch_size: Some(1),
                precision: Some(ModelPrecision::Float32),
                custom_config: HashMap::new(),
            },
        ),
    ];

    for (model_id, config) in model_configs {
        info!("  🔧 Loading model: {}", model_id);

        let load_request = ModelLoadRequest {
            model_id: model_id.to_string(),
            configuration: config,
            resource_requirements: ResourceRequirements {
                memory_gb: Some(8.0),
                gpu_memory_gb: Some(4.0),
                cpu_cores: Some(4),
                storage_gb: Some(20.0),
                network_bandwidth_mbps: Some(100),
            },
            optimization_settings: OptimizationSettings {
                enable_quantization: false,
                enable_compilation: true,
                enable_caching: true,
                enable_batching: true,
                optimization_level: OptimizationLevel::Basic,
            },
        };

        match registry.load_model(load_request).await {
            Ok(handle) => {
                info!("  ✅ Model loaded successfully");
                info!("    Handle ID: {}", handle.handle_id);
                info!("    Provider: {}", handle.provider);
                info!("    Loaded at: {}", handle.loaded_at);
                info!(
                    "    Memory usage: {:.2} GB",
                    handle.resource_usage.memory_used_gb
                );
                info!(
                    "    GPU memory: {:.2} GB",
                    handle.resource_usage.gpu_memory_used_gb
                );
            }
            Err(e) => warn!("  ❌ Failed to load model: {}", e),
        }
    }

    Ok(())
}

async fn universal_inference_demo(registry: &Arc<UniversalModelRegistry>) -> Result<()> {
    info!("🧠 Demo 5: Universal Inference");

    // Create a mock model handle for demonstration
    let mock_handle = ModelHandle {
        handle_id: Uuid::new_v4(),
        model_id: "mock-model".to_string(),
        provider: "demo".to_string(),
        loaded_at: chrono::Utc::now(),
        configuration: ModelConfiguration {
            parameters: ModelParameters {
                max_tokens: Some(100),
                temperature: Some(0.7),
                top_p: Some(0.9),
                frequency_penalty: None,
                presence_penalty: None,
                stop_sequences: None,
                system_prompt: None,
                custom_parameters: HashMap::new(),
            },
            context_length: Some(2048),
            batch_size: Some(1),
            precision: Some(ModelPrecision::Float32),
            custom_config: HashMap::new(),
        },
        resource_usage: nestgate_core::universal_model_api::ResourceUsage {
            memory_used_gb: 4.0,
            gpu_memory_used_gb: 2.0,
            cpu_utilization: 0.5,
            storage_used_gb: 1.0,
            network_usage_mbps: 10.0,
        },
    };

    // Example inference requests
    let inference_requests = vec![
        (
            "Text Generation",
            InferenceInput::Text("What are the benefits of Rust programming language?".to_string()),
        ),
        (
            "Structured Data",
            InferenceInput::Structured(json!({
                "prompt": "Analyze this data structure",
                "context": "programming",
                "format": "json"
            })),
        ),
    ];

    for (description, input) in inference_requests {
        info!("  🔍 Running inference: {}", description);

        let inference_request = InferenceRequest {
            request_id: Uuid::new_v4(),
            model_handle: mock_handle.clone(),
            input,
            parameters: Some(ModelParameters {
                max_tokens: Some(200),
                temperature: Some(0.8),
                top_p: Some(0.95),
                frequency_penalty: None,
                presence_penalty: None,
                stop_sequences: None,
                system_prompt: None,
                custom_parameters: HashMap::new(),
            }),
            output_format: nestgate_core::universal_model_api::OutputFormat::Json,
            streaming: false,
        };

        match registry.inference(inference_request).await {
            Ok(response) => {
                info!("  ✅ Inference completed successfully");
                info!("    Response ID: {}", response.response_id);
                info!("    Latency: {:.2} ms", response.metrics.latency_ms);
                if let Some(tokens) = response.metrics.tokens_generated {
                    info!("    Tokens generated: {}", tokens);
                }
                if let Some(throughput) = response.metrics.throughput_tokens_per_second {
                    info!("    Throughput: {:.2} tokens/sec", throughput);
                }
            }
            Err(e) => warn!("  ❌ Inference failed: {}", e),
        }
    }

    Ok(())
}

async fn registry_health_demo(registry: &Arc<UniversalModelRegistry>) -> Result<()> {
    info!("🏥 Demo 6: Registry Health and Metrics");

    match registry.get_health().await {
        Ok(health) => {
            info!("  📊 Registry Health Report");
            info!("    Total providers: {}", health.total_providers);
            info!("    Healthy providers: {}", health.healthy_providers);
            info!("    Total loaded models: {}", health.total_models);

            info!("  🔍 Provider Health Details:");
            for provider_health in health.provider_healths {
                info!("    Provider: {}", provider_health.provider);
                info!("      Status: {:?}", provider_health.status);
                info!(
                    "      Response time: {:.2} ms",
                    provider_health.response_time_ms
                );
                info!(
                    "      Available models: {}",
                    provider_health.available_models
                );
                info!(
                    "      Active connections: {}",
                    provider_health.active_connections
                );
                info!(
                    "      Error rate: {:.2}%",
                    provider_health.error_rate * 100.0
                );
            }
        }
        Err(e) => warn!("  ❌ Failed to get registry health: {}", e),
    }

    Ok(())
}

/// Example configuration for different model providers
#[allow(dead_code)]
fn example_configurations() -> Vec<ProviderConfig> {
    vec![
        // HuggingFace configuration
        ProviderConfig {
            provider_type: "huggingface".to_string(),
            base_url: Some("https://api-inference.huggingface.co".to_string()),
            api_key: Some("hf_your_api_key_here".to_string()),
            timeout_seconds: 30,
            max_retries: 3,
            custom_headers: Some({
                let mut headers = HashMap::new();
                headers.insert("User-Agent".to_string(), "NestGate/1.0".to_string());
                headers
            }),
            custom_config: {
                let mut config = HashMap::new();
                config.insert("use_cache".to_string(), json!(true));
                config.insert("wait_for_model".to_string(), json!(true));
                config
            },
            max_tokens: Some(2048),
            enable_streaming: true,
            model_filter: vec![],
            supports_embeddings: true,
            model_path: Some("/tmp/huggingface_cache".to_string()),
        },
        // OpenAI configuration
        ProviderConfig {
            provider_type: "openai".to_string(),
            base_url: Some("https://api.openai.com/v1".to_string()),
            api_key: Some("sk-your_api_key_here".to_string()),
            timeout_seconds: 60,
            max_retries: 3,
            custom_headers: None,
            custom_config: {
                let mut config = HashMap::new();
                config.insert("model_fallback".to_string(), json!(true));
                config.insert("organization".to_string(), json!("org-your_org_id"));
                config
            },
            max_tokens: Some(4096),
            enable_streaming: true,
            model_filter: vec![],
            supports_embeddings: true,
            model_path: None,
        },
        // Claude configuration
        ProviderConfig {
            provider_type: "claude".to_string(),
            base_url: Some("https://api.anthropic.com/v1".to_string()),
            api_key: Some("sk-ant-your_api_key_here".to_string()),
            timeout_seconds: 60,
            max_retries: 3,
            custom_headers: Some({
                let mut headers = HashMap::new();
                headers.insert("anthropic-version".to_string(), "2023-06-01".to_string());
                headers
            }),
            custom_config: HashMap::new(),
            max_tokens: Some(4096),
            enable_streaming: true,
            model_filter: vec![],
            supports_embeddings: false,
            model_path: None,
        },
        // Local model configuration
        ProviderConfig {
            provider_type: "local".to_string(),
            base_url: Some("http://localhost:8080".to_string()),
            api_key: None,
            timeout_seconds: 120,
            max_retries: 1,
            custom_headers: None,
            custom_config: {
                let mut config = HashMap::new();
                config.insert("gpu_layers".to_string(), json!(32));
                config.insert("context_length".to_string(), json!(4096));
                config.insert("batch_size".to_string(), json!(512));
                config.insert("threads".to_string(), json!(8));
                config
            },
            max_tokens: Some(4096),
            enable_streaming: true,
            model_filter: vec![],
            supports_embeddings: true,
            model_path: Some("/models".to_string()),
        },
        // Ollama configuration
        ProviderConfig {
            provider_type: "ollama".to_string(),
            base_url: Some("http://localhost:11434".to_string()),
            api_key: None,
            timeout_seconds: 60,
            max_retries: 2,
            custom_headers: None,
            custom_config: {
                let mut config = HashMap::new();
                config.insert("keep_alive".to_string(), json!("5m"));
                config.insert("num_ctx".to_string(), json!(4096));
                config
            },
            max_tokens: Some(4096),
            enable_streaming: true,
            model_filter: vec![],
            supports_embeddings: true,
            model_path: None,
        },
    ]
}
