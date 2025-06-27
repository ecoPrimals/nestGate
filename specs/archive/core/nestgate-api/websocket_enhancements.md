---
title: NestGate WebSocket Enhancements Specification
description: Real-time notification system for dataset and snapshot operations
version: 0.1.0
date: July 2024
status: Draft
priority: Medium
---

# WebSocket Enhancements Specification

## Overview

This specification defines enhancements to the NestGate WebSocket interface to provide real-time notifications for dataset and snapshot operations. These enhancements will enable clients to receive immediate updates about changes to the storage system without polling the API.

## Current WebSocket Implementation

The current WebSocket implementation provides basic status updates for system health and pool status. Clients can subscribe to these updates using the following subscription format:

```json
{
  "type": "subscribe",
  "target": "system-status"
}
```

And receive updates in the following format:

```json
{
  "type": "system-status",
  "data": {
    "cpu": 5.2,
    "memory": 45.8,
    "uptime": 342125,
    "timestamp": "2024-07-26T12:00:00Z"
  }
}
```

## Enhancement Goals

1. Extend the WebSocket interface to provide real-time notifications for:
   - Dataset operations (create, modify, delete)
   - Snapshot operations (create, rollback, delete)
   - Scheduled snapshot execution
   - Error conditions and warnings

2. Implement subscription management for these new notification types
3. Ensure efficient and reliable message delivery
4. Provide structured data that clients can easily consume
5. Support filtering of notifications by dataset or operation

## New Notification Types

### Dataset Notifications

#### Dataset Created

```json
{
  "type": "dataset",
  "action": "created",
  "data": {
    "name": "poolname/datasetname",
    "mountpoint": "/poolname/datasetname",
    "properties": {
      "compression": "lz4",
      "recordsize": "128K"
    }
  },
  "timestamp": "2024-07-26T12:00:00Z"
}
```

#### Dataset Modified

```json
{
  "type": "dataset",
  "action": "modified",
  "data": {
    "name": "poolname/datasetname",
    "properties": {
      "compression": "zstd",
      "quota": "200G"
    },
    "previous": {
      "compression": "lz4",
      "quota": "100G"
    }
  },
  "timestamp": "2024-07-26T12:05:00Z"
}
```

#### Dataset Deleted

```json
{
  "type": "dataset",
  "action": "deleted",
  "data": {
    "name": "poolname/datasetname"
  },
  "timestamp": "2024-07-26T12:10:00Z"
}
```

### Snapshot Notifications

#### Snapshot Created

```json
{
  "type": "snapshot",
  "action": "created",
  "data": {
    "name": "poolname/datasetname@backup-20240726",
    "dataset": "poolname/datasetname",
    "creation": "2024-07-26T12:15:00Z",
    "properties": {
      "comment": "Weekly backup"
    }
  },
  "timestamp": "2024-07-26T12:15:00Z"
}
```

#### Snapshot Rolled Back

```json
{
  "type": "snapshot",
  "action": "rolledback",
  "data": {
    "name": "poolname/datasetname@backup-20240726",
    "dataset": "poolname/datasetname",
    "destroyedSnapshots": ["poolname/datasetname@backup-20240727"]
  },
  "timestamp": "2024-07-26T12:20:00Z"
}
```

#### Snapshot Deleted

```json
{
  "type": "snapshot",
  "action": "deleted",
  "data": {
    "name": "poolname/datasetname@backup-20240726",
    "dataset": "poolname/datasetname"
  },
  "timestamp": "2024-07-26T12:25:00Z"
}
```

### Schedule Notifications

#### Schedule Created

```json
{
  "type": "schedule",
  "action": "created",
  "data": {
    "id": "schedule-12345",
    "dataset": "poolname/datasetname",
    "nextRun": "2024-07-27T00:00:00Z"
  },
  "timestamp": "2024-07-26T12:30:00Z"
}
```

#### Schedule Executed

```json
{
  "type": "schedule",
  "action": "executed",
  "data": {
    "id": "schedule-12345",
    "dataset": "poolname/datasetname",
    "result": {
      "status": "success",
      "snapshot": "poolname/datasetname@auto-20240726-123000",
      "nextRun": "2024-07-27T00:00:00Z"
    }
  },
  "timestamp": "2024-07-26T12:30:00Z"
}
```

#### Schedule Execution Failed

```json
{
  "type": "schedule",
  "action": "failed",
  "data": {
    "id": "schedule-12345",
    "dataset": "poolname/datasetname",
    "error": {
      "code": "ENOSPC",
      "message": "No space left on device"
    },
    "nextRun": "2024-07-27T00:00:00Z"
  },
  "timestamp": "2024-07-26T12:30:00Z"
}
```

#### Schedule Modified

```json
{
  "type": "schedule",
  "action": "modified",
  "data": {
    "id": "schedule-12345",
    "dataset": "poolname/datasetname",
    "changes": {
      "expression": "0 0 */2 * *",
      "nextRun": "2024-07-28T00:00:00Z"
    }
  },
  "timestamp": "2024-07-26T12:35:00Z"
}
```

