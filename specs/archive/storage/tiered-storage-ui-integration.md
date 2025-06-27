# Tiered Storage UI Integration Specification

## Overview

This specification outlines the integration between the ZFS-powered tiered storage system and the NestGate UI. It defines the user interface components, data flows, and interactions needed to visualize and manage the tiered storage architecture.

## Goals

1. Provide a clear, intuitive UI for managing tiered storage (hot, warm, cold)
2. Visualize data flows and migration between tiers
3. Enable manual and policy-based data migration
4. Display real-time filesystem monitoring events
5. Configure tier-specific monitoring rules
6. Manage ZFS properties and settings through the UI

## Architecture

```
┌─────────────────────┐     ┌───────────────────┐     ┌────────────────┐
│ Tiered Storage UI   │◄────┤ API Layer         │◄────┤ FS Monitor     │
│ Components          │     │                   │     │ Service        │
└─────────────┬───────┘     └───────────────────┘     └────────────────┘
              │                                              ▲
              ▼                                              │
┌─────────────────────┐     ┌───────────────────┐     ┌─────┴──────────┐
│ User Controls       │────►│ ZFS Management    │────►│ Storage Tiers  │
│ & Visualization     │     │ Commands          │     │ (Hot/Warm/Cold)│
└─────────────────────┘     └───────────────────┘     └────────────────┘
```

## UI Components

### 1. Storage Tier Dashboard

The main view for visualizing the tiered storage system:

- **Tier Cards**: Visual representation of hot, warm, and cold tiers
- **Usage Metrics**: Capacity, compression ratio, performance stats
- **Activity Indicators**: Real-time monitoring of file operations
- **Health Status**: Warnings and alerts for each tier

### 2. Tier Management Panel

UI for configuring tier properties and policies:

- **ZFS Property Controls**: Configure compression, recordsize, etc.
- **Migration Rules**: Set up automatic data migration policies
- **Filter Configuration**: Configure event monitoring filters
- **Performance Tuning**: Optimize for specific workloads

### 3. Data Migration Tool

Interface for moving data between tiers:

- **Drag-and-Drop Interface**: Manual data movement
- **Batch Migration**: Select multiple items for migration
- **Scheduled Migration**: Set up timed migration jobs
- **Migration History**: View past operations

### 4. Event Monitor Dashboard

Real-time view of filesystem events:

- **Event Stream**: Live feed of monitored events
- **Filtering Controls**: Filter by event type, path, etc.
- **Tier-Specific Views**: Toggle between tier views
- **Event Analytics**: Charts showing event patterns

## API Endpoints

### Filesystem Monitor Integration

```typescript
// Get real-time events from a specific tier
GET /api/storage/tier/:tierId/events

// Configure event filters for a tier
PUT /api/storage/tier/:tierId/filters
```

### ZFS Management

```typescript
// Get tier properties
GET /api/storage/tier/:tierId/properties

// Update tier properties
PATCH /api/storage/tier/:tierId/properties

// Migrate data between tiers
POST /api/storage/migrate
```

## Data Models

### Storage Tier

```typescript
interface StorageTier {
  id: string;
  name: string;  // "hot", "warm", "cold"
  path: string;  // ZFS dataset path
  properties: ZfsProperties;
  usage: {
    available: number;
    used: number;
    total: number;
    compressionRatio: number;
  };
  monitoring: {
    enabled: boolean;
    filters: EventFilter[];
    activeEvents: number;
  };
}
```

### Event Filter

```typescript
interface EventFilter {
  id: string;
  name: string;
  includeDirectories: boolean;
  includeHidden: boolean;
  extensions: string[];
  patterns: string[];
  eventTypes: string[];
}
```

### Migration Job

```typescript
interface MigrationJob {
  id: string;
  sourceTierId: string;
  targetTierId: string;
  paths: string[];
  status: "pending" | "in-progress" | "completed" | "failed";
  progress: number;
  startTime: string;
  endTime?: string;
  error?: string;
}
```

## Implementation Phases

### Phase 1: Basic Tier Visualization

- Implement tier cards with usage metrics
- Display basic ZFS properties
- Simple event monitoring display

### Phase 2: Configuration & Management

- ZFS property management UI
- Event filter configuration
- Basic data migration tools

### Phase 3: Advanced Features

- Automated policy-based migration
- Advanced event analytics
- Performance optimization tools
- Historical data analysis

## Integration with Existing Components

The tiered storage UI will integrate with:

1. **Storage Dashboard**: Add tiered storage section
2. **Performance Optimizer**: Extend to handle tier-specific tuning
3. **Monitoring System**: Incorporate filesystem events
4. **Backup System**: Configure tier-aware backup policies

## Accessibility & UX Considerations

- Use distinct visual cues for different tiers (colors, icons)
- Provide clear feedback for migration operations
- Include confirmation steps for data movement operations
- Support keyboard navigation for all actions
- Follow design system guidelines for consistency

## Testing Strategy

1. **Unit Tests**: For individual UI components
2. **Integration Tests**: Between UI and API layers
3. **End-to-End Tests**: Full workflows from UI to ZFS
4. **Performance Tests**: For large datasets and high event rates

## Documentation

- User guides for tier management
- API documentation for backend integration
- Administration guide for ZFS property tuning
- Troubleshooting guide for common issues

## Future Enhancements

- Machine learning for predictive data migration
- Enhanced visualization for data access patterns
- Snapshot management integrated with tiers
- Cross-node tiered storage management 