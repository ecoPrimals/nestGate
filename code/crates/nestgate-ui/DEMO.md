# NestGate UI Half Marathon Demo

## Overview
This demo showcases the work completed in the UI Half Marathon, focusing on live API integration, real-time updates via WebSocket, and robust service implementations.

## Features Demonstrated
1. **Live API Integration**: Connection to real backend services
2. **NotificationCenter**: A comprehensive notification system
3. **WebSocket Integration**: Real-time updates and notifications
4. **BackupService**: Complete backup management with live API connection
5. **SystemMonitor**: System metrics visualization
6. **Test Improvements**: Robust testing with proper mocking

## Environment Configuration

### Environment Variables
Create a `.env.local` file in the project root with these settings:
```
# API URLs
REACT_APP_API_BASE_URL=http://localhost:8000
REACT_APP_WEBSOCKET_URL=ws://localhost:8080/ws

# Mock Mode Settings (set to 'true' to use mock data)
# REACT_APP_USE_MOCK_ALL=true
REACT_APP_USE_MOCK_BACKUP=false
REACT_APP_USE_MOCK_TELEMETRY=false
REACT_APP_USE_MOCK_WEBSOCKET=false
REACT_APP_USE_MOCK_NOTIFICATIONS=false

# API Configuration
REACT_APP_API_RETRY_ATTEMPTS=3
REACT_APP_API_RETRY_INTERVAL=2000
REACT_APP_API_TIMEOUT=10000
```

### API Server Setup
Ensure the backend API servers are running:

```bash
# Start the API server
cd /path/to/nestgate-api
cargo run

# Start the WebSocket server
cd /path/to/nestgate-ws
cargo run
```

## How to Run the Demo

### Starting the Dev Server
```bash
cd crates/nestgate-ui
npm start
```

Visit http://localhost:3000 in your browser to see the application.

### Live API Connections

The UI now connects to live API endpoints by default. Services include:

- **BackupService**: Manages backup jobs, targets, and restore operations
- **WebSocketService**: Provides real-time updates and notifications
- **TelemetryService**: Collects system metrics and performance data

To verify live connections, check these indicators:
1. The system dashboard shows real-time metrics (not demo data)
2. Backup jobs reflect the actual system configuration
3. Notifications come from real system events

### Mock Mode (For Development)

Mock mode can be enabled if needed:

```bash
# Add this to your .env.local file
REACT_APP_USE_MOCK_BACKUP=true
REACT_APP_USE_MOCK_TELEMETRY=true
REACT_APP_USE_MOCK_WEBSOCKET=true
```

Or enable mock mode for all services:
```bash
REACT_APP_USE_MOCK_ALL=true
```

### Demonstrating BackupService Integration

1. Navigate to Backup → Jobs to see real backup jobs from the system
2. Create a new backup job to test API integration
3. Monitor job status updates via WebSocket notifications
4. View backup targets to see available storage locations
5. All operations now connect to the real API with retry logic and error handling

### WebSocket Service Improvements

The WebSocketService has been significantly enhanced with the following features:

1. **Flexible Message Handling**: The service now accepts both enum values and string message types for subscriptions
2. **String Command Support**: Send commands like `wsService.send('get_disks')` without creating message objects
3. **Command with Data Support**: Send command with payload data like `wsService.send('get_pool_performance', { pool_name: 'main' })`
4. **Auto Reconnection**: The service will automatically attempt to reconnect if the connection is lost
5. **Connection Status Monitoring**: Components can subscribe to connection status updates
6. **Mock Mode Support**: Automatically switches to mock mode if real connection fails after several attempts
7. **Comprehensive Test Suite**: All features are now covered by extensive unit tests

Example usage:
```typescript
// Subscribe to notifications (with string type)
wsService.subscribe('notification', (message) => {
  console.log('Received notification:', message.data);
});

// Or using enum
wsService.subscribe(WebSocketMessageType.NOTIFICATION, (message) => {
  console.log('Received notification:', message.data);
});

// Send a command (simple string)
wsService.send('get_disks');

// Send a command with data
wsService.send('get_pool_performance', {
  pool_name: 'main_pool',
  time_range: '7d'
});
```

### Demonstrating NotificationCenter

The NotificationCenter is visible in the top-right corner of the application as a bell icon. Key features include:

- Real-time notifications from system events
- Notification badge showing unread count
- Click to open notifications popover/drawer
- Filter notifications by type (all, unread, info, warning, error, success)
- Mark notifications as read
- Clear notifications
- Notification settings
- View notification details
- Take action on actionable notifications
- Notification sounds (when enabled)

### Triggering Test Notifications

For demonstration purposes, you can still trigger test notifications:

1. Open your browser's developer console (F12 or Right-click → Inspect → Console)
2. Copy and paste the entire content of `src/demo-notification.js` into the console
3. Use the following commands to trigger notifications:

