#!/bin/bash
# ==============================================================================
# NestGate Data Service for Ecosystem Integration
# ==============================================================================
#
# Starts REST API and streaming service for primal integration
#
# Usage:
#   ./start_data_service.sh [OPTIONS]
#
# Options:
#   --port PORT              API port (default: 9000)
#   --enable-streaming       Enable WebSocket streaming (default: true)
#   --enable-versioning      Enable model versioning (default: true)
#   --data-dir DIR           Data directory (default: /nestgate_data)
#   --log-level LEVEL        Log level (default: info)
#   --help                   Show this help
#
# ==============================================================================

set -euo pipefail

PORT=9000
ENABLE_STREAMING=true
ENABLE_VERSIONING=true
DATA_DIR="/nestgate_data"
LOG_LEVEL="info"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${BLUE}▶ $1${NC}"; }
log_success() { echo -e "${GREEN}✓ $1${NC}"; }

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --port) PORT="$2"; shift 2 ;;
        --enable-streaming) ENABLE_STREAMING="$2"; shift 2 ;;
        --enable-versioning) ENABLE_VERSIONING="$2"; shift 2 ;;
        --data-dir) DATA_DIR="$2"; shift 2 ;;
        --log-level) LOG_LEVEL="$2"; shift 2 ;;
        --help) sed -n '/^# Usage:/,/^# ===/p' "$0" | sed 's/^# //' | head -n -1; exit 0 ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

echo ""
echo "╔════════════════════════════════════════════════════════════╗"
echo "║      🌐 NESTGATE DATA SERVICE                             ║"
echo "║      Ecosystem Integration API                            ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

log_info "Starting NestGate Data Service..."
echo ""
echo "Configuration:"
echo "  Port:              $PORT"
echo "  Streaming:         $ENABLE_STREAMING"
echo "  Versioning:        $ENABLE_VERSIONING"
echo "  Data Directory:    $DATA_DIR"
echo "  Log Level:         $LOG_LEVEL"
echo ""

# Check if data directory exists
if [ ! -d "$DATA_DIR" ]; then
    log_info "Creating data directory: $DATA_DIR"
    mkdir -p "$DATA_DIR"/{datasets,models,metadata}
fi

# Create service configuration
CONFIG_FILE="/tmp/nestgate_data_service_config.toml"
cat > "$CONFIG_FILE" <<EOF
[service]
port = $PORT
host = "0.0.0.0"
log_level = "$LOG_LEVEL"

[features]
streaming = $ENABLE_STREAMING
versioning = $ENABLE_VERSIONING
compression = true
authentication = false  # Set to true for production

[storage]
data_dir = "$DATA_DIR"
datasets_dir = "$DATA_DIR/datasets"
models_dir = "$DATA_DIR/models"
metadata_dir = "$DATA_DIR/metadata"

[limits]
max_file_size_gb = 100
max_concurrent_downloads = 10
rate_limit_requests_per_minute = 1000

[primals]
# Primal access configuration
[primals.beardog]
enabled = true
quota_gb = 500

[primals.squirrel]
enabled = true
quota_gb = 1000

[primals.toadstool]
enabled = true
quota_gb = 750

[primals.songbird]
enabled = true
quota_gb = 500

[primals.biomeOS]
enabled = true
quota_gb = 250
EOF

log_success "Configuration created: $CONFIG_FILE"

# Create API documentation
API_DOC_FILE="/tmp/nestgate_api_docs.md"
cat > "$API_DOC_FILE" <<EOF
# NestGate Data Service API Documentation

## Base URL
http://localhost:$PORT/api/v1

## Endpoints

### Datasets

