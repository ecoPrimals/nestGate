---
title: NestGate Dataset Operations API
description: Comprehensive API for ZFS dataset management operations
version: 0.1.0
date: July 2024
status: Draft
priority: High
---

# Dataset Operations API Specification

## Overview

This specification defines the API endpoints for comprehensive dataset management operations in the NestGate ZFS storage system. These endpoints enable creating, modifying, and deleting datasets with full property control.

## Endpoints

### Dataset Creation

#### POST /api/datasets

Creates a new ZFS dataset.

**Request Body**:
```json
{
  "name": "poolname/datasetname",
  "properties": {
    "compression": "lz4",
    "recordsize": "128K",
    "quota": "100G",
    "reservation": "50G",
    "sharenfs": "on",
    "sharesmb": "on",
    "mountpoint": "/custom/mountpoint",
    "casesensitivity": "sensitive"
  }
}
```

**Response**:
```json
{
  "status": "success",
  "data": {
    "name": "poolname/datasetname",
    "mountpoint": "/custom/mountpoint",
    "used": 0,
    "available": 10737418240,
    "compression": "lz4",
    "recordsize": "128K",
    "type": "filesystem",
    "quota": "100G",
    "reservation": "50G"
  },
  "message": "Dataset created successfully",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

**Error Responses**:

- 400 Bad Request: Invalid parameters
```json
{
  "status": "error",
  "data": null,
  "message": "Invalid parameter: compression must be one of [off, lz4, gzip, zstd]",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

- 409 Conflict: Dataset already exists
```json
{
  "status": "error",
  "data": null,
  "message": "Dataset 'poolname/datasetname' already exists",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

- 500 Internal Server Error: ZFS command failed
```json
{
  "status": "error",
  "data": null,
  "message": "Failed to create dataset: internal error",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

### Dataset Modification

#### PUT /api/datasets/{datasetName}

Updates properties of an existing ZFS dataset.

**Parameters**:
- `datasetName`: The name of the ZFS dataset (including pool name, e.g., "poolname/datasetname")

**Request Body**:
```json
{
  "properties": {
    "compression": "zstd",
    "quota": "200G",
    "recordsize": "1M"
  }
}
```

**Response**:
```json
{
  "status": "success",
  "data": {
    "name": "poolname/datasetname",
    "compression": "zstd",
    "quota": "200G",
    "recordsize": "1M"
  },
  "message": "Dataset properties updated successfully",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

**Error Responses**:

- 404 Not Found: Dataset doesn't exist
```json
{
  "status": "error",
  "data": null,
  "message": "Dataset 'poolname/datasetname' not found",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

- 400 Bad Request: Invalid property value
```json
{
  "status": "error",
  "data": null,
  "message": "Invalid value for property 'recordsize': must be power of 2 between 512 and 1M",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

### Dataset Deletion

#### DELETE /api/datasets/{datasetName}

Deletes a ZFS dataset.

**Parameters**:
- `datasetName`: The name of the ZFS dataset (including pool name, e.g., "poolname/datasetname")

**Query Parameters**:
- `recursive` (boolean, optional): If true, recursively delete all child datasets and snapshots
- `force` (boolean, optional): If true, force deletion even if dataset is busy

**Response**:
```json
{
  "status": "success",
  "data": null,
  "message": "Dataset 'poolname/datasetname' deleted successfully",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

**Error Responses**:

- 404 Not Found: Dataset doesn't exist
```json
{
  "status": "error",
  "data": null,
  "message": "Dataset 'poolname/datasetname' not found",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

- 400 Bad Request: Cannot delete dataset with children without recursive flag
```json
{
  "status": "error",
  "data": null,
  "message": "Cannot delete dataset with children without recursive flag",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

### Batch Operations

#### POST /api/datasets/batch

Performs batch operations on multiple datasets.

**Request Body**:
```json
{
  "operations": [
    {
      "operation": "create",
      "name": "poolname/dataset1",
      "properties": {
        "compression": "lz4"
      }
    },
    {
      "operation": "update",
      "name": "poolname/dataset2",
      "properties": {
        "quota": "50G"
      }
    },
    {
      "operation": "delete",
      "name": "poolname/dataset3",
      "recursive": true
    }
  ]
}
```

**Response**:
```json
{
  "status": "success",
  "data": {
    "results": [
      {
        "operation": "create",
        "name": "poolname/dataset1",
        "status": "success",
        "message": "Dataset created successfully"
      },
      {
        "operation": "update",
        "name": "poolname/dataset2",
        "status": "success",
        "message": "Dataset properties updated successfully"
      },
      {
        "operation": "delete",
        "name": "poolname/dataset3",
        "status": "success",
        "message": "Dataset deleted successfully"
      }
    ],
    "summary": {
      "total": 3,
      "successful": 3,
      "failed": 0
    }
  },
  "message": "Batch operations completed successfully",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

**Error Response Example**:
```json
{
  "status": "error",
  "data": {
    "results": [
      {
        "operation": "create",
        "name": "poolname/dataset1",
        "status": "success",
        "message": "Dataset created successfully"
      },
      {
        "operation": "update",
        "name": "poolname/dataset2",
        "status": "error",
        "message": "Dataset not found"
      },
      {
        "operation": "delete",
        "name": "poolname/dataset3",
        "status": "success",
        "message": "Dataset deleted successfully"
      }
    ],
    "summary": {
      "total": 3,
      "successful": 2,
      "failed": 1
    }
  },
  "message": "Batch operations completed with errors",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

## ZFS Dataset Properties

The following ZFS properties can be configured:

| Property | Type | Description | Default |
|----------|------|-------------|---------|
| compression | string | Compression algorithm (off, lz4, gzip, zstd) | lz4 |
| atime | string | Access time updates (on, off) | on |
| relatime | string | Relative access time (on, off) | off |
| devices | string | Allow device files (on, off) | on |
| exec | string | Allow execution of binaries (on, off) | on |
| setuid | string | Honor setuid bit (on, off) | on |
| readonly | string | Read-only mount (on, off) | off |
| snapdir | string | Make .zfs directory visible (hidden, visible) | hidden |
| aclinherit | string | ACL inheritance (discard, noallow, restricted, passthrough, passthrough-x) | restricted |
| canmount | string | Mountable (on, off, noauto) | on |
| casesensitivity | string | Case sensitivity (sensitive, insensitive, mixed) | sensitive |
| nbmand | string | Non-blocking mandatory locks (on, off) | off |
| normalization | string | Unicode normalization (none, formC, formD, formKC, formKD) | none |
| utf8only | string | Reject non-UTF-8 filenames (on, off) | off |
| vscan | string | Virus scan on file open (on, off) | off |
| copies | number | Number of data copies (1, 2, 3) | 1 |
| quota | string | Max filesystem size (none, size) | none |
| refquota | string | Max dataset size (none, size) | none |
| reservation | string | Min guaranteed space (none, size) | none |
| refreservation | string | Min guaranteed space (none, size) | none |
| recordsize | string | Record size (512B to 1M, power of 2) | 128K |
| mountpoint | string | Mount point | /poolname/datasetname |
| sharenfs | string | NFS share settings (on, off, opts) | off |
| sharesmb | string | SMB share settings (on, off, opts) | off |
| logbias | string | Log bias (latency, throughput) | latency |
| primarycache | string | Primary cache (all, none, metadata) | all |
| secondarycache | string | Secondary cache (all, none, metadata) | all |
| sync | string | Sync behavior (standard, always, disabled) | standard |

## Implementation Requirements

The implementation of these API endpoints should:

1. Validate all input parameters before passing to ZFS commands
2. Implement proper error handling with appropriate HTTP status codes
3. Use transactional operations where possible to ensure consistency
4. Follow the NestGate error handling framework for consistent error responses
5. Ensure all operations are properly logged for audit purposes
6. Implement appropriate permission checks (future implementation)
7. Optimize for HDD-based storage systems with appropriate default values

## Testing Requirements

The following tests should be implemented:

1. Unit tests for input validation
2. Integration tests for each endpoint
3. Error case testing for all possible error conditions
4. Performance testing under various load conditions
5. Mock data tests for UI development

## UI Integration

These endpoints will be integrated with the React-based UI components:

1. Dataset creation form with property configuration
2. Dataset property editor
3. Dataset deletion confirmation dialog
4. Batch operations interface

## Next Steps

1. Implement core dataset CRUD operations
2. Develop comprehensive property validation
3. Create integration tests for all endpoints
4. Update UI components to use new API
5. Implement batch operations endpoint
6. Add parameter validation and error handling

## Version History

- 0.1.0 (July 2024): Initial draft 