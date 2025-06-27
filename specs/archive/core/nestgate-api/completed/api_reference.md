---
title: NestGate API Reference Specification
description: Comprehensive API documentation for NestGate ZFS storage management
version: 1.0.0
status: Completed
completion_date: July 2024
---

# API Reference Specification - COMPLETED

> **Completion Note:** This specification has been fully implemented as of July 2024. The comprehensive API reference document is available in `docs/API_REFERENCE.md` and includes documentation for all REST endpoints and WebSocket interfaces.

## Overview

The NestGate API Reference provides complete documentation for all available endpoints in the NestGate ZFS storage management application. This includes REST API endpoints for configuration and management as well as WebSocket interfaces for real-time monitoring and updates.

## API Conventions

### Base URL

All API endpoints are relative to the base URL:

```
http://localhost:3000
```

### Response Format

All API responses follow a standardized format:

```json
{
  "status": "success",  // or "error" for error responses
  "data": {},           // Response data object or null for errors
  "message": "",        // Empty for success, error message for failures
  "timestamp": "2023-10-18T10:30:45Z" // ISO 8601 timestamp
}
```

### Error Responses

Error responses use the same structure with appropriate HTTP status codes:

```json
{
  "status": "error",
  "data": null,
  "message": "Error message details",
  "timestamp": "2023-10-18T10:30:45Z"
}
```

## REST Endpoints

### System Information

#### GET /api/status

Returns system status information.

**Response**:
```json
{
  "status": "success",
  "data": {
    "cpuUsage": 12.5,
    "memoryUsage": 35.7,
    "uptime": 1234567,
    "hostname": "nestgate-storage",
    "osInfo": "Linux 6.1.0-13-amd64 x86_64",
    "zfsVersion": "2.1.11-1",
    "networkInterfaces": [
      {
        "name": "eth0",
        "ipAddress": "192.168.1.100",
        "macAddress": "00:11:22:33:44:55",
        "status": "up",
        "speed": "1Gbps"
      }
    ],
    "loadAverage": {
      "1min": 0.45,
      "5min": 0.52,
      "15min": 0.48
    }
  },
  "message": "",
  "timestamp": "2023-10-18T10:30:45Z"
}
```

#### GET /api/health

Returns system health information and service statuses.

**Response**:
```json
{
  "status": "success",
  "data": {
    "status": "healthy",
    "timestamp": "2023-10-18T10:30:45Z",
    "services": {
      "zfs": {
        "status": "healthy",
        "message": "ZFS module loaded and functioning normally",
        "lastCheck": "2023-10-18T10:30:40Z"
      },
      "network": {
        "status": "healthy",
        "message": "All network interfaces are up",
        "lastCheck": "2023-10-18T10:30:42Z"
      },
      "disk": {
        "status": "healthy",
        "message": "All disks are functioning normally",
        "lastCheck": "2023-10-18T10:30:39Z"
      },
      "api": {
        "status": "healthy",
        "message": "API server is responding normally",
        "lastCheck": "2023-10-18T10:30:45Z"
      },
      "websocket": {
        "status": "healthy",
        "message": "WebSocket server is accepting connections",
        "lastCheck": "2023-10-18T10:30:43Z"
      }
    }
  },
  "message": "",
  "timestamp": "2023-10-18T10:30:45Z"
}
```

### ZFS Storage Management

#### GET /api/pools

Returns a list of all ZFS pools.

**Response**:
```json
{
  "status": "success",
  "data": [
    {
      "name": "testpool",
      "size": 1099511627776,
      "allocated": 549755813888,
      "free": 549755813888,
      "health": "ONLINE",
      "fragmentation": 10,
      "lastScrub": "2023-10-15T08:00:00Z"
    },
    {
      "name": "backuppool",
      "size": 2199023255552,
      "allocated": 1099511627776,
      "free": 1099511627776,
      "health": "ONLINE",
      "fragmentation": 5,
      "lastScrub": "2023-10-10T08:00:00Z"
    }
  ],
  "message": "",
  "timestamp": "2023-10-18T10:30:45Z"
}
```

#### GET /api/pools/{poolName}

Returns detailed information about a specific ZFS pool.

**Parameters**:
- `poolName`: The name of the ZFS pool

