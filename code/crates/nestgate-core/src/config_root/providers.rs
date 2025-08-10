use crate::error::{NetworkError};
use std::path::PathBuf;
use std::marker::PhantomData;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::error::Result;
use crate::traits::config::ConfigProvider;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
}

/// File-based configuration provider
pub struct FileConfigProvider<T> {
    path: PathBuf,
    format: ConfigFormat,
    _phantom: PhantomData<T>,
}

impl<T> FileConfigProvider<T> {
    pub fn new(path: PathBuf, format: ConfigFormat) -> Self {
        Self {
            path,
            format,
            _phantom: PhantomData,
        }
    }

    /// Get the file path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the configuration format
    pub fn format(&self) -> &ConfigFormat {
        &self.format
    }
}

#[async_trait]
impl<T> ConfigProvider<T> for FileConfigProvider<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    async fn load_config(&self) -> Result<T> {
        let content = tokio::fs::read_to_string(&self.path).await?;

        let config = match self.format {
            ConfigFormat::Json => serde_json::from_str(&content)?,
            ConfigFormat::Yaml => serde_yaml::from_str(&content)?,
            ConfigFormat::Toml => toml::from_str(&content)?,
        };

        Ok(config)
    }

    async fn provider_info(&self) -> String {
        format!("FileConfigProvider(path: {:?}, format: {:?})", self.path, self.format)
    }

    async fn watch_config(&self) -> Result<impl futures_util::Stream<Item = Result<T>> + Send> {
        // Simple implementation that just returns empty stream
        // In a real implementation, this would watch the file for changes
        Ok(futures_util::stream::empty())
    }
}