---
title: NestGate Snapshot Operations API
description: API specification for ZFS snapshot management operations
version: 0.1.0
date: July 2024
status: Draft
priority: High
---

# Snapshot Operations API Specification

## Overview

This specification defines the API endpoints for comprehensive ZFS snapshot management in the NestGate storage system. These endpoints enable creating, listing, rolling back to, and deleting snapshots with scheduling capabilities.

## Endpoints

### Snapshot Creation

#### POST /api/snapshots

Creates a new ZFS snapshot.

**Request Body**:
```json
{
  "dataset": "poolname/datasetname",
  "name": "backup-20240726",
  "recursive": true,
  "properties": {
    "comment": "Weekly backup snapshot",
    "retention": "30d"
  }
}
```

**Response**:
```json
{
  "status": "success",
  "data": {
    "name": "poolname/datasetname@backup-20240726",
    "creation": "2024-07-26T10:30:45Z",
    "referenced": "10G",
    "written": "1G",
    "properties": {
      "comment": "Weekly backup snapshot",
      "retention": "30d"
    }
  },
  "message": "Snapshot created successfully",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

**Error Responses**:

- 400 Bad Request: Invalid parameters
```json
{
  "status": "error",
  "data": null,
  "message": "Invalid snapshot name: may only contain alphanumeric characters, hyphen, and underscore",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

- 404 Not Found: Dataset doesn't exist
```json
{
  "status": "error",
  "data": null,
  "message": "Dataset 'poolname/datasetname' not found",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

- 409 Conflict: Snapshot already exists
```json
{
  "status": "error",
  "data": null,
  "message": "Snapshot 'poolname/datasetname@backup-20240726' already exists",
  "timestamp": "2024-07-26T10:30:45Z"
}
```

### List Snapshots

#### GET /api/snapshots

Lists all snapshots or filters by dataset.

**Query Parameters**:
- `dataset` (optional): Filter snapshots by dataset (e.g., "poolname/datasetname")
- `recursive` (boolean, optional): Include snapshots of child datasets
- `limit` (number, optional): Maximum number of snapshots to return
- `offset` (number, optional): Offset for pagination
- `sort` (string, optional): Sort field (creation, referenced, written)
- `order` (string, optional): Sort order (asc, desc)

**Response**:
```json
{
  "status": "success",
  "data": {
    "snapshots": [
      {
        "name": "poolname/datasetname@backup-20240726",
        "dataset": "poolname/datasetname",
        "creation": "2024-07-26T10:30:45Z",
        "referenced": "10G",
        "written": "1G",
        "properties": {
          "comment": "Weekly backup snapshot",
          "retention": "30d"
        }
      },
      {
        "name": "poolname/datasetname@backup-20240719",
        "dataset": "poolname/datasetname",
        "creation": "2024-07-19T10:30:45Z",
        "referenced": "9.5G",
        "written": "500M",
        "properties": {
          "comment": "Weekly backup snapshot",
          "retention": "30d"
        }
      }
    ],
    "total": 2,
    "limit": 10,
    "offset": 0
  },
  "message": "Snapshots retrieved successfully",
  "timestamp": "2024-07-26T10:35:45Z"
}
```

### Get Snapshot Details

#### GET /api/snapshots/{snapshotName}

Gets detailed information about a specific snapshot.

**Parameters**:
- `snapshotName`: The full name of the snapshot (e.g., "poolname/datasetname@backup-20240726")

**Response**:
```json
{
  "status": "success",
  "data": {
    "name": "poolname/datasetname@backup-20240726",
    "dataset": "poolname/datasetname",
    "creation": "2024-07-26T10:30:45Z",
    "referenced": "10G",
    "written": "1G",
    "user_properties": {
      "comment": "Weekly backup snapshot",
      "retention": "30d"
    },
    "holds": [],
    "clones": []
  },
  "message": "Snapshot details retrieved successfully",
  "timestamp": "2024-07-26T10:35:45Z"
}
```

**Error Responses**:

- 404 Not Found: Snapshot doesn't exist
```json
{
  "status": "error",
  "data": null,
  "message": "Snapshot 'poolname/datasetname@backup-20240726' not found",
  "timestamp": "2024-07-26T10:35:45Z"
}
```

### Rollback to Snapshot

#### POST /api/snapshots/{snapshotName}/rollback

Rolls back a dataset to a specified snapshot.

**Parameters**:
- `snapshotName`: The full name of the snapshot (e.g., "poolname/datasetname@backup-20240726")

**Request Body**:
```json
{
  "force": true,
  "destroyMoreRecent": false
}
```

**Response**:
```json
{
  "status": "success",
  "data": {
    "name": "poolname/datasetname@backup-20240726",
    "dataset": "poolname/datasetname"
  },
  "message": "Successfully rolled back to snapshot 'backup-20240726'",
  "timestamp": "2024-07-26T11:30:45Z"
}
```

**Error Responses**:

- 404 Not Found: Snapshot doesn't exist
```json
{
  "status": "error",
  "data": null,
  "message": "Snapshot 'poolname/datasetname@backup-20240726' not found",
  "timestamp": "2024-07-26T11:30:45Z"
}
```

- 400 Bad Request: Cannot rollback
```json
{
  "status": "error",
  "data": null,
  "message": "Cannot rollback: more recent snapshots exist and destroyMoreRecent is false",
  "timestamp": "2024-07-26T11:30:45Z"
}
```

### Delete Snapshot

#### DELETE /api/snapshots/{snapshotName}

Deletes a ZFS snapshot.

**Parameters**:
- `snapshotName`: The full name of the snapshot (e.g., "poolname/datasetname@backup-20240726")

**Query Parameters**:
- `recursive` (boolean, optional): If true, also delete the snapshot from all descendent datasets
- `defer` (boolean, optional): If true, defer snapshot deletion if it cannot be immediately removed

**Response**:
```json
{
  "status": "success",
  "data": null,
  "message": "Snapshot 'poolname/datasetname@backup-20240726' deleted successfully",
  "timestamp": "2024-07-26T11:35:45Z"
}
```

**Error Responses**:

- 404 Not Found: Snapshot doesn't exist
```json
{
  "status": "error",
  "data": null,
  "message": "Snapshot 'poolname/datasetname@backup-20240726' not found",
  "timestamp": "2024-07-26T11:35:45Z"
}
```

- 400 Bad Request: Cannot delete snapshot
```json
{
  "status": "error",
  "data": null,
  "message": "Cannot delete snapshot: snapshot has dependent clones",
  "timestamp": "2024-07-26T11:35:45Z"
}
```

### Snapshot Scheduling

#### POST /api/snapshot-schedules

Creates a new snapshot schedule.

**Request Body**:
```json
{
  "dataset": "poolname/datasetname",
  "recursive": true,
  "namePattern": "auto-{timestamp}",
  "schedule": {
    "type": "cron",
    "expression": "0 0 * * *"
  },
  "retention": {
    "count": 30,
    "duration": "90d"
  },
  "enabled": true,
  "properties": {
    "comment": "Automatic daily snapshot"
  }
}
```

**Response**:
```json
{
  "status": "success",
  "data": {
    "id": "schedule-12345",
    "dataset": "poolname/datasetname",
    "recursive": true,
    "namePattern": "auto-{timestamp}",
    "schedule": {
      "type": "cron",
      "expression": "0 0 * * *",
      "nextRun": "2024-07-27T00:00:00Z"
    },
    "retention": {
      "count": 30,
      "duration": "90d"
    },
    "enabled": true,
    "properties": {
      "comment": "Automatic daily snapshot"
    }
  },
  "message": "Snapshot schedule created successfully",
  "timestamp": "2024-07-26T12:00:00Z"
}
```

#### GET /api/snapshot-schedules

Lists all snapshot schedules.

**Query Parameters**:
- `dataset` (optional): Filter schedules by dataset

**Response**:
```json
{
  "status": "success",
  "data": {
    "schedules": [
      {
        "id": "schedule-12345",
        "dataset": "poolname/datasetname",
        "recursive": true,
        "namePattern": "auto-{timestamp}",
        "schedule": {
          "type": "cron",
          "expression": "0 0 * * *",
          "nextRun": "2024-07-27T00:00:00Z"
        },
        "retention": {
          "count": 30,
          "duration": "90d"
        },
        "enabled": true
      },
      {
        "id": "schedule-12346",
        "dataset": "poolname/dataset2",
        "recursive": false,
        "namePattern": "weekly-{timestamp}",
        "schedule": {
          "type": "cron",
          "expression": "0 0 * * 0",
          "nextRun": "2024-07-28T00:00:00Z"
        },
        "retention": {
          "count": 12,
          "duration": "90d"
        },
        "enabled": true
      }
    ],
    "total": 2
  },
  "message": "Snapshot schedules retrieved successfully",
  "timestamp": "2024-07-26T12:05:00Z"
}
```

#### PUT /api/snapshot-schedules/{scheduleId}

Updates an existing snapshot schedule.

**Parameters**:
- `scheduleId`: The ID of the schedule to update

**Request Body**:
```json
{
  "recursive": false,
  "schedule": {
    "type": "cron",
    "expression": "0 0 */2 * *"
  },
  "retention": {
    "count": 15
  },
  "enabled": true
}
```

**Response**:
```json
{
  "status": "success",
  "data": {
    "id": "schedule-12345",
    "dataset": "poolname/datasetname",
    "recursive": false,
    "namePattern": "auto-{timestamp}",
    "schedule": {
      "type": "cron",
      "expression": "0 0 */2 * *",
      "nextRun": "2024-07-28T00:00:00Z"
    },
    "retention": {
      "count": 15,
      "duration": "90d"
    },
    "enabled": true
  },
  "message": "Snapshot schedule updated successfully",
  "timestamp": "2024-07-26T12:10:00Z"
}
```

#### DELETE /api/snapshot-schedules/{scheduleId}

Deletes a snapshot schedule.

**Parameters**:
- `scheduleId`: The ID of the schedule to delete

**Response**:
```json
{
  "status": "success",
  "data": null,
  "message": "Snapshot schedule deleted successfully",
  "timestamp": "2024-07-26T12:15:00Z"
}
```

## Schedule Types

The API supports the following schedule types:

### Cron Schedule

```json
{
  "type": "cron",
  "expression": "0 0 * * *"
}
```

The cron expression follows the standard format: `minute hour day-of-month month day-of-week`.

### Interval Schedule

```json
{
  "type": "interval",
  "interval": "1d"
}
```

Valid interval units: `m` (minutes), `h` (hours), `d` (days), `w` (weeks).

### One-time Schedule

```json
{
  "type": "one-time",
  "time": "2024-08-01T12:00:00Z"
}
```

## Name Pattern Variables

The `namePattern` field supports the following variables:

- `{timestamp}`: Current timestamp in the format YYYYMMDD-HHMMSS
- `{date}`: Current date in the format YYYYMMDD
- `{time}`: Current time in the format HHMMSS
- `{hostname}`: System hostname
- `{increment}`: Auto-incrementing number

## Implementation Requirements

The implementation of these API endpoints should:

1. Validate all input parameters before passing to ZFS commands
2. Implement proper error handling with appropriate HTTP status codes
3. Use background processing for schedule execution
4. Implement proper locking mechanisms when manipulating snapshots
5. Use transactional operations where possible to ensure consistency
6. Ensure proper cleanup of old snapshots according to retention policies
7. Implement appropriate error recovery mechanisms
8. Log all snapshot operations for audit purposes

## Testing Requirements

The following tests should be implemented:

1. Unit tests for parameter validation and schedule parsing
2. Integration tests for each endpoint
3. Schedule execution tests using time manipulation
4. Error case testing for all possible error conditions
5. Performance testing for large numbers of snapshots
6. Load testing for concurrent snapshot operations

## UI Integration

These endpoints will be integrated with the React-based UI components:

1. Snapshot browser with filtering options
2. Snapshot creation dialog
3. Rollback confirmation dialog
4. Scheduling interface with:
   - Schedule type selection
   - Retention policy configuration
   - Recursive snapshot option
   - Dataset selector

## WebSocket Notifications

The system will emit WebSocket notifications for the following events:

1. Snapshot creation
2. Snapshot deletion
3. Rollback operations
4. Schedule execution
5. Schedule changes

**Sample WebSocket Message**:
```json
{
  "type": "snapshot",
  "action": "created",
  "data": {
    "name": "poolname/datasetname@backup-20240726",
    "dataset": "poolname/datasetname",
    "creation": "2024-07-26T10:30:45Z"
  },
  "timestamp": "2024-07-26T10:30:45Z"
}
```

## Next Steps

1. Implement core snapshot CRUD operations
2. Develop scheduler subsystem
3. Create integration tests for all endpoints
4. Update UI components to use new API
5. Implement WebSocket notifications
6. Develop retention policy enforcement

## Version History

- 0.1.0 (July 2024): Initial draft 