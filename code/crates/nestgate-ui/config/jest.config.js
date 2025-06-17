/**
 * Jest Configuration
 * 
 * This config ensures tests use the mock data provider rather than
 * trying to access real APIs.
 */

module.exports = {
  // Root directory is the current directory
  rootDir: '.',
  
  // Test files pattern
  testMatch: [
    '<rootDir>/src/**/__tests__/**/*.{js,jsx,ts,tsx}',
    '<rootDir>/src/**/*.{spec,test}.{js,jsx,ts,tsx}'
  ],
  
  // Files to transform
  transform: {
    '^.+\\.(js|jsx|ts|tsx)$': ['babel-jest', { presets: ['react-app'] }]
  },
  
  // Module name mapping
  moduleNameMapper: {
    '^src/(.*)$': '<rootDir>/src/$1',
    '\\.(css|less|scss|sass)$': 'identity-obj-proxy'
  },
  
  // Code coverage configuration
  collectCoverageFrom: [
    'src/**/*.{js,jsx,ts,tsx}',
    '!src/**/*.d.ts',
    '!src/index.tsx',
    '!src/reportWebVitals.ts',
    '!src/setupTests.ts'
  ],
  
  // Files/directories to ignore
  testPathIgnorePatterns: [
    '/node_modules/',
    '/dist/',
    '/build/'
  ],
  
  // Setup files for the test environment
  setupFilesAfterEnv: [
    './__mocks__/setupTests.ts'
  ],
  
  // Test environment
  testEnvironment: 'jsdom',
  
  // Automatically clear mock calls between tests
  clearMocks: true,
  
  // Coverage directory
  coverageDirectory: 'coverage',
  
  // Indicates whether the coverage information should be collected while executing the test
  collectCoverage: false,
  
  // The directory where Jest should output its coverage files
  coverageReporters: ['text', 'lcov', 'clover'],
  
  // Indicates whether each individual test should be reported during the run
  verbose: true
}; 