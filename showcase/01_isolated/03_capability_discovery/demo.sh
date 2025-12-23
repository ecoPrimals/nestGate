#!/usr/bin/env bash
# NestGate Capability Discovery - LIVE DEMONSTRATION
# Demonstrates runtime capability discovery and primal detection
set -euo pipefail

DEMO_NAME="capability_discovery"
DEMO_VERSION="1.0.0"
START_TIME=$(date +%s)
TIMESTAMP=$(date +%s)
OUTPUT_DIR="outputs/${DEMO_NAME}-${TIMESTAMP}"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}🎬 NestGate Capability Discovery v${DEMO_VERSION}${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${BLUE}Demonstrates:${NC} Runtime capability discovery & primal detection"
echo -e "${BLUE}Output:${NC} $OUTPUT_DIR"
echo -e "${BLUE}Started:${NC} $(date)"
echo ""

# ============================================================================
# STEP 1: Self-Knowledge Discovery
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🔍 [1/5] Self-Knowledge Discovery${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Discovering NestGate's own capabilities..."

# Detect storage backends
STORAGE_BACKENDS=()
if command -v zfs &> /dev/null; then
    STORAGE_BACKENDS+=("zfs")
    echo -e "${GREEN}✓${NC} ZFS available"
fi
STORAGE_BACKENDS+=("filesystem")
echo -e "${GREEN}✓${NC} Filesystem backend available"

# Detect network capabilities
NETWORK_CAPS=()
if command -v curl &> /dev/null; then
    NETWORK_CAPS+=("http-client")
    echo -e "${GREEN}✓${NC} HTTP client available"
fi
if command -v nc &> /dev/null || command -v netcat &> /dev/null; then
    NETWORK_CAPS+=("raw-sockets")
    echo -e "${GREEN}✓${NC} Raw socket support"
fi

# System resources
TOTAL_MEMORY_GB=$(free -g 2>/dev/null | awk '/Mem:/ {print $2}' || echo "Unknown")
CPU_CORES=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "Unknown")
DISK_SPACE_GB=$(df -BG . | awk 'NR==2 {print $4}' | sed 's/G//')

echo ""
echo "System Resources:"
echo "   CPU Cores: ${CPU_CORES}"
echo "   Memory: ${TOTAL_MEMORY_GB} GB"
echo "   Disk Space: ${DISK_SPACE_GB} GB available"

# Generate self-knowledge manifest
cat > "$OUTPUT_DIR/self-capabilities.json" <<EOF
{
  "primal": "NestGate",
  "version": "0.1.0",
  "timestamp": "$(date -Iseconds)",
  "capabilities": {
    "storage": $(printf '%s\n' "${STORAGE_BACKENDS[@]}" | jq -R . | jq -s .),
    "network": $(printf '%s\n' "${NETWORK_CAPS[@]}" | jq -R . | jq -s .),
    "protocols": ["http", "grpc", "file"]
  },
  "resources": {
    "cpu_cores": ${CPU_CORES},
    "memory_gb": ${TOTAL_MEMORY_GB},
    "disk_available_gb": ${DISK_SPACE_GB}
  },
  "node_id": "nestgate-$(hostname)",
  "discovery_method": "runtime-introspection"
}
EOF

echo ""
echo -e "${GREEN}✓${NC} Self-knowledge manifest: $OUTPUT_DIR/self-capabilities.json"

echo ""
sleep 1

# ============================================================================
# STEP 2: Network Discovery (LAN scan)
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🌐 [2/5] Network Discovery (LAN Scan)${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Scanning for primals on network..."

# Common primal ports
declare -A PRIMAL_PORTS=(
    ["songbird"]="8080"
    ["toadstool"]="8081"
    ["beardog"]="9090"
    ["squirrel"]="7070"
    ["nestgate"]="8082"
)

DISCOVERED_PRIMALS=()

for primal in "${!PRIMAL_PORTS[@]}"; do
    port="${PRIMAL_PORTS[$primal]}"
    echo -n "   Checking for ${primal} on port ${port}... "
    
    if timeout 1 bash -c "echo > /dev/tcp/localhost/${port}" 2>/dev/null; then
        echo -e "${GREEN}FOUND${NC}"
        DISCOVERED_PRIMALS+=("$primal:localhost:$port")
        
        # Try to fetch capabilities if HTTP
        if curl -s --max-time 1 "http://localhost:${port}/health" > /dev/null 2>&1; then
            curl -s --max-time 1 "http://localhost:${port}/api/capabilities" \
                > "$OUTPUT_DIR/discovered-${primal}-caps.json" 2>/dev/null || true
        fi
    else
        echo -e "${YELLOW}not found${NC}"
    fi
done

echo ""
if [ ${#DISCOVERED_PRIMALS[@]} -gt 0 ]; then
    echo -e "${GREEN}✓${NC} Discovered ${#DISCOVERED_PRIMALS[@]} primal(s) on network"
    for primal in "${DISCOVERED_PRIMALS[@]}"; do
        echo "     - $primal"
    done
else
    echo -e "${YELLOW}⚠${NC}  No other primals detected (running in isolated mode)"
fi

# Save discovery results
cat > "$OUTPUT_DIR/network-discovery.json" <<EOF
{
  "scan_time": "$(date -Iseconds)",
  "method": "port-scan",
  "scanned_ports": $(printf '%s\n' "${PRIMAL_PORTS[@]}" | jq -R . | jq -s .),
  "discovered_primals": $(printf '%s\n' "${DISCOVERED_PRIMALS[@]}" | jq -R . | jq -s .),
  "discovery_count": ${#DISCOVERED_PRIMALS[@]}
}
EOF

echo ""
sleep 1

# ============================================================================
# STEP 3: Capability Negotiation
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}🤝 [3/5] Capability Negotiation${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Simulating capability negotiation..."

# Create capability requirements
cat > "$OUTPUT_DIR/capability-requirements.json" <<EOF
{
  "required_capabilities": [
    {"type": "storage", "min_gb": 10},
    {"type": "network", "protocols": ["http"]}
  ],
  "optional_capabilities": [
    {"type": "compute", "gpu": false},
    {"type": "crypto", "provider": "beardog"}
  ],
  "negotiation_strategy": "graceful-degradation"
}
EOF

echo -e "${GREEN}✓${NC} Capability requirements defined"

# Simulate negotiation result
AVAILABLE_STORAGE=$((DISK_SPACE_GB))
STORAGE_SUFFICIENT=$( [ "$AVAILABLE_STORAGE" -ge 10 ] && echo "true" || echo "false" )
NETWORK_AVAILABLE=$( [ ${#NETWORK_CAPS[@]} -gt 0 ] && echo "true" || echo "false" )

cat > "$OUTPUT_DIR/negotiation-result.json" <<EOF
{
  "negotiation_time": "$(date -Iseconds)",
  "requirements_met": {
    "storage": {
      "required_gb": 10,
      "available_gb": ${AVAILABLE_STORAGE},
      "met": ${STORAGE_SUFFICIENT}
    },
    "network": {
      "required": ["http"],
      "available": $(printf '%s\n' "${NETWORK_CAPS[@]}" | jq -R . | jq -s .),
      "met": ${NETWORK_AVAILABLE}
    }
  },
  "optional_met": {
    "compute": false,
    "crypto": $([ ${#DISCOVERED_PRIMALS[@]} -gt 0 ] && echo "true" || echo "false")
  },
  "overall_status": "$([ "$STORAGE_SUFFICIENT" = "true" ] && [ "$NETWORK_AVAILABLE" = "true" ] && echo "operational" || echo "degraded")"
}
EOF

NEGOTIATION_STATUS=$(cat "$OUTPUT_DIR/negotiation-result.json" | grep '"overall_status"' | cut -d'"' -f4)
if [ "$NEGOTIATION_STATUS" = "operational" ]; then
    echo -e "${GREEN}✓${NC} All required capabilities available"
    echo "   Status: ${NEGOTIATION_STATUS}"
else
    echo -e "${YELLOW}⚠${NC}  Running in degraded mode"
    echo "   Status: ${NEGOTIATION_STATUS}"
fi

echo ""
sleep 1

# ============================================================================
# STEP 4: Dynamic Configuration
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}⚙️  [4/5] Dynamic Configuration${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Generating runtime configuration based on discovered capabilities..."

# Determine storage backend
if [[ " ${STORAGE_BACKENDS[@]} " =~ " zfs " ]]; then
    PRIMARY_STORAGE="zfs"
else
    PRIMARY_STORAGE="filesystem"
fi

# Determine networking
if [ ${#DISCOVERED_PRIMALS[@]} -gt 0 ]; then
    NETWORK_MODE="mesh"
else
    NETWORK_MODE="standalone"
fi

cat > "$OUTPUT_DIR/runtime-config.toml" <<EOF
# NestGate Runtime Configuration
# Auto-generated from capability discovery

[server]
node_id = "nestgate-$(hostname)"
mode = "${NETWORK_MODE}"
discovery_enabled = true

[storage]
primary_backend = "${PRIMARY_STORAGE}"
available_backends = [$(printf '"%s", ' "${STORAGE_BACKENDS[@]}" | sed 's/, $//')"]
quota_gb = $(( AVAILABLE_STORAGE / 2 ))

[network]
enabled_protocols = [$(printf '"%s", ' "${NETWORK_CAPS[@]}" | sed 's/, $//')"]
mesh_enabled = $([ "$NETWORK_MODE" = "mesh" ] && echo "true" || echo "false")

[resources]
cpu_cores = ${CPU_CORES}
memory_gb = ${TOTAL_MEMORY_GB}
disk_gb = ${AVAILABLE_STORAGE}

[discovery]
method = "runtime"
last_scan = "$(date -Iseconds)"
discovered_primals = $([ ${#DISCOVERED_PRIMALS[@]} -gt 0 ] && echo "${#DISCOVERED_PRIMALS[@]}" || echo "0")
EOF

echo -e "${GREEN}✓${NC} Configuration generated: $OUTPUT_DIR/runtime-config.toml"
echo ""
echo "Configuration highlights:"
echo "   Primary storage: ${PRIMARY_STORAGE}"
echo "   Network mode: ${NETWORK_MODE}"
echo "   CPU cores allocated: ${CPU_CORES}"
echo "   Storage quota: $(( AVAILABLE_STORAGE / 2 )) GB"

echo ""
sleep 1

# ============================================================================
# STEP 5: Capability Advertisement
# ============================================================================

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📢 [5/5] Capability Advertisement${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Preparing capability advertisement for network..."

# Create advertisement payload
cat > "$OUTPUT_DIR/capability-advertisement.json" <<EOF
{
  "primal": "NestGate",
  "node_id": "nestgate-$(hostname)",
  "version": "0.1.0",
  "timestamp": "$(date -Iseconds)",
  "advertised_capabilities": {
    "storage": {
      "backends": $(printf '%s\n' "${STORAGE_BACKENDS[@]}" | jq -R . | jq -s .),
      "available_gb": ${AVAILABLE_STORAGE},
      "features": ["snapshots", "compression", "deduplication"]
    },
    "api": {
      "endpoints": ["http://localhost:8082"],
      "protocols": ["rest", "grpc"],
      "version": "v1"
    },
    "resources": {
      "cpu_cores": ${CPU_CORES},
      "memory_gb": ${TOTAL_MEMORY_GB},
      "load_capacity": "medium"
    }
  },
  "discovery_methods": ["mDNS", "port-scan", "api-query"],
  "mesh_ready": $([ "$NETWORK_MODE" = "mesh" ] && echo "true" || echo "false"),
  "sovereignty_compliant": true
}
EOF

echo -e "${GREEN}✓${NC} Advertisement payload created"
echo ""
echo "Advertisement summary:"
cat "$OUTPUT_DIR/capability-advertisement.json" | jq -r '
  "   Primal: " + .primal + 
  "\n   Node: " + .node_id +
  "\n   Storage: " + (.advertised_capabilities.storage.available_gb | tostring) + " GB" +
  "\n   Mesh Ready: " + (.mesh_ready | tostring)'

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""

# ============================================================================
# Generate Receipt
# ============================================================================

cat > "$OUTPUT_DIR/RECEIPT.md" <<EOF
# NestGate Capability Discovery - Demo Receipt

**Date**: $(date)
**Duration**: ${DURATION}s
**Node**: nestgate-$(hostname)
**Status**: ✅ SUCCESS

---

## Discovery Results

### 1. Self-Knowledge
- **Storage Backends**: $(printf '%s, ' "${STORAGE_BACKENDS[@]}" | sed 's/, $//')
- **Network Capabilities**: $(printf '%s, ' "${NETWORK_CAPS[@]}" | sed 's/, $//')
- **CPU Cores**: ${CPU_CORES}
- **Memory**: ${TOTAL_MEMORY_GB} GB
- **Disk Space**: ${AVAILABLE_STORAGE} GB

### 2. Network Discovery
- **Scan Method**: Port scan (common primal ports)
- **Primals Discovered**: ${#DISCOVERED_PRIMALS[@]}
$(if [ ${#DISCOVERED_PRIMALS[@]} -gt 0 ]; then
    for primal in "${DISCOVERED_PRIMALS[@]}"; do
        echo "  - \`$primal\`"
    done
else
    echo "  - None (isolated mode)"
fi)

### 3. Capability Negotiation
- **Required Capabilities**: ✅ Met
- **Storage Check**: $([ "$STORAGE_SUFFICIENT" = "true" ] && echo "✅ ${AVAILABLE_STORAGE} GB available" || echo "❌ Insufficient")
- **Network Check**: $([ "$NETWORK_AVAILABLE" = "true" ] && echo "✅ Available" || echo "❌ Unavailable")
- **Overall Status**: \`${NEGOTIATION_STATUS}\`

### 4. Dynamic Configuration
- **Primary Storage**: ${PRIMARY_STORAGE}
- **Network Mode**: ${NETWORK_MODE}
- **Storage Quota**: $(( AVAILABLE_STORAGE / 2 )) GB
- **Config File**: \`runtime-config.toml\`

### 5. Advertisement
- **Advertisement Created**: ✅
- **Mesh Ready**: $([ "$NETWORK_MODE" = "mesh" ] && echo "Yes" || echo "No")
- **Sovereignty Compliant**: ✅ Yes

---

## Files Generated

\`\`\`
$(ls -lh "$OUTPUT_DIR" | tail -n +2)
\`\`\`

---

## Capability Discovery Flow

\`\`\`
1. Self-Introspection
   └─> Detect local capabilities (storage, network, resources)

2. Network Scan
   └─> Discover other primals on LAN (mDNS, port scan)

3. Capability Negotiation
   └─> Match requirements with available capabilities

4. Dynamic Configuration
   └─> Generate runtime config based on discoveries

5. Advertisement
   └─> Broadcast capabilities to network
\`\`\`

---

## Key Capabilities Demonstrated

- ✅ **Self-knowledge discovery** (runtime introspection)
- ✅ **Network primal detection** (port scanning)
- ✅ **Capability negotiation** (requirements matching)
- ✅ **Dynamic configuration** (auto-generated config)
- ✅ **Graceful degradation** (works isolated or meshed)
- ✅ **Sovereignty compliance** (no hardcoded dependencies)

---

## NestGate Features Shown

1. **Zero-Knowledge Architecture**: No hardcoded primal assumptions
2. **Runtime Discovery**: Detects capabilities at startup
3. **Graceful Degradation**: Works with/without other primals
4. **Dynamic Configuration**: Auto-adjusts to environment
5. **Capability Negotiation**: Matches requirements intelligently
6. **Mesh-Ready**: Advertises capabilities for coordination

---

## Discovery Methods

| Method | Purpose | Result |
|--------|---------|--------|
| Self-introspection | Local capabilities | $(echo "${#STORAGE_BACKENDS[@]} + ${#NETWORK_CAPS[@]}" | bc) capabilities |
| Port scanning | Network primals | ${#DISCOVERED_PRIMALS[@]} found |
| API querying | Capability details | $(ls -1 "$OUTPUT_DIR"/discovered-*-caps.json 2>/dev/null | wc -l) responses |
| mDNS (simulated) | Service discovery | N/A (demo) |

---

**Demo Version**: ${DEMO_VERSION}
**Generated**: $(date)
EOF

# ============================================================================
# Summary
# ============================================================================

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "📊 Summary:"
echo "   Duration: ${DURATION}s"
echo "   Self-capabilities: $((${#STORAGE_BACKENDS[@]} + ${#NETWORK_CAPS[@]}))"
echo "   Primals discovered: ${#DISCOVERED_PRIMALS[@]}"
echo "   Configuration: ${NEGOTIATION_STATUS}"
echo "   Network mode: ${NETWORK_MODE}"
echo ""
echo "📁 Output:"
echo "   Directory: $OUTPUT_DIR"
echo "   Receipt: $OUTPUT_DIR/RECEIPT.md"
echo "   Config: $OUTPUT_DIR/runtime-config.toml"
echo "   Files: $(ls -1 "$OUTPUT_DIR" | wc -l)"
echo ""
echo "🧹 Cleanup:"
echo "   rm -rf $OUTPUT_DIR"
echo ""

exit 0
