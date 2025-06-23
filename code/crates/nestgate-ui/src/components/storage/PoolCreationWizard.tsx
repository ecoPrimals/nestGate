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
  Card,
  CardContent,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  Chip,
  List,
  ListItem,
  ListItemText,
  ListItemSecondaryAction,
  IconButton,
  Alert,
  LinearProgress,
  Checkbox,
  FormControlLabel,
  Grid,
  Divider,
  Tooltip,
  CircularProgress,
} from '@mui/material';
import {
  Add as AddIcon,
  Delete as DeleteIcon,
  Storage as StorageIcon,
  Security as SecurityIcon,
  Speed as SpeedIcon,
  Info as InfoIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
} from '@mui/icons-material';
import { ZfsPoolService } from '../../services/zfs-pool.service';

// Types
interface PoolDevice {
  id: string;
  name: string;
  path: string;
  size: number;
  type: 'disk' | 'file' | 'mirror' | 'raidz' | 'raidz2' | 'raidz3';
  status: 'available' | 'in_use' | 'error';
}

interface PoolConfiguration {
  name: string;
  devices: PoolDevice[];
  vdevType: 'single' | 'mirror' | 'raidz1' | 'raidz2' | 'raidz3';
  properties: {
    compression: string;
    atime: boolean;
    encryption: boolean;
    encryptionKey?: string;
    recordsize: string;
    ashift: number;
  };
  features: {
    enableDeduplication: boolean;
    enableCompression: boolean;
    enableEncryption: boolean;
    enableScrubSchedule: boolean;
  };
}

interface PoolCreationWizardProps {
  open: boolean;
  onClose: () => void;
  onPoolCreated: (poolName: string) => void;
}

const steps = [
  'Pool Name & Type',
  'Device Selection',
  'Configuration',
  'Review & Create'
];

