---
title: MCP Protocol Buffer Definitions
description: Standard Protocol Buffer definitions for MCP implementation
version: 0.1.0
category: protocol
status: draft
validation_required: true
crossRefs:
  - protocol/mcp_interface.md
  - protocol/implementation.md
---

# MCP Protocol Buffer Definitions

## Machine Configuration (70%)

```protobuf
syntax = "proto3";

package mcp.v1;

// Core message types used by all MCP systems
message Message {
  Header header = 1;
  oneof body {
    Command command = 2;
    Response response = 3;
    Event event = 4;
    State state = 5;
  }
}

message Header {
  string system_id = 1;
  string protocol_version = 2;
  MessageType message_type = 3;
  repeated Capability capabilities = 4;
  map<string, string> metadata = 5;
}

enum MessageType {
  MESSAGE_TYPE_UNSPECIFIED = 0;
  MESSAGE_TYPE_COMMAND = 1;
  MESSAGE_TYPE_RESPONSE = 2;
  MESSAGE_TYPE_EVENT = 3;
  MESSAGE_TYPE_STATE = 4;
}

message Capability {
  string name = 1;
  string version = 2;
  repeated Parameter inputs = 3;
  repeated Parameter outputs = 4;
  map<string, Parameter> parameters = 5;
}

message Parameter {
  string name = 1;
  ParameterType type = 2;
  bool required = 3;
  string description = 4;
}

enum ParameterType {
  PARAMETER_TYPE_UNSPECIFIED = 0;
  PARAMETER_TYPE_STRING = 1;
  PARAMETER_TYPE_INT32 = 2;
  PARAMETER_TYPE_INT64 = 3;
  PARAMETER_TYPE_UINT32 = 4;
  PARAMETER_TYPE_UINT64 = 5;
  PARAMETER_TYPE_BOOL = 6;
  PARAMETER_TYPE_BYTES = 7;
  PARAMETER_TYPE_MAP = 8;
  PARAMETER_TYPE_ARRAY = 9;
}

// Command message type
message Command {
  string capability = 1;
  map<string, Value> parameters = 2;
  string correlation_id = 3;
}

// Response message type
message Response {
  string correlation_id = 1;
  Status status = 2;
  map<string, Value> results = 3;
  Error error = 4;
}

// Event message type
message Event {
  string event_type = 1;
  map<string, Value> data = 2;
  int64 timestamp = 3;
}

// State message type
message State {
  string state_type = 1;
  map<string, Value> data = 2;
  int64 version = 3;
}

// Common types
message Value {
  oneof value {
    string string_value = 1;
    int32 int32_value = 2;
    int64 int64_value = 3;
    uint32 uint32_value = 4;
    uint64 uint64_value = 5;
    bool bool_value = 6;
    bytes bytes_value = 7;
    MapValue map_value = 8;
    ArrayValue array_value = 9;
  }
}

message MapValue {
  map<string, Value> fields = 1;
}

message ArrayValue {
  repeated Value values = 1;
}

message Status {
  int32 code = 1;
  string message = 2;
  map<string, string> details = 3;
}

message Error {
  int32 code = 1;
  string message = 2;
  map<string, string> details = 3;
  repeated string context = 4;
}

// Service definitions
service MCP {
  // Bidirectional streaming for all message types
  rpc Stream(stream Message) returns (stream Message) {}
  
  // Unary RPC for simple commands
  rpc Execute(Command) returns (Response) {}
  
  // Server streaming for events
  rpc Subscribe(Command) returns (stream Event) {}
  
  // Client streaming for state updates
  rpc UpdateState(stream State) returns (Response) {}
}
```

## Technical Context (30%)

### Protocol Buffer Usage

1. Message Structure
   - Header contains system identification
   - Body uses oneof for different message types
   - All fields are properly typed
   - Optional fields marked appropriately

2. Service Definition
   - Bidirectional streaming for general communication
   - Unary RPC for simple commands
   - Server streaming for events
   - Client streaming for state updates

3. Type System
   - Strong typing for all values
   - Support for complex data structures
   - Extensible for future additions
   - Backward compatible

### Implementation Guidelines

1. Message Handling
   - Validate all messages
   - Handle all message types
   - Process headers correctly
   - Manage correlation IDs

2. Error Handling
   - Use standard error codes
   - Include detailed context
   - Handle all error cases
   - Provide recovery options

3. State Management
   - Track message versions
   - Handle state conflicts
   - Maintain consistency
   - Support rollback

## Technical Metadata
```yaml
metadata:
  category: "protocol"
  priority: "P0"
  owner: "protocol-team"
  review_required: true
  validation_level: "high"
``` 