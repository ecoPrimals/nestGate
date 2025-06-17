/**
 * Data Provider Factory
 * 
 * Creates and returns the data provider for accessing system data.
 * For production use, this always returns the real data provider.
 * Mock implementations are only used in tests via Jest mocking.
 */

import { DataProvider } from './interfaces';
import { createRealDataProvider } from './real/realDataProvider';

/**
 * In-memory cache of the data provider instance
 */
let dataProviderInstance: DataProvider | null = null;

/**
 * Creates or returns a cached data provider
 * In production, this always uses real data with no mock fallbacks
 */
export function getDataProvider(): DataProvider {
  // Return cached instance if available
  if (dataProviderInstance) {
    return dataProviderInstance;
  }
  
  // Create the real data provider
  try {
    dataProviderInstance = createRealDataProvider();
  } catch (error) {
    throw new Error(`Failed to create data provider: ${error instanceof Error ? error.message : String(error)}`);
  }
  
  // Return the created instance
  return dataProviderInstance;
}

/**
 * Reset the data provider instance (useful for testing)
 */
export function resetDataProvider(): void {
  dataProviderInstance = null;
}

export default {
  getDataProvider,
  resetDataProvider
}; 