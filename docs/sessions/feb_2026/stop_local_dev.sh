#!/bin/bash
# Stop local NestGate development instance

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

if [ -f ~/.nestgate/service.pid ]; then
    PID=$(cat ~/.nestgate/service.pid)
    
    if ps -p $PID > /dev/null 2>&1; then
        echo "Stopping NestGate (PID: $PID)..."
        kill $PID
        
        # Wait for graceful shutdown
        for i in {1..5}; do
            if ! ps -p $PID > /dev/null 2>&1; then
                break
            fi
            sleep 1
        done
        
        # Force kill if still running
        if ps -p $PID > /dev/null 2>&1; then
            echo "Force stopping..."
            kill -9 $PID
        fi
        
        echo -e "${GREEN}✓${NC} NestGate stopped"
    else
        echo -e "${YELLOW}⚠${NC} NestGate not running (stale PID file)"
    fi
    
    rm ~/.nestgate/service.pid
else
    echo -e "${YELLOW}⚠${NC} NestGate not running (no PID file)"
fi

