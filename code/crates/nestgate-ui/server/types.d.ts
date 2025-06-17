/**
 * Type declarations for modules without their own type definitions
 */

// Declare modules for JS files that haven't been converted to TS yet
declare module '*.js' {
  const value: any;
  export default value;
}

import { User } from './types/index';

declare global {
  namespace Express {
    interface Request {
      user?: User;
    }
  }
} 