- \`GET /datasets\` - List all available datasets
- \`GET /datasets/{id}\` - Get dataset metadata
- \`GET /datasets/{id}/stream\` - Stream dataset (chunked transfer)
- \`GET /datasets/{id}/download\` - Download complete dataset
- \`POST /datasets/{id}/snapshot\` - Create dataset snapshot

### Models

- \`GET /models/{primal}\` - List models for primal
- \`GET /models/{primal}/{model_id}\` - Get model metadata
- \`GET /models/{primal}/{model_id}/download?version=latest\` - Download model
- \`POST /models/{primal}/{model_id}\` - Upload model
- \`POST /models/{primal}/{model_id}/version\` - Create new version

### Health & Metrics

- \`GET /health\` - Service health check
- \`GET /metrics\` - Prometheus metrics
- \`GET /status\` - Service status

## Authentication

Include in headers:
\`\`\`
X-Primal: beardog
X-API-Key: your_api_key_here
\`\`\`

## Examples

### List datasets
\`\`\`bash
curl http://localhost:$PORT/api/v1/datasets
\`\`\`

### Stream dataset
\`\`\`bash
curl http://localhost:$PORT/api/v1/datasets/hsm_training_v1/stream \\
  -H "X-Primal: beardog" \\
  --output training_data.tar.gz
\`\`\`

### Upload model
\`\`\`bash
curl -X POST http://localhost:$PORT/api/v1/models/beardog/hsm_model \\
  -H "X-Primal: beardog" \\
  -F "model=@hsm_v2.safetensors" \\
  -F "metadata=@metadata.json"
\`\`\`
EOF

log_success "API documentation created: $API_DOC_FILE"

# Check if port is available
if lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo ""
    echo "⚠️  Port $PORT is already in use!"
    echo "    Kill existing process or use --port to specify different port"
    echo ""
    exit 1
fi

# Create simple HTTP server for testing
# In production, this would be a proper Rust API server
log_info "Starting HTTP service on port $PORT..."

# Create a simple Python server as placeholder
cat > /tmp/nestgate_data_service.py <<'EOF'
#!/usr/bin/env python3
import http.server
import socketserver
import json
import os
from datetime import datetime

PORT = int(os.environ.get('PORT', 9000))
DATA_DIR = os.environ.get('DATA_DIR', '/nestgate_data')

class NestGateHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/health' or self.path == '/api/v1/health':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = {
                'status': 'healthy',
                'timestamp': datetime.utcnow().isoformat() + 'Z',
                'service': 'nestgate-data-service',
                'version': '1.0.0'
            }
            self.wfile.write(json.dumps(response, indent=2).encode())
        
        elif self.path == '/status' or self.path == '/api/v1/status':
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = {
                'service': 'nestgate-data-service',
                'status': 'running',
                'uptime_seconds': 0,
                'active_connections': 1,
                'total_requests': 0,
                'data_dir': DATA_DIR,
                'features': {
                    'streaming': True,
                    'versioning': True,
                    'compression': True
                }
            }
            self.wfile.write(json.dumps(response, indent=2).encode())
        
        elif self.path.startswith('/api/v1/datasets'):
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = {
                'datasets': [
                    {'id': 'hsm_training_v1', 'size_gb': 5, 'type': 'mixed'},
                    {'id': 'llm_corpus_v1', 'size_gb': 50, 'type': 'text'},
                    {'id': 'vision_training_v1', 'size_gb': 75, 'type': 'images'}
                ]
            }
            self.wfile.write(json.dumps(response, indent=2).encode())
        
        elif self.path.startswith('/api/v1/models'):
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            response = {
                'models': [
                    {'name': 'hsm_model', 'version': 'v2.0.0', 'size_mb': 4200},
                    {'name': 'llm_base', 'version': 'v1.0.0', 'size_mb': 7500}
                ]
            }
            self.wfile.write(json.dumps(response, indent=2).encode())
        
        else:
            super().do_GET()
    
    def log_message(self, format, *args):
        # Custom log format
        timestamp = datetime.now().strftime('[%Y-%m-%d %H:%M:%S]')
        print(f"{timestamp} {self.address_string()} - {format%args}")

Handler = NestGateHandler
httpd = socketserver.TCPServer(("", PORT), Handler)

print("")
print("╔════════════════════════════════════════════════════════════╗")
print("║      ✅ NESTGATE DATA SERVICE RUNNING                     ║")
print("╚════════════════════════════════════════════════════════════╝")
print("")
print(f"Service URL:     http://localhost:{PORT}")
print(f"Health Check:    http://localhost:{PORT}/health")
print(f"API Docs:        {os.environ.get('API_DOC_FILE', '/tmp/nestgate_api_docs.md')}")
print("")
print("API Endpoints:")
print(f"  GET  /api/v1/datasets")
print(f"  GET  /api/v1/models/{{primal}}")
print(f"  GET  /api/v1/health")
print(f"  GET  /api/v1/status")
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
EOF

chmod +x /tmp/nestgate_data_service.py

# Start the service
export PORT=$PORT
export DATA_DIR=$DATA_DIR
export API_DOC_FILE=$API_DOC_FILE

log_success "Service starting..."
echo ""

# Run the service
python3 /tmp/nestgate_data_service.py

exit 0

