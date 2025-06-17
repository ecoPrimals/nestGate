/**
 * Route Guards
 * 
 * Provides route protection and navigation control based on feature availability.
 */

import { Feature, isFeatureEnabled } from './featureFlags';

/**
 * Route restriction type
 */
export enum RouteRestriction {
  NONE = 'none',               // No restrictions
  PREVIEW = 'preview',         // Preview features that may not be fully ready
  PROTECTED = 'protected',     // Routes that require authentication
  ADMIN_ONLY = 'adminOnly'     // Routes that require admin privileges
}

/**
 * Protected route definition
 */
export interface ProtectedRoute {
  path: string;
  restriction: RouteRestriction;
  requiredFeatures?: Feature[];
}

/**
 * List of protected routes with their restrictions
 */
export const protectedRoutes: ProtectedRoute[] = [
  // Core application routes
  { path: '/dashboard', restriction: RouteRestriction.NONE },
  { path: '/pools', restriction: RouteRestriction.NONE },
  { path: '/datasets', restriction: RouteRestriction.NONE },
  { path: '/snapshots', restriction: RouteRestriction.NONE },
  
  // Advanced features
  { path: '/backup', restriction: RouteRestriction.PREVIEW, requiredFeatures: [Feature.REMOTE_BACKUP] },
  { path: '/replication', restriction: RouteRestriction.PREVIEW, requiredFeatures: [Feature.REPLICATION] },
  { path: '/encryption', restriction: RouteRestriction.PREVIEW, requiredFeatures: [Feature.ENCRYPTION] },
  
  // Admin routes
  { path: '/admin', restriction: RouteRestriction.ADMIN_ONLY },
  { path: '/settings', restriction: RouteRestriction.ADMIN_ONLY },
  { path: '/system', restriction: RouteRestriction.ADMIN_ONLY }
];

/**
 * Get the protected route definition for a path
 */
export function getRouteRestriction(path: string): ProtectedRoute | undefined {
  // Check exact matches first
  const exactMatch = protectedRoutes.find(route => route.path === path);
  if (exactMatch) {
    return exactMatch;
  }
  
  // Check parent paths if no exact match
  // e.g., /admin/users should inherit restrictions from /admin
  return protectedRoutes.find(route => 
    path.startsWith(route.path + '/')
  );
}

/**
 * Check if a route is accessible
 */
export function isRouteAccessible(path: string): boolean {
  const route = getRouteRestriction(path);
  
  // If no route definition is found, allow access by default
  if (!route) {
    return true;
  }
  
  // Check route restriction
  switch (route.restriction) {
    case RouteRestriction.NONE:
      return true;
      
    case RouteRestriction.PREVIEW:
      // For preview features, check if all required features are enabled
      if (route.requiredFeatures && route.requiredFeatures.length > 0) {
        return route.requiredFeatures.every(feature => isFeatureEnabled(feature));
      }
      return true;
      
    case RouteRestriction.PROTECTED:
      // TODO: Implement authentication check
      return true;
      
    case RouteRestriction.ADMIN_ONLY:
      // TODO: Implement admin privilege check
      return true;
      
    default:
      return true;
  }
}

/**
 * Get a redirect path if a route is not accessible
 */
export function getRedirectPath(path: string): string | null {
  if (isRouteAccessible(path)) {
    return null;
  }
  
  return '/dashboard';
}

export default {
  RouteRestriction,
  protectedRoutes,
  getRouteRestriction,
  isRouteAccessible,
  getRedirectPath
}; 