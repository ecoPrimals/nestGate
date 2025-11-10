#!/bin/bash
# ==============================================================================
# NestGate Federation Service Starter
# ==============================================================================
#
# Starts NestGate service and registers with local songbird/toadstool federation
#
# ==============================================================================

set -euo pipefail

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${BLUE}▶ $1${NC}"; }
log_success() { echo -e "${GREEN}✓ $1${NC}"; }
log_warn() { echo -e "${YELLOW}⚠ $1${NC}"; }
log_error() { echo -e "${RED}✗ $1${NC}"; }

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NESTGATE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_FILE="$NESTGATE_ROOT/config/federation-local.toml"
SERVICE_PORT=9001
HEALTH_PORT=9002
HOST_IP="192.168.1.144"

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║      🌍 NESTGATE FEDERATION SERVICE                       ║"
echo "║      Connecting to Local Ecosystem                        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# ==============================================================================
# Pre-flight Checks
# ==============================================================================

log_info "Running pre-flight checks..."

# Check if ports are available
if lsof -Pi :$SERVICE_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    log_error "Port $SERVICE_PORT already in use!"
    lsof -Pi :$SERVICE_PORT -sTCP:LISTEN
    exit 1
fi

if lsof -Pi :$HEALTH_PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    log_error "Port $HEALTH_PORT already in use!"
    exit 1
fi

log_success "Ports available: $SERVICE_PORT, $HEALTH_PORT"

# Check if songbird is running
log_info "Checking for songbird orchestrator..."
if curl -s http://$HOST_IP:8080/health > /dev/null 2>&1; then
    log_success "songbird orchestrator found at http://$HOST_IP:8080"
    SONGBIRD_AVAILABLE=true
else
    log_warn "songbird orchestrator not responding at http://$HOST_IP:8080"
    SONGBIRD_AVAILABLE=false
fi

# Check if toadstool is running
log_info "Checking for toadstool BYOB server..."
if curl -s http://$HOST_IP:8084/health > /dev/null 2>&1; then
    log_success "toadstool BYOB server found at http://$HOST_IP:8084"
    TOADSTOOL_AVAILABLE=true
else
    log_warn "toadstool BYOB server not responding at http://$HOST_IP:8084"
    TOADSTOOL_AVAILABLE=false
fi

# Create data directories
log_info "Setting up data directories..."
mkdir -p ~/nestgate_data/{datasets,models,metadata,temp}
log_success "Data directories created at ~/nestgate_data"

echo ""
echo "════════════════════════════════════════════════════════════"
echo "Configuration"
echo "════════════════════════════════════════════════════════════"
echo "  Host IP:           $HOST_IP"
echo "  Service Port:      $SERVICE_PORT"
echo "  Health Port:       $HEALTH_PORT"
echo "  Config File:       $CONFIG_FILE"
echo "  songbird:          $([ "$SONGBIRD_AVAILABLE" = true ] && echo '✅ Available' || echo '❌ Not available')"
echo "  toadstool:         $([ "$TOADSTOOL_AVAILABLE" = true ] && echo '✅ Available' || echo '❌ Not available')"
echo ""

# ==============================================================================
# Start Federation Service
# ==============================================================================

log_info "Starting NestGate federation service..."

# Create Python service (placeholder for Rust implementation)
cat > /tmp/nestgate_federation_service.py <<'PYTHON_EOF'
#!/usr/bin/env python3
"""
NestGate Federation Service
Provides storage and data management for the ecosystem
"""

import http.server
import socketserver
import json
import os
import requests
from datetime import datetime
from threading import Thread
import time

PORT = int(os.environ.get('SERVICE_PORT', 9001))
HEALTH_PORT = int(os.environ.get('HEALTH_PORT', 9002))
HOST_IP = os.environ.get('HOST_IP', '192.168.1.144')
SONGBIRD_URL = f"http://{HOST_IP}:8080"
TOADSTOOL_URL = f"http://{HOST_IP}:8084"

