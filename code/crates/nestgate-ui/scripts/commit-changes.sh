#!/bin/bash

# Commit changes with proper format following the style guide
git add .

# Commit with proper format
git commit -m "fix(ui): fix axios response handling in services

- Update ZfsPoolService with proper axios response handling
- Fix TypeScript errors in service methods
- Update component tests with more flexible assertions
- Fix display issues with disk size formatting
- Add test script for fixed components

State: InProgress -> Review
Components: ui/services, ui/test
" 