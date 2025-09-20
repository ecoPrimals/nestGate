//! Security testing functionality for E2E tests

use reqwest::Client;
use serde_json;

pub struct SecurityTester {
    pub http_client: Client,
    pub base_url: String,
}

impl SecurityTester {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
    }

    pub async fn test_sql_injection_protection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/data/query", self.base_url);
        let malicious_query = "'; DROP TABLE users; --";
        
        let response = self.http_client
            .post(&url)
            .json(&serde_json::json!({"query": malicious_query}))
            .send()
            .await?;
        
        // Should return error or sanitized response, not success
        if response.status().is_client_error() {
            Ok(())
        } else {
            Err("SQL injection protection failed".into())
        }
    }

    pub async fn test_xss_protection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/data/input", self.base_url);
        let malicious_input = "<script>alert('xss')</script>";
        
        let response = self.http_client
            .post(&url)
            .json(&serde_json::json!({"input": malicious_input}))
            .send()
            .await?;
        
        if response.status().is_client_error() {
            Ok(())
        } else {
            Err("XSS protection failed".into())
        }
    }

    pub async fn test_csrf_protection(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/api/data/modify", self.base_url);
        
        // Try request without CSRF token
        let response = self.http_client
            .post(&url)
            .json(&serde_json::json!({"data": "test"}))
            .send()
            .await?;
        
        if response.status().is_client_error() {
            Ok(())
        } else {
            Err("CSRF protection failed".into())
        }
    }
} 