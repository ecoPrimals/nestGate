/**
 * Type definitions for Filesystem Monitor client
 */

/**
 * Event type enumeration
 */
export enum EventKind {
  Create = 'Create',
  Modify = 'Modify',
  Delete = 'Delete',
  Rename = 'Rename',
  Permissions = 'Permissions',
  Other = 'Other'
}

/**
 * Event metadata
 */
export interface EventMetadata {
  fileSize?: number;
  extension?: string;
  modified?: string;
  created?: string;
  attributes: Record<string, string>;
}

/**
 * Filesystem event information
 */
export interface FsEvent {
  kind: EventKind;
  path: string;
  relatedPath?: string;
  timestamp: string;
  isDirectory: boolean;
  metadata?: EventMetadata;
}

/**
 * Event filter configuration
 */
export interface EventFilter {
  kinds?: EventKind[];
  includePatterns?: string[];
  excludePatterns?: string[];
  extensions?: string[];
  includeDirectories: boolean;
  includeHidden: boolean;
}

/**
 * Watch directory request
 */
export interface WatchRequest {
  path: string;
  recursive?: boolean;
  id?: string;
  includePatterns?: string[];
  excludePatterns?: string[];
  extensions?: string[];
  includeDirectories?: boolean;
  includeHidden?: boolean;
  onEventCommand?: string;
}

/**
 * Watch information
 */
export interface WatchInfo {
  id: string;
  path: string;
  options: {
    recursive: boolean;
    filter: EventFilter;
    onEventCommand?: string;
  };
}

/**
 * API response wrapper
 */
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
} 