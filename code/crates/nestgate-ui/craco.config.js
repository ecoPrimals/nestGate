const path = require('path');

module.exports = {
  webpack: {
    configure: (webpackConfig) => {
      // Find and remove WorkboxWebpackPlugin
      const workboxPluginIndex = webpackConfig.plugins.findIndex(
        (plugin) => plugin.constructor.name === 'GenerateSW'
      );
      
      if (workboxPluginIndex !== -1) {
        webpackConfig.plugins.splice(workboxPluginIndex, 1);
      }
      
      return webpackConfig;
    },
    alias: {
      '@components': path.resolve(__dirname, 'src/components'),
      '@services': path.resolve(__dirname, 'src/services'),
      '@utils': path.resolve(__dirname, 'src/utils')
    }
  },
  jest: {
    configure: {
      moduleNameMapper: {
        '^@components/(.*)$': '<rootDir>/src/components/$1',
        '^@services/(.*)$': '<rootDir>/src/services/$1',
        '^@utils/(.*)$': '<rootDir>/src/utils/$1'
      }
    }
  }
}; 