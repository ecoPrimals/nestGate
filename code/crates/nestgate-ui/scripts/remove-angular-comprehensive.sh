#!/bin/bash

echo "====== COMPREHENSIVE ANGULAR REMOVAL SCRIPT ======"
echo "This script will systematically identify and remove all Angular-related files and dependencies"
echo "------------------------------------------------------"

# Create backup directory if it doesn't exist
BACKUP_DIR="angular-backup"
mkdir -p $BACKUP_DIR

# Log file for tracking removed items
LOG_FILE="$BACKUP_DIR/angular-removal-log.txt"
echo "Angular components removed on $(date)" > $LOG_FILE

# Function to backup a file before removing it
backup_file() {
  local file=$1
  local relative_path=${file#./}
  local target_dir="$BACKUP_DIR/$(dirname $relative_path)"
  
  mkdir -p "$target_dir"
  cp "$file" "$target_dir/"
  echo "Backed up: $file" >> $LOG_FILE
}

# Function to check if file contains Angular imports
contains_angular() {
  grep -q "import.*@angular\|import.*angular" "$1"
  return $?
}

echo "Step 1: Moving existing Angular backup files if any..."
if [ -d ".angular" ]; then
  echo "  - Backing up .angular directory"
  mv -f .angular $BACKUP_DIR/ 2>/dev/null
  echo "Moved .angular directory to $BACKUP_DIR" >> $LOG_FILE
fi

echo "Step 2: Checking for Angular configuration files..."
ANGULAR_CONFIG_FILES=("angular.json" "angular-cli.json" ".angular-cli.json" "karma.conf.js" "ngsw-config.json")
for config_file in "${ANGULAR_CONFIG_FILES[@]}"; do
  if [ -f "$config_file" ]; then
    echo "  - Found $config_file, moving to backup"
    backup_file "$config_file"
    rm "$config_file"
    echo "Removed: $config_file" >> $LOG_FILE
  fi
done

echo "Step 3: Identifying and removing Angular components from app directory..."
if [ -d "app" ]; then
  echo "  - Moving app directory to backup"
  mv app $BACKUP_DIR/
  echo "Moved app directory to $BACKUP_DIR" >> $LOG_FILE
fi

echo "Step 4: Searching for Angular imports in source files..."
ANGULAR_FILES=$(find ./src -type f \( -name "*.ts" -o -name "*.tsx" \) -exec grep -l "import.*@angular\|import.*angular" {} \;)
for file in $ANGULAR_FILES; do
  echo "  - Found Angular file: $file"
  backup_file "$file"
  rm "$file"
  echo "Removed: $file" >> $LOG_FILE
done

echo "Step 5: Updating package.json to remove Angular dependencies..."
if [ -f "package.json" ]; then
  echo "  - Creating backup of package.json"
  backup_file "package.json"
  
  # Create a new package.json without Angular dependencies
  echo "  - Removing Angular dependencies from package.json"
  jq 'del(.dependencies | with_entries(select(.key | startswith("@angular")))) | 
      del(.devDependencies | with_entries(select(.key | startswith("@angular")))) | 
      del(.dependencies["angular"]) | 
      del(.dependencies["angular-cli"]) | 
      del(.devDependencies["angular-cli"]) | 
      del(.dependencies["@ngx-translate/core"]) | 
      del(.dependencies["@ngx-translate/http-loader"]) | 
      del(.dependencies["zone.js"])' package.json > package.json.new
  
  # Only replace if jq command succeeded
  if [ $? -eq 0 ]; then
    mv package.json.new package.json
    echo "Updated package.json to remove Angular dependencies" >> $LOG_FILE
  else
    echo "  - Error: Failed to update package.json"
    echo "Failed to update package.json" >> $LOG_FILE
    rm -f package.json.new
  fi
fi

echo "Step 6: Updating tsconfig.json for React compatibility..."
if [ -f "tsconfig.json" ]; then
  echo "  - Creating backup of tsconfig.json"
  backup_file "tsconfig.json"
  
  # Create a new React-focused tsconfig.json
  cat > tsconfig.json << 'EOF'
{
  "compilerOptions": {
    "baseUrl": "./",
    "outDir": "./dist",
    "forceConsistentCasingInFileNames": true,
    "strict": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "sourceMap": true,
    "declaration": false,
    "moduleResolution": "node",
    "target": "ES2022",
    "module": "ES2022",
    "lib": [
      "ES2022",
      "dom"
    ],
    "jsx": "react-jsx",
    "esModuleInterop": true,
    "allowSyntheticDefaultImports": true,
    "skipLibCheck": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "paths": {
      "@/*": ["./src/*"],
      "@components/*": ["./src/components/*"],
      "@services/*": ["./src/services/*"],
      "@utils/*": ["./src/utils/*"],
      "@hooks/*": ["./src/hooks/*"],
      "@types/*": ["./src/types/*"],
      "@assets/*": ["./src/assets/*"]
    }
  },
  "include": [
    "src/**/*"
  ],
  "exclude": [
    "node_modules",
    "angular-backup",
    "**/*.spec.ts",
    "**/*.test.ts",
    "**/*.spec.tsx",
    "**/*.test.tsx"
  ]
}
EOF
  echo "Updated tsconfig.json for React compatibility" >> $LOG_FILE
fi

echo "Step 7: Updating test configuration files..."
# Create/update test.ts for React testing
if [ -f "src/test.ts" ]; then
  echo "  - Backing up and updating src/test.ts"
  backup_file "src/test.ts"
  
  cat > src/test.ts << 'EOF'
// This file is required by jest.config.js and sets up the test environment
import '@testing-library/jest-dom';
EOF
  echo "Updated src/test.ts for React testing" >> $LOG_FILE
fi

# Create/update jest.config.js
echo "  - Creating/updating jest.config.js for React testing"
cat > jest.config.js << 'EOF'
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  moduleNameMapper: {
    "\\.(css|less|scss|sass)$": "identity-obj-proxy",
    "^@/(.*)$": "<rootDir>/src/$1"
  },
  setupFilesAfterEnv: [
    "<rootDir>/src/test.ts"
  ],
  transform: {
    "^.+\\.(ts|tsx)$": "ts-jest",
    "^.+\\.(js|jsx)$": "babel-jest"
  },
  transformIgnorePatterns: [
    "/node_modules/(?!antd|@ant-design|rc-.*?|@babel/runtime).+(js|jsx)$"
  ],
  testPathIgnorePatterns: [
    "/node_modules/",
    "/angular-backup/"
  ],
  collectCoverageFrom: [
    "src/**/*.{ts,tsx}",
    "!src/**/*.d.ts",
    "!src/index.tsx",
    "!src/test.ts"
  ],
  coverageDirectory: "coverage",
  coverageReporters: [
    "text",
    "lcov"
  ],
  testMatch: [
    "<rootDir>/src/**/__tests__/**/*.{ts,tsx}",
    "<rootDir>/src/**/*.{spec,test}.{ts,tsx}"
  ]
};
EOF
echo "Created/updated jest.config.js for React testing" >> $LOG_FILE

