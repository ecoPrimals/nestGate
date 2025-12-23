#!/bin/bash
# ==============================================================================
# Model Storage and Versioning System
# ==============================================================================
#
# Store ML models with automatic versioning, metadata tracking, and ZFS snapshots
#
# Usage:
#   ./store_model.sh [OPTIONS]
#
# Options:
#   --model PATH         Path to model file
#   --primal PRIMAL      Primal name (beardog, squirrel, toadstool, songbird)
#   --name NAME          Model name (default: filename)
#   --version VERSION    Version (default: auto-increment)
#   --metadata FILE      Path to metadata JSON file
#   --description TEXT   Model description
#   --tags TAGS          Comma-separated tags
#   --auto-snapshot      Create ZFS snapshot (default: true)
#   --help               Show this help
#
# Examples:
#   ./store_model.sh --model hsm_v2.safetensors --primal beardog
#   ./store_model.sh --model llm_base.gguf --primal squirrel --version 2.0.0
#
# ==============================================================================

set -euo pipefail

# Configuration
BASE_DIR="/nestgate_data/models"
METADATA_DIR="/nestgate_data/metadata/models"
AUTO_SNAPSHOT=true

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${BLUE}▶ $1${NC}"; }
log_success() { echo -e "${GREEN}✓ $1${NC}"; }
log_warn() { echo -e "${YELLOW}⚠ $1${NC}"; }

# Parse arguments
MODEL_PATH=""
PRIMAL=""
NAME=""
VERSION=""
METADATA_FILE=""
DESCRIPTION=""
TAGS=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --model) MODEL_PATH="$2"; shift 2 ;;
        --primal) PRIMAL="$2"; shift 2 ;;
        --name) NAME="$2"; shift 2 ;;
        --version) VERSION="$2"; shift 2 ;;
        --metadata) METADATA_FILE="$2"; shift 2 ;;
        --description) DESCRIPTION="$2"; shift 2 ;;
        --tags) TAGS="$2"; shift 2 ;;
        --auto-snapshot) AUTO_SNAPSHOT="$2"; shift 2 ;;
        --help) sed -n '/^# Usage:/,/^# ===/p' "$0" | sed 's/^# //' | head -n -1; exit 0 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Validation
[ -z "$MODEL_PATH" ] && { echo "Error: --model required"; exit 1; }
[ -z "$PRIMAL" ] && { echo "Error: --primal required"; exit 1; }
[ ! -f "$MODEL_PATH" ] && { echo "Error: Model file not found: $MODEL_PATH"; exit 1; }

# Default name from filename
[ -z "$NAME" ] && NAME=$(basename "$MODEL_PATH" | sed 's/\.[^.]*$//')

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║      🤖 MODEL STORAGE SYSTEM                              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

log_info "Storing model for $PRIMAL..."

# Create directories
PRIMAL_DIR="$BASE_DIR/$PRIMAL"
MODEL_DIR="$PRIMAL_DIR/$NAME"
mkdir -p "$MODEL_DIR"
mkdir -p "$METADATA_DIR/$PRIMAL"

# Auto-increment version if not specified
if [ -z "$VERSION" ]; then
    LATEST_VERSION=$(ls -1 "$MODEL_DIR" 2>/dev/null | grep -E '^v[0-9]+\.[0-9]+\.[0-9]+$' | sort -V | tail -1 || echo "v0.0.0")
    if [ "$LATEST_VERSION" = "v0.0.0" ]; then
        VERSION="v1.0.0"
    else
        # Increment patch version
        MAJOR=$(echo "$LATEST_VERSION" | cut -d'.' -f1 | sed 's/v//')
        MINOR=$(echo "$LATEST_VERSION" | cut -d'.' -f2)
        PATCH=$(echo "$LATEST_VERSION" | cut -d'.' -f3)
        VERSION="v${MAJOR}.${MINOR}.$((PATCH + 1))"
    fi
    log_info "Auto-incremented version: $VERSION"
fi

VERSION_DIR="$MODEL_DIR/$VERSION"
mkdir -p "$VERSION_DIR"

