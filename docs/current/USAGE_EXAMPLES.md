# NestGate Usage Examples and Recipes

This document provides comprehensive examples and recipes for using NestGate in various scenarios.

## Table of Contents
1. [Basic Storage Operations](#basic-storage-operations)
2. [BYOB Workspace Management](#byob-workspace-management)
3. [AI-Powered Optimization](#ai-powered-optimization)
4. [Data Science Workflows](#data-science-workflows)
5. [Performance Monitoring](#performance-monitoring)
6. [Backup and Recovery](#backup-and-recovery)
7. [Integration Examples](#integration-examples)

---

## Basic Storage Operations

### Creating Your First ZFS Pool

```bash
# Create a simple single-device pool
curl -X POST http://localhost:8080/api/v1/pools \
  -H "Content-Type: application/json" \
  -d '{
    "name": "datapool",
    "devices": ["/dev/sdb"],
    "pool_type": "stripe",
    "properties": {
      "compression": "lz4",
      "atime": "off"
    }
  }'

# Create a mirrored pool for redundancy
curl -X POST http://localhost:8080/api/v1/pools \
  -H "Content-Type: application/json" \
  -d '{
    "name": "mirrorpool",
    "devices": ["/dev/sdc", "/dev/sdd"],
    "pool_type": "mirror",
    "properties": {
      "compression": "gzip",
      "dedup": "on"
    }
  }'
```

### Dataset Management

```bash
# Create a dataset with quotas
curl -X POST http://localhost:8080/api/v1/datasets \
  -H "Content-Type: application/json" \
  -d '{
    "pool": "datapool",
    "name": "research-data",
    "properties": {
      "quota": "100G",
      "compression": "lz4",
      "recordsize": "1M"
    }
  }'

# Create a dataset optimized for databases
curl -X POST http://localhost:8080/api/v1/datasets \
  -H "Content-Type: application/json" \
  -d '{
    "pool": "datapool",
    "name": "databases",
    "properties": {
      "recordsize": "8K",
      "compression": "lz4",
      "primarycache": "metadata",
      "logbias": "throughput"
    }
  }'
```

---

## BYOB Workspace Management

### Development Environment Setup

```bash
# Create a development workspace
curl -X POST http://localhost:8080/api/v1/byob/workspaces \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ml-dev-environment",
    "description": "Machine learning development workspace",
    "storage_tier": "hot",
    "storage_gb": 500,
    "backup_frequency": "hourly",
    "features": {
      "gpu_support": true,
      "jupyter_enabled": true,
      "git_integration": true
    }
  }'

# Clone a template workspace
curl -X POST http://localhost:8080/api/v1/byob/workspaces/clone \
  -H "Content-Type: application/json" \
  -d '{
    "source_workspace": "ml-dev-environment",
    "target_name": "experiment-branch",
    "description": "Experimental branch for new features"
  }'
```

### Team Collaboration

```bash
# Create a shared team workspace
curl -X POST http://localhost:8080/api/v1/byob/workspaces \
  -H "Content-Type: application/json" \
  -d '{
    "name": "team-bioinformatics",
    "description": "Shared bioinformatics analysis workspace",
    "storage_tier": "warm",
    "storage_gb": 2000,
    "backup_frequency": "daily",
    "sharing": {
      "enabled": true,
      "access_level": "read-write",
      "users": ["alice", "bob", "charlie"]
    }
  }'

# Grant access to additional users
curl -X POST http://localhost:8080/api/v1/byob/workspaces/team-bioinformatics/access \
  -H "Content-Type: application/json" \
  -d '{
    "user": "david",
    "access_level": "read-only",
    "expiration": "2024-12-31"
  }'
```

---

## AI-Powered Optimization

### Universal AI Module Integration

```bash
# Check integrated AI modules
curl http://localhost:8080/api/v1/modules/status/ai

# Check AI module capabilities
curl http://localhost:8080/api/v1/modules/capabilities/ai

# Get integrated AI modules configuration
curl http://localhost:8080/api/v1/modules/config/ai
```

### Intelligent Storage Tiering

```bash
# Enable AI-powered tier migration with integrated AI modules
curl -X POST http://localhost:8080/api/v1/ai/tier-migration/enable \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": true,
    "hot_threshold": 0.9,
    "cold_threshold": 0.1,
    "analysis_window_days": 7,
    "ai_module_preference": "auto"
  }'

# Get AI recommendations for storage optimization
curl http://localhost:8080/api/v1/ai/recommendations?workspace=ml-dev-environment
```

### Predictive Maintenance

```bash
# Enable predictive maintenance alerts
curl -X POST http://localhost:8080/api/v1/ai/maintenance/enable \
  -H "Content-Type: application/json" \
  -d '{
    "enabled": true,
    "prediction_horizon_days": 30,
    "alert_threshold": 0.7
  }'

# Get health predictions
curl http://localhost:8080/api/v1/ai/health/predictions
```

---

## Data Science Workflows

### NCBI Data Integration

```bash
# Search for genomic data
curl -X GET "http://localhost:8080/api/v1/data-sources/ncbi/search" \
  -H "Content-Type: application/json" \
  -d '{
    "database": "genome",
    "query": "Homo sapiens[Organism]",
    "limit": 10
  }'

# Download and cache genomic data
curl -X POST http://localhost:8080/api/v1/data-sources/ncbi/download \
  -H "Content-Type: application/json" \
  -d '{
    "accession": "GCA_000001405.29",
    "workspace": "genomics-lab",
    "cache_tier": "warm"
  }'
```

### HuggingFace Model Integration

```bash
# List available models
curl http://localhost:8080/api/v1/data-sources/huggingface/models?task=text-generation

# Download and cache a model
curl -X POST http://localhost:8080/api/v1/data-sources/huggingface/download \
  -H "Content-Type: application/json" \
  -d '{
    "model_id": "microsoft/DialoGPT-medium",
    "workspace": "nlp-experiments",
    "cache_tier": "hot"
  }'
```

---

## Performance Monitoring

### Real-time Metrics

```bash
# Get comprehensive system metrics
curl http://localhost:8080/api/v1/metrics/system

# Get pool-specific performance
curl http://localhost:8080/api/v1/metrics/pools/datapool

# Get workspace performance metrics
curl http://localhost:8080/api/v1/metrics/workspaces/ml-dev-environment
```

### Historical Analysis

```bash
# Get performance history for the last 24 hours
curl "http://localhost:8080/api/v1/metrics/history?duration=24h&resolution=5m"

# Get tier migration history
curl "http://localhost:8080/api/v1/metrics/tier-migrations?workspace=ml-dev-environment"
```

---

## Backup and Recovery

### Automated Backup Strategies

```bash
# Configure automated backup policy
curl -X POST http://localhost:8080/api/v1/backup/policies \
  -H "Content-Type: application/json" \
  -d '{
    "name": "research-backup",
    "schedule": {
      "hourly": {"keep": 24},
      "daily": {"keep": 7},
      "weekly": {"keep": 4},
      "monthly": {"keep": 12}
    },
    "compression": "gzip",
    "verification": true
  }'

# Apply backup policy to workspace
curl -X POST http://localhost:8080/api/v1/byob/workspaces/genomics-lab/backup-policy \
  -H "Content-Type: application/json" \
  -d '{"policy": "research-backup"}'
```

### Point-in-Time Recovery

```bash
# List available snapshots
curl http://localhost:8080/api/v1/snapshots?workspace=genomics-lab

# Create manual snapshot
curl -X POST http://localhost:8080/api/v1/snapshots \
  -H "Content-Type: application/json" \
  -d '{
    "workspace": "genomics-lab",
    "name": "pre-experiment-snapshot",
    "description": "Before running mutation analysis"
  }'

# Restore from snapshot
curl -X POST http://localhost:8080/api/v1/restore \
  -H "Content-Type: application/json" \
  -d '{
    "workspace": "genomics-lab",
    "snapshot": "pre-experiment-snapshot",
    "restore_type": "in-place"
  }'
```

---

## Integration Examples

### Python Integration

```python
import requests
import json

class NestGateClient:
    def __init__(self, base_url="http://localhost:8080"):
        self.base_url = base_url
        self.session = requests.Session()
    
    def create_workspace(self, name, description, storage_gb=100):
        """Create a new BYOB workspace"""
        payload = {
            "name": name,
            "description": description,
            "storage_gb": storage_gb,
            "storage_tier": "warm",
            "backup_frequency": "daily"
        }
        
        response = self.session.post(
            f"{self.base_url}/api/v1/byob/workspaces",
            json=payload
        )
        return response.json()
    
    def get_metrics(self, workspace_name):
        """Get performance metrics for a workspace"""
        response = self.session.get(
            f"{self.base_url}/api/v1/metrics/workspaces/{workspace_name}"
        )
        return response.json()
    
    def backup_workspace(self, workspace_name):
        """Create a backup of the workspace"""
        response = self.session.post(
            f"{self.base_url}/api/v1/byob/workspaces/{workspace_name}/backup"
        )
        return response.json()

# Usage example
client = NestGateClient()

# Create a workspace for data science
workspace = client.create_workspace(
    name="protein-analysis",
    description="Protein structure analysis workspace",
    storage_gb=500
)

# Monitor performance
metrics = client.get_metrics("protein-analysis")
print(f"Storage utilization: {metrics['storage_utilization']}%")

# Create backup before major analysis
backup = client.backup_workspace("protein-analysis")
print(f"Backup created: {backup['snapshot_name']}")
```

### Rust Integration

```rust
use reqwest::Client;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let base_url = "http://localhost:8080";
    
    // Create a workspace
    let workspace_payload = json!({
        "name": "rust-analysis",
        "description": "Rust performance analysis workspace",
        "storage_gb": 200,
        "storage_tier": "hot",
        "backup_frequency": "hourly"
    });
    
    let response = client
        .post(&format!("{}/api/v1/byob/workspaces", base_url))
        .json(&workspace_payload)
        .send()
        .await?;
    
    println!("Workspace created: {}", response.status());
    
    // Get AI recommendations
    let recommendations = client
        .get(&format!("{}/api/v1/ai/recommendations", base_url))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    println!("AI recommendations: {:#}", recommendations);
    
    Ok(())
}
```

---

## Best Practices

### Storage Optimization

1. **Choose the right tier**: Use `hot` for frequently accessed data, `warm` for regular access, `cold` for archival
2. **Enable compression**: Use `lz4` for performance, `gzip` for better compression ratios
3. **Set appropriate record sizes**: Use `1M` for large files, `8K` for databases
4. **Monitor performance**: Regular checks prevent bottlenecks

### Backup Strategy

1. **Regular snapshots**: Use hourly snapshots for active development
2. **Tier-based retention**: Keep more recent backups in faster tiers
3. **Verify backups**: Enable verification to ensure backup integrity
4. **Test recovery**: Regularly test recovery procedures

### AI Integration

1. **Enable predictive features**: Use AI for proactive maintenance
2. **Monitor recommendations**: Review AI suggestions regularly
3. **Gradual adoption**: Start with non-critical workloads
4. **Feedback loops**: Provide feedback to improve AI accuracy

---

## Troubleshooting

### Common Issues

**Pool creation fails:**
```bash
# Check available devices
curl http://localhost:8080/api/v1/system/devices

# Verify ZFS utilities
which zpool zfs
```

**Workspace access denied:**
```bash
# Check permissions
curl http://localhost:8080/api/v1/byob/workspaces/my-workspace/permissions

# Verify user access
curl http://localhost:8080/api/v1/auth/user-info
```

**AI features not working:**
```bash
# Check AI service status
curl http://localhost:8080/api/v1/ai/status

# Verify AI module integration
curl http://localhost:8080/api/v1/modules/health/ai

# Check available AI module capabilities
curl http://localhost:8080/api/v1/modules/capabilities/ai

# Test AI module communication
curl http://localhost:8080/api/v1/modules/test/ai
```

### Getting Help

- Check the logs: `journalctl -u nestgate`
- API documentation: `http://localhost:8080/api/docs`
- System status: `http://localhost:8080/api/v1/system/status`
- Community support: GitHub Issues

---

*This document is continuously updated. For the latest examples and features, visit the [NestGate documentation](https://docs.nestgate.io).* 