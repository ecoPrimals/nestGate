# Songbird Workflows for NestGate Federation

This directory contains **Songbird workflow definitions** for orchestrating NestGate storage operations across a federated tower network.

## 🌐 Dynamic Federation (Network Effect)

**Key Innovation**: NO hardcoded topology! 

Towers self-register with Songbird's service registry and are discovered dynamically based on capabilities.

### How It Works

1. **NestGate towers** register on startup with capabilities (e.g., "cold-storage", "replication")
2. **Songbird** maintains a live registry of all available services
3. **Workflows** query the registry and select towers based on capabilities, not hardcoded names
4. **Network effect**: Federation adapts automatically as nodes join/leave

### Example Topology

Your current federation might look like:

| Tower | Capabilities | Role | Status |
|-------|-------------|------|--------|
| **Westgate** | storage, cold-storage, large-scale | Primary | ✅ Online |
| **Northgate** | storage, failover | Failover | ❌ Down |
| **Stradgate** | storage, replication, backup | Backup | ✅ Online |

But the workflow doesn't hardcode these names - it discovers them at runtime!

## 📋 Available Workflows

### 1. Data Federation (`data-federation.yaml`)

**Version**: 2.0.0 (Dynamic Discovery)

**Purpose**: Orchestrate data storage, replication, and sharding across the tower federation using **dynamic service discovery**.

**Stages**:
1. **Discover Towers** - Query Songbird service registry for available NestGate nodes
2. **Store to Primary** - Dynamically select tower with "cold-storage" capability
3. **Replicate to Backup** - Dynamically select tower with "replication" capability
4. **Replicate to Additional** - Use any remaining storage nodes for extra redundancy
5. **Verify Federation** - Health check across all discovered towers

**Features**:
- ✅ **Dynamic service discovery** (no hardcoded topology!)
- ✅ **Capability-based selection** (choose by what towers can do)
- ✅ **Network effect** (works with 1 to N nodes)
- ✅ Automatic replication (2x minimum)
- ✅ Continues with 1+ healthy nodes
- ✅ Model-specific storage policies
- ✅ Compression and sharding
- ✅ Health monitoring and alerts
- ✅ Auto-adapts to node failures and additions

**Usage**:
```bash
# Via Songbird CLI (when implemented)
songbird workflow run data-federation.yaml \
  --input data='{"model": "llama-3-70b", "size_gb": 140}'

# Via API
curl -X POST http://localhost:8080/api/v1/workflows/execute \
  -H "Content-Type: application/json" \
  -d '{
    "workflow": "nestgate-data-federation",
    "input": {
      "data": {
        "model_name": "llama-3-70b",
        "size_gb": 140
      }
    }
  }'
```

## 🎯 Workflow Features

### Dynamic Tower Selection

The workflow uses **capability-based selection** (not hardcoded names):

```yaml
tower_selection:
  primary:
    prefer_capabilities: ["cold-storage", "large-scale"]
    fallback: "highest-capacity"
  
  backup:
    prefer_capabilities: ["replication", "backup"]
    fallback: "any-available"
```

**Example**:
- Primary: Discovers Westgate (has "cold-storage" capability)
- Backup: Discovers Stradgate (has "replication" capability)
- Additional: Any other nodes with "storage" capability

### Failover Strategy

The workflow uses **capability-based failover**:

1. **Primary** selected by "cold-storage" capability
   - If down → backup promoted to primary
2. **Backup** selected by "replication" capability
   - Always replicates from primary
3. **Additional** selected from remaining storage nodes
   - Provides extra copies if available
   - Federation continues with any 1+ healthy node

### Data Policy

```yaml
replication_factor: 2  # Minimum copies
sharding: consistent-hash  # Distribution strategy
model_storage: westgate  # Large models → cold storage
auto_replicate: true  # Automatic backup
compression: true  # Save space
```

### Health Monitoring

- **Health checks**: Every 30 seconds
- **Alerts**: Tower down, replication failed, primary down
- **Auto-recovery**: Retry failed operations
- **Metrics**: Tower health, replication lag, capacity, failover events

## 🔧 Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| Workflow YAML Definition | ✅ Complete | Ready for Songbird integration |
| Tower Health Checks | ✅ Working | Real HTTP calls to nodes |
| Multi-node Storage | ✅ Working | Demo script validated |
| Songbird Workflow Engine | 🚧 Pending | Requires Songbird workflow executor |
| Auto-failover | 🚧 Pending | Needs Songbird orchestration logic |
| Sharding Implementation | 🎯 Planned | Next phase |

## 🚀 Next Steps

To fully implement this workflow:

1. **Songbird Integration**:
   - Add workflow YAML parser to Songbird
   - Implement stage execution engine
   - Add variable substitution (`${input.data}`, etc.)

2. **NestGate Enhancements**:
   - Expose `/api/v1/data/store` endpoint
   - Implement sharding logic
   - Add replication receipts

3. **Federation Features**:
   - Erasure coding for data redundancy
   - Load balancing for read operations
   - Automatic node discovery and registration

4. **Monitoring**:
   - Metrics collection
   - Alert system
   - Dashboard for federation health

## 📖 Related Documentation

- [Live Integration Demo](../03-live-integration.sh)
- [Data Federation Demo](../05-data-federation.sh)
- [NestGate + Songbird Integration Plan](../../NESTGATE_SONGBIRD_INTEGRATION_PLAN_DEC_21_2025.md)
- [Ecosystem Integration Plan](../../../ECOSYSTEM_INTEGRATION_PLAN.md)

## 🎓 Learning Path

1. **Start Here**: Run `../05-data-federation.sh` to see the live demo
2. **Study**: Review this workflow YAML to understand orchestration
3. **Expand**: Contribute to Songbird's workflow engine
4. **Integrate**: Use these patterns in your own primal integrations

---

**Status**: ✅ WORKFLOW DEFINED - Ready for Songbird integration  
**Version**: 1.0.0  
**Last Updated**: December 21, 2025

