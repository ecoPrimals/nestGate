// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! File-backed [`super::MetadataBackend`] implementation.

use super::{MetadataBackend, Result, ServiceRecord};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use std::future::Future;
use std::path::{Path, PathBuf};
use tokio::fs;

/// Subdirectory under [`FileMetadataBackend`] base for service JSON files (`{base}/services/*.json`).
pub const METADATA_SERVICES_NAMESPACE: &str = "services";

pub fn service_record_path(base_dir: &Path, name: &str) -> PathBuf {
    let file_key = format!("{}.json", URL_SAFE_NO_PAD.encode(name.as_bytes()));
    base_dir.join(METADATA_SERVICES_NAMESPACE).join(file_key)
}

/// File-backed metadata: one JSON file per service under `{base_dir}/services/{key}.json`
/// where `key` is the URL-safe base64 of the UTF-8 service name.
#[derive(Debug, Clone)]
pub struct FileMetadataBackend {
    base_dir: PathBuf,
}

impl FileMetadataBackend {
    /// Open or create the backend, ensuring `{base_dir}/services` exists.
    ///
    /// # Errors
    ///
    /// Returns if the directory cannot be created.
    pub fn new(base_dir: PathBuf) -> Result<Self> {
        let services = base_dir.join(METADATA_SERVICES_NAMESPACE);
        std::fs::create_dir_all(&services)?;
        Ok(Self { base_dir })
    }

    /// Base directory passed to [`Self::new`].
    #[must_use]
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }

    async fn iter_service_records(&self) -> Result<Vec<ServiceRecord>> {
        let dir = self.base_dir.join(METADATA_SERVICES_NAMESPACE);
        let mut entries = match fs::read_dir(&dir).await {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e.into()),
        };

        let mut out = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            if !path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
            {
                continue;
            }
            let Ok(bytes) = fs::read(&path).await else {
                continue;
            };
            if let Ok(record) = serde_json::from_slice::<ServiceRecord>(&bytes) {
                out.push(record);
            }
        }
        Ok(out)
    }
}

impl MetadataBackend for FileMetadataBackend {
    #[expect(
        clippy::manual_async_fn,
        reason = "trait requires impl Future with Send bound"
    )]
    fn store_service(&self, record: ServiceRecord) -> impl Future<Output = Result<()>> + Send + '_ {
        async move {
            let path = service_record_path(&self.base_dir, &record.name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let bytes = serde_json::to_vec_pretty(&record)?;
            fs::write(&path, bytes).await?;
            Ok(())
        }
    }

    fn get_service(&self, name: &str) -> impl Future<Output = Result<ServiceRecord>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let path = service_record_path(&self.base_dir, &name);
            let bytes = fs::read(&path).await.map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    nestgate_types::error::NestGateError::not_found(format!("service `{name}`"))
                } else {
                    e.into()
                }
            })?;
            let record: ServiceRecord = serde_json::from_slice(&bytes)?;
            Ok(record)
        }
    }

    fn find_by_capability(
        &self,
        capability: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let capability = capability.to_owned();
        async move {
            let all = self.iter_service_records().await?;
            let mut out: Vec<ServiceRecord> = all
                .into_iter()
                .filter(|s| s.capabilities.iter().any(|c| c == &capability))
                .collect();
            out.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(out)
        }
    }

    fn list_services_by_name_prefix(
        &self,
        prefix: &str,
    ) -> impl Future<Output = Result<Vec<ServiceRecord>>> + Send + '_ {
        let prefix = prefix.to_owned();
        async move {
            let all = self.iter_service_records().await?;
            let mut matches: Vec<ServiceRecord> = all
                .into_iter()
                .filter(|s| s.name.starts_with(&*prefix))
                .collect();
            matches.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(matches)
        }
    }

    fn delete_service(&self, name: &str) -> impl Future<Output = Result<()>> + Send + '_ {
        let name = name.to_owned();
        async move {
            let path = service_record_path(&self.base_dir, &name);
            match fs::remove_file(&path).await {
                Ok(()) => Ok(()),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(
                    nestgate_types::error::NestGateError::not_found(format!("service `{name}`")),
                ),
                Err(e) => Err(e.into()),
            }
        }
    }
}
