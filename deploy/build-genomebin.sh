#!/usr/bin/env bash
# build-genomebin.sh - Build and package NestGate genomeBin
# Multi-architecture self-deploying binary
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
VERSION="${VERSION:-1.0.0}"
DIST_DIR="dist/nestgate-genome"
WRAPPER_SCRIPT="deploy/nestgate.genome.sh"

# Targets to build (adjust based on available toolchains)
TARGETS=(
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
    # "aarch64-linux-android"  # Uncomment when Android NDK configured
    # "x86_64-apple-darwin"     # Uncomment when building on macOS
    # "aarch64-apple-darwin"    # Uncomment for Apple Silicon
)

# ============================================================================
# Helper Functions
# ============================================================================

log() {
    echo -e "${GREEN}[genomeBin]${NC} $*"
}

warn() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
    exit 1
}

check_prerequisites() {
    log "Checking prerequisites..."
    
    # Check Rust toolchain
    if ! command -v cargo >/dev/null 2>&1; then
        error "cargo not found. Please install Rust: https://rustup.rs/"
    fi
    
    # Check for required targets
    for target in "${TARGETS[@]}"; do
        if ! rustup target list | grep -q "$target (installed)"; then
            warn "Target $target not installed. Installing..."
            rustup target add "$target" || error "Failed to add target $target"
        fi
    done
    
    # Check for cross-compilation linkers
    if [[ " ${TARGETS[@]} " =~ "aarch64-unknown-linux-musl" ]]; then
        if ! command -v aarch64-linux-musl-gcc >/dev/null 2>&1; then
            warn "aarch64-linux-musl-gcc not found. ARM64 musl build may fail."
            warn "Install: sudo apt install gcc-aarch64-linux-gnu musl-tools"
        fi
    fi
    
    # Check for Android NDK (if building Android target)
    if [[ " ${TARGETS[@]} " =~ "aarch64-linux-android" ]]; then
        if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
            warn "ANDROID_NDK_HOME not set. Android build will be skipped."
            warn "Set: export ANDROID_NDK_HOME=\"\$HOME/Android/Sdk/ndk/26.0.10792818\""
        fi
    fi
    
    log "Prerequisites check complete."
}

clean_dist() {
    log "Cleaning dist directory..."
    rm -rf dist
    mkdir -p "$DIST_DIR"
}

build_target() {
    local target=$1
    log "Building for $target..."
    
    # Build with release profile
    if cargo build --release --target "$target"; then
        log "✅ Build successful: $target"
        
        # Copy binary to dist
        local binary_name="${PRIMAL_NAME}-${target}"
        cp "target/$target/release/$PRIMAL_NAME" "$DIST_DIR/$binary_name"
        
        # Show binary size
        local size=$(du -h "$DIST_DIR/$binary_name" | cut -f1)
        log "Binary size: $size ($binary_name)"
        
        return 0
    else
        warn "❌ Build failed: $target (skipping)"
        return 1
    fi
}

build_all_targets() {
    log "Building all targets..."
    local success_count=0
    local fail_count=0
    
    for target in "${TARGETS[@]}"; do
        if build_target "$target"; then
            ((success_count++))
        else
            ((fail_count++))
        fi
    done
    
    log "Build summary: $success_count successful, $fail_count failed"
    
    if [[ $success_count -eq 0 ]]; then
        error "All builds failed!"
    fi
}

create_archive() {
    log "Creating multi-arch archive..."
    cd "$DIST_DIR"
    
    # Create tarball with all binaries
    tar czf ../nestgate-binaries.tar.gz *
    
    cd ../..
    
    local size=$(du -h "dist/nestgate-binaries.tar.gz" | cut -f1)
    log "Archive size: $size"
}

create_genomebin() {
    log "Creating genomeBin wrapper..."
    
    # Check if wrapper script exists
    if [[ ! -f "$WRAPPER_SCRIPT" ]]; then
        error "Wrapper script not found: $WRAPPER_SCRIPT"
    fi
    
    # Combine wrapper + archive
    cat "$WRAPPER_SCRIPT" dist/nestgate-binaries.tar.gz > dist/nestgate.genome
    chmod +x dist/nestgate.genome
    
    local size=$(du -h dist/nestgate.genome | cut -f1)
    log "✅ genomeBin created: dist/nestgate.genome ($size)"
}

test_genomebin() {
    log "Testing genomeBin (dry-run)..."
    
    # Test if genomeBin is executable
    if [[ ! -x dist/nestgate.genome ]]; then
        error "genomeBin is not executable!"
    fi
    
    # Validate archive integrity
    if ! tar tzf <(tail -n +$(awk '/^__ARCHIVE_START__$/ { print NR + 1; exit 0; }' dist/nestgate.genome) dist/nestgate.genome) >/dev/null 2>&1; then
        error "genomeBin archive validation failed!"
    fi
    
    log "✅ genomeBin validation passed"
}

show_summary() {
    log "================================="
    log "genomeBin Build Complete!"
    log "================================="
    echo ""
    log "Output: dist/nestgate.genome"
    log "Size: $(du -h dist/nestgate.genome | cut -f1)"
    echo ""
    log "Included architectures:"
    for target in "${TARGETS[@]}"; do
        if [[ -f "$DIST_DIR/${PRIMAL_NAME}-${target}" ]]; then
            local size=$(du -h "$DIST_DIR/${PRIMAL_NAME}-${target}" | cut -f1)
            log "  ✅ $target ($size)"
        fi
    done
    echo ""
    log "Test deployment:"
    log "  ./dist/nestgate.genome --help"
    echo ""
    log "Deploy to plasmidBin:"
    log "  cp dist/nestgate.genome ~/plasmidBin/stable/genomes/"
}

# ============================================================================
# Main Execution
# ============================================================================

main() {
    log "🧬 NestGate genomeBin Build"
    log "Version: $VERSION"
    echo ""
    
    check_prerequisites
    clean_dist
    build_all_targets
    create_archive
    create_genomebin
    test_genomebin
    show_summary
}

# Run main
main "$@"
