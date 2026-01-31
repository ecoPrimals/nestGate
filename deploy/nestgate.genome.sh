#!/usr/bin/env bash
# nestgate.genome - Self-deploying genomeBin wrapper for NestGate
# Universal Storage Primal - Works everywhere!
# Version: 1.0.0
# Created: January 31, 2026

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PRIMAL_NAME="nestgate"
PRIMAL_VERSION="1.0.0"
PRIMAL_DESCRIPTION="Universal Storage & Persistence"

# ============================================================================
# Helper Functions
# ============================================================================

log() {
    echo -e "${GREEN}[NestGate]${NC} $*"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
    exit 1
}

# ============================================================================
# Architecture Detection
# ============================================================================

detect_arch() {
    local arch=$(uname -m)
    case "$arch" in
        x86_64|amd64)
            echo "x86_64"
            ;;
        aarch64|arm64)
            echo "aarch64"
            ;;
        armv7l|armv7)
            echo "armv7"
            ;;
        riscv64)
            echo "riscv64"
            ;;
        *)
            error "Unsupported architecture: $arch"
            ;;
    esac
}

# ============================================================================
# Platform Detection
# ============================================================================

detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    case "$os" in
        linux)
            # Check if Android
            if [[ -d /system/bin && -d /system/app ]]; then
                echo "android"
            else
                echo "linux"
            fi
            ;;
        darwin)
            echo "macos"
            ;;
        freebsd)
            echo "freebsd"
            ;;
        *)
            error "Unsupported platform: $os"
            ;;
    esac
}

# ============================================================================
# Binary Selection
# ============================================================================

select_binary() {
    local arch=$1
    local platform=$2
    
    case "${platform}-${arch}" in
        linux-x86_64)
            echo "${PRIMAL_NAME}-x86_64-unknown-linux-musl"
            ;;
        linux-aarch64)
            echo "${PRIMAL_NAME}-aarch64-unknown-linux-musl"
            ;;
        android-aarch64)
            echo "${PRIMAL_NAME}-aarch64-linux-android"
            ;;
        macos-x86_64)
            echo "${PRIMAL_NAME}-x86_64-apple-darwin"
            ;;
        macos-aarch64)
            echo "${PRIMAL_NAME}-aarch64-apple-darwin"
            ;;
        freebsd-x86_64)
            echo "${PRIMAL_NAME}-x86_64-unknown-freebsd"
            ;;
        *)
            error "No binary available for ${platform}-${arch}"
            ;;
    esac
}

# ============================================================================
# Installation Directory
# ============================================================================

install_dir() {
    local platform=$1
    
    case "$platform" in
        android)
            echo "/data/local/tmp/biomeos/${PRIMAL_NAME}"
            ;;
        linux|freebsd)
            # Check if we have root privileges
            if [[ $EUID -eq 0 ]]; then
                echo "/opt/biomeos/${PRIMAL_NAME}"
            else
                echo "$HOME/.local/biomeos/${PRIMAL_NAME}"
            fi
            ;;
        macos)
            if [[ $EUID -eq 0 ]]; then
                echo "/usr/local/biomeos/${PRIMAL_NAME}"
            else
                echo "$HOME/.local/biomeos/${PRIMAL_NAME}"
            fi
            ;;
        *)
            echo "$HOME/.local/biomeos/${PRIMAL_NAME}"
            ;;
    esac
}

# ============================================================================
# Storage Directory
# ============================================================================

storage_dir() {
    local platform=$1
    local install_path=$2
    
    case "$platform" in
        android)
            echo "/data/local/tmp/biomeos/${PRIMAL_NAME}/storage"
            ;;
        *)
            echo "${install_path}/storage"
            ;;
    esac
}

# ============================================================================
# Health Check
# ============================================================================

health_check() {
    local binary_path=$1
    
    log "Running health check..."
    
    # Check if binary is executable
    if [[ ! -x "$binary_path" ]]; then
        error "Binary is not executable: $binary_path"
    fi
    
    # Check version
    if ! "$binary_path" --version >/dev/null 2>&1; then
        error "Health check failed: binary not responding"
    fi
    
    log "✅ Health check passed"
}

# ============================================================================
# Main Deployment
# ============================================================================

main() {
    log "🧬 NestGate genomeBin Deployment"
    log "Version: $PRIMAL_VERSION"
    log "Description: $PRIMAL_DESCRIPTION"
    echo ""
    
    # Detect environment
    log "Detecting environment..."
    local arch=$(detect_arch)
    local platform=$(detect_platform)
    local binary=$(select_binary "$arch" "$platform")
    local install_path=$(install_dir "$platform")
    local storage_path=$(storage_dir "$platform" "$install_path")
    
    log "Platform: $platform"
    log "Architecture: $arch"
    log "Binary: $binary"
    log "Install path: $install_path"
    log "Storage path: $storage_path"
    echo ""
    
    # Create directories
    log "Creating directories..."
    mkdir -p "$install_path"
    mkdir -p "$storage_path"
    
    # Extract embedded archive
    log "Extracting NestGate..."
    local archive_line=$(awk '/^__ARCHIVE_START__$/ { print NR + 1; exit 0; }' "$0")
    
    if ! tail -n +"$archive_line" "$0" | tar xzf - -C "$install_path"; then
        error "Failed to extract archive"
    fi
    
    # Set executable permissions
    log "Setting permissions..."
    chmod +x "$install_path/$binary"
    
    # Create convenience symlink
    ln -sf "$install_path/$binary" "$install_path/${PRIMAL_NAME}"
    
    # Health check
    health_check "$install_path/${PRIMAL_NAME}"
    
    # Success!
    echo ""
    log "================================="
    log "✅ NestGate Deployed Successfully!"
    log "================================="
    echo ""
    log "Binary: $install_path/${PRIMAL_NAME}"
    log "Storage: $storage_path"
    echo ""
    log "Get started:"
    log "  $install_path/${PRIMAL_NAME} --help"
    log "  $install_path/${PRIMAL_NAME} --version"
    echo ""
    
    # Show primal self-knowledge
    log "Primal Self-Knowledge:"
    log "  Name: NestGate"
    log "  Role: Universal Storage & Persistence"
    log "  Capabilities: RocksDB, SQLite, ZFS, Object Storage"
    log "  Discovery: Runtime via Songbird"
    echo ""
    
    # Add to PATH suggestion
    if [[ "$platform" != "android" ]]; then
        log "💡 Tip: Add to PATH for easy access:"
        log "  echo 'export PATH=\"$install_path:\$PATH\"' >> ~/.bashrc"
        log "  source ~/.bashrc"
    fi
}

# Run deployment
main "$@"
exit 0

# ============================================================================
# Embedded Archive (appended during build)
# ============================================================================
__ARCHIVE_START__
