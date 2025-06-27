# NestGate Startup Guide

This guide explains how to start the NestGate system using the unified TypeScript startup scripts.

## Quick Start

### Start Everything (Recommended)

The simplest way to start everything (server, API, React frontend) in strict live mode:

```bash
npm run start
```

This will:
1. Run the prestart script to ensure React symlinks and dependencies
2. Start the port manager (if needed)
3. Kill any conflicting processes
4. Compile TypeScript files
5. Start the server, API server, and React frontend
6. Use proper port allocation

### Mode Options

NestGate can run in different modes:

- **Strict Live Mode (Default)** - Uses real data, never falls back to mocks
  ```bash
  npm run start
  # or
  npm run start:strict
  ```

- **Live Mode** - Uses real data, can fall back to mocks if needed
  ```bash
  npm run start:live
  ```

- **Mock Mode** - Uses mock data only
  ```bash
  npm run start:mock
  ```

### Backend Only

To start just the backend (server + API) without the React frontend:

```bash
npm run start:noui
```

### Individual Components

To run individual components:

**Server Only:**
```bash
npm run start:server          # Strict live mode
npm run start:server:mock     # Mock mode
```

**API Server Only:**
```bash
npm run start:api             # Strict live mode
npm run start:api:mock        # Mock mode
```

## Development

For development with ESLint disabled:

```bash
npm run dev           # Strict live mode
npm run dev:mock      # Mock mode
```

### Windows Development

On Windows:

```powershell
npm run dev:win           # Strict live mode
npm run dev:win:mock      # Mock mode
```

## Port Management

NestGate uses a port manager to allocate and manage ports. By default:

- React UI: Port 3000
- NestGate Server: Port 3002
- API Server: Port 3003

To manually start/stop the port manager:

```bash
npm run port-manager           # Linux/Mac
npm run port-manager:win       # Windows
```

## Stopping Services

To stop all running Node.js processes:

```bash
npm run stop           # Linux/Mac
npm run stop:win       # Windows
```

## TypeScript Development

All scripts are now written in TypeScript. To compile them:

```bash
npm run build:scripts
```

The following TypeScript configurations are used:

- `tsconfig.json` - For the React frontend
- `tsconfig.server.json` - For the server and API
- `scripts/tsconfig.json` - For the startup scripts

## Project Structure

The startup system is organized as follows:

```
scripts/
├── core/
│   ├── startup.ts     - Main startup system
│   └── service-runner.ts - Individual service runner
├── utils/
│   └── core.ts        - Core utilities
├── prestart.ts        - Prestart setup
├── start-unified.ts   - Unified startup entry point
├── start-server.ts    - Server-only starter
└── start-api.ts       - API-only starter
```

## Troubleshooting

### React Refresh Issues

React refresh symlinks are automatically created by the prestart script. If you still encounter issues:

```bash
# Check the symlink exists
ls -la src/node_modules/react-refresh

# If it doesn't, manually create it
mkdir -p src/node_modules
ln -sf ../../node_modules/react-refresh src/node_modules/
```

### TypeScript Compilation Errors

Fix TypeScript errors and then recompile:

```bash
npm run build:server
npm run build:scripts
```

### Port Conflicts

If you encounter port conflicts:

1. Stop all services: `npm run stop`
2. Kill processes on specific ports:
   ```bash
   # Linux/Mac
   lsof -i:3000 -t | xargs kill -9
   lsof -i:3002 -t | xargs kill -9
   lsof -i:3003 -t | xargs kill -9
   ``` 