echo "Step 8: Checking for Angular styles in CSS/SCSS files..."
ANGULAR_STYLE_FILES=$(find ./src -type f \( -name "*.css" -o -name "*.scss" \) -exec grep -l "::ng-\|^\s*\[ng" {} \;)
for file in $ANGULAR_STYLE_FILES; do
  echo "  - Found Angular style file: $file"
  backup_file "$file"
  
  # Clean up Angular-specific styles but keep the file
  sed -i '/::ng-/d; /\[ng/d; /^\s*mat-/d' "$file"
  echo "Cleaned Angular styles from: $file" >> $LOG_FILE
done

echo "Step 9: Creating test script for React components..."
cat > test-react-components.sh << 'EOF'
#!/bin/bash

echo "Running tests for React components..."

# Find all test files
TEST_FILES=$(find src -name "*.spec.tsx" -o -name "*.test.tsx" -o -name "*.spec.ts" -o -name "*.test.ts" | grep -v "__tests__" | sort)

if [ -z "$TEST_FILES" ]; then
  echo "No test files found!"
  exit 1
fi

# Run tests
npx jest $TEST_FILES

echo "All React component tests completed!"
EOF
chmod +x test-react-components.sh
echo "Created test-react-components.sh" >> $LOG_FILE

echo "Step 10: Final cleanup and reporting..."
# Generate a report of removed files
echo "  - Generating removal report"
echo "====== Angular Removal Report ======" >> $LOG_FILE
echo "Removed on: $(date)" >> $LOG_FILE
echo "Total files backed up: $(find $BACKUP_DIR -type f | wc -l)" >> $LOG_FILE
echo "====================================" >> $LOG_FILE

echo ""
echo "====== ANGULAR REMOVAL COMPLETE ======"
echo "A backup of all removed files can be found in: $BACKUP_DIR"
echo "A detailed log file is available at: $LOG_FILE"
echo ""
echo "Next steps:"
echo "1. Run 'npm install' to update dependencies"
echo "2. Run './test-react-components.sh' to verify React components still work"
echo "3. Once confirmed working, you can delete the $BACKUP_DIR directory"
echo "======================================" 