const PoolCreationWizard: React.FC<PoolCreationWizardProps> = ({
  open,
  onClose,
  onPoolCreated
}) => {
  // State
  const [activeStep, setActiveStep] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [availableDevices, setAvailableDevices] = useState<PoolDevice[]>([]);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});
  
  const [config, setConfig] = useState<PoolConfiguration>({
    name: '',
    devices: [],
    vdevType: 'single',
    properties: {
      compression: 'lz4',
      atime: false,
      encryption: false,
      recordsize: '1M',
      ashift: 12,
    },
    features: {
      enableDeduplication: false,
      enableCompression: true,
      enableEncryption: false,
      enableScrubSchedule: true,
    }
  });

  // Load available devices on mount
  useEffect(() => {
    if (open) {
      loadAvailableDevices();
    }
  }, [open]);

  const loadAvailableDevices = async () => {
    try {
      setLoading(true);
      // Mock devices for now - replace with real API call
      const mockDevices: PoolDevice[] = [
        {
          id: '1',
          name: 'nvme0n1',
          path: '/dev/nvme0n1',
          size: 2000000000000, // 2TB
          type: 'disk',
          status: 'available'
        },
        {
          id: '2', 
          name: 'nvme1n1',
          path: '/dev/nvme1n1',
          size: 2000000000000, // 2TB
          type: 'disk',
          status: 'available'
        },
        {
          id: '3',
          name: 'sda',
          path: '/dev/sda',
          size: 4000000000000, // 4TB
          type: 'disk',
          status: 'available'
        }
      ];
      setAvailableDevices(mockDevices);
    } catch (err) {
      setError('Failed to load available devices');
    } finally {
      setLoading(false);
    }
  };

  const formatBytes = (bytes: number): string => {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  };

  const validateStep = (step: number): boolean => {
    const errors: Record<string, string> = {};
    
    switch (step) {
      case 0:
        if (!config.name.trim()) {
          errors.name = 'Pool name is required';
        } else if (!/^[a-zA-Z][a-zA-Z0-9_-]*$/.test(config.name)) {
          errors.name = 'Pool name must start with a letter and contain only letters, numbers, underscores, and hyphens';
        }
        break;
        
      case 1:
        if (config.devices.length === 0) {
          errors.devices = 'At least one device must be selected';
        } else if (config.vdevType === 'mirror' && config.devices.length < 2) {
          errors.devices = 'Mirror requires at least 2 devices';
        } else if (config.vdevType.startsWith('raidz') && config.devices.length < 3) {
          errors.devices = 'RAIDZ requires at least 3 devices';
        }
        break;
        
      case 2:
        if (config.features.enableEncryption && !config.properties.encryptionKey) {
          errors.encryptionKey = 'Encryption key is required when encryption is enabled';
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

  const handleDeviceToggle = (device: PoolDevice) => {
    const isSelected = config.devices.some(d => d.id === device.id);
    
    if (isSelected) {
      setConfig(prev => ({
        ...prev,
        devices: prev.devices.filter(d => d.id !== device.id)
      }));
    } else {
      setConfig(prev => ({
        ...prev,
        devices: [...prev.devices, device]
      }));
    }
  };

  const handleCreatePool = async () => {
    if (!validateStep(activeStep)) return;
    
    try {
      setLoading(true);
      setError(null);
      
      // Create pool via API
      const devicePaths = config.devices.map(d => d.path);
      await ZfsPoolService.createPool({
        name: config.name,
        devices: devicePaths,
        vdevType: config.vdevType,
        properties: config.properties
      });
      
      onPoolCreated(config.name);
      onClose();
      
      // Reset wizard state
      setActiveStep(0);
      setConfig({
        name: '',
        devices: [],
        vdevType: 'single',
        properties: {
          compression: 'lz4',
          atime: false,
          encryption: false,
          recordsize: '1M',
          ashift: 12,
        },
        features: {
          enableDeduplication: false,
          enableCompression: true,
          enableEncryption: false,
          enableScrubSchedule: true,
        }
      });
      
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create pool');
    } finally {
      setLoading(false);
    }
  };

  const renderStepContent = (step: number) => {
    switch (step) {
      case 0:
        return (
          <Box sx={{ mt: 2 }}>
            <TextField
              fullWidth
              label="Pool Name"
              value={config.name}
              onChange={(e) => setConfig(prev => ({ ...prev, name: e.target.value }))}
              error={!!validationErrors.name}
              helperText={validationErrors.name || 'Enter a unique name for your ZFS pool'}
              sx={{ mb: 3 }}
            />
            
            <FormControl fullWidth sx={{ mb: 3 }}>
              <InputLabel>VDEV Type</InputLabel>
              <Select
                value={config.vdevType}
                label="VDEV Type"
                onChange={(e) => setConfig(prev => ({ ...prev, vdevType: e.target.value as any }))}
              >
                <MenuItem value="single">Single Disk</MenuItem>
                <MenuItem value="mirror">Mirror (RAID 1)</MenuItem>
                <MenuItem value="raidz1">RAIDZ1 (RAID 5)</MenuItem>
                <MenuItem value="raidz2">RAIDZ2 (RAID 6)</MenuItem>
                <MenuItem value="raidz3">RAIDZ3 (Triple Parity)</MenuItem>
              </Select>
            </FormControl>
            
            <Alert severity="info" sx={{ mt: 2 }}>
              <Typography variant="body2">
                <strong>VDEV Type Guide:</strong><br/>
                • Single: No redundancy, maximum performance<br/>
                • Mirror: 50% capacity, can lose 1 disk<br/>
                • RAIDZ1: ~75% capacity, can lose 1 disk<br/>
                • RAIDZ2: ~66% capacity, can lose 2 disks<br/>
                • RAIDZ3: ~60% capacity, can lose 3 disks
              </Typography>
            </Alert>
          </Box>
        );
        
      case 1:
        return (
          <Box sx={{ mt: 2 }}>
            <Typography variant="h6" gutterBottom>
              Available Devices
            </Typography>
            
            {validationErrors.devices && (
              <Alert severity="error" sx={{ mb: 2 }}>
                {validationErrors.devices}
              </Alert>
            )}
            
            <List>
              {availableDevices.map((device) => {
                const isSelected = config.devices.some(d => d.id === device.id);
                return (
                  <ListItem
                    key={device.id}
                    sx={{
                      border: 1,
                      borderColor: isSelected ? 'primary.main' : 'divider',
                      borderRadius: 1,
                      mb: 1,
                      backgroundColor: isSelected ? 'primary.light' : 'background.paper',
                      cursor: 'pointer'
                    }}
                    onClick={() => handleDeviceToggle(device)}
                  >
                    <Checkbox
                      checked={isSelected}
                      onChange={() => handleDeviceToggle(device)}
                    />
                    <ListItemText
                      primary={
                        <Box display="flex" alignItems="center" gap={1}>
                          <StorageIcon />
                          <Typography variant="subtitle1">{device.name}</Typography>
                          <Chip
                            label={formatBytes(device.size)}
                            size="small"
                            color="primary"
                            variant="outlined"
                          />
                        </Box>
                      }
                      secondary={device.path}
                    />
                  </ListItem>
                );
              })}
            </List>
            
            <Box sx={{ mt: 2, p: 2, backgroundColor: 'background.default', borderRadius: 1 }}>
              <Typography variant="subtitle2" gutterBottom>
                Selected Configuration
              </Typography>
              <Typography variant="body2">
                <strong>Type:</strong> {config.vdevType}<br/>
                <strong>Devices:</strong> {config.devices.length}<br/>
                <strong>Total Raw Capacity:</strong> {formatBytes(config.devices.reduce((sum, d) => sum + d.size, 0))}<br/>
                <strong>Estimated Usable:</strong> {formatBytes(
                  config.devices.reduce((sum, d) => sum + d.size, 0) * 
                  (config.vdevType === 'mirror' ? 0.5 : 
                   config.vdevType === 'raidz1' ? 0.75 :
                   config.vdevType === 'raidz2' ? 0.66 :
                   config.vdevType === 'raidz3' ? 0.6 : 1)
                )}
              </Typography>
            </Box>
          </Box>
        );
        
      case 2:
        return (
          <Box sx={{ mt: 2 }}>
            <Typography variant="h6" gutterBottom>
              Pool Properties
            </Typography>
            
            <Box display="flex" flexWrap="wrap">
              <Box>
                <FormControl fullWidth sx={{ mb: 2 }}>
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
                    <MenuItem value="lz4">LZ4 (Recommended)</MenuItem>
                    <MenuItem value="zstd">ZSTD</MenuItem>
                    <MenuItem value="gzip">GZIP</MenuItem>
                  </Select>
                </FormControl>
              </Box>
              
              <Box>
                <FormControl fullWidth sx={{ mb: 2 }}>
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
              </Box>
            </Box>
            
            <Divider sx={{ my: 2 }} />
            
            <Typography variant="h6" gutterBottom>
              Features
            </Typography>
            
            <FormControlLabel
              control={
                <Checkbox
                  checked={config.features.enableCompression}
                  onChange={(e) => setConfig(prev => ({
                    ...prev,
                    features: { ...prev.features, enableCompression: e.target.checked }
                  }))}
                />
              }
              label="Enable Compression"
            />
            
            <FormControlLabel
              control={
                <Checkbox
                  checked={config.features.enableEncryption}
                  onChange={(e) => setConfig(prev => ({
                    ...prev,
                    features: { ...prev.features, enableEncryption: e.target.checked },
                    properties: { ...prev.properties, encryption: e.target.checked }
                  }))}
                />
              }
              label="Enable Encryption"
            />
            
            {config.features.enableEncryption && (
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
                sx={{ mt: 2 }}
              />
            )}
            
            <FormControlLabel
              control={
                <Checkbox
                  checked={config.features.enableScrubSchedule}
                  onChange={(e) => setConfig(prev => ({
                    ...prev,
                    features: { ...prev.features, enableScrubSchedule: e.target.checked }
                  }))}
                />
              }
              label="Enable Automatic Scrub Schedule"
            />
          </Box>
        );
        
      case 3:
        return (
          <Box sx={{ mt: 2 }}>
            <Typography variant="h6" gutterBottom>
              Review Pool Configuration
            </Typography>
            
            <Card sx={{ mb: 2 }}>
              <CardContent>
                <Typography variant="subtitle1" gutterBottom>
                  <StorageIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
                  Pool Details
                </Typography>
                <Typography variant="body2">
                  <strong>Name:</strong> {config.name}<br/>
                  <strong>Type:</strong> {config.vdevType}<br/>
                  <strong>Devices:</strong> {config.devices.length}<br/>
                  <strong>Total Capacity:</strong> {formatBytes(config.devices.reduce((sum, d) => sum + d.size, 0))}
                </Typography>
              </CardContent>
            </Card>
            
            <Card sx={{ mb: 2 }}>
              <CardContent>
                <Typography variant="subtitle1" gutterBottom>
                  <SpeedIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
                  Properties
                </Typography>
                <Typography variant="body2">
                  <strong>Compression:</strong> {config.properties.compression}<br/>
                  <strong>Record Size:</strong> {config.properties.recordsize}<br/>
                  <strong>Access Time:</strong> {config.properties.atime ? 'Enabled' : 'Disabled'}
                </Typography>
              </CardContent>
            </Card>
            
            <Card sx={{ mb: 2 }}>
              <CardContent>
                <Typography variant="subtitle1" gutterBottom>
                  <SecurityIcon sx={{ mr: 1, verticalAlign: 'middle' }} />
                  Security & Features
                </Typography>
                <Typography variant="body2">
                  <strong>Encryption:</strong> {config.features.enableEncryption ? 'Enabled' : 'Disabled'}<br/>
                  <strong>Compression:</strong> {config.features.enableCompression ? 'Enabled' : 'Disabled'}<br/>
                  <strong>Auto Scrub:</strong> {config.features.enableScrubSchedule ? 'Enabled' : 'Disabled'}
                </Typography>
              </CardContent>
            </Card>
            
            <Alert severity="warning" sx={{ mt: 2 }}>
              <Typography variant="body2">
                <strong>Warning:</strong> This operation will create a new ZFS pool. 
                All selected devices will be formatted and any existing data will be lost.
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
          <StorageIcon />
          <Typography variant="h6">Create ZFS Pool</Typography>
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
            type="submit"
            variant="contained"
            color="primary"
            disabled={loading}
            startIcon={loading ? <CircularProgress size={20} /> : <CheckCircleIcon />}
            sx={{ mt: 2 }}
            onClick={handleCreatePool}
          >
            {loading ? 'Creating Pool...' : 'Create Pool'}
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

export default PoolCreationWizard; 