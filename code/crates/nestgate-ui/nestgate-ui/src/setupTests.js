// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom';

// Import any additional setup from the original setupTests file
// This ensures compatibility with the existing test suite
try {
  require('../__mocks__/setupTests.ts');
} catch (e) {
  console.warn('Could not load original setupTests.ts. Some tests may fail:', e);
}

// Set up global mocks needed for tests
global.matchMedia = global.matchMedia || function() {
  return {
    matches: false,
    addListener: jest.fn(),
    removeListener: jest.fn(),
  };
}; 