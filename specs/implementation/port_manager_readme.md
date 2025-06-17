# NestGate Port Manager Integration

This document explains how to use the NestGate Port Manager integration for dynamic port allocation.

## Overview

The Port Manager is a service that dynamically allocates ports to different NestGate components, avoiding hardcoded ports and potential conflicts. It's primarily used for:

1. FileSystem Monitor service
2. Data API service
3. UI service
4. Other microservices that need network ports

## Quick Start

### Linux/Mac

```bash
# Toggle Port Manager and FileSystem Monitor services (start if not running, stop if running)
npm run port-manager

# Toggle Live Service Mode (Port Manager, FileSystem Monitor, Data API, UI)
npm run live-service
```

### Windows

```powershell
# Toggle Port Manager and FileSystem Monitor services (start if not running, stop if running)
npm run port-manager:win

# Toggle Live Service Mode (Port Manager, FileSystem Monitor, Data API, UI)
npm run live-service:win
```

## Architecture

The Port Manager works by:

1. Starting as a daemon that manages other services
2. Dynamically allocating ports when services request them
3. Monitoring service health
4. Managing service dependencies and startup order

## API Overview

### Port Manager Client

The TypeScript client API is available at `src/services/port-manager.ts`. Use this to interact with the port manager:

```typescript
import { PortManagerClient, ServiceType } from '../services/port-manager';

// Create client
const client = new PortManagerClient({
  baseUrl: 'http://localhost:9400'
});

// Request a port
const port = await client.allocatePort({
  serviceId: 'my-service',
  serviceType: ServiceType.API,
  preferredPort: 8000
});

// Get port info
const portInfo = await client.getPortInfo(port);

// Release a port when done
await client.deallocatePort(port);
```

### React Hook

For React components, use the provided hook:

```typescript
import { usePortManager } from '../hooks/usePortManager';

function MyComponent() {
  const {
    isConnected,
    allocatedPort,
    error,
    portInfo,
    allocatePort,
    deallocatePort
  } = usePortManager({
    portManagerUrl: 'http://localhost:9400',
    serviceId: 'my-component',
    serviceType: ServiceType.UI,
    preferredPort: 8080,
    autoAllocate: true
  });

  // Port is automatically allocated and released on component mount/unmount
  
  return (
    <div>
      {allocatedPort ? `Using port: ${allocatedPort}` : 'Allocating port...'}
    </div>
  );
}
```

## FileSystem Monitor Integration

The FileSystem Monitor uses the Port Manager to allocate its listening port. The integration between these systems is handled through the `useFileSystemMonitor` hook:

```typescript
import { useFileSystemMonitor } from '../hooks/useFileSystemMonitor';

function FileSystemComponent() {
  const {
    isConnected,
    events,
    watchedDirectories,
    allocatedPort,
    addWatch,
    removeWatch
  } = useFileSystemMonitor({
    preferredPort: 9500,
    autoConnect: true
  });
  
  // Now you can monitor filesystem events
  
  return (
    <div>
      <p>Connected: {isConnected ? 'Yes' : 'No'}</p>
      <p>Port: {allocatedPort || 'Not allocated'}</p>
      <button onClick={() => addWatch('/my/directory', true)}>
        Watch Directory
      </button>
    </div>
  );
}
```

## Scripts

All services should be managed through npm commands:

- `npm run port-manager`: Toggle Port Manager and FileSystem Monitor (start/stop)
- `npm run port-manager:win`: Windows version of the toggle command
- `npm run live-service`: Toggle Live Service Mode (start/stop) including all components

## Troubleshooting

### Common Issues

1. **Port Manager won't start**:
   - Check if another instance is already running
   - Verify port 9400 is not in use
   - Check logs in `logs/port-manager.log`

2. **Port allocation fails**:
   - Ensure your preferred port is not already in use
   - Check if service ID is unique

3. **FileSystem Monitor can't connect**:
   - Verify Port Manager is running
   - Check logs in `logs/filesystem-monitor.log`

### Logs

All logs are stored in the `logs/` directory:
- `port-manager.log`
- `filesystem-monitor.log`
- `data-service.log`
- `ui.log`

## Contributing

When adding new services that need port allocation:

1. Create a service type in `src/services/port-manager.ts`
2. Use the `usePortManager` hook to allocate ports
3. Update scripts to register the new service
4. Add appropriate tests 