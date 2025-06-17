import axios from 'axios';
import { API_BASE_URL } from '../constants';

// Use the API_BASE_URL from constants
const API_URL = API_BASE_URL;

export interface LoginResponse {
  token: string;
  refreshToken: string;
  tokenExpiry: number; // timestamp when token expires
  user: {
    id: string;
    username: string;
    role: string;
    email?: string;
    lastLogin?: string;
    preferences?: UserPreferences;
  };
}

export interface UserPreferences {
  theme?: 'light' | 'dark' | 'system';
  dashboardLayout?: string;
  language?: string;
  notifications?: boolean;
}

export interface PasswordPolicy {
  minLength: number;
  requireUppercase: boolean;
  requireLowercase: boolean;
  requireNumbers: boolean;
  requireSpecial: boolean;
}

export class AuthService {
  private static useMockAuth = process.env.NODE_ENV === 'development';
  private static readonly TOKEN_KEY = 'auth_token';
  private static readonly REFRESH_TOKEN_KEY = 'refresh_token';
  private static readonly TOKEN_EXPIRY_KEY = 'token_expiry';
  private static readonly USER_KEY = 'user';
  private static readonly LEGACY_API_KEY = 'apiKey';

  // Get the default password policy
  private static readonly DEFAULT_PASSWORD_POLICY: PasswordPolicy = {
    minLength: 8,
    requireUppercase: true,
    requireLowercase: true,
    requireNumbers: true,
    requireSpecial: true
  };

  static async login(username: string, password: string): Promise<LoginResponse> {
    try {
      let response;

      if (this.useMockAuth) {
        // Mock implementation for development
        if (username === 'admin' && password === 'admin') {
          const now = Date.now();
          const tokenExpiry = now + (30 * 60 * 1000); // 30 minutes

          response = {
            data: {
              token: 'mock-token-' + now,
              refreshToken: 'mock-refresh-token-' + now,
              tokenExpiry,
              user: {
                id: '1',
                username: 'admin',
                role: 'admin',
                email: 'admin@nestgate.local',
                lastLogin: new Date().toISOString(),
                preferences: {
                  theme: 'system',
                  dashboardLayout: 'default',
                  language: 'en',
                  notifications: true
                }
              }
            }
          };
          console.log('Using mock authentication in development mode');
        } else {
          throw new Error('Invalid credentials');
        }
      } else {
        // Use the real API endpoint for authentication
        response = await axios.post(`${API_URL}/api/auth/login`, { username, password });
      }
      
      // Store token and user details
      localStorage.setItem(this.TOKEN_KEY, response.data.token);
      localStorage.setItem(this.REFRESH_TOKEN_KEY, response.data.refreshToken);
      localStorage.setItem(this.TOKEN_EXPIRY_KEY, response.data.tokenExpiry.toString());
      localStorage.setItem(this.USER_KEY, JSON.stringify(response.data.user));
      
      // Also store as apiKey for backward compatibility
      localStorage.setItem(this.LEGACY_API_KEY, response.data.token);
      
      return response.data;
    } catch (error) {
      console.error('Login failed:', error);
      throw new Error('Invalid credentials');
    }
  }

  static async refreshToken(): Promise<LoginResponse> {
    try {
      const refreshToken = this.getRefreshToken();
      
      if (!refreshToken) {
        throw new Error('No refresh token available');
      }
      
      let response;
      
      if (this.useMockAuth) {
        // Mock implementation for development
        const user = this.getUser();
        const now = Date.now();
        const tokenExpiry = now + (30 * 60 * 1000); // 30 minutes

        response = {
          data: {
            token: 'mock-token-' + now,
            refreshToken: 'mock-refresh-token-' + now,
            tokenExpiry,
            user: user || {
              id: '1',
              username: 'admin',
              role: 'admin',
              email: 'admin@nestgate.local',
              lastLogin: new Date().toISOString()
            }
          }
        };
        console.log('Using mock token refresh in development mode');
      } else {
        // Call refresh token endpoint
        response = await axios.post(
          `${API_URL}/api/auth/refresh`,
          { refreshToken },
          { headers: { Authorization: `Bearer ${this.getToken()}` } }
        );
      }
      
      // Update token and user details
      localStorage.setItem(this.TOKEN_KEY, response.data.token);
      localStorage.setItem(this.REFRESH_TOKEN_KEY, response.data.refreshToken);
      localStorage.setItem(this.TOKEN_EXPIRY_KEY, response.data.tokenExpiry.toString());
      localStorage.setItem(this.USER_KEY, JSON.stringify(response.data.user));
      localStorage.setItem(this.LEGACY_API_KEY, response.data.token);
      
      return response.data;
    } catch (error) {
      console.error('Token refresh failed:', error);
      this.logout(); // Clear credentials on refresh failure
      throw error;
    }
  }

