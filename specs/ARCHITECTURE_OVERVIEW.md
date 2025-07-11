---
title: NestGate Universal Primal Storage Architecture Overview
description: Universal primal storage system architecture with capability-based discovery
version: 3.0.0
date: 2025-01-26
---

# NestGate Universal Primal Storage Architecture Overview

This document provides an overview of the NestGate Universal Primal Storage system architecture.

## **Universal Primal Architectural Philosophy**

NestGate follows the **Universal Primal Architecture** pattern used by beardog, squirrel, and songbird:

1. **Universal Interfaces**: Works with any primal ecosystem using capability-based discovery
2. **Auto-Discovery**: Automatically finds available primals on the network
3. **Capability-Based**: Dynamic feature negotiation rather than fixed interfaces
4. **Future-Proof**: New primals integrate without NestGate code changes
5. **Agnostic Design**: No hardcoded dependencies on specific primal ecosystems

## System Components

The NestGate Universal Primal system consists of these key components:

1. **Universal Primal Interface** - Core interface for any primal communication
2. **Auto-Discovery System** - Network scanning and service discovery
3. **Capability Negotiation** - Dynamic feature detection and agreement
4. **Storage Primal Provider** - NestGate's ZFS-based storage implementation
5. **Configuration System** - TOML-based universal configuration
6. **Health Monitoring** - Real-time primal health and performance tracking

## Universal Primal Architecture Diagram

```mermaid
---
title: NestGate Universal Primal Architecture
---
flowchart TB
    subgraph "Any Primal Ecosystem"
        BEARDOG[BearDog Security]
        SQUIRREL[Squirrel AI]
        SONGBIRD[Songbird Distribution]
        TOADSTOOL[ToadStool Compute]
        CUSTOM[Custom Primals]
    end

    subgraph "NestGate Universal Primal"
        DISCOVERY[Auto-Discovery System]
        INTERFACE[Universal Primal Interface]
        STORAGE[Storage Primal Provider]
        CONFIG[Configuration System]
        HEALTH[Health Monitoring]
    end

    subgraph "NestGate Core"
        ZFS[ZFS Manager]
        TIERS[Tiered Storage]
        MONITOR[Performance Monitor]
        API[REST API]
    end

    subgraph "Storage Layer"
        HOT[Hot Tier<br/>NVMe Cache]
        WARM[Warm Tier<br/>Primary ZFS]
        COLD[Cold Tier<br/>Archive Storage]
    end

    %% Universal Discovery
    DISCOVERY --> BEARDOG
    DISCOVERY --> SQUIRREL
    DISCOVERY --> SONGBIRD
    DISCOVERY --> TOADSTOOL
    DISCOVERY --> CUSTOM

    %% Capability-Based Communication
    INTERFACE <--> BEARDOG
    INTERFACE <--> SQUIRREL
    INTERFACE <--> SONGBIRD
    INTERFACE <--> TOADSTOOL
    INTERFACE <--> CUSTOM

    %% NestGate Internal
    INTERFACE --> STORAGE
    STORAGE --> CONFIG
    STORAGE --> HEALTH
    STORAGE --> ZFS
    ZFS --> TIERS
    TIERS --> MONITOR
    MONITOR --> API

    %% Storage Tiers
    ZFS --> HOT
    ZFS --> WARM
    ZFS --> COLD

    %% Health Monitoring
    HEALTH -.-> BEARDOG
    HEALTH -.-> SQUIRREL
    HEALTH -.-> SONGBIRD
    HEALTH -.-> TOADSTOOL
    HEALTH -.-> CUSTOM

    %% Styling
    classDef universal fill:#e1f5fe,stroke:#01579b,stroke-width:3px
    classDef primal fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef storage fill:#e8f5e8,stroke:#1b5e20,stroke-width:2px
    classDef core fill:#fff3e0,stroke:#e65100,stroke-width:2px

    class DISCOVERY,INTERFACE,STORAGE,CONFIG,HEALTH universal
    class BEARDOG,SQUIRREL,SONGBIRD,TOADSTOOL,CUSTOM primal
    class HOT,WARM,COLD storage
    class ZFS,TIERS,MONITOR,API core
```

## Universal Primal Communication Flow

### Auto-Discovery Process

```mermaid
---
title: Universal Primal Auto-Discovery Flow
---
sequenceDiagram
    participant N as NestGate
    participant D as Discovery System
    participant P as Available Primal
    participant C as Capability Negotiation

    N->>D: Start auto-discovery
    D->>P: Network scan (mDNS, service registry, etc.)
    P->>D: Announce capabilities
    D->>C: Initiate capability negotiation
    C->>P: Request capability details
    P->>C: Provide capability manifest
    C->>N: Register discovered primal
    N->>P: Establish communication channel

    Note over N,P: Future-proof: New primals<br/>integrate automatically
```

### Capability-Based Request/Response

```mermaid
---
title: Universal Primal Request/Response Flow
---
sequenceDiagram
    participant C as Client
    participant N as NestGate
    participant P as Any Primal
    participant S as Storage Layer

    C->>N: Storage request
    N->>P: Check capabilities
    P->>N: Capability confirmation
    N->>S: Execute storage operation
    S->>N: Operation result
    N->>P: Notify primal (if needed)
    P->>N: Acknowledgment
    N->>C: Response with results

    Note over N,P: Works with any primal<br/>No hardcoded dependencies
```

## Universal Benefits

### 1. **Future-Proof Design**
- New primals integrate automatically without code changes
- Capability-based discovery handles unknown primals gracefully
- Universal interface adapts to any primal ecosystem

### 2. **Zero Configuration**
- Auto-discovery eliminates manual configuration
- Capability negotiation handles feature detection
- Environment variable overrides for specific deployments

### 3. **Composable Architecture**
- Multiple primals can be combined for complex workflows
- Each primal contributes its unique capabilities
- NestGate orchestrates storage across all primals

### 4. **Production Ready**
- Comprehensive health monitoring
- Performance metrics for all primal interactions
- Audit logging and security built-in

## Configuration Architecture

### Universal Configuration Structure
```toml
[nestgate]
server.host = "0.0.0.0"
server.port = 8080
storage.pool_name = "nestpool"

[primal_ecosystem]
auto_discovery = true
discovery_timeout = 30
health_check_interval = 60

[integrations.beardog]
security_requests = true
encryption_level = "aes-256-gcm"

[integrations.squirrel]
ai_data_requests = true
vector_storage = true

[integrations.songbird]
network_storage = true
geo_distribution = true
```

## Universal Primal Capabilities

### Core Capabilities
- **Storage**: ZFS-based tiered storage management
- **Security**: Encryption, access control, audit trails
- **Network**: Multi-protocol access (NFS, SMB, iSCSI, S3)
- **AI**: Vector storage, model storage, training data
- **Custom**: User-defined capabilities for extensibility

### Dynamic Capability Detection
- Each primal announces its capabilities
- NestGate negotiates feature compatibility
- Runtime feature enablement based on available primals
- Graceful degradation when primals are unavailable

This architecture ensures NestGate remains agnostic, universal, and future-proof while providing enterprise-grade storage management capabilities. 