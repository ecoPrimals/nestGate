# NestGate UI

## Quick Start Guide

The application now supports running both the backend and UI in a single command:

```bash
# Start everything in live mode (backend + UI)
npm start

# Start with mock data for testing
npm run test:mock

# Stop all services
npm run stop
```

For detailed setup instructions, see [RUN_INSTRUCTIONS.md](./RUN_INSTRUCTIONS.md).

# NestGate UI Application

Modern React UI for NestGate storage management.

## Development Quick Start

```bash
# Install dependencies
npm install

# Start development server
npm start                 # Default mode
npm run start:live        # Live data mode
npm run start:mock        # Mock data mode
npm run start:strict      # Strict live mode

# Stop running server
npm run stop
```

## Data Modes

The NestGate UI supports multiple data modes to facilitate development, testing, and production deployments:

| Mode           | Description                                       | Command                 |
|----------------|---------------------------------------------------|-------------------------|
| Default        | Regular development mode                          | `npm start`             |
| Mock           | Simulated data for all services                   | `npm run start:mock`    |
| Live           | Connect to real backend services                  | `npm run start:live`    |
| Strict Live    | Force live connections, no fallbacks to mock data | `npm run start:strict`  |

You can also use command-line options with `start-dev.sh` directly:

```bash
./start-dev.sh --live      # Live data mode
./start-dev.sh --mock      # Force mock data mode
./start-dev.sh --strict    # Strict live mode (implies --live)
```

For more details about live mode implementation, see [README_LIVE_MODE.md](src/README_LIVE_MODE.md).

## Building for Production

```bash
npm run build
```

## Testing

```bash
# Run tests once
npm test

# Run tests in watch mode
npm run test:watch
```

## Code Quality

```bash
# Lint code
npm run lint

# Format code
npm run format
```

## Documentation

The project includes several documentation files:

- [README_LIVE_MODE.md](src/README_LIVE_MODE.md): Live mode implementation guide
- [DEMO.md](DEMO.md): Demo mode instructions
- [DEMO_INSTRUCTIONS.md](DEMO_INSTRUCTIONS.md): Additional demo setup guide

## Architecture

The NestGate UI follows these architecture principles:

1. **Component-Based**: UI elements are built as reusable React components
2. **Live-Data First**: Components are designed to work with real data by default
3. **Graceful Fallbacks**: Components handle connection failures and provide meaningful feedback
4. **Mock Data Segregation**: All mock data generation is isolated for easy disabling in production

## Deployment

For production deployment, build the application and serve it with a static file server:

```bash
npm run build
npm run static-demo   # Serves the built app for testing
```

## License

Copyright © 2023 NestGate Storage Solutions, Inc. All Rights Reserved. 