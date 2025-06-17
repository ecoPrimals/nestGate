# NestGate UI Run Instructions

This document provides instructions for running the NestGate UI in various configurations.

## Quick Start with NPM

```bash
# Start everything in live mode (backend + UI)
npm start

# Start with mock data for testing
npm run test:mock

# Start in strict live mode (no fallback to mock data)
npm run start:strict

# Stop all services
npm run stop
```

## Using Shell Scripts Directly

To run the complete stack (backend + UI):

```bash
# Start everything in live mode
./scripts/start-full-stack.sh

# Start with mock data
./scripts/start-full-stack.sh --mock

# Start in strict live mode (no fallback to mock data)
./scripts/start-full-stack.sh --strict
```

To stop all services:

```bash
./scripts/stop-full-stack.sh
```

## Component Start Options

### Backend Server Only

```bash
# Using npm
npm run start:backend

# Using script directly
./scripts/start-backend-server.sh
```

Stop the backend:

```bash
# Using npm
npm run stop:backend

# Using script directly
./scripts/stop-backend-server.sh
```

### UI Only (if backend is already running)

```bash
# Using npm - live mode
npm run start:ui-live

# Using npm - mock mode
npm run start:ui-mock

# Using scripts directly
./scripts/start-dev.sh --live
./scripts/start-dev.sh --mock
./scripts/start-dev.sh --live --strict
```

Stop the UI:

```bash
# Using npm
npm run stop:ui

# Using script directly
./scripts/stop-dev.sh
```

## Development Commands

For development workflows:

```bash
# Development mode with full stack (live data)
npm run dev

# Development mode with mock data
npm run dev:mock
```

## Troubleshooting

### WebSocket Connection Issues

If you see WebSocket connection errors in the browser console:

1. Check if the backend server is running:
   ```bash
   lsof -i :8080
   ```
   
2. If not, start it:
   ```bash
   npm run start:backend
   ```

3. Restart the UI if needed:
   ```bash
   npm run restart
   ```

### Router Error

If you see a React Router error about nested routers:

1. This indicates that there are multiple `<Router>` components in the application
2. The fix is to make sure there's only one `<BrowserRouter>` in the entire application
3. If you see this error, please check `src/index.tsx` and `src/routes/index.tsx`

## Understanding Data Modes

- **Live Mode**: UI connects to real backend services
- **Mock Mode**: UI uses simulated data for development
- **Strict Live Mode**: UI only uses real data, never falls back to mock data

## Port Usage

- UI: http://localhost:3000
- WebSocket Backend: ws://localhost:8080/ws 