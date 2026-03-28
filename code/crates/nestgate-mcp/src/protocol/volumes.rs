// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **VOLUME OPERATION TYPES**
//!
//! Volume create, mount, unmount, and list payload types.

use crate::types::{MountOptions, StorageProtocol, StorageTier};
use serde::{Deserialize, Serialize};

/// Volume Create Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeCreatePayload {
    /// Volume name
    pub name: String,
    /// Size in bytes
    pub size: u64,
    /// Storage tier
    pub tier: StorageTier,
    /// Protocol
    pub protocol: StorageProtocol,
}

/// Volume Delete Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeDeletePayload {
    /// Volume ID
    pub volume_id: String,
}

/// Volume Mount Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMountPayload {
    /// Volume ID
    pub volume_id: String,
    /// Mount point
    pub mount_point: String,
    /// Mount options
    pub options: MountOptions,
}

/// Volume Unmount Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeUnmountPayload {
    /// Volume ID
    pub volume_id: String,
    /// Mount point
    pub mount_point: String,
}

/// Volume List Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeListPayload {
    /// Filter
    pub filter: Option<String>,
}

/// Volume Info Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfoPayload {
    /// Volume ID
    pub volume_id: String,
}
