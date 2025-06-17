#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Read the package.json file
console.log('Reading package.json...');
const packageJsonPath = path.resolve('./package.json');
let packageJson;

try {
  packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
} catch (err) {
  console.error('Error reading package.json:', err.message);
  process.exit(1);
}

// Function to remove Angular dependencies
const removeAngularDeps = (deps) => {
  if (!deps) return {};
  
  // Create a new object without Angular dependencies
  const filteredDeps = Object.entries(deps).reduce((acc, [key, value]) => {
    if (!key.startsWith('@angular/') && 
        key !== 'angular' && 
        key !== 'angular-cli' && 
        key !== '@ngx-translate/core' && 
        key !== '@ngx-translate/http-loader' && 
        key !== 'zone.js') {
      acc[key] = value;
    } else {
      console.log(`Removing dependency: ${key}`);
    }
    return acc;
  }, {});
  
  return filteredDeps;
};

// Remove Angular dependencies
console.log('Removing Angular dependencies...');
packageJson.dependencies = removeAngularDeps(packageJson.dependencies);
packageJson.devDependencies = removeAngularDeps(packageJson.devDependencies);

// Write the updated package.json
console.log('Writing updated package.json...');
try {
  fs.writeFileSync(packageJsonPath, JSON.stringify(packageJson, null, 2));
  console.log('Successfully updated package.json');
} catch (err) {
  console.error('Error writing package.json:', err.message);
  process.exit(1);
} 