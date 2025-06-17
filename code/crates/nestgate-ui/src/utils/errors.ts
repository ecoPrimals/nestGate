/**
 * Custom error types for NestGate application
 * 
 * These error types make error handling more consistent and provides
 * better context for errors across the application.
 */

/**
 * Error thrown when data is unavailable from a service
 * Replaces previous pattern of falling back to mock data
 */
export class DataUnavailableError extends Error {
  public readonly originalError?: Error;
  public readonly serviceName: string;
  public readonly retryable: boolean;

  constructor(message: string, options?: {
    originalError?: Error;
    serviceName?: string;
    retryable?: boolean;
  }) {
    super(message);
    this.name = 'DataUnavailableError';
    this.originalError = options?.originalError;
    this.serviceName = options?.serviceName || 'unknown';
    this.retryable = options?.retryable !== undefined ? options.retryable : true;

    // Ensures proper prototype chain for instanceof checks
    Object.setPrototypeOf(this, DataUnavailableError.prototype);
  }

  /**
   * Get the full error message including original error details if available
   */
  public getFullMessage(): string {
    if (this.originalError) {
      return `${this.message} (Original error: ${this.originalError.message})`;
    }
    return this.message;
  }
}

/**
 * Error thrown when an API request fails
 */
export class ApiError extends Error {
  public readonly status?: number;
  public readonly retryable: boolean;
  public readonly originalError?: Error;

  constructor(message: string, options?: {
    status?: number;
    retryable?: boolean;
    originalError?: Error;
  }) {
    super(message);
    this.name = 'ApiError';
    this.status = options?.status;
    this.retryable = options?.retryable !== undefined ? options.retryable : true;
    this.originalError = options?.originalError;

    // Ensures proper prototype chain for instanceof checks
    Object.setPrototypeOf(this, ApiError.prototype);
  }
} 