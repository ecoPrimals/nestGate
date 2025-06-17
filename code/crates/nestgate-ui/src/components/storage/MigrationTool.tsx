import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Card,
  CardContent,
  Button,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  Grid,
  Paper,
  List,
  ListItem,
  ListItemText,
  Divider,
  Alert,
  CircularProgress,
  LinearProgress,
  IconButton,
  Chip,
} from '@mui/material';
import {
  AddCircleOutline as AddIcon,
  Delete as DeleteIcon,
  MoveToInbox as MigrateIcon,
  Refresh as RefreshIcon,
} from '@mui/icons-material';
import { StorageTier } from './TierCard';
import { tieredStorageService, MigrationJob } from '../../services/storage/tieredStorageService';

interface MigrationToolProps {
  sourceTierId: string;
  availableTiers: StorageTier[];
  refreshInterval?: number;
}

const MigrationTool: React.FC<MigrationToolProps> = ({
  sourceTierId,
  availableTiers,
  refreshInterval = 5000,
}) => {
  const [selectedTargetTier, setSelectedTargetTier] = useState<string>('');
  const [paths, setPaths] = useState<string[]>([]);
  const [pathInput, setPathInput] = useState<string>('');
  const [activeMigrations, setActiveMigrations] = useState<MigrationJob[]>([]);
  const [migrationHistory, setMigrationHistory] = useState<MigrationJob[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Load active migrations and history
  useEffect(() => {
    const fetchMigrations = async () => {
      try {
        const [active, history] = await Promise.all([
          tieredStorageService.getActiveMigrations(),
          tieredStorageService.getMigrationHistory(5),
        ]);
        setActiveMigrations(active);
        setMigrationHistory(history);
        setError(null);
      } catch (err) {
        console.error('Failed to load migrations:', err);
        setError('Failed to load migration data. Please try again.');
      }
    };

    fetchMigrations();

    // Set up refresh interval
    const intervalId = setInterval(fetchMigrations, refreshInterval);
    
    return () => clearInterval(intervalId);
  }, [refreshInterval]);

  // Select target tier
  const handleTargetTierChange = (event: React.ChangeEvent<{ value: unknown }>) => {
    setSelectedTargetTier(event.target.value as string);
  };

  // Add path to list
  const handleAddPath = () => {
    if (!pathInput.trim()) return;
    
    setPaths([...paths, pathInput.trim()]);
    setPathInput('');
  };

  // Remove path from list
  const handleRemovePath = (index: number) => {
    setPaths(paths.filter((_, i) => i !== index));
  };

  // Start migration
  const handleStartMigration = async () => {
    if (!selectedTargetTier || paths.length === 0) {
      setError('Please select a target tier and add at least one path');
      return;
    }

    setLoading(true);
    setError(null);
    setSuccess(null);

    try {
      await tieredStorageService.startMigration({
        sourceTierId,
        targetTierId: selectedTargetTier,
        paths,
      });
      
      setSuccess('Migration job started successfully');
      setPaths([]);
      
      // Refresh active migrations
      const active = await tieredStorageService.getActiveMigrations();
      setActiveMigrations(active);
    } catch (err) {
      console.error('Failed to start migration:', err);
      setError('Failed to start migration. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  // Format timestamp to readable format
  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString();
  };

  // Get tier name by ID
  const getTierName = (tierId: string) => {
    if (tierId === sourceTierId) {
      return 'This tier';
    }
    
    const tier = availableTiers.find(t => t.id === tierId);
    return tier ? tier.name : tierId;
  };

  // Get status color
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'completed':
        return 'success';
      case 'in-progress':
        return 'primary';
      case 'pending':
        return 'warning';
      case 'failed':
        return 'error';
      default:
        return 'default';
    }
  };

  return (
    <Box>
      <Typography variant="h5" gutterBottom>
        Data Migration Tool
      </Typography>
      
      <Typography variant="body1" paragraph>
        Move data between storage tiers to optimize performance and storage usage.
      </Typography>
      
      <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 3 }}>
        {/* Migration Configuration */}
        <Box sx={{ flex: '1 1 400px', minWidth: { xs: '100%', md: '400px' } }}>
          <Card variant="outlined">
            <CardContent>
              <Typography variant="h6" gutterBottom>
                Configure Migration
              </Typography>
              
              <Box mb={3}>
                <FormControl fullWidth>
                  <InputLabel id="target-tier-label">Target Tier</InputLabel>
                  <Select
                    labelId="target-tier-label"
                    id="target-tier"
                    value={selectedTargetTier}
                    label="Target Tier"
                    onChange={handleTargetTierChange as any}
                    disabled={loading}
                  >
                    {availableTiers.map((tier) => (
                      <MenuItem key={tier.id} value={tier.id}>
                        {tier.name.charAt(0).toUpperCase() + tier.name.slice(1)} Tier
                      </MenuItem>
                    ))}
                  </Select>
                </FormControl>
              </Box>
              
              <Box mb={3}>
                <Typography variant="subtitle1" gutterBottom>
                  Paths to Migrate
                </Typography>
                <Box display="flex" mb={1}>
                  <TextField
                    fullWidth
                    label="Enter path"
                    value={pathInput}
                    onChange={(e) => setPathInput(e.target.value)}
                    disabled={loading}
                    placeholder="/path/to/file/or/directory"
                  />
                  <IconButton 
                    color="primary" 
                    onClick={handleAddPath}
                    disabled={!pathInput.trim() || loading}
                  >
                    <AddIcon />
                  </IconButton>
                </Box>
                
                <Paper variant="outlined" sx={{ maxHeight: 200, overflow: 'auto', p: 1 }}>
                  {paths.length === 0 ? (
                    <Typography color="text.secondary" p={1} align="center">
                      No paths added yet
                    </Typography>
                  ) : (
                    <List dense>
                      {paths.map((path, index) => (
                        <ListItem
                          key={index}
                          secondaryAction={
                            <IconButton 
                              edge="end" 
                              size="small"
                              onClick={() => handleRemovePath(index)}
                              disabled={loading}
                            >
                              <DeleteIcon fontSize="small" />
                            </IconButton>
                          }
                        >
                          <ListItemText primary={path} />
                        </ListItem>
                      ))}
                    </List>
                  )}
                </Paper>
              </Box>
              
              <Button
                variant="contained"
                color="primary"
                startIcon={<MigrateIcon />}
                disabled={!selectedTargetTier || paths.length === 0 || loading}
                onClick={handleStartMigration}
              >
                Start Migration
              </Button>
              
              {error && (
                <Alert severity="error" sx={{ mt: 2 }}>
                  {error}
                </Alert>
              )}
              
              {success && (
                <Alert severity="success" sx={{ mt: 2 }}>
                  {success}
                </Alert>
              )}
            </CardContent>
          </Card>
        </Box>
        
        {/* Active Migrations */}
        <Box sx={{ flex: '1 1 400px', minWidth: { xs: '100%', md: '400px' } }}>
          <Card variant="outlined">
            <CardContent>
              <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
                <Typography variant="h6">
                  Active Migrations
                </Typography>
                <IconButton size="small" onClick={() => tieredStorageService.getActiveMigrations()}>
                  <RefreshIcon fontSize="small" />
                </IconButton>
              </Box>
              
              {loading && activeMigrations.length === 0 ? (
                <Box display="flex" justifyContent="center" p={2}>
                  <CircularProgress size={24} />
                </Box>
              ) : activeMigrations.length === 0 ? (
                <Alert severity="info">No active migration jobs</Alert>
              ) : (
                <List>
                  {activeMigrations.map((job) => (
                    <React.Fragment key={job.id}>
                      <ListItem alignItems="flex-start">
                        <ListItemText
                          primary={
                            <Box display="flex" justifyContent="space-between" alignItems="center">
                              <Typography variant="subtitle1">
                                {getTierName(job.sourceTierId)} → {getTierName(job.targetTierId)}
                              </Typography>
                              <Chip
                                label={job.status}
                                color={getStatusColor(job.status) as any}
                                size="small"
                              />
                            </Box>
                          }
                          secondary={
                            <Box>
                              <Typography variant="body2" color="text.secondary">
                                Started: {formatTimestamp(job.startTime)}
                              </Typography>
                              <Typography variant="body2" color="text.secondary">
                                Files: {job.paths.length}
                              </Typography>
                              <Box mt={1}>
                                <LinearProgress 
                                  variant="determinate" 
                                  value={job.progress} 
                                  sx={{ height: 8, borderRadius: 1 }}
                                />
                                <Typography variant="body2" align="right">
                                  {job.progress}%
                                </Typography>
                              </Box>
                            </Box>
                          }
                        />
                      </ListItem>
                      <Divider component="li" />
                    </React.Fragment>
                  ))}
                </List>
              )}
              
              {/* Migration History */}
              <Typography variant="h6" mt={3} mb={2}>
                Recent Migrations
              </Typography>
              
              {migrationHistory.length === 0 ? (
                <Alert severity="info">No migration history found</Alert>
              ) : (
                <List>
                  {migrationHistory.map((job) => (
                    <React.Fragment key={job.id}>
                      <ListItem alignItems="flex-start">
                        <ListItemText
                          primary={
                            <Box display="flex" justifyContent="space-between" alignItems="center">
                              <Typography variant="subtitle1">
                                {getTierName(job.sourceTierId)} → {getTierName(job.targetTierId)}
                              </Typography>
                              <Chip
                                label={job.status}
                                color={getStatusColor(job.status) as any}
                                size="small"
                              />
                            </Box>
                          }
                          secondary={
                            <Box>
                              <Typography variant="body2" color="text.secondary">
                                Completed: {formatTimestamp(job.endTime || job.startTime)}
                              </Typography>
                              <Typography variant="body2" color="text.secondary">
                                Files: {job.paths.length}
                              </Typography>
                              {job.error && (
                                <Typography variant="body2" color="error">
                                  Error: {job.error}
                                </Typography>
                              )}
                            </Box>
                          }
                        />
                      </ListItem>
                      <Divider component="li" />
                    </React.Fragment>
                  ))}
                </List>
              )}
            </CardContent>
          </Card>
        </Box>
      </Box>
    </Box>
  );
};

export default MigrationTool; 