  static logout(): void {
    localStorage.removeItem(this.TOKEN_KEY);
    localStorage.removeItem(this.REFRESH_TOKEN_KEY);
    localStorage.removeItem(this.TOKEN_EXPIRY_KEY);
    localStorage.removeItem(this.USER_KEY);
    localStorage.removeItem(this.LEGACY_API_KEY);
  }

  static getToken(): string | null {
    return localStorage.getItem(this.TOKEN_KEY);
  }

  static getRefreshToken(): string | null {
    return localStorage.getItem(this.REFRESH_TOKEN_KEY);
  }

  static getTokenExpiry(): number | null {
    const expiry = localStorage.getItem(this.TOKEN_EXPIRY_KEY);
    return expiry ? parseInt(expiry, 10) : null;
  }

  static getUser(): any {
    const user = localStorage.getItem(this.USER_KEY);
    return user ? JSON.parse(user) : null;
  }

  static isAuthenticated(): boolean {
    const token = this.getToken();
    const expiry = this.getTokenExpiry();
    
    if (!token || !expiry) {
      return false;
    }
    
    // Check if token is still valid (not expired)
    return Date.now() < expiry;
  }

  static getAuthHeader() {
    const token = this.getToken();
    return token ? { Authorization: `Bearer ${token}` } : {};
  }

  static getPasswordPolicy(): PasswordPolicy {
    // In a real app, this could be fetched from the server
    return this.DEFAULT_PASSWORD_POLICY;
  }

  static validatePassword(password: string): { valid: boolean; errors: string[] } {
    const policy = this.getPasswordPolicy();
    const errors: string[] = [];
    
    if (password.length < policy.minLength) {
      errors.push(`Password must be at least ${policy.minLength} characters long`);
    }
    
    if (policy.requireUppercase && !/[A-Z]/.test(password)) {
      errors.push('Password must contain at least one uppercase letter');
    }
    
    if (policy.requireLowercase && !/[a-z]/.test(password)) {
      errors.push('Password must contain at least one lowercase letter');
    }
    
    if (policy.requireNumbers && !/[0-9]/.test(password)) {
      errors.push('Password must contain at least one number');
    }
    
    if (policy.requireSpecial && !/[^A-Za-z0-9]/.test(password)) {
      errors.push('Password must contain at least one special character');
    }
    
    return {
      valid: errors.length === 0,
      errors
    };
  }

  static async updateUserProfile(userId: string, profileData: Partial<{ email: string; preferences: UserPreferences }>): Promise<any> {
    try {
      let response;
      
      if (this.useMockAuth) {
        // Mock implementation for development
        const user = this.getUser();
        if (!user) {
          throw new Error('User not found');
        }
        
        const updatedUser = {
          ...user,
          ...profileData,
          preferences: {
            ...user.preferences,
            ...profileData.preferences
          }
        };
        
        localStorage.setItem(this.USER_KEY, JSON.stringify(updatedUser));
        
        response = {
          data: updatedUser
        };
        console.log('Using mock profile update in development mode', updatedUser);
      } else {
        // Call the real API endpoint
        response = await authAxios.put(`${API_URL}/api/users/${userId}/profile`, profileData);
      }
      
      return response.data;
    } catch (error) {
      console.error('Profile update failed:', error);
      throw error;
    }
  }

  static async changePassword(userId: string, currentPassword: string, newPassword: string): Promise<boolean> {
    try {
      // Validate the new password first
      const validation = this.validatePassword(newPassword);
      if (!validation.valid) {
        throw new Error(validation.errors.join(', '));
      }
      
      if (this.useMockAuth) {
        // Mock implementation for development
        console.log('Mock password change for user:', userId);
        
        // Simulate checking the current password
        if (currentPassword !== 'admin') {
          throw new Error('Current password is incorrect');
        }
        
        return true;
      } else {
        // Call the real API endpoint
        const response = await authAxios.post(`${API_URL}/api/users/${userId}/change-password`, {
          currentPassword,
          newPassword
        });
        
        return response.status === 200;
      }
    } catch (error) {
      console.error('Password change failed:', error);
      throw error;
    }
  }
}

// Create axios instance with auth header
export const authAxios = axios.create({
  baseURL: API_URL
});

// Add interceptor to add auth header to all requests
authAxios.interceptors.request.use(
  (config) => {
    const token = AuthService.getToken();
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Add response interceptor to handle token expiration
authAxios.interceptors.response.use(
  (response) => {
    return response;
  },
  async (error) => {
    const originalRequest = error.config;
    
    // If the error is due to an expired token and we haven't tried to refresh yet
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;
      
      try {
        // Try to refresh the token
        await AuthService.refreshToken();
        
        // Update the token in the current request
        originalRequest.headers.Authorization = `Bearer ${AuthService.getToken()}`;
        
        // Retry the original request
        return authAxios(originalRequest);
      } catch (refreshError) {
        // If refresh fails, redirect to login
        AuthService.logout();
        window.location.href = '/login';
        return Promise.reject(refreshError);
      }
    }
    
    return Promise.reject(error);
  }
); 