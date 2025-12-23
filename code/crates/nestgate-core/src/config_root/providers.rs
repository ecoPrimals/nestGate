use crate::error::{NetworkError};
use std::marker::PhantomData;
use serde::de::DeserializeOwned;

use crate::Result;
use crate::traits::config::ConfigProvider;
use serde::Deserialize;

#[derive(Debug, Clone)]
/// Configformat
pub enum ConfigFormat {
    /// Json
    Json,
    /// Yaml
    Yaml,
    /// Toml
    Toml,
}

/// File-based configuration provider
pub struct FileConfigProvider<T> {
    format: ConfigFormat,
    _phantom: PhantomData<T>,
}
impl<T> FileConfigProvider<T> {
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

impl<T> ConfigProvider<T> for FileConfigProvider<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    /// Load Config
    async fn load_config(&self) -> Result<T> {
        let content = tokio::fs::read_to_string(&self.path).await?;

        let config = match self.format {
            ConfigFormat::Json => serde_json::from_str(&content)?,
            ConfigFormat::Yaml => serde_yaml::from_str(&content)?,
            ConfigFormat::Toml => toml::from_str(&content)?,
        };

        Ok(config)
    }

    /// Provider Info
    async fn provider_info(&self) -> String {
    }

    /// Watch Config
    async fn watch_config(&self) -> Result<impl futures_util::Stream<Item = Result<T>> + Send> {
        // Simple implementation that just returns empty stream
        // In a real implementation, this would watch the file for changes
        Ok(futures_util::stream::empty())
    }
