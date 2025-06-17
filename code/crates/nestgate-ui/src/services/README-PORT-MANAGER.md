# Port Manager

## Overview

The Port Manager is a key component of the NestGate system that handles:

1. Dynamic port allocation for all NestGate services
2. Service lifecycle management (start, stop, monitor)
3. Process management with event notifications
4. System-agnostic deployment support

This system enables NestGate to run in various deployment scenarios without port conflicts, making it easier to deploy multiple instances or integrate with existing systems.

## Architecture

The Port Manager consists of two main components:

1. **Port Manager Service (`port-manager.ts`)**: A TypeScript service class that provides:
   - Port discovery and allocation
   - Service registration and tracking
   - Process lifecycle management
   - Event notification system

2. **Port Manager Initialization Script (`port-manager.js`)**: A Node.js script that:
   - Bootstraps the NestGate system with dynamic ports
   - Manages process lifecycle across all components
   - Handles cleanup on system shutdown
   - Provides CLI interface for startup options

## Usage

### In TypeScript Code

```typescript
import { PortManager, ServiceType } from './services/port-manager';

// Get the singleton instance
const portManager = PortManager.getInstance();

// Allocate a port for a service
const apiPort = await portManager.getPort(ServiceType.API);

// Register a service
portManager.registerService('my-service', ServiceType.API, apiPort);

// Start a service
await portManager.startService('my-service', 'node', ['server.js'], {
  NODE_ENV: 'production'
});

// Listen for service events
portManager.on('service:started', ({ serviceId, port }) => {
  console.log(`Service ${serviceId} started on port ${port}`);
});

// Stop a service
await portManager.stopService('my-service');
```

### From Command Line

```bash
# Start with dynamic port allocation
npm run start:dynamic

# Start in development mode with dynamic ports
npm run start:dynamic:dev

# Start without UI
npm run start:dynamic:noui
```

## Configuration

Port ranges for different service types can be configured in the `DEFAULT_PORT_RANGES` constant:

```typescript
const DEFAULT_PORT_RANGES = {
  UI: { start: 3000, end: 3050 },
  API: { start: 3051, end: 3100 },
  SERVER: { start: 3101, end: 3150 },
  WEBSOCKET: { start: 3151, end: 3200 }
};
```

## Benefits

1. **System-Agnostic**: Works across different operating systems and environments
2. **Dynamic Scaling**: Enables multiple instances to run simultaneously
3. **Containerization Support**: Perfect for container deployments
4. **Conflict Resolution**: Avoids port conflicts with other applications
5. **Process Management**: Handles process lifecycle and graceful shutdown 