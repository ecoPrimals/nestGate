import React, { useState, useEffect } from 'react';
import { 
  Box, 
  Typography, 
  Tabs, 
  Tab, 
  CircularProgress, 
  Alert 
} from '@mui/material';
import TierCard, { StorageTier } from './TierCard';
import EventStream from './EventStream';
import ZfsPropertyEditor from './ZfsPropertyEditor';
import MigrationTool from './MigrationTool';
import { tieredStorageService } from '../../services/storage/tieredStorageService';

// Export types that other modules need
export interface MigrationJob {
  id: string;
  sourceDataset: string;
  targetDataset: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  progress: number;
  bytesTransferred: number;
  totalBytes: number;
  startTime: string;
  endTime?: string;
  error?: string;
}

export interface EventFilter {
  eventTypes?: string[];
  datasets?: string[];
  timeRange?: {
    start: string;
    end: string;
  };
  severity?: 'info' | 'warning' | 'error';
}

export interface FsEvent {
  id: string;
  timestamp: string;
  type: string;
  dataset: string;
  message: string;
  severity: 'info' | 'warning' | 'error';
  metadata?: Record<string, any>;
}

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`tier-tabpanel-${index}`}
      aria-labelledby={`tier-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ p: 3 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

function a11yProps(index: number) {
  return {
    id: `tier-tab-${index}`,
    'aria-controls': `tier-tabpanel-${index}`,
  };
}

const TieredStorageManager: React.FC = () => {
  const [tiers, setTiers] = useState<StorageTier[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedTier, setSelectedTier] = useState<StorageTier | null>(null);
  const [tabValue, setTabValue] = useState(0);

  // Load all tiers on mount
  useEffect(() => {
    const fetchTiers = async () => {
      try {
        setLoading(true);
        const data = await tieredStorageService.getTiers();
        setTiers(data);
        if (data.length > 0) {
          setSelectedTier(data[0]);
        }
        setError(null);
      } catch (err) {
        console.error('Failed to load storage tiers:', err);
        setError('Failed to load storage tiers. Please try again later.');
      } finally {
        setLoading(false);
      }
    };

    fetchTiers();
  }, []);

  // Handle tab change
  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setTabValue(newValue);
  };

  // Handle tier selection
  const handleTierSelect = (tier: StorageTier) => {
    setSelectedTier(tier);
    setTabValue(0); // Reset to overview tab
  };

  // Handle property update
  const handlePropertyUpdate = async (property: string, value: any) => {
    if (!selectedTier) return;
    
    try {
      const updatedTier = await tieredStorageService.updateTierProperty(
        selectedTier.id,
        property,
        value
      );
      
      // Update the tiers array with the updated tier
      setTiers(tiers.map(tier => 
        tier.id === updatedTier.id ? updatedTier : tier
      ));
      
      // Update selected tier
      setSelectedTier(updatedTier);
    } catch (err) {
      console.error(`Failed to update ${property}:`, err);
      setError(`Failed to update ${property}. Please try again.`);
    }
  };

  // Loading state
  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
        <Typography variant="h6" ml={2}>
          Loading storage tiers...
        </Typography>
      </Box>
    );
  }

  // Error state
  if (error) {
    return (
      <Alert severity="error" sx={{ mt: 2 }}>
        {error}
      </Alert>
    );
  }

  return (
    <Box sx={{ width: '100%' }}>
      <Typography variant="h4" gutterBottom>
        Tiered Storage Management
      </Typography>
      
      {/* Tier Cards */}
      <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2, mb: 4 }}>
        {tiers.map((tier) => (
          <Box key={tier.id} sx={{ flex: '1 1 300px', minWidth: { xs: '100%', md: '300px' } }}>
            <TierCard 
              tier={tier} 
              isSelected={selectedTier?.id === tier.id}
              onClick={() => handleTierSelect(tier)}
            />
          </Box>
        ))}
      </Box>
      
      {/* Selected Tier Details */}
      {selectedTier && (
        <Box sx={{ border: 1, borderColor: 'divider', borderRadius: 1 }}>
          <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
            <Tabs value={tabValue} onChange={handleTabChange} aria-label="tier management tabs">
              <Tab label="Overview" {...a11yProps(0)} />
              <Tab label="ZFS Properties" {...a11yProps(1)} />
              <Tab label="Event Monitor" {...a11yProps(2)} />
              <Tab label="Migration" {...a11yProps(3)} />
            </Tabs>
          </Box>
          
          {/* Overview Tab */}
          <TabPanel value={tabValue} index={0}>
            <Typography variant="h5">
              {selectedTier.name.charAt(0).toUpperCase() + selectedTier.name.slice(1)} Tier Overview
            </Typography>
            <Typography variant="body1" paragraph>
              Path: {selectedTier.path}
            </Typography>
            <Typography variant="body1" paragraph>
              Used: {(selectedTier.usage.used / (1024 * 1024 * 1024)).toFixed(2)} GB / 
              Total: {(selectedTier.usage.total / (1024 * 1024 * 1024)).toFixed(2)} GB
            </Typography>
            <Typography variant="body1">
              Compression Ratio: {selectedTier.usage.compressionRatio.toFixed(2)}
            </Typography>
          </TabPanel>
          
          {/* ZFS Properties Tab */}
          <TabPanel value={tabValue} index={1}>
            <ZfsPropertyEditor 
              tierId={selectedTier.id}
              properties={selectedTier.properties}
              onPropertyUpdate={handlePropertyUpdate}
            />
          </TabPanel>
          
          {/* Event Monitor Tab */}
          <TabPanel value={tabValue} index={2}>
            <EventStream tierId={selectedTier.id} />
          </TabPanel>
          
          {/* Migration Tab */}
          <TabPanel value={tabValue} index={3}>
            <MigrationTool 
              sourceTierId={selectedTier.id}
              availableTiers={tiers.filter(tier => tier.id !== selectedTier.id)}
            />
          </TabPanel>
        </Box>
      )}
    </Box>
  );
};

export default TieredStorageManager; 