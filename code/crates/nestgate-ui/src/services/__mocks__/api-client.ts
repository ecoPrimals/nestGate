/**
 * Mock API Client for Testing
 * 
 * Used in test environment to simulate API responses without actual HTTP calls
 */

import axios from 'axios';
import { API_BASE_URL } from '../../config';

// Create mock axios instance for API calls
const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 10000 // 10 second timeout
});

// Mock request interceptor
apiClient.interceptors.request.use(
  (config) => {
    console.log('Mock API Client: Request intercepted', config);
    return config;
  },
  (error) => {
    console.error('Mock API Client: Request error', error);
    return Promise.reject(error);
  }
);

// Mock response interceptor
apiClient.interceptors.response.use(
  (response) => {
    console.log('Mock API Client: Response intercepted', response);
    return response;
  },
  (error) => {
    console.error('Mock API Client: Response error', error);
    return Promise.reject(error);
  }
);

export default apiClient; 