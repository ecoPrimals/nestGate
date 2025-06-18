import React, { useState, useEffect } from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  Stepper,
  Step,
  StepLabel,
  Box,
  Typography,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  FormControlLabel,
  Checkbox,
  Alert,
  LinearProgress,
  Grid,
  Card,
  CardContent,
  Chip,
  Divider,
  Tooltip,
  RadioGroup,
  Radio,
  Slider,
} from '@mui/material';
import {
  Storage as StorageIcon,
  Folder as FolderIcon,
  Settings as SettingsIcon,
  CheckCircle as CheckCircleIcon,
  Info as InfoIcon,
  Warning as WarningIcon,
} from '@mui/icons-material';
import { ZfsPoolService } from '../../services/zfs-pool.service';
import { StorageTier } from '../../types/storage';

// Types
interface DatasetConfiguration {
  name: string;
  parent: string;
  pool: string;
  type: 'filesystem' | 'volume';
  tier: StorageTier;
  mountpoint: string;
  properties: {
    compression: string;
    recordsize: string;
    quota: number; // in bytes, 0 = no quota
    reservation: number; // in bytes, 0 = no reservation
    readonly: boolean;
    atime: boolean;
    encryption: boolean;
    encryptionKey?: string;
    deduplication: boolean;
  };
  advanced: {
    copies: number;
    primarycache: string;
    secondarycache: string;
    logbias: string;
    sync: string;
  };
}

interface DatasetCreationWizardProps {
  open: boolean;
  onClose: () => void;
  onDatasetCreated: (datasetName: string) => void;
  defaultPool?: string;
  defaultParent?: string;
}

const steps = [
  'Basic Information',
  'Storage Tier & Type',
  'Properties & Quotas',
  'Advanced Settings',
  'Review & Create'
];