**Response**:
```json
{
  "status": "success",
  "data": {
    "name": "testpool",
    "size": 1099511627776,
    "allocated": 549755813888,
    "free": 549755813888,
    "health": "ONLINE",
    "fragmentation": 10,
    "lastScrub": "2023-10-15T08:00:00Z",
    "datasets": [
      {
        "name": "testpool/data",
        "used": 107374182400,
        "available": 549755813888,
        "mountpoint": "/testpool/data",
        "compression": "lz4",
        "recordsize": "128K",
        "type": "filesystem"
      }
    ],
    "vdevs": [
      {
        "name": "mirror-0",
        "type": "mirror",
        "state": "ONLINE",
        "devices": [
          {
            "name": "/dev/sda",
            "state": "ONLINE",
            "read": 0,
            "write": 0,
            "checksum": 0
          },
          {
            "name": "/dev/sdb",
            "state": "ONLINE",
            "read": 0,
            "write": 0,
            "checksum": 0
          }
        ]
      }
    ]
  },
  "message": "",
  "timestamp": "2023-10-18T10:30:45Z"
}
```

#### GET /api/datasets/{datasetName}

Returns detailed information about a specific ZFS dataset.

**Parameters**:
- `datasetName`: The name of the ZFS dataset (including pool name, e.g., "poolname/datasetname")

**Response**:
```json
{
  "status": "success",
  "data": {
    "name": "testpool/data",
    "used": 107374182400,
    "available": 549755813888,
    "mountpoint": "/testpool/data",
    "compression": "lz4",
    "recordsize": "128K",
    "type": "filesystem",
    "written": 53687091200,
    "creation": "2023-10-01T00:00:00Z",
    "referenced": 107374182400,
    "compressratio": "1.32x",
    "origin": "-",
    "quota": "none",
    "reservation": "none",
    "volsize": "-",
    "volblocksize": "-",
    "snapshots": [
      {
        "name": "testpool/data@daily-2023-10-15",
        "used": 10737418240,
        "referenced": 107374182400,
        "creation": "2023-10-15T00:00:00Z"
      }
    ]
  },
  "message": "",
  "timestamp": "2023-10-18T10:30:45Z"
}
```

## WebSocket API

The WebSocket API provides real-time updates for system status, pool status, and more.

### Connection

Connect to the WebSocket endpoint:

```
ws://localhost:3000/ws
```

### Message Format

Messages sent through the WebSocket connection follow a consistent format:

```json
{
  "type": "messageType",  // Type of message
  "data": {},             // Message payload
  "timestamp": "2023-10-18T10:30:45Z" // ISO 8601 timestamp
}
```

### Server-to-Client Messages

The server sends the following types of messages:

#### Status Update

```json
{
  "type": "status",
  "data": {
    "cpuUsage": 12.5,
    "memoryUsage": 35.7,
    "uptime": 1234567,
    "hostname": "nestgate-storage"
  },
  "timestamp": "2023-10-18T10:30:45Z"
}
```

#### Pools Update

```json
{
  "type": "pools",
  "data": [
    {
      "name": "testpool",
      "size": 1099511627776,
      "allocated": 549755813888,
      "free": 549755813888,
      "health": "ONLINE",
      "fragmentation": 10
    }
  ],
  "timestamp": "2023-10-18T10:30:45Z"
}
```

### Client-to-Server Messages

Clients can send the following types of messages:

#### Request Data

```json
{
  "type": "request",
  "target": "pool",
  "id": "testpool"
}
```

The `target` can be one of:
- `pool`: Request details for a specific pool
- `dataset`: Request details for a specific dataset
- `system`: Request system status
- `performance`: Request performance metrics
- `disk-health`: Request disk health information

## Error Codes

Error responses include appropriate HTTP status codes:

- `200 OK`: Successful request
- `400 Bad Request`: Invalid parameters or request format
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server-side error

## Implementation Status

This API Reference specification has been fully implemented and is considered complete. All endpoints have been documented and tested, and the complete documentation is available in `docs/API_REFERENCE.md`.

## Future Enhancements

While this specification is considered complete, the following enhancements could be considered in future iterations:

1. Add authentication and authorization endpoints
2. Implement additional snapshot management endpoints
3. Add backup and replication API endpoints
4. Implement advanced search and filtering capabilities
5. Add pagination for large dataset collections 