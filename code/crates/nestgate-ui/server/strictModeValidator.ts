/**
 * Strict Mode Validator Service
 * 
 * Validates that the server is properly configured for strict mode operation
 * and provides warnings if placeholder data might be used.
 */

import * as fs from 'fs';
import * as path from 'path';
import { isStrictLiveMode, getConfig } from '../src/config';

/**
 * Interface for validation results
 */
interface ValidationResult {
  valid: boolean;
  warnings: string[];
  errors: string[];
}

/**
 * Validates that the server is properly configured for strict mode
 */
export function validateStrictMode(): ValidationResult {
  const result: ValidationResult = {
    valid: true,
    warnings: [],
    errors: []
  };

  // Get the current configuration
  const config = getConfig();
  
  // Check if strict mode is enabled
  if (!isStrictLiveMode()) {
    result.warnings.push('Server is not running in strict mode. Placeholder data may be used.');
    result.valid = false;
  }
  
  // Check environment variables
  if (!config.STRICT_DATA_MODE) {
    result.errors.push('STRICT_DATA_MODE is not set to true. Set this environment variable to enforce strict mode.');
    result.valid = false;
  }
  
  if (!config.USE_REAL_DISKS) {
    result.errors.push('USE_REAL_DISKS is not set to true. Set this environment variable to use real disk data.');
    result.valid = false;
  }
  
  // Check for existence of old mock data paths that may have been missed in cleanup
  const mockDataPaths = [
    path.join(process.cwd(), 'src', 'mock'),
    path.join(process.cwd(), 'public', 'mock'),
    path.join(process.cwd(), 'server', 'mock'),
    path.join(process.cwd(), 'crates', 'nestgate-ui', 'src', 'mock')
  ];
  
  mockDataPaths.forEach(mockPath => {
    if (fs.existsSync(mockPath)) {
      result.warnings.push(`Mock data directory found at ${mockPath}. This could contain legacy mock data that might cause issues.`);
    }
  });
  
  return result;
}

/**
 * Logs validation results to the console
 */
export function logValidationResults(results: ValidationResult): void {
  console.log('\n=== STRICT MODE VALIDATION ===');
  
  if (results.valid) {
    console.log('✅ Server is properly configured for strict mode operation.');
  } else {
    console.log('⚠️ Server is not properly configured for strict mode operation:');
    
    if (results.errors.length > 0) {
      console.log('\n🚫 ERRORS:');
      results.errors.forEach(error => {
        console.log(`  - ${error}`);
      });
    }
    
    if (results.warnings.length > 0) {
      console.log('\n⚠️ WARNINGS:');
      results.warnings.forEach(warning => {
        console.log(`  - ${warning}`);
      });
    }
  }
  
  console.log('==============================\n');
}

/**
 * Run validation during server startup
 */
export function validateStrictModeOnStartup(): void {
  const results = validateStrictMode();
  logValidationResults(results);
  
  if (!results.valid && isStrictLiveMode()) {
    console.warn('\n⛔ WARNING: Server is configured to run in strict mode but validation failed.\n');
  }
}

export default {
  validateStrictMode,
  logValidationResults,
  validateStrictModeOnStartup
}; 