# Angular Removal Guide for NestGate

This guide provides a step-by-step process for completely removing Angular dependencies from the NestGate project and transitioning to a React-based architecture.

## Prerequisites

Before beginning the Angular removal process, ensure you have:

- Backed up your current codebase
- Node.js and npm installed
- Git installed
- Basic understanding of React and Angular concepts

## Step 1: Run the Angular Removal Script

We've created a comprehensive script that automates most of the Angular removal process. The script performs the following actions:

1. Creates backups of all Angular files before removal
2. Removes Angular configuration files
3. Identifies and removes Angular components
4. Updates package.json to remove Angular dependencies
5. Configures tsconfig.json for React compatibility
6. Sets up testing infrastructure for React components

```bash
# Navigate to the UI directory
cd crates/nestgate-ui

# Make the script executable
chmod +x remove-angular-comprehensive.sh

# Run the script
./remove-angular-comprehensive.sh
```

After running the script, check the generated log file in `angular-backup/angular-removal-log.txt` for details on all removed files.

## Step 2: Fix the package.json Dependencies

The script attempts to remove Angular dependencies from package.json, but in some cases, you may need to run the helper script to ensure all dependencies are properly updated:

```bash
# Navigate to the project root
cd /home/nestgate/Development/nestgate

# Make the script executable
chmod +x fix-package-json.js

# Run the script
node fix-package-json.js
```

## Step 3: Install React and Required Dependencies

Install the core React libraries and supporting tools:

```bash
cd crates/nestgate-ui

# Install React dependencies
npm install --save react react-dom @types/react @types/react-dom

# Install additional dependencies for UI components
npm install --save antd @ant-design/icons styled-components

# Install testing libraries
npm install --save-dev @testing-library/react @testing-library/jest-dom jest-environment-jsdom
```

## Step 4: Update Test Configuration for React Components

Ensure the test environment is properly configured for React components:

1. The script has already updated `src/test.ts` to include window.matchMedia mocks required by Ant Design components
2. The script created a proper `jest.config.js` for React testing

Verify that test setup is working by running:

```bash
npx jest src/components/dashboard/NasMetrics.spec.tsx
```

## Step 5: Component Migration Strategy

For components that need to be migrated from Angular to React:

1. Create a new React component with the same name in the same directory
2. Implement the React version with equivalent functionality
3. Update imports in all files that reference the component
4. Run tests to verify functionality
5. Remove the Angular version once the React version is stable

### Migration Example:

**Angular component (before):**
```typescript
// Old Angular component
@Component({
  selector: 'app-nas-metrics',
  templateUrl: './nas-metrics.component.html',
  styleUrls: ['./nas-metrics.component.scss']
})
export class NasMetricsComponent implements OnInit {
  // Angular implementation
}
```

**React component (after):**
```tsx
// New React component
import React, { useEffect, useState } from 'react';
import { Card, Row, Col } from 'antd';

const NasMetrics: React.FC = () => {
  // React implementation
  return (
    <div className="nas-metrics">
      {/* React JSX implementation */}
    </div>
  );
};

export default NasMetrics;
```

## Step 6: Update Routing

1. Replace Angular's routing module with React Router:

```bash
npm install --save react-router-dom @types/react-router-dom
```

2. Create a new routing configuration in a file like `src/routes/index.tsx`:

```tsx
import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Dashboard from '../components/dashboard/Dashboard';
import PerformanceOptimizer from '../components/storage/PerformanceOptimizer';
// Import other components

const AppRoutes: React.FC = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/performance" element={<PerformanceOptimizer />} />
        {/* Define other routes */}
      </Routes>
    </BrowserRouter>
  );
};

export default AppRoutes;
```

## Step 7: Create a New Entry Point

1. Create or update `src/index.tsx` to render your React application:

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import AppRoutes from './routes';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
  <React.StrictMode>
    <AppRoutes />
  </React.StrictMode>
);
```

## Step 8: Test Your Application

Run the test suite to ensure all React components are working:

```bash
cd crates/nestgate-ui
npm test
```

## Step 9: Update Build Scripts

Update your package.json scripts to use React's build process:

```json
"scripts": {
  "start": "react-scripts start",
  "build": "react-scripts build",
  "test": "react-scripts test",
  "eject": "react-scripts eject"
}
```

## Step 10: Final Cleanup

Once your application is fully migrated and tests are passing:

1. Review the Angular backup directory to ensure no important code was lost
2. Remove the Angular backup directory if everything is working correctly:

```bash
rm -rf angular-backup
```

## Troubleshooting

### Common Issues:

1. **Missing dependencies**: If you encounter "Module not found" errors, install the missing packages.

2. **Type errors**: React and TypeScript may require updated type definitions. Install the necessary @types packages.

3. **Test failures**: If tests fail with matchMedia errors, ensure your `src/test.ts` file includes the matchMedia mock.

4. **Routing issues**: If routes don't work, check your React Router configuration and component imports.

## Conclusion

By following this guide, you should have successfully removed Angular from your NestGate project and migrated to a React-based architecture. The modular nature of React will provide better performance and maintainability going forward.

If you encounter any issues during the migration process, refer to the React documentation or reach out to the development team for assistance. 