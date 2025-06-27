# Tiered Storage Manager UI Component

## Overview

The TieredStorageManager is a React component that provides a comprehensive interface for monitoring and managing NestGate's tiered storage system. It visualizes the three storage tiers (hot, warm, cold), displays real-time filesystem events, and provides controls for data migration and tier configuration.

## Component Structure

```
TieredStorageManager
├── TierDashboard
│   ├── TierCard (Hot)
│   ├── TierCard (Warm)
│   └── TierCard (Cold)
├── EventMonitor
│   ├── EventStream
│   ├── EventFilter
│   └── EventAnalytics
├── MigrationTool
│   ├── MigrationControls
│   ├── PathSelector
│   └── MigrationHistory
└── TierSettings
    ├── ZfsPropertyEditor
    ├── MonitoringConfig
    └── PolicyManager
```

## Props Interface

```typescript
interface TieredStorageManagerProps {
  // Optional initial tier selection
  initialTier?: 'hot' | 'warm' | 'cold';
  
  // Optional event filtering
  initialEventFilter?: EventFilter;
  
  // Optional refresh interval (ms)
  refreshInterval?: number;
  
  // Callbacks
  onTierSelect?: (tier: string) => void;
  onMigrationStart?: (job: MigrationJob) => void;
  onPropertyChange?: (tier: string, property: string, value: any) => void;
  
  // Display options
  showEventMonitor?: boolean;
  showMigrationTool?: boolean;
  showTierSettings?: boolean;
  
  // Theme and styling
  theme?: 'light' | 'dark' | 'system';
  className?: string;
  style?: React.CSSProperties;
}
```

## State Management

The component will manage the following state:

```typescript
interface TieredStorageState {
  tiers: {
    hot: StorageTier;
    warm: StorageTier;
    cold: StorageTier;
  };
  selectedTier: string;
  events: {
    recent: FsEvent[];
    filtered: FsEvent[];
    stats: EventStats;
  };
  migrations: {
    active: MigrationJob[];
    history: MigrationJob[];
    selected: string[];
  };
  settings: {
    isEditing: boolean;
    editingTier: string | null;
    editingProperty: string | null;
    validationErrors: Record<string, string>;
  };
}
```

## API Interactions

The component will interact with the backend through these API calls:

```typescript
// Fetch tier information
const fetchTiers = async () => {
  const response = await api.get('/api/storage/tiers');
  return response.data;
};

// Fetch filesystem events
const fetchEvents = async (tierId: string, filter?: EventFilter) => {
  const response = await api.get(`/api/storage/tier/${tierId}/events`, {
    params: { filter }
  });
  return response.data;
};

// Update tier properties
const updateTierProperty = async (
  tierId: string,
  property: string,
  value: any
) => {
  return api.patch(`/api/storage/tier/${tierId}/properties`, {
    [property]: value
  });
};

// Start migration job
const startMigration = async (migrationJob: MigrationJob) => {
  return api.post('/api/storage/migrate', migrationJob);
};
```

## Subcomponents

### TierCard

Displays information about a single storage tier.

```typescript
interface TierCardProps {
  tier: StorageTier;
  isSelected: boolean;
  onSelect: () => void;
  className?: string;
}
```

Features:
- Visual representation of tier (hot/warm/cold)
- Capacity usage bar
- Key metrics (compression ratio, activity level)
- Selection state

### EventStream

Displays real-time filesystem events from the selected tier.

```typescript
interface EventStreamProps {
  events: FsEvent[];
  filter: EventFilter;
  onFilterChange: (filter: EventFilter) => void;
  maxItems?: number;
  autoScroll?: boolean;
}
```

Features:
- Real-time event feed
- Event type icons
- Path highlighting
- Auto-scrolling option
- Time indicators

### MigrationControls

Interface for moving data between tiers.

```typescript
interface MigrationControlsProps {
  sourceTier: StorageTier;
  availableTiers: StorageTier[];
  selectedPaths: string[];
  onPathSelect: (paths: string[]) => void;
  onTargetTierSelect: (tierId: string) => void;
  onSubmit: () => void;
  isSubmitting: boolean;
}
```

Features:
- Source/destination tier selection
- Path selection interface
- Migration options (move vs. copy)
- Progress indication
- Validation checks

### ZfsPropertyEditor

Edits ZFS properties for a specific tier.

```typescript
interface ZfsPropertyEditorProps {
  tier: StorageTier;
  editableProperties: string[];
  onPropertyChange: (property: string, value: any) => void;
  validationErrors: Record<string, string>;
}
```

Features:
- Property editing controls
- Value validation
- Help text and documentation
- Apply/cancel buttons
- Current value display

## Usage Example

```tsx
import { TieredStorageManager } from '@nestgate/ui-components';

function StoragePage() {
  return (
    <div className="storage-page">
      <h1>Storage Management</h1>
      
      <TieredStorageManager 
        initialTier="hot"
        refreshInterval={5000}
        showEventMonitor={true}
        showMigrationTool={true}
        showTierSettings={true}
        onMigrationStart={(job) => console.log('Migration started:', job)}
      />
    </div>
  );
}
```

## Styling

The component will use a modular CSS approach:

```scss
// tiered-storage-manager.scss
.tiered-storage-manager {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  
  &__header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  &__tier-dashboard {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1rem;
  }
  
  // Tier-specific colors
  .tier-hot {
    --tier-color: var(--color-hot, #f56565);
  }
  
  .tier-warm {
    --tier-color: var(--color-warm, #ed8936);
  }
  
  .tier-cold {
    --tier-color: var(--color-cold, #4299e1);
  }
}
```

## Accessibility

The component will implement the following accessibility features:

- Keyboard navigation between tiers and controls
- ARIA labels for all interactive elements
- Sufficient color contrast for tier indicators
- Focus management for complex interactions
- Screen reader announcements for event updates
- Semantic HTML structure

## Responsive Behavior

The component adapts to different screen sizes:

- **Desktop**: Three-column tier display, side-by-side tools
- **Tablet**: Two-column tier display, stacked tools
- **Mobile**: Single-column layout, collapsible sections

## Animation and Transitions

The component will use subtle animations to enhance UX:

- Smooth transitions between tier selections
- Fade-in for new events
- Progress animations for data migrations
- Property change highlighting

## Error Handling

The component implements robust error handling:

- Connection loss detection and retry
- Invalid property value feedback
- Migration failure reporting
- Fallback UI when data cannot be loaded
- Meaningful error messages

## Performance Considerations

To ensure good performance with real-time events:

- Virtualized list for event stream
- Throttled updates for high-frequency events
- Memoization of expensive calculations
- Lazy loading of advanced features
- Background data refreshing

## Testing Strategy

The component will include:

- Unit tests for business logic
- Component tests using React Testing Library
- Visual regression tests
- Performance tests for event rendering
- Mock API tests for async behavior

## Documentation

The component will be documented with:

- Storybook examples
- Props documentation
- Usage examples
- Theme customization guide
- API integration guide

## Implementation Plan

1. **Phase 1**: Basic tier visualization
   - TierDashboard and TierCard components
   - Basic tier metrics display
   - Tier selection

2. **Phase 2**: Event monitoring
   - EventStream component
   - Basic filtering
   - Real-time updates

3. **Phase 3**: Migration tools
   - MigrationControls component
   - Path selection
   - Job submission

4. **Phase 4**: Advanced features
   - ZFS property editing
   - Advanced event analytics
   - Migration history and reporting 