//! Test API client for NestGate API
//!
//! This module provides a client for testing the NestGate API endpoints.

use reqwest::{Client, StatusCode};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::{Result, anyhow};

/// API response structure
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: Option<String>,
    #[serde(flatten)]
    pub data: Option<T>,
    pub error_code: Option<String>,
}

/// Test API client
pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            api_key: None,
        }
    }
    
    /// Set the API key
    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }
    
    /// Get the health status
    pub async fn get_health(&self) -> Result<ApiResponse<Value>> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        let status = response.status();
        let body: Value = response.json().await?;
        
        // Handle error responses
        if !status.is_success() {
            let message = body["message"].as_str().unwrap_or("Unknown error").to_string();
            return Err(anyhow!("API error: {}", message));
        }
        
        // Parse success response
        Ok(ApiResponse {
            status: "success".to_string(),
            message: None,
            data: Some(body),
            error_code: None,
        })
    }
    
    /// Get the pools
    pub async fn get_pools(&self) -> Result<ApiResponse<Vec<Value>>> {
        let url = format!("{}/pools", self.base_url);
        let mut builder = self.client.get(&url);
        
        // Add API key if set
        if let Some(api_key) = &self.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = builder.send().await?;
        let status = response.status();
        let body: Value = response.json().await?;
        
        // Handle error responses
        if !status.is_success() {
            let message = body["message"].as_str().unwrap_or("Unknown error").to_string();
            let error_code = body["error_code"].as_str().map(|s| s.to_string());
            
            return Ok(ApiResponse {
                status: "error".to_string(),
                message: Some(message),
                data: None,
                error_code,
            });
        }
        
        // Parse success response
        if let Some(pools) = body.as_array() {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: None,
                data: Some(pools.clone()),
                error_code: None,
            })
        } else {
            Err(anyhow!("Expected array of pools, got: {:?}", body))
        }
    }
    
    /// Get the datasets
    pub async fn get_datasets(&self) -> Result<ApiResponse<Vec<Value>>> {
        let url = format!("{}/datasets", self.base_url);
        let mut builder = self.client.get(&url);
        
        // Add API key if set
        if let Some(api_key) = &self.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = builder.send().await?;
        let status = response.status();
        let body: Value = response.json().await?;
        
        // Handle error responses
        if !status.is_success() {
            let message = body["message"].as_str().unwrap_or("Unknown error").to_string();
            let error_code = body["error_code"].as_str().map(|s| s.to_string());
            
            return Ok(ApiResponse {
                status: "error".to_string(),
                message: Some(message),
                data: None,
                error_code,
            });
        }
        
        // Parse success response
        if let Some(datasets) = body.as_array() {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: None,
                data: Some(datasets.clone()),
                error_code: None,
            })
        } else {
            Err(anyhow!("Expected array of datasets, got: {:?}", body))
        }
    }
    
    /// Get the HDD health status
    pub async fn get_hdd_health(&self) -> Result<ApiResponse<Value>> {
        let url = format!("{}/hdd-health", self.base_url);
        let mut builder = self.client.get(&url);
        
        // Add API key if set
        if let Some(api_key) = &self.api_key {
            builder = builder.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = builder.send().await?;
        let status = response.status();
        let body: Value = response.json().await?;
        
        // Handle error responses
        if !status.is_success() {
            let message = body["message"].as_str().unwrap_or("Unknown error").to_string();
            let error_code = body["error_code"].as_str().map(|s| s.to_string());
            
            return Ok(ApiResponse {
                status: "error".to_string(),
                message: Some(message),
                data: None,
                error_code,
            });
        }
        
        // Parse success response
        Ok(ApiResponse {
            status: "success".to_string(),
            message: None,
            data: Some(body),
            error_code: None,
        })
    }
} 