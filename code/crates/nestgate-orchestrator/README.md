# NestGate Port Manager

The NestGate Port Manager is a service coordination layer that handles dynamic port allocation, service discovery, and lifecycle management across the NestGate ecosystem. It solves problems with hardcoded ports, connection failures, and service coordination by providing a central registry and management system.

## Features

- **Dynamic Port Allocation**: Automatically allocate ports for services based on service type and requirements
- **Service Registry**: Central registry of all services with status tracking
- **Process Management**: Start, stop, and restart services with dependency resolution
- **Health Monitoring**: Check service health and status with various health checks
- **REST API**: Simple API for service management and discovery
- **TypeScript Client**: Client library and React hooks for frontend integration

## Getting Started

### Prerequisites

- Rust toolchain (1.70+)
- Node.js (for UI integration)

### Building

```shell
cd crates/nestgate-port-manager
cargo build
```

### Running

```shell
cargo run --release
```

Or use the CLI options:

```shell
cargo run --release -- --port 9000 --log-level debug
```

### Using the Start Script

We provide a convenient start script that launches the port manager along with other NestGate services:

```shell
./start-with-port-manager.sh
```

## Configuration

The port manager can be configured through a YAML or JSON configuration file:

```shell
cargo run --release -- --config config/port-manager.yaml
```

Alternatively, use environment variables:

```shell
NESTGATE_PORT_MANAGER_PORT=9001 NESTGATE_LOG_LEVEL=debug cargo run --release
```

### Default Port Ranges

- UI: 3000-3999
- API: 4000-4999
- WebSocket: 5000-5999
- Database: 6000-6999
- Metrics: 7000-7999
- Admin: 8000-8999

## API Endpoints

The port manager provides a RESTful API for service management:

- `/services` - List, register services
- `/services/:id` - Get, update, delete services
- `/services/:id/start` - Start a service
- `/services/:id/stop` - Stop a service
- `/services/:id/restart` - Restart a service
- `/services/:id/health` - Get service health
- `/ports` - List allocated ports
- `/ports/available` - List available ports by service type
- `/system/info` - Get system information

## TypeScript Client

The port manager includes a TypeScript client library for integration with the UI:

```typescript
import { PortManagerClient } from './services/port-manager/client';
import { usePortManager, useServices } from './services/port-manager/hooks';

// Direct client usage
const client = new PortManagerClient({
  baseUrl: 'http://localhost:9000',
});

// React hooks integration
const YourComponent = () => {
  const { services, startService, stopService } = useServices();
  
  return (
    <div>
      {services.map(service => (
        <div key={service.definition.id}>
          <h3>{service.definition.name}</h3>
          <p>Status: {service.status}</p>
          <button onClick={() => startService(service.definition.id)}>Start</button>
          <button onClick={() => stopService(service.definition.id)}>Stop</button>
        </div>
      ))}
    </div>
  );
};
```

## WebSocket Integration

The port manager is designed to work seamlessly with WebSocket connections:

```typescript
import { usePortManagerWebSocket } from './hooks/usePortManagerWebSocket';

const YourComponent = () => {
  const { isConnected, send, subscribe } = usePortManagerWebSocket();
  
  useEffect(() => {
    const unsubscribe = subscribe('telemetry', (message) => {
      console.log('Received telemetry:', message);
    });
    
    return unsubscribe;
  }, [subscribe]);
  
  return (
    <div>
      <p>WebSocket Connected: {isConnected ? 'Yes' : 'No'}</p>
      <button onClick={() => send('get_system_status')}>
        Get System Status
      </button>
    </div>
  );
};
```

## License

MIT 