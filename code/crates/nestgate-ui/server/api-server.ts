/**
 * NestGate API Server
 * 
 * Provides dedicated API endpoints for NestGate services
 */

import express from 'express';
import cors from 'cors';
import morgan from 'morgan';
import apiRouter from './routes/api';

// Initialize Express app
const app = express();

// Require API_PORT from environment - no hardcoded fallbacks
const API_PORT = process.env.API_PORT;
if (!API_PORT) {
  console.error('ERROR: API_PORT environment variable is required');
  console.error('This should be set by the NestGate Port Manager');
  process.exit(1);
}

const port = parseInt(API_PORT, 10);
if (isNaN(port) || port < 1 || port > 65535) {
  console.error(`ERROR: Invalid API_PORT value: ${API_PORT}`);
  process.exit(1);
}

// Configure middleware
app.use(cors());
app.use(express.json());
app.use(morgan('dev'));

// Set up API routes
app.use('/api', apiRouter);

// Handle root route
app.get('/', (req, res) => {
  res.json({
    message: 'NestGate API Server',
    version: '0.1.0',
    status: 'running'
  });
});

// Error handler
app.use((err: Error, req: express.Request, res: express.Response, next: express.NextFunction) => {
  console.error(err.stack);
  res.status(500).json({
    error: 'Internal Server Error',
    message: process.env.NODE_ENV === 'production' ? 'An error occurred' : err.message
  });
});

// Start the API server
app.listen(port, () => {
  console.log(`NestGate API server running on port ${port}`);
  console.log(`API available at http://localhost:${port}/api`);
});

// Handle shutdown
process.on('SIGINT', () => {
  console.log('Shutting down API server...');
  process.exit(0);
}); 