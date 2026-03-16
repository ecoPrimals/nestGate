---
title: NestGate API Contract
description: HTTP API contract for NAS management
version: 0.6.0
author: ecoPrimals Contributors
---

# NestGate API Contract

## Overview
This document defines the HTTP API contract for the NestGate NAS management system.

## Base URL
```
http(s)://<host>:<port>/api/v1
```

## Authentication
Initial phase uses API key authentication via the `X-API-Key` header.

## Common Headers
```yaml
request:
  X-API-Key: "API key for authentication"
  Content-Type: "application/json"

response:
  Content-Type: "application/json"
```

## Common Response Format
```json
{
  "data": "<response data>",
  "metadata": {
    "timestamp": "ISO8601 timestamp",
    "request_id": "unique request identifier"
  }
}
```

## Error Response Format
```json
{
  "error": {
    "message": "Human readable error message",
    "code": "MACHINE_READABLE_CODE",
    "details": {
      "additional": "error context"
    }
  }
}
```

## Endpoints

### Health Check
```yaml
GET /health:
  description: "Basic health check endpoint"
  auth_required: false
  response:
    200:
      body: "OK"
```

### System Status
```yaml
GET /api/v1/status:
  description: "Get detailed system status"
  auth_required: true
  response:
    200:
      body:
        version: "string"
        uptime: "number (seconds)"
        status: "string (healthy|degraded|error)"
```

### Storage Management
```yaml
GET /api/v1/storage/volumes:
  description: "List all storage volumes"
  auth_required: true
  response:
    200:
      body:
        volumes:
          - name: "string"
            status: "string (mounted|unmounted)"
            size: "number (bytes)"
            used: "number (bytes)"

GET /api/v1/storage/volumes/{name}:
  description: "Get volume details"
  auth_required: true
  parameters:
    name: "string (path)"
  response:
    200:
      body:
        name: "string"
        status: "string"
        size: "number"
        used: "number"
        mount_point: "string"
        filesystem: "string"

POST /api/v1/storage/volumes/{name}/mount:
  description: "Mount a volume"
  auth_required: true
  parameters:
    name: "string (path)"
  response:
    200:
      body:
        status: "string (success|error)"
        mount_point: "string"

POST /api/v1/storage/volumes/{name}/unmount:
  description: "Unmount a volume"
  auth_required: true
  parameters:
    name: "string (path)"
  response:
    200:
      body:
        status: "string (success|error)"
```

### System Configuration
```yaml
GET /api/v1/system/config:
  description: "Get system configuration"
  auth_required: true
  response:
    200:
      body:
        network:
          bind_address: "string"
          port: "number"
        storage:
          default_mount_point: "string"
        logging:
          level: "string"

PUT /api/v1/system/config:
  description: "Update system configuration"
  auth_required: true
  request:
    body:
      network:
        bind_address: "string (optional)"
        port: "number (optional)"
      storage:
        default_mount_point: "string (optional)"
      logging:
        level: "string (optional)"
  response:
    200:
      body:
        status: "string (success|error)"
```

## Status Codes
```yaml
200: "Success"
400: "Bad Request - Invalid parameters"
401: "Unauthorized - Missing or invalid API key"
404: "Not Found - Resource doesn't exist"
500: "Internal Server Error"
```

## Rate Limiting
```yaml
limits:
  default: "100 requests per minute"
  burst: "200 requests per minute"
headers:
  X-RateLimit-Limit: "requests per minute allowed"
  X-RateLimit-Remaining: "requests remaining in current window"
  X-RateLimit-Reset: "UTC timestamp when limit resets"
```

## Technical Metadata
- Version: 0.6.0
- Status: Active
- Last Updated: 2024-03-14 