# Service metadata
SERVICE_INFO = {
    'service_id': 'nestgate-storage-001',
    'service_name': 'NestGate Storage Provider',
    'service_type': 'storage',
    'version': '0.11.3',
    'capabilities': [
        'zfs_storage',
        'dataset_management',
        'model_storage',
        'versioning',
        'compression',
        'snapshots'
    ],
    'endpoints': {
        'base': f'http://{HOST_IP}:{PORT}',
        'datasets': f'http://{HOST_IP}:{PORT}/api/v1/datasets',
        'models': f'http://{HOST_IP}:{PORT}/api/v1/models',
        'health': f'http://{HOST_IP}:{HEALTH_PORT}/health'
    },
    'performance': {
        'read_throughput_mbs': 850,
        'write_throughput_mbs': 450,
        'api_latency_ms': 5
    }
}

class NestGateFederationHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/health':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = {
                'status': 'healthy',
                'service': 'nestgate-storage',
                'version': '0.11.3',
                'timestamp': datetime.utcnow().isoformat() + 'Z',
                'federation': {
                    'songbird': check_service_health(SONGBIRD_URL),
                    'toadstool': check_service_health(TOADSTOOL_URL)
                }
            }
            self.wfile.write(json.dumps(response, indent=2).encode())
        
        elif self.path == '/status':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = {
                **SERVICE_INFO,
                'status': 'online',
                'uptime_seconds': int(time.time() - start_time),
                'federation_members': get_federation_members()
            }
            self.wfile.write(json.dumps(response, indent=2).encode())
        
        elif self.path == '/api/v1/capabilities':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = {
                'capabilities': SERVICE_INFO['capabilities'],
                'performance': SERVICE_INFO['performance'],
                'endpoints': SERVICE_INFO['endpoints']
            }
            self.wfile.write(json.dumps(response, indent=2).encode())
        
        elif self.path.startswith('/api/v1/datasets'):
            self.handle_datasets_request()
        
        elif self.path.startswith('/api/v1/models'):
            self.handle_models_request()
        
        else:
            self.send_response(404)
            self.end_headers()
    
    def handle_datasets_request(self):
        """Handle dataset API requests"""
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        
        # Mock dataset list
        datasets = [
            {
                'id': 'hsm_training_v1',
                'primal': 'beardog',
                'size_gb': 5,
                'type': 'mixed',
                'compression': '2.1x',
                'created': '2025-11-10T12:00:00Z'
            },
            {
                'id': 'llm_corpus_v1',
                'primal': 'squirrel',
                'size_gb': 50,
                'type': 'text',
                'compression': '4.2x',
                'created': '2025-11-10T12:00:00Z'
            },
            {
                'id': 'vision_training_v1',
                'primal': 'toadstool',
                'size_gb': 75,
                'type': 'images',
                'compression': '1.8x',
                'created': '2025-11-10T12:00:00Z'
            }
        ]
        
        response = {
            'datasets': datasets,
            'total_count': len(datasets),
            'total_size_gb': sum(d['size_gb'] for d in datasets)
        }
        
        self.wfile.write(json.dumps(response, indent=2).encode())
    
    def handle_models_request(self):
        """Handle model API requests"""
        self.send_response(200)
        self.send_header('Content-type', 'application/json')
        self.end_headers()
        
        # Mock model list
        models = [
            {
                'primal': 'beardog',
                'model_id': 'hsm_model_v2',
                'version': 'v2.0.0',
                'size_mb': 4200,
                'format': 'safetensors'
            },
            {
                'primal': 'squirrel',
                'model_id': 'llm_base',
                'version': 'v1.0.0',
                'size_mb': 7500,
                'format': 'gguf'
            }
        ]
        
        response = {
            'models': models,
            'total_count': len(models)
        }
        
        self.wfile.write(json.dumps(response, indent=2).encode())
    
    def log_message(self, format, *args):
        timestamp = datetime.now().strftime('[%Y-%m-%d %H:%M:%S]')
        print(f"{timestamp} {self.address_string()} - {format%args}")

def check_service_health(url):
    """Check if a service is healthy"""
    try:
        response = requests.get(f"{url}/health", timeout=2)
        return response.status_code == 200
    except:
        return False

