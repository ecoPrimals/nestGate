# Live-Only Implementation Plan

## Remaining Marathon Components

### 1. Backup Scheduling and Automation
- Replace mock backup job data with live API endpoints
- Implement placeholder UI for unfinished features
- Create API services for schedule management and execution
- Build live-only backup target management interface

### 2. Remote Access Gateway
- Implement secure authentication for remote access
- Create bandwidth-optimized interface with live data only
- Add connection monitoring with real-time data
- Develop security controls for remote access without mock fallbacks

### 3. Advanced User Management
- Complete role-based permissions with live backend integration
- Implement user creation workflows with direct API connectivity
- Add password management features without mock data
- Create user profile management interface

## Live-Only Strategy for Existing Components

### For All Components
- Remove any remaining mock data fallbacks
- Implement proper error states instead of mock data
- Add "To be added" placeholders for incomplete features
- Ensure all data source indicators properly show LIVE or PLACEHOLDER status

### Service Layer Updates
1. **BackupService.ts**
   - Remove mock snapshot generators
   - Replace with direct API calls
   - Add proper error handling with placeholders

2. **ZfsPoolService**
   - Remove getMockPools() and getMockDatasets() functions
   - Ensure all methods use only live data or placeholders

3. **SnapshotService**
   - Remove getMockSnapshots() capability
   - Implement placeholder responses for unfinished endpoints

### Testing Adaptations
- Update tests to work with API mocks rather than hardcoded mock data
- Add tests specifically for placeholder states
- Ensure all tests validate proper data source indicators

## Environment and Configuration
- Create a strict enforcement that prevents mock data in any form
- Update environment variables to lock in live-only mode:
  ```
  USE_REAL_DISKS=true
  USE_REAL_ZFS=true
  STRICT_DATA_MODE=true
  REACT_APP_STRICT_DATA_MODE=true
  REACT_APP_USE_MOCK_ALL=false
  ```

## Implementation Timeline
1. **Week 1**: Complete service layer modifications
2. **Week 2**: Implement backup scheduling and partial remote access
3. **Week 3**: Finish remote access and user management components

## Development Approach
- All new features start with API specification first
- Implement backend endpoints before frontend components
- Create placeholder states for features awaiting backend implementation
- Remove any mock data generation code as we go

## Success Metrics
- Zero mock data fallbacks in the entire codebase
- All components clearly indicate their data source
- Proper placeholder display for unimplemented features
- Complete test coverage for live data paths and placeholder states 