#!/bin/bash
# NestGate Production Deployment Script
# Generated: September 17, 2025
# Status: Production Ready

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
NESTGATE_USER="nestgate"
NESTGATE_GROUP="nestgate"
INSTALL_DIR="/opt/nestgate"
CONFIG_DIR="/etc/nestgate"
LOG_DIR="/var/log/nestgate"
DATA_DIR="/var/lib/nestgate"
SERVICE_NAME="nestgate"

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        log_error "This script must be run as root"
        exit 1
    fi
}

check_dependencies() {
    log_info "Checking system dependencies..."
    
    local deps=("curl" "systemctl" "cargo" "git")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            log_error "Required dependency '$dep' is not installed"
            exit 1
        fi
    done
    
    log_success "All dependencies are installed"
}

create_user() {
    log_info "Creating NestGate system user..."
    
    if ! id "$NESTGATE_USER" &>/dev/null; then
        useradd --system --home-dir "$DATA_DIR" --shell /bin/false "$NESTGATE_USER"
        log_success "Created user: $NESTGATE_USER"
    else
        log_info "User $NESTGATE_USER already exists"
    fi
}

create_directories() {
    log_info "Creating directory structure..."
    
    local dirs=("$INSTALL_DIR" "$CONFIG_DIR" "$LOG_DIR" "$DATA_DIR" "$DATA_DIR/data" "$DATA_DIR/temp")
    
    for dir in "${dirs[@]}"; do
        mkdir -p "$dir"
        chown "$NESTGATE_USER:$NESTGATE_GROUP" "$dir"
        chmod 755 "$dir"
    done
    
    # Set special permissions for log directory
    chmod 750 "$LOG_DIR"
    
    log_success "Directory structure created"
}

build_nestgate() {
    log_info "Building NestGate from source..."
    
    # Build in release mode
    cargo build --release --workspace
    
    if [[ $? -eq 0 ]]; then
        log_success "NestGate built successfully"
    else
        log_error "Failed to build NestGate"
        exit 1
    fi
}

install_binaries() {
    log_info "Installing NestGate binaries..."
    
    # Copy main binaries
    cp target/release/nestgate-api-server "$INSTALL_DIR/"
    cp target/release/nestgate-installer "$INSTALL_DIR/"
    
    # Set permissions
    chown "$NESTGATE_USER:$NESTGATE_GROUP" "$INSTALL_DIR"/*
    chmod 755 "$INSTALL_DIR"/*
    
    log_success "Binaries installed to $INSTALL_DIR"
}

install_config() {
    log_info "Installing configuration files..."
    
    # Copy production config
    cp config/production-ready.toml "$CONFIG_DIR/nestgate.toml"
    
    # Set permissions
    chown "$NESTGATE_USER:$NESTGATE_GROUP" "$CONFIG_DIR/nestgate.toml"
    chmod 640 "$CONFIG_DIR/nestgate.toml"
    
    log_success "Configuration installed to $CONFIG_DIR"
}

create_systemd_service() {
    log_info "Creating systemd service..."
    
    cat > "/etc/systemd/system/$SERVICE_NAME.service" << EOF
[Unit]
Description=NestGate Distributed Storage System
Documentation=https://github.com/ecoPrimals/nestgate
After=network.target
Wants=network.target

[Service]
Type=exec
User=$NESTGATE_USER
Group=$NESTGATE_GROUP
ExecStart=$INSTALL_DIR/nestgate-api-server --config $CONFIG_DIR/nestgate.toml
ExecReload=/bin/kill -HUP \$MAINPID
Restart=always
RestartSec=5
TimeoutStopSec=30

# Security settings
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=$DATA_DIR $LOG_DIR
CapabilityBoundingSet=CAP_NET_BIND_SERVICE

# Environment
Environment=RUST_LOG=info
Environment=NESTGATE_CONFIG_PATH=$CONFIG_DIR/nestgate.toml

[Install]
WantedBy=multi-user.target
EOF

    systemctl daemon-reload
    log_success "Systemd service created"
}

setup_logrotate() {
    log_info "Setting up log rotation..."
    
    cat > "/etc/logrotate.d/$SERVICE_NAME" << EOF
$LOG_DIR/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 640 $NESTGATE_USER $NESTGATE_GROUP
    postrotate
        systemctl reload $SERVICE_NAME > /dev/null 2>&1 || true
    endscript
}
EOF

    log_success "Log rotation configured"
}

generate_ssl_certs() {
    log_info "Generating self-signed SSL certificates for development..."
    
    local ssl_dir="$CONFIG_DIR/tls"
    mkdir -p "$ssl_dir"
    
    # Generate self-signed certificate (for development only)
    openssl req -x509 -newkey rsa:4096 -keyout "$ssl_dir/key.pem" -out "$ssl_dir/cert.pem" \
        -days 365 -nodes -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"
    
    # Set permissions
    chown -R "$NESTGATE_USER:$NESTGATE_GROUP" "$ssl_dir"
    chmod 600 "$ssl_dir/key.pem"
    chmod 644 "$ssl_dir/cert.pem"
    
    log_warning "Self-signed certificates generated for development. Replace with proper certificates in production!"
}

print_post_install_info() {
    log_success "NestGate installation completed successfully!"
    echo
    echo "==================================="
    echo "POST-INSTALLATION STEPS:"
    echo "==================================="
    echo
    echo "1. Set required environment variables:"
    echo "   export NESTGATE_DB_PASSWORD='your-secure-password'"
    echo "   export NESTGATE_REDIS_PASSWORD='your-redis-password'"
    echo "   export NESTGATE_JWT_SECRET='your-jwt-secret'"
    echo
    echo "2. Review and customize configuration:"
    echo "   sudo nano $CONFIG_DIR/nestgate.toml"
    echo
    echo "3. Start and enable the service:"
    echo "   sudo systemctl enable $SERVICE_NAME"
    echo "   sudo systemctl start $SERVICE_NAME"
    echo
    echo "4. Check service status:"
    echo "   sudo systemctl status $SERVICE_NAME"
    echo
    echo "5. View logs:"
    echo "   sudo journalctl -u $SERVICE_NAME -f"
    echo
    echo "6. API will be available at:"
    echo "   http://localhost:8080/api/v1"
    echo "   https://localhost:8080/api/v1 (with TLS)"
    echo
    echo "7. Health check endpoint:"
    echo "   curl http://localhost:8082/health"
    echo
    echo "==================================="
}

# Main execution
main() {
    log_info "Starting NestGate production deployment..."
    
    check_root
    check_dependencies
    create_user
    create_directories
    build_nestgate
    install_binaries
    install_config
    create_systemd_service
    setup_logrotate
    generate_ssl_certs
    print_post_install_info
    
    log_success "NestGate deployment completed!"
}

# Run main function
main "$@" 