#### Schedule Deleted

```json
{
  "type": "schedule",
  "action": "deleted",
  "data": {
    "id": "schedule-12345",
    "dataset": "poolname/datasetname"
  },
  "timestamp": "2024-07-26T12:40:00Z"
}
```

## Subscription Management

### New Subscription Types

Clients can subscribe to specific notification types using the following format:

```json
{
  "type": "subscribe",
  "target": "dataset",
  "filter": {
    "name": "poolname/datasetname",
    "actions": ["created", "modified", "deleted"]
  }
}
```

```json
{
  "type": "subscribe",
  "target": "snapshot",
  "filter": {
    "dataset": "poolname/datasetname"
  }
}
```

```json
{
  "type": "subscribe",
  "target": "schedule",
  "filter": {
    "actions": ["executed", "failed"]
  }
}
```

### Subscription Response

```json
{
  "type": "subscription",
  "status": "success",
  "data": {
    "id": "sub-12345",
    "target": "dataset",
    "filter": {
      "name": "poolname/datasetname",
      "actions": ["created", "modified", "deleted"]
    }
  },
  "timestamp": "2024-07-26T12:45:00Z"
}
```

### Unsubscribe Request

```json
{
  "type": "unsubscribe",
  "id": "sub-12345"
}
```

### Unsubscribe Response

```json
{
  "type": "subscription",
  "status": "removed",
  "data": {
    "id": "sub-12345"
  },
  "timestamp": "2024-07-26T12:50:00Z"
}
```

## Implementation Requirements

### Server-Side

1. **Message Queue Integration**
   - Implement a message queue system to ensure reliable delivery
   - Messages should be persisted until successfully delivered
   - Support for handling client reconnections

2. **Event Generation**
   - Trigger events from all API endpoints that modify datasets or snapshots
   - Generate events for scheduled operations
   - Include appropriate error events when operations fail

3. **Subscription Management**
   - Efficient storage and lookup of client subscriptions
   - Support for filtering based on dataset name and action type
   - Rate limiting to prevent excessive notifications

4. **Message Formatting**
   - Standardized message format for all notification types
   - Consistent timestamp format (ISO 8601)
   - Appropriate data compression for large messages

### Client-Side

1. **Subscription Handling**
   - Support for subscribing to specific notification types
   - Filtering capabilities to reduce unnecessary traffic
   - Automatic resubscription after reconnection

2. **Message Processing**
   - Parse and validate incoming messages
   - Dispatch to appropriate UI components
   - Handle out-of-order messages

3. **Connection Management**
   - Automatic reconnection with exponential backoff
   - Heartbeat mechanism to detect disconnections
   - Queue outgoing messages during disconnection

## Performance Considerations

1. **Message Size**
   - Limit the size of notification messages
   - Include only necessary data in each message
   - Support for pagination of large data sets

2. **Connection Scaling**
   - Support for thousands of concurrent WebSocket connections
   - Efficient message broadcasting to multiple clients
   - Load balancing across multiple server instances

3. **Rate Limiting**
   - Implement client-specific rate limits
   - Coalesce rapid sequences of similar notifications
   - Prioritize critical notifications

## Security Considerations

1. **Authentication**
   - WebSocket connections must be authenticated
   - Use same authentication mechanism as REST API
   - Support for token refresh without disconnection

2. **Authorization**
   - Check permissions before sending notifications
   - Filter notification content based on user permissions
   - Log all subscription requests for audit purposes

3. **Data Protection**
   - Encrypt WebSocket traffic using TLS
   - Sanitize sensitive information from notifications
   - Implement strict origin checking

## Testing Requirements

1. **Unit Tests**
   - Test notification generation for all API endpoints
   - Verify subscription management logic
   - Test message formatting and validation

2. **Integration Tests**
   - Test end-to-end notification delivery
   - Verify client reconnection behavior
   - Test high-volume message scenarios

3. **Performance Tests**
   - Measure notification latency under load
   - Test with large numbers of concurrent connections
   - Verify message delivery reliability

## UI Integration

The WebSocket enhancements will integrate with the following UI components:

1. **Dataset Browser**
   - Real-time updates of dataset properties
   - Immediate appearance of new datasets
   - Visual indication of dataset deletion

2. **Snapshot Browser**
   - Real-time updates of snapshot list
   - Visual indication of snapshot creation/deletion
   - Progress indication during rollback

3. **Activity Feed**
   - Chronological display of system events
   - Filtering by event type and dataset
   - User-friendly rendering of technical events

4. **Notification Center**
   - Aggregation of important system events
   - User-dismissible notifications
   - Persistence of unread notifications

## Next Steps

1. Design and implement the message queue backend
2. Enhance API endpoints to generate events
3. Implement subscription management
4. Develop client-side WebSocket handler
5. Create UI components for displaying notifications
6. Implement comprehensive testing suite

## Version History

- 0.1.0 (July 2024): Initial draft 