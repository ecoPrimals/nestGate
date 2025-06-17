/**
 * Custom Babel configuration for NestGate
 * 
 * This disables React Fast Refresh to avoid runtime.js errors
 */
module.exports = {
  presets: [
    '@babel/preset-env',
    '@babel/preset-react',
    '@babel/preset-typescript'
  ],
  plugins: [
    // React Fast Refresh is explicitly disabled
    // This is because we're having issues with the runtime.js module
    // which causes errors when imported from outside src directory
    // We use a mock implementation in src/node_modules/react-refresh instead
  ]
}; 