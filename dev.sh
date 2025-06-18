#!/bin/bash

set -e

function check_deps() {
    echo "Checking dependencies..."
    command -v cargo >/dev/null 2>&1 || { echo "cargo is required but not installed. Aborting." >&2; exit 1; }
    command -v rustc >/dev/null 2>&1 || { echo "rustc is required but not installed. Aborting." >&2; exit 1; }
}

function lint() {
    echo "Running lints..."
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings
}

function test() {
    echo "Running tests..."
    cargo test --all-features --workspace
}

function build() {
    echo "Building project..."
    cargo build --all-features
}

function clean() {
    echo "Cleaning project..."
    cargo clean
}

function doc() {
    echo "Generating documentation..."
    cargo doc --no-deps --all-features
}

function help() {
    echo "NestGate Development Helper"
    echo "Usage: ./dev.sh [command]"
    echo ""
    echo "Commands:"
    echo "  lint    - Run all lints"
    echo "  test    - Run all tests"
    echo "  build   - Build the project"
    echo "  clean   - Clean build artifacts"
    echo "  doc     - Generate documentation"
    echo "  help    - Show this help message"
}

# Check dependencies first
check_deps

# Process command
case "$1" in
    "lint")
        lint
        ;;
    "test")
        test
        ;;
    "build")
        build
        ;;
    "clean")
        clean
        ;;
    "doc")
        doc
        ;;
    "help"|"")
        help
        ;;
    *)
        echo "Unknown command: $1"
        help
        exit 1
        ;;
esac 