#!/bin/bash

# NestGate TrueNAS ZFS Integration Installer
# This script installs the NestGate ZFS Tier Management middleware plugins for TrueNAS

# Script must be run as root
if [ "$(id -u)" -ne 0 ]; then
    echo "This script must be run as root"
    exit 1
fi

# Check that we're running on a TrueNAS system
if [ ! -f /etc/version ]; then
    echo "This script must be run on a TrueNAS system"
    exit 1
fi

# Check TrueNAS version
TRUENAS_VERSION=$(cat /etc/version)
echo "TrueNAS version: $TRUENAS_VERSION"

# Set Python path based on TrueNAS version
PYTHON_PATH="/usr/local/lib/python3.9/site-packages"
if [[ "$TRUENAS_VERSION" == *"SCALE"* ]]; then
    # SCALE uses Python 3.9
    PYTHON_PATH="/usr/local/lib/python3.9/site-packages"
elif [[ "$TRUENAS_VERSION" == *"CORE"* ]]; then
    # CORE uses Python 3.7
    PYTHON_PATH="/usr/local/lib/python3.7/site-packages"
else
    echo "Unknown TrueNAS version. This script is designed for TrueNAS SCALE or CORE."
    echo "Proceeding with Python 3.9 path. Installation may fail."
    PYTHON_PATH="/usr/local/lib/python3.9/site-packages"
fi

# Configuration
PLUGIN_DIR="$PYTHON_PATH/middlewared/plugins/nestgate"
SQL_DIR="/usr/local/etc/middleware/sql"
DB_PATH="/data/freenas-v1.db"
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Colors for prettier output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${GREEN}INFO:${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}WARNING:${NC} $1"
}

log_error() {
    echo -e "${RED}ERROR:${NC} $1"
}

# Create plugin directory
log_info "Creating plugin directory: $PLUGIN_DIR"
mkdir -p "$PLUGIN_DIR"
if [ $? -ne 0 ]; then
    log_error "Failed to create plugin directory"
    exit 1
fi

# Create __init__.py file
log_info "Creating __init__.py file"
cat << 'EOF' > "$PLUGIN_DIR/__init__.py"
# NestGate ZFS Tier Management middleware plugins
from middlewared.service import Service

def setup(middleware):
    middleware.logger.info('NestGate ZFS Tier Management plugins loaded')
EOF

# Copy plugin files
log_info "Copying plugin files"
for plugin_file in zfs_tier_manager.py ai_workload_detector.py telemetry_collector.py; do
    if [ -f "$SCRIPT_DIR/$plugin_file" ]; then
        cp "$SCRIPT_DIR/$plugin_file" "$PLUGIN_DIR/"
        chmod 644 "$PLUGIN_DIR/$plugin_file"
        log_info "Copied $plugin_file to $PLUGIN_DIR/"
    else
        log_error "Plugin file not found: $plugin_file"
        exit 1
    fi
done

# Create SQL directory
log_info "Creating SQL directory: $SQL_DIR"
mkdir -p "$SQL_DIR"
if [ $? -ne 0 ]; then
    log_error "Failed to create SQL directory"
    exit 1
fi

# Copy SQL schema file
if [ -f "$SCRIPT_DIR/nestgate_schema.sql" ]; then
    cp "$SCRIPT_DIR/nestgate_schema.sql" "$SQL_DIR/"
    chmod 644 "$SQL_DIR/nestgate_schema.sql"
    log_info "Copied nestgate_schema.sql to $SQL_DIR/"
else
    log_error "SQL schema file not found: nestgate_schema.sql"
    exit 1
fi

# Execute SQL schema file
log_info "Creating database schema"
if [ -f "$DB_PATH" ]; then
    sqlite3 "$DB_PATH" < "$SQL_DIR/nestgate_schema.sql"
    if [ $? -ne 0 ]; then
        log_error "Failed to execute SQL schema file"
        exit 1
    fi
    log_info "Database schema created successfully"
else
    log_error "Database file not found: $DB_PATH"
    exit 1
fi

# Restart middleware service
log_info "Restarting middleware service"
systemctl restart middlewared
if [ $? -ne 0 ]; then
    log_error "Failed to restart middleware service"
    exit 1
fi

# Add delay to allow middleware to restart
log_info "Waiting for middleware to restart (15 seconds)..."
sleep 15

# Verify installation
log_info "Verifying installation"
midclt ping > /dev/null 2>&1
if [ $? -ne 0 ]; then
    log_error "Middleware is not responding after restart"
    exit 1
fi

# Check if plugin is loaded
midclt call plugin.query "[\"id\", \"=\", \"nestgate.tiering\"]" > /dev/null 2>&1
if [ $? -ne 0 ]; then
    log_warn "Plugin verification failed. The plugin might not have loaded properly."
    log_warn "Check logs with: midclt call core.get_jobs '[[\"\id\",\">\",1000]]' | grep -i nestgate"
else
    log_info "Plugin loaded successfully"
fi

log_info "Installation complete! See README_NESTGATE.md for usage instructions."
echo ""
echo "You can verify the installation with the following command:"
echo "  midclt call nestgate.tiering.get_tier_datasets <pool_name>"
echo ""
echo "To enable AI workload detection for a dataset:"
echo "  midclt call nestgate.aidetector.configure '{\"dataset\": \"pool/dataset\", \"enabled\": true, \"auto_tune\": true}'"
echo ""
echo "For more information, refer to the README_NESTGATE.md file."

exit 0 