# Copy model
MODEL_FILENAME=$(basename "$MODEL_PATH")
DEST_PATH="$VERSION_DIR/$MODEL_FILENAME"

log_info "Copying model file..."
cp "$MODEL_PATH" "$DEST_PATH"

MODEL_SIZE=$(stat -f%z "$DEST_PATH" 2>/dev/null || stat -c%s "$DEST_PATH")
MODEL_SIZE_MB=$((MODEL_SIZE / 1024 / 1024))

log_success "Model stored ($MODEL_SIZE_MB MB)"

# Generate checksum
log_info "Generating checksum..."
CHECKSUM=$(sha256sum "$DEST_PATH" | awk '{print $1}')
echo "$CHECKSUM" > "$VERSION_DIR/checksum.txt"

# Create metadata
log_info "Creating metadata..."

TIMESTAMP=$(date -u +%Y-%m-%dT%H:%M:%SZ)

cat > "$VERSION_DIR/metadata.json" <<EOF
{
  "model_id": "${PRIMAL}_${NAME}_${VERSION}",
  "primal": "${PRIMAL}",
  "name": "${NAME}",
  "version": "${VERSION}",
  "created": "${TIMESTAMP}",
  "size_bytes": ${MODEL_SIZE},
  "size_mb": ${MODEL_SIZE_MB},
  "filename": "${MODEL_FILENAME}",
  "checksum_sha256": "${CHECKSUM}",
  "description": "${DESCRIPTION}",
  "tags": [$(echo "$TAGS" | sed 's/,/", "/g' | sed 's/^/"/' | sed 's/$/"/')]
}
EOF

# Copy user-provided metadata if available
if [ -n "$METADATA_FILE" ] && [ -f "$METADATA_FILE" ]; then
    log_info "Copying user metadata..."
    cp "$METADATA_FILE" "$VERSION_DIR/user_metadata.json"
fi

# Create ZFS snapshot if enabled
if [ "$AUTO_SNAPSHOT" = true ]; then
    log_info "Creating ZFS snapshot..."
    SNAPSHOT_NAME="${PRIMAL}_${NAME}_${VERSION}_$(date +%Y%m%d_%H%M%S)"
    
    # Check if on ZFS
    if df -T "$VERSION_DIR" 2>/dev/null | grep -q zfs; then
        DATASET=$(df "$VERSION_DIR" | tail -1 | awk '{print $1}')
        zfs snapshot "${DATASET}@${SNAPSHOT_NAME}" 2>/dev/null || log_warn "ZFS snapshot failed (may need sudo)"
    else
        log_warn "Not on ZFS filesystem, skipping snapshot"
    fi
fi

# Update model registry
REGISTRY_FILE="$METADATA_DIR/$PRIMAL/registry.json"

if [ ! -f "$REGISTRY_FILE" ]; then
    echo "{\"models\": []}" > "$REGISTRY_FILE"
fi

# Add to registry (simplified - in production use jq)
log_info "Updating model registry..."

cat > "$METADATA_DIR/$PRIMAL/${NAME}_versions.json" <<EOF
{
  "name": "${NAME}",
  "latest_version": "${VERSION}",
  "versions": ["${VERSION}"],
  "last_updated": "${TIMESTAMP}"
}
EOF

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Model Stored Successfully! ✨"
echo "════════════════════════════════════════════════════════════"
echo ""
echo "Model Details:"
echo "  Primal:      $PRIMAL"
echo "  Name:        $NAME"
echo "  Version:     $VERSION"
echo "  Size:        ${MODEL_SIZE_MB} MB"
echo "  Checksum:    ${CHECKSUM:0:16}..."
echo ""
echo "Locations:"
echo "  Model:       $DEST_PATH"
echo "  Metadata:    $VERSION_DIR/metadata.json"
echo "  Registry:    $METADATA_DIR/$PRIMAL/"
echo ""
echo "Retrieve with:"
echo "  ./get_model.sh --primal $PRIMAL --name $NAME --version $VERSION"
echo ""

exit 0