```javascript
// Send a single notification
sendMockNotification('info', 'This is a test notification', 'Demo');

// Send an actionable notification
sendActionableNotification('System requires your attention', 'System');

// Send multiple random notifications
sendMultipleNotifications(10);
```

Try different notification types:
```javascript
sendMockNotification('info', 'Information notification');
sendMockNotification('warning', 'Warning notification');
sendMockNotification('error', 'Error notification');
sendMockNotification('success', 'Success notification');
```

### Testing Success

We've successfully fixed the test issues in:
- `SystemMonitor.test.tsx`
- `NotificationCenter.test.tsx`
- `BackupService.ts`
- `WebSocketService.test.ts`

You can verify this by running:
```bash
npm test -- --watchAll=false "src/__tests__/components/(SystemMonitor|NotificationCenter).test.tsx"
npm test -- --watchAll=false "src/__tests__/services/WebSocketService.test.ts"
```

## Architecture Overview

### BackupService
1. Connects to real API endpoints for backup management
2. Provides retry mechanism for API calls with configurable retries
3. Monitors connection status to the API
4. Falls back to mock data only when explicitly configured
5. Includes proper error handling and logging

### WebSocket Service
1. Maintains a real-time connection to system events
2. Subscribes to multiple message types (notifications, telemetry, etc.)
3. Maintains connection status and reconnects automatically
4. Provides message filtering and callback management
5. Supports both string and enum message type references
6. Handles string commands and command+data combinations
7. Robust test coverage with simulated WebSocket interactions

### NotificationCenter
1. Connects to the WebSocket service for real-time notifications
2. Maintains a list of notifications in state
3. Provides UI for viewing, managing and interacting with notifications
4. Persists user settings in localStorage
5. Triggers sound notifications for important events

## Troubleshooting

### Connection Issues
If you're having trouble connecting to the backend services:

1. Verify the API servers are running
2. Check that the URLs in `.env.local` match your server configuration
3. Ensure your firewall allows connections to the API ports
4. Try enabling mock mode temporarily to verify UI functionality

### API Access Errors
If you see API access errors:

1. Check browser console for detailed error messages
2. Verify that you have proper authentication credentials
3. Try restarting the API server

### Development Tips
For continued development:

1. Use the environment utilities in `src/utils/env.ts` to handle environment configuration
2. Test both live and mock modes before deployment
3. Use the retry pattern implemented in the services for all API calls

## Future Improvements

- Enhance connection status indicators
- Add offline mode capabilities
- Implement more robust retry strategies
- Improve real-time metrics visualization
- Add more notification interaction options
- Enhance error handling and user feedback

- Complete remaining UI component tests
- Add more notification interaction options
- Enhance notification details view
- Improve accessibility features
- Add notification grouping by source/category
- Implement notification snooze functionality

## WebSocket Integration with React Hooks

We've introduced a new custom React hook called `useWebSocket` that makes it easy to integrate real-time updates into any component. This hook provides a cleaner, more React-friendly way to interact with the WebSocket service.

### Using the useWebSocket Hook

The `useWebSocket` hook provides the following features:

- Automatic connection management
- Simple subscription to message types
- Connection status tracking
- Type-safe message handling
- Cleanup on component unmount

Example usage:

```tsx
import useWebSocket from '../../hooks/useWebSocket';
import { WebSocketMessageType } from '../../services/websocket.service';

const MyComponent = () => {
  const { connected, send } = useWebSocket({
    autoConnect: true,
    subscriptions: [
      {
        type: WebSocketMessageType.NOTIFICATION,
        handler: (message) => {
          console.log('Received notification:', message.data);
        }
      }
    ]
  });

  // Use connected status to show connection state
  return (
    <div>
      <div>Connection status: {connected ? 'Connected' : 'Disconnected'}</div>
      <button onClick={() => send('PING')}>Send Ping</button>
    </div>
  );
};
```

### Component Improvements

We've updated several components to use the new `useWebSocket` hook:

1. **NotificationCenter Component**
   - Now uses the `useWebSocket` hook to subscribe to notifications
   - Adds filtering by notification type
   - Adds sound notification support
   - Improved UI with tabs for different notification types

2. **SystemMonitor Component**
   - Subscribes to telemetry updates via the `useWebSocket` hook
   - Shows connection status indicator
   - Displays real-time system metrics
   - Features responsive layout with metric cards

### Advantages of the Hook Approach

The new hook-based approach offers several advantages:

1. **Component-Specific Subscriptions**: Each component only subscribes to the message types it needs
2. **Automatic Cleanup**: Subscriptions are automatically cleaned up when components unmount
3. **Reactive Connection Status**: Components can easily access the WebSocket connection status
4. **Type Safety**: Improved type checking for message handlers
5. **Consistent Pattern**: Standardized approach to WebSocket integration across components

### Future Enhancements

- Add support for reconnection strategies
- Implement a context provider for global WebSocket settings
- Add support for WebSocket authentication
- Create specialized hooks for specific message types 