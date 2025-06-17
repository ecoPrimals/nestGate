#!/bin/bash

echo "Removing Angular-related files and directories..."

# Create a backup directory
mkdir -p angular-backup

# First, remove the Angular cache directory
echo "Removing Angular cache directory..."
if [ -d ".angular" ]; then
  mv .angular angular-backup/
fi

# Remove angular.json if it exists
echo "Removing Angular configuration file..."
if [ -f "angular.json" ]; then
  mv angular.json angular-backup/
fi

# Remove Angular-related files in app directory
echo "Removing Angular components and services..."
if [ -d "app" ]; then
  mv app angular-backup/
fi

# Remove test.ts if it contains Angular imports
echo "Cleaning up Angular test files..."
if grep -q "import.*angular" src/test.ts; then
  mv src/test.ts angular-backup/
  
  # Create a new React-compatible test.ts
  cat > src/test.ts << 'EOF'
// This file is required by jest.config.js and sets up the test environment.
import '@testing-library/jest-dom';
EOF
fi

# Create a jest.config.js file if it doesn't exist
if [ ! -f "jest.config.js" ]; then
  echo "Creating Jest configuration file..."
  cat > jest.config.js << 'EOF'
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  moduleNameMapper: {
    "\\.(css|less|scss|sass)$": "identity-obj-proxy"
  },
  setupFilesAfterEnv: [
    "<rootDir>/src/test.ts"
  ],
  transformIgnorePatterns: [
    "/node_modules/(?!antd|@ant-design|rc-.*?|@babel/runtime).+(js|jsx)$"
  ],
  testPathIgnorePatterns: [
    "/node_modules/",
    "/angular-backup/"
  ]
};
EOF
fi

echo "Removing any remaining Angular references in source code..."
find src -type f -name "*.ts" -o -name "*.tsx" | xargs grep -l "import.*angular" | xargs -I{} mv {} angular-backup/

echo "Creating a record of removed files..."
find angular-backup -type f | sort > angular-backup/removed-files.txt

echo "Done! All Angular files have been moved to the 'angular-backup' directory."
echo "You can safely delete the angular-backup directory once you've verified everything works." 