def get_federation_members():
    """Get list of federation members"""
    members = []
    
    # Check songbird
    if check_service_health(SONGBIRD_URL):
        members.append({
            'name': 'songbird',
            'type': 'orchestrator',
            'url': SONGBIRD_URL,
            'status': 'healthy'
        })
    
    # Check toadstool
    if check_service_health(TOADSTOOL_URL):
        members.append({
            'name': 'toadstool',
            'type': 'byob_server',
            'url': TOADSTOOL_URL,
            'status': 'healthy'
        })
    
    return members

def register_with_songbird():
    """Register NestGate with songbird orchestrator"""
    try:
        print(f"\n🔗 Registering with songbird at {SONGBIRD_URL}...")
        
        registration_data = {
            **SERVICE_INFO,
            'primal_name': 'nestgate',
            'registration_time': datetime.utcnow().isoformat() + 'Z'
        }
        
        # Try to register (songbird may not have this endpoint yet)
        response = requests.post(
            f"{SONGBIRD_URL}/api/v1/register",
            json=registration_data,
            timeout=5
        )
        
        if response.status_code == 200:
            print(f"✅ Successfully registered with songbird")
            return True
        else:
            print(f"⚠️  songbird returned {response.status_code}")
            return False
            
    except requests.exceptions.RequestException as e:
        print(f"⚠️  Could not register with songbird: {e}")
        print("   (songbird may not support registration yet)")
        return False

def heartbeat_loop():
    """Send periodic heartbeats to federation"""
    while True:
        time.sleep(30)  # Every 30 seconds
        
        # Send heartbeat to songbird
        try:
            requests.post(
                f"{SONGBIRD_URL}/api/v1/heartbeat",
                json={'service_id': SERVICE_INFO['service_id']},
                timeout=2
            )
        except:
            pass  # Silently fail

def start_health_server():
    """Start health check server on separate port"""
    handler = NestGateFederationHandler
    httpd = socketserver.TCPServer(("", HEALTH_PORT), handler)
    httpd.serve_forever()

# Main service
start_time = time.time()

# Start health server in background
health_thread = Thread(target=start_health_server, daemon=True)
health_thread.start()

# Register with songbird
time.sleep(2)  # Give health server time to start
register_with_songbird()

# Start heartbeat loop
heartbeat_thread = Thread(target=heartbeat_loop, daemon=True)
heartbeat_thread.start()

# Start main service
Handler = NestGateFederationHandler
httpd = socketserver.TCPServer(("", PORT), Handler)

print("")
print("╔════════════════════════════════════════════════════════════╗")
print("║      ✅ NESTGATE FEDERATION SERVICE ONLINE                ║")
print("╚════════════════════════════════════════════════════════════╝")
print("")
print(f"Service URL:     http://{HOST_IP}:{PORT}")
print(f"Health Check:    http://{HOST_IP}:{HEALTH_PORT}/health")
print(f"Status:          http://{HOST_IP}:{PORT}/status")
print(f"Capabilities:    http://{HOST_IP}:{PORT}/api/v1/capabilities")
print("")
print("API Endpoints:")
print(f"  GET  /api/v1/datasets")
print(f"  GET  /api/v1/models/{{primal}}")
print(f"  GET  /health")
print(f"  GET  /status")
print("")
print("Federation:")
for member in get_federation_members():
    print(f"  ✅ {member['name']}: {member['url']}")
print("")
print("Press Ctrl+C to stop")
print("")

try:
    httpd.serve_forever()
except KeyboardInterrupt:
    print("\n")
    print("Shutting down gracefully...")
    httpd.shutdown()
    print("Service stopped.")
PYTHON_EOF

chmod +x /tmp/nestgate_federation_service.py

# Export environment
export SERVICE_PORT=$SERVICE_PORT
export HEALTH_PORT=$HEALTH_PORT
export HOST_IP=$HOST_IP

# Start the service
log_success "Starting federation service..."
echo ""

python3 /tmp/nestgate_federation_service.py

exit 0