const DatasetCreationWizard: React.FC<DatasetCreationWizardProps> = ({
  open,
  onClose,
  onDatasetCreated,
  defaultPool,
  defaultParent
}) => {
  // State
  const [activeStep, setActiveStep] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [availablePools, setAvailablePools] = useState<string[]>([]);
  const [availableParents, setAvailableParents] = useState<string[]>([]);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});

  const [config, setConfig] = useState<DatasetConfiguration>({
    name: '',
    parent: defaultParent || '',
    pool: defaultPool || '',
    type: 'filesystem',
    tier: StorageTier.Warm,
    mountpoint: '',
    properties: {
      compression: 'lz4',
      recordsize: '1M',
      quota: 0,
      reservation: 0,
      readonly: false,
      atime: false,
      encryption: false,
      deduplication: false,
    },
    advanced: {
      copies: 1,
      primarycache: 'all',
      secondarycache: 'all',
      logbias: 'latency',
      sync: 'standard',
    }
  });

  // Load available pools and datasets on mount
  useEffect(() => {
    if (open) {
      loadAvailableOptions();
    }
  }, [open]);

  // Update mountpoint when name or parent changes
  useEffect(() => {
    if (config.type === 'filesystem' && config.name) {
      const basePath = config.parent ? `/${config.parent}` : `/${config.pool}`;
      setConfig(prev => ({
        ...prev,
        mountpoint: `${basePath}/${config.name}`
      }));
    }
  }, [config.name, config.parent, config.pool, config.type]);

  const loadAvailableOptions = async () => {
    try {
      setLoading(true);
      const pools = await ZfsPoolService.getPools();
      const poolNames = pools.map(p => p.name);
      setAvailablePools(poolNames);

      // Load datasets for parent selection
      const allDatasets = [];
      for (const pool of pools) {
        const datasets = await ZfsPoolService.getDatasets(pool.name);
        allDatasets.push(...datasets.map(d => d.name));
      }
      setAvailableParents(allDatasets);

      // Set default pool if not set
      if (!config.pool && poolNames.length > 0) {
        setConfig(prev => ({ ...prev, pool: poolNames[0] }));
      }
    } catch (err) {
      setError('Failed to load available pools and datasets');
    } finally {
      setLoading(false);
    }
  };

  const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  };

  const parseSize = (sizeStr: string): number => {
    const units = { B: 1, KB: 1024, MB: 1024**2, GB: 1024**3, TB: 1024**4 };
    const match = sizeStr.match(/^(\d+(?:\.\d+)?)\s*(B|KB|MB|GB|TB)$/i);
    if (match) {
      const value = parseFloat(match[1]);
      const unit = match[2].toUpperCase() as keyof typeof units;
      return value * units[unit];
    }
    return 0;
  };

  const validateStep = (step: number): boolean => {
    const errors: Record<string, string> = {};
    
    switch (step) {
      case 0:
        if (!config.name.trim()) {
          errors.name = 'Dataset name is required';
        } else if (!/^[a-zA-Z][a-zA-Z0-9_-]*$/.test(config.name)) {
          errors.name = 'Dataset name must start with a letter and contain only letters, numbers, underscores, and hyphens';
        }
        if (!config.pool) {
          errors.pool = 'Pool selection is required';
        }
        break;
        
      case 1:
        // Tier and type validation
        if (config.type === 'volume' && !config.properties.quota) {
          errors.quota = 'Volume size is required for volume datasets';
        }
        break;
        
      case 2:
        if (config.properties.encryption && !config.properties.encryptionKey) {
          errors.encryptionKey = 'Encryption key is required when encryption is enabled';
        }
        if (config.properties.quota && config.properties.reservation > config.properties.quota) {
          errors.reservation = 'Reservation cannot be larger than quota';
        }
        break;
    }
    
    setValidationErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const handleNext = () => {
    if (validateStep(activeStep)) {
      setActiveStep((prevActiveStep) => prevActiveStep + 1);
    }
  };

  const handleBack = () => {
    setActiveStep((prevActiveStep) => prevActiveStep - 1);
  };

  const handleCreateDataset = async () => {
    if (!validateStep(activeStep)) return;
    
    try {
      setLoading(true);
      setError(null);
      
      // Build full dataset name
      const fullName = config.parent 
        ? `${config.parent}/${config.name}`
        : `${config.pool}/${config.name}`;
      
      // Create dataset via API
      await ZfsPoolService.createDataset({
        name: fullName,
        type: config.type,
        properties: {
          ...config.properties,
          ...config.advanced,
          mountpoint: config.mountpoint,
          tier: config.tier,
        }
      });
      
      onDatasetCreated(fullName);
      onClose();
      
      // Reset wizard state
      setActiveStep(0);
      setConfig({
        name: '',
        parent: defaultParent || '',
        pool: defaultPool || availablePools[0] || '',
        type: 'filesystem',
        tier: StorageTier.Warm,
        mountpoint: '',
        properties: {
          compression: 'lz4',
          recordsize: '1M',
          quota: 0,
          reservation: 0,
          readonly: false,
          atime: false,
          encryption: false,
          deduplication: false,
        },
        advanced: {
          copies: 1,
          primarycache: 'all',
          secondarycache: 'all',
          logbias: 'latency',
          sync: 'standard',
        }
      });
      
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create dataset');
    } finally {
      setLoading(false);
    }
  };

  const getTierDescription = (tier: StorageTier) => {
    switch (tier) {
      case StorageTier.Hot:
        return 'High-performance storage for frequently accessed data. Optimized for speed with minimal compression.';
      case StorageTier.Warm:
        return 'Balanced storage for regularly accessed data. Good performance with moderate compression.';
      case StorageTier.Cold:
        return 'High-capacity storage for infrequently accessed data. Maximum compression and space efficiency.';
      default:
        return 'Standard storage tier.';
    }
  };

  const renderStepContent = (step: number) => {
    switch (step) {
      case 0:
        return (
          <Box sx={{ mt: 2 }}>
            <Grid container spacing={3}>
              <Grid item xs={12}>
                <TextField
                  fullWidth
                  label="Dataset Name"
                  value={config.name}
                  onChange={(e) => setConfig(prev => ({ ...prev, name: e.target.value }))}
                  error={!!validationErrors.name}
                  helperText={validationErrors.name || 'Enter a unique name for your dataset'}
                />
              </Grid>
              
              <Grid item xs={12} md={6}>
                <FormControl fullWidth error={!!validationErrors.pool}>
                  <InputLabel>Pool</InputLabel>
                  <Select
                    value={config.pool}
                    label="Pool"
                    onChange={(e) => setConfig(prev => ({ ...prev, pool: e.target.value }))}
                  >
                    {availablePools.map(pool => (
                      <MenuItem key={pool} value={pool}>{pool}</MenuItem>
                    ))}
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} md={6}>
                <FormControl fullWidth>
                  <InputLabel>Parent Dataset (Optional)</InputLabel>
                  <Select
                    value={config.parent}
                    label="Parent Dataset (Optional)"
                    onChange={(e) => setConfig(prev => ({ ...prev, parent: e.target.value }))}
                  >
                    <MenuItem value="">None (Root Level)</MenuItem>
                    {availableParents
                      .filter(parent => parent.startsWith(config.pool))
                      .map(parent => (
                        <MenuItem key={parent} value={parent}>{parent}</MenuItem>
                      ))}
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12}>
                <Alert severity="info">
                  <Typography variant="body2">
                    <strong>Full Dataset Path:</strong> {config.parent ? `${config.parent}/${config.name}` : `${config.pool}/${config.name || '<name>'}`}
                  </Typography>
                </Alert>
              </Grid>
            </Grid>
          </Box>
        );
        
      case 1:
        return (
          <Box sx={{ mt: 2 }}>
            <Typography variant="h6" gutterBottom>Dataset Type</Typography>
            <FormControl component="fieldset" sx={{ mb: 3 }}>
              <RadioGroup
                value={config.type}
                onChange={(e) => setConfig(prev => ({ ...prev, type: e.target.value as 'filesystem' | 'volume' }))}
              >
                <FormControlLabel
                  value="filesystem"
                  control={<Radio />}
                  label={
                    <Box>
                      <Typography variant="subtitle2">Filesystem</Typography>
                      <Typography variant="body2" color="text.secondary">
                        Standard dataset for storing files and directories
                      </Typography>
                    </Box>
                  }
                />
                <FormControlLabel
                  value="volume"
                  control={<Radio />}
                  label={
                    <Box>
                      <Typography variant="subtitle2">Volume (ZVOL)</Typography>
                      <Typography variant="body2" color="text.secondary">
                        Block device for virtual machines or raw storage
                      </Typography>
                    </Box>
                  }
                />
              </RadioGroup>
            </FormControl>
            
            <Divider sx={{ my: 3 }} />
            
            <Typography variant="h6" gutterBottom>Storage Tier</Typography>
            <Grid container spacing={2}>
              {Object.values(StorageTier).map(tier => (
                <Grid item xs={12} md={4} key={tier}>
                  <Card
                    sx={{
                      cursor: 'pointer',
                      border: config.tier === tier ? 2 : 1,
                      borderColor: config.tier === tier ? 'primary.main' : 'divider',
                    }}
                    onClick={() => setConfig(prev => ({ ...prev, tier }))}
                  >
                    <CardContent>
                      <Typography variant="h6" color={
                        tier === StorageTier.Hot ? 'error.main' :
                        tier === StorageTier.Warm ? 'warning.main' : 'info.main'
                      }>
                        {tier} Tier
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        {getTierDescription(tier)}
                      </Typography>
                    </CardContent>
                  </Card>
                </Grid>
              ))}
            </Grid>
            
            {config.type === 'filesystem' && (
              <Box sx={{ mt: 3 }}>
                <TextField
                  fullWidth
                  label="Mountpoint"
                  value={config.mountpoint}
                  onChange={(e) => setConfig(prev => ({ ...prev, mountpoint: e.target.value }))}
                  helperText="Where the dataset will be mounted in the filesystem"
                />
              </Box>
            )}
          </Box>
        );
        
      case 2:
        return (
          <Box sx={{ mt: 2 }}>
            <Grid container spacing={3}>
              <Grid item xs={12} md={6}>
                <FormControl fullWidth>
                  <InputLabel>Compression</InputLabel>
                  <Select
                    value={config.properties.compression}
                    label="Compression"
                    onChange={(e) => setConfig(prev => ({
                      ...prev,
                      properties: { ...prev.properties, compression: e.target.value }
                    }))}
                  >
                    <MenuItem value="off">Off</MenuItem>
                    <MenuItem value="lz4">LZ4 (Fast)</MenuItem>
                    <MenuItem value="zstd">ZSTD (Balanced)</MenuItem>
                    <MenuItem value="gzip">GZIP (High Compression)</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} md={6}>
                <FormControl fullWidth>
                  <InputLabel>Record Size</InputLabel>
                  <Select
                    value={config.properties.recordsize}
                    label="Record Size"
                    onChange={(e) => setConfig(prev => ({
                      ...prev,
                      properties: { ...prev.properties, recordsize: e.target.value }
                    }))}
                  >
                    <MenuItem value="128K">128K (Database)</MenuItem>
                    <MenuItem value="1M">1M (General Purpose)</MenuItem>
                    <MenuItem value="16M">16M (Large Files)</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} md={6}>
                <TextField
                  fullWidth
                  label="Quota (0 = No Limit)"
                  placeholder="e.g., 100GB, 1TB"
                  helperText="Maximum space this dataset can use"
                  onChange={(e) => {
                    const quota = parseSize(e.target.value) || 0;
                    setConfig(prev => ({
                      ...prev,
                      properties: { ...prev.properties, quota }
                    }));
                  }}
                />
              </Grid>
              
              <Grid item xs={12} md={6}>
                <TextField
                  fullWidth
                  label="Reservation (0 = No Reservation)"
                  placeholder="e.g., 10GB, 100GB"
                  helperText="Guaranteed space reserved for this dataset"
                  error={!!validationErrors.reservation}
                  onChange={(e) => {
                    const reservation = parseSize(e.target.value) || 0;
                    setConfig(prev => ({
                      ...prev,
                      properties: { ...prev.properties, reservation }
                    }));
                  }}
                />
              </Grid>
              
              <Grid item xs={12}>
                <Typography variant="subtitle2" gutterBottom>Dataset Options</Typography>
                <FormControlLabel
                  control={
                    <Checkbox
                      checked={config.properties.readonly}
                      onChange={(e) => setConfig(prev => ({
                        ...prev,
                        properties: { ...prev.properties, readonly: e.target.checked }
                      }))}
                    />
                  }
                  label="Read-only dataset"
                />
                <FormControlLabel
                  control={
                    <Checkbox
                      checked={config.properties.atime}
                      onChange={(e) => setConfig(prev => ({
                        ...prev,
                        properties: { ...prev.properties, atime: e.target.checked }
                      }))}
                    />
                  }
                  label="Update access time (atime)"
                />
                <FormControlLabel
                  control={
                    <Checkbox
                      checked={config.properties.encryption}
                      onChange={(e) => setConfig(prev => ({
                        ...prev,
                        properties: { ...prev.properties, encryption: e.target.checked }
                      }))}
                    />
                  }
                  label="Enable encryption"
                />
                <FormControlLabel
                  control={
                    <Checkbox
                      checked={config.properties.deduplication}
                      onChange={(e) => setConfig(prev => ({
                        ...prev,
                        properties: { ...prev.properties, deduplication: e.target.checked }
                      }))}
                    />
                  }
                  label="Enable deduplication"
                />
              </Grid>
              
              {config.properties.encryption && (
                <Grid item xs={12}>
                  <TextField
                    fullWidth
                    type="password"
                    label="Encryption Key"
                    value={config.properties.encryptionKey || ''}
                    onChange={(e) => setConfig(prev => ({
                      ...prev,
                      properties: { ...prev.properties, encryptionKey: e.target.value }
                    }))}
                    error={!!validationErrors.encryptionKey}
                    helperText={validationErrors.encryptionKey || 'Enter a strong encryption key'}
                  />
                </Grid>
              )}
            </Grid>
          </Box>
        );
        
      case 3:
        return (
          <Box sx={{ mt: 2 }}>
            <Alert severity="warning" sx={{ mb: 3 }}>
              <Typography variant="body2">
                <strong>Advanced Settings:</strong> These settings affect performance and should only be modified by experienced users.
              </Typography>
            </Alert>
            
            <Grid container spacing={3}>
              <Grid item xs={12} md={6}>
                <Typography gutterBottom>Copies: {config.advanced.copies}</Typography>
                <Slider
                  value={config.advanced.copies}
                  onChange={(_, value) => setConfig(prev => ({
                    ...prev,
                    advanced: { ...prev.advanced, copies: value as number }
                  }))}
                  min={1}
                  max={3}
                  marks
                  step={1}
                />
              </Grid>
              
              <Grid item xs={12} md={6}>
                <FormControl fullWidth>
                  <InputLabel>Primary Cache</InputLabel>
                  <Select
                    value={config.advanced.primarycache}
                    label="Primary Cache"
                    onChange={(e) => setConfig(prev => ({
                      ...prev,
                      advanced: { ...prev.advanced, primarycache: e.target.value }
                    }))}
                  >
                    <MenuItem value="all">All</MenuItem>
                    <MenuItem value="none">None</MenuItem>
                    <MenuItem value="metadata">Metadata Only</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} md={6}>
                <FormControl fullWidth>
                  <InputLabel>Secondary Cache</InputLabel>
                  <Select
                    value={config.advanced.secondarycache}
                    label="Secondary Cache"
                    onChange={(e) => setConfig(prev => ({
                      ...prev,
                      advanced: { ...prev.advanced, secondarycache: e.target.value }
                    }))}
                  >
                    <MenuItem value="all">All</MenuItem>
                    <MenuItem value="none">None</MenuItem>
                    <MenuItem value="metadata">Metadata Only</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} md={6}>
                <FormControl fullWidth>
                  <InputLabel>Sync</InputLabel>
                  <Select
                    value={config.advanced.sync}
                    label="Sync"
                    onChange={(e) => setConfig(prev => ({
                      ...prev,
                      advanced: { ...prev.advanced, sync: e.target.value }
                    }))}
                  >
                    <MenuItem value="standard">Standard</MenuItem>
                    <MenuItem value="always">Always</MenuItem>
                    <MenuItem value="disabled">Disabled</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
            </Grid>
          </Box>
        );
        
      case 4:
        return (
          <Box sx={{ mt: 2 }}>
            <Typography variant="h6" gutterBottom>Review Dataset Configuration</Typography>
            
            <Grid container spacing={2}>
              <Grid item xs={12} md={6}>
                <Card>
                  <CardContent>
                    <Typography variant="subtitle1" gutterBottom>
                      <FolderIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
                      Basic Information
                    </Typography>
                    <Typography variant="body2">
                      <strong>Name:</strong> {config.parent ? `${config.parent}/${config.name}` : `${config.pool}/${config.name}`}<br/>
                      <strong>Type:</strong> {config.type}<br/>
                      <strong>Tier:</strong> {config.tier}<br/>
                      {config.type === 'filesystem' && (
                        <>
                          <strong>Mountpoint:</strong> {config.mountpoint}
                        </>
                      )}
                    </Typography>
                  </CardContent>
                </Card>
              </Grid>
              
              <Grid item xs={12} md={6}>
                <Card>
                  <CardContent>
                    <Typography variant="subtitle1" gutterBottom>
                      <SettingsIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
                      Properties
                    </Typography>
                    <Typography variant="body2">
                      <strong>Compression:</strong> {config.properties.compression}<br/>
                      <strong>Record Size:</strong> {config.properties.recordsize}<br/>
                      {config.properties.quota > 0 && (
                        <>
                          <strong>Quota:</strong> {formatBytes(config.properties.quota)}<br/>
                        </>
                      )}
                      {config.properties.reservation > 0 && (
                        <>
                          <strong>Reservation:</strong> {formatBytes(config.properties.reservation)}<br/>
                        </>
                      )}
                    </Typography>
                  </CardContent>
                </Card>
              </Grid>
              
              <Grid item xs={12}>
                <Card>
                  <CardContent>
                    <Typography variant="subtitle1" gutterBottom>Features</Typography>
                    <Box display="flex" flexWrap="wrap" gap={1}>
                      {config.properties.readonly && <Chip label="Read-only" size="small" />}
                      {config.properties.atime && <Chip label="Access Time" size="small" />}
                      {config.properties.encryption && <Chip label="Encrypted" size="small" color="secondary" />}
                      {config.properties.deduplication && <Chip label="Deduplicated" size="small" />}
                      {config.advanced.copies > 1 && <Chip label={`${config.advanced.copies} Copies`} size="small" />}
                    </Box>
                  </CardContent>
                </Card>
              </Grid>
            </Grid>
            
            <Alert severity="warning" sx={{ mt: 2 }}>
              <Typography variant="body2">
                <strong>Warning:</strong> This operation will create a new dataset. 
                Make sure all settings are correct before proceeding.
              </Typography>
            </Alert>
          </Box>
        );
        
      default:
        return null;
    }
  };

  return (
    <Dialog
      open={open}
      onClose={onClose}
      maxWidth="md"
      fullWidth
      PaperProps={{
        sx: { minHeight: '600px' }
      }}
    >
      <DialogTitle>
        <Box display="flex" alignItems="center" gap={1}>
          <FolderIcon />
          <Typography variant="h6">Create Dataset</Typography>
        </Box>
      </DialogTitle>
      
      <DialogContent>
        <Stepper activeStep={activeStep} sx={{ mb: 3 }}>
          {steps.map((label) => (
            <Step key={label}>
              <StepLabel>{label}</StepLabel>
            </Step>
          ))}
        </Stepper>
        
        {loading && <LinearProgress sx={{ mb: 2 }} />}
        
        {error && (
          <Alert severity="error" sx={{ mb: 2 }}>
            {error}
          </Alert>
        )}
        
        {renderStepContent(activeStep)}
      </DialogContent>
      
      <DialogActions>
        <Button onClick={onClose} disabled={loading}>
          Cancel
        </Button>
        
        <Button
          disabled={activeStep === 0 || loading}
          onClick={handleBack}
        >
          Back
        </Button>
        
        {activeStep === steps.length - 1 ? (
          <Button
            variant="contained"
            onClick={handleCreateDataset}
            disabled={loading}
            startIcon={loading ? <LinearProgress size={20} /> : <CheckCircleIcon />}
          >
            {loading ? 'Creating...' : 'Create Dataset'}
          </Button>
        ) : (
          <Button
            variant="contained"
            onClick={handleNext}
            disabled={loading}
          >
            Next
          </Button>
        )}
      </DialogActions>
    </Dialog>
  );
};

export default DatasetCreationWizard; 