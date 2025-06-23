import React, { useState } from 'react';
import { 
  Typography, 
  Tabs, 
  Tab,
  Button, 
  Stack,
  Table, 
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Select, 
  MenuItem,
  FormControl,
  InputLabel,
  Snackbar,
  Alert,
  Box,
  CircularProgress,
  IconButton,
  Menu,
  Chip,
  Tooltip,
} from '@mui/material';
import { 
  Add as PlusIcon,
  Settings as SettingsIcon,
  Refresh as ReloadIcon,
  MoreVert as MoreVertIcon,
  Warning as ExclamationIcon,
  ImportExport as ImportIcon,
} from '@mui/icons-material';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

import { ZfsPoolService, ZfsPool, ZfsDataset } from '../services/zfs-pool.service';
import { formatCapacity } from '../utils/format';
import TabPanel from '../components/common/TabPanel';
import StatusChip from '../components/common/StatusChip';

interface Dataset {
  id: string;
  name: string;
  mountpoint: string;
  available: number;
  used: number;
  compression: string;
  recordsize: string;
  readonly: boolean;
}

interface CreatePoolFormValues {
  name: string;
  type: 'stripe' | 'mirror' | 'raidz1' | 'raidz2' | 'raidz3';
  disks: string[];
}

interface CreateDatasetFormValues {
  name: string;
  mountpoint: string;
  recordsize: string;
  compression: string;
}

const StoragePage: React.FC = () => {
  const queryClient = useQueryClient();
  
  // Component state
  const [activeTab, setActiveTab] = useState(0);
  const [isCreatePoolModalOpen, setIsCreatePoolModalOpen] = useState(false);
  const [isImportPoolModalOpen, setIsImportPoolModalOpen] = useState(false);
  const [isCreateDatasetModalOpen, setIsCreateDatasetModalOpen] = useState(false);
  const [selectedPool, setSelectedPool] = useState<ZfsPool | null>(null);
  const [poolNameToImport, setPoolNameToImport] = useState('');
  const [menuAnchorEl, setMenuAnchorEl] = useState<null | HTMLElement>(null);
  const [menuPool, setMenuPool] = useState<ZfsPool | null>(null);
  
  // Form state
  const [createPoolForm, setCreatePoolForm] = useState<CreatePoolFormValues>({
    name: '',
    type: 'stripe',
    disks: [],
  });
  const [createDatasetForm, setCreateDatasetForm] = useState<CreateDatasetFormValues>({
    name: '',
    mountpoint: '',
    recordsize: '128K',
    compression: 'lz4',
  });
  
  // Form errors
  const [poolFormErrors, setPoolFormErrors] = useState<Record<string, string>>({});
  const [datasetFormErrors, setDatasetFormErrors] = useState<Record<string, string>>({});
  
  // Snackbar state
  const [snackbar, setSnackbar] = useState<{
    open: boolean;
    message: string;
    severity: 'success' | 'error';
  }>({
    open: false,
    message: '',
    severity: 'success',
  });
  
  // Query for fetching pools
  const { 
    data: pools, 
    isLoading: isPoolsLoading,
    refetch: refetchPools 
  } = useQuery({
    queryKey: ['pools'],
    queryFn: ZfsPoolService.getPools,
  });
  
  // Query for fetching datasets
  const {
    data: datasets,
    isLoading: isDatasetsLoading,
    refetch: refetchDatasets
  } = useQuery({
    queryKey: ['datasets'],
    queryFn: () => ZfsPoolService.getDatasets(selectedPool?.name || ''),
    enabled: !!selectedPool?.name
  });

  const showNotification = (message: string, severity: 'success' | 'error' = 'success') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar(prev => ({ ...prev, open: false }));
  };

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  // Mutations
  const importPoolMutation = useMutation({
    mutationFn: ZfsPoolService.importPool,
    onSuccess: () => {
      showNotification('The pool was successfully imported.');
      queryClient.invalidateQueries({ queryKey: ['pools'] });
    },
    onError: (error) => {
      showNotification('Failed to import the pool. Please try again.', 'error');
      console.error('Import error:', error);
    }
  });

  const exportPoolMutation = useMutation({
    mutationFn: ZfsPoolService.exportPool,
    onSuccess: () => {
      showNotification('The pool was successfully exported.');
      queryClient.invalidateQueries({ queryKey: ['pools'] });
    },
    onError: (error) => {
      showNotification('Failed to export the pool. Please try again.', 'error');
      console.error('Export error:', error);
    }
  });

  const scrubPoolMutation = useMutation({
    mutationFn: ZfsPoolService.scrubPool,
    onSuccess: () => {
      showNotification('The scrub operation has been started.');
    },
    onError: (error) => {
      showNotification('Failed to start the scrub operation. Please try again.', 'error');
      console.error('Scrub error:', error);
    }
  });

  // Handle pool actions
  const handlePoolAction = (action: string, pool: ZfsPool) => {
    switch (action) {
      case 'export':
        if (window.confirm(`Export pool "${pool.name}"? This will unmount all filesystems in the pool and make them unavailable.`)) {
            exportPoolMutation.mutate(pool.name);
          }
        break;
      case 'scrub':
        if (window.confirm(`Start scrub on "${pool.name}"? This will verify all data in the pool and may take a long time.`)) {
            scrubPoolMutation.mutate(pool.name);
          }
        break;
      default:
        break;
    }
    setMenuAnchorEl(null);
    setMenuPool(null);
  };

  // Handle opening create dataset modal
  const handleCreateDataset = (pool: ZfsPool) => {
    setSelectedPool(pool);
    setCreateDatasetForm({
      name: `${pool.name}/`,
      mountpoint: `/mnt/${pool.name}/`,
      recordsize: '128K',
      compression: 'lz4',
    });
    setDatasetFormErrors({});
    setIsCreateDatasetModalOpen(true);
  };

  // Handle import pool
  const handleImportPool = () => {
    if (!poolNameToImport) {
      showNotification('Please enter a valid pool name to import.', 'error');
      return;
    }
    
    importPoolMutation.mutate(poolNameToImport);
    setIsImportPoolModalOpen(false);
    setPoolNameToImport('');
  };

  // Form validation
  const validatePoolForm = (): boolean => {
    const errors: Record<string, string> = {};
    
    if (!createPoolForm.name.trim()) {
      errors.name = 'Please enter a pool name';
    }
    
    if (!createPoolForm.type) {
      errors.type = 'Please select a RAID type';
    }
    
    if (!createPoolForm.disks.length) {
      errors.disks = 'Please select at least one disk';
    }
    
    setPoolFormErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const validateDatasetForm = (): boolean => {
    const errors: Record<string, string> = {};
    
    if (!createDatasetForm.name.trim()) {
      errors.name = 'Please enter a dataset name';
    }
    
    if (!createDatasetForm.mountpoint.trim()) {
      errors.mountpoint = 'Please enter a mount point';
    }
    
    setDatasetFormErrors(errors);
    return Object.keys(errors).length === 0;
  };

  // Handle form submissions
  const handleCreatePool = () => {
    if (!validatePoolForm()) {
      return;
    }
    
    console.log('Creating pool with values:', createPoolForm);
    setIsCreatePoolModalOpen(false);
    showNotification(`Pool "${createPoolForm.name}" has been created successfully.`);
    setCreatePoolForm({ name: '', type: 'stripe', disks: [] });
    setPoolFormErrors({});
  };

  const handleCreateDatasetSubmit = () => {
    if (!validateDatasetForm()) {
      return;
    }
    
    console.log('Creating dataset with values:', createDatasetForm);
    setIsCreateDatasetModalOpen(false);
    showNotification(`Dataset "${createDatasetForm.name}" has been created successfully.`);
    setCreateDatasetForm({ name: '', mountpoint: '', recordsize: '128K', compression: 'lz4' });
    setDatasetFormErrors({});
  };

  const getStatusColor = (status: string): 'success' | 'warning' | 'error' => {
    if (status === 'ONLINE') return 'success';
    if (status === 'DEGRADED') return 'warning';
    if (['FAULTED', 'OFFLINE', 'UNAVAIL', 'REMOVED'].includes(status)) return 'error';
    return 'success';
  };

  const getUsageColor = (percentage: number): 'success' | 'warning' | 'error' => {
    if (percentage > 90) return 'error';
    if (percentage > 80) return 'warning';
    return 'success';
  };

  return (
    <Box sx={{ padding: 3 }}>
      <Typography variant="h4" component="h1" gutterBottom>
        Storage Management
      </Typography>
      
      <Box sx={{ borderBottom: 1, borderColor: 'divider', marginBottom: 3 }}>
        <Tabs value={activeTab} onChange={handleTabChange} aria-label="storage tabs">
          <Tab label="Storage Pools" />
          <Tab label="Datasets" />
        </Tabs>
      </Box>

      {/* Storage Pools Tab */}
      <TabPanel value={activeTab} index={0}>
        <Stack>
          <Stack direction="row">
              <Button 
              variant="outlined"
              startIcon={isPoolsLoading ? <CircularProgress size={20} /> : <ReloadIcon />}
                onClick={() => refetchPools()} 
              disabled={isPoolsLoading}
              >
                Refresh
              </Button>
              <Button 
              variant="contained"
              startIcon={<PlusIcon />}
                onClick={() => setIsCreatePoolModalOpen(true)}
              >
                Create Pool
              </Button>
              <Button 
              variant="outlined"
              startIcon={<ImportIcon />}
                onClick={() => setIsImportPoolModalOpen(true)}
              >
                Import Pool
              </Button>
          </Stack>
          
          {isPoolsLoading ? (
            <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', py: 4 }}>
              <CircularProgress />
              <Typography variant="body2" color="text.secondary" sx={{ mt: 2 }}>
                Loading pools...
              </Typography>
            </Box>
          ) : (
            <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Name</TableCell>
                    <TableCell>Status</TableCell>
                    <TableCell>Size</TableCell>
                    <TableCell>Used</TableCell>
                    <TableCell>Free</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {!pools || pools.length === 0 ? (
                    <TableRow>
                      <TableCell colSpan={6} sx={{ textAlign: 'center', py: 4 }}>
                        <Typography color="text.secondary">No storage pools found</Typography>
                      </TableCell>
                    </TableRow>
                  ) : (
                    pools.map((pool) => {
                      const usagePercentage = Math.round((pool.used / pool.size) * 100);
                      return (
                        <TableRow key={pool.id}>
                          <TableCell>
                            <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
                              {pool.name}
                            </Typography>
                          </TableCell>
                          <TableCell>
                            <StatusChip
                              status={getStatusColor(pool.health)}
                              label={pool.health}
                              size="small"
                            />
                          </TableCell>
                          <TableCell>{formatCapacity(pool.size)}</TableCell>
                          <TableCell>
                            <Stack direction="row" alignItems="center">
                              <Typography variant="body2">
                                {formatCapacity(pool.used)} ({usagePercentage}%)
                              </Typography>
                              <Chip
                                label={`${usagePercentage}%`}
                                size="small"
                                color={getUsageColor(usagePercentage)}
                                variant="outlined"
                              />
                            </Stack>
                          </TableCell>
                          <TableCell>{formatCapacity(pool.free)}</TableCell>
                          <TableCell>
                            <Stack direction="row">
                              <Button
                                size="small"
                                variant="contained"
                                onClick={() => handleCreateDataset(pool)}
                              >
                                Add Dataset
                              </Button>
                              <IconButton
                                size="small"
                                onClick={(event) => {
                                  setMenuAnchorEl(event.currentTarget);
                                  setMenuPool(pool);
                                }}
                              >
                                <MoreVertIcon />
                              </IconButton>
                            </Stack>
                          </TableCell>
                        </TableRow>
                      );
                    })
                  )}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </Stack>
      </TabPanel>

      {/* Datasets Tab */}
      <TabPanel value={activeTab} index={1}>
        <Stack>
          <Stack direction="row" alignItems="center">
            <FormControl sx={{ minWidth: 200 }}>
              <InputLabel>Select a pool</InputLabel>
            <Select
                value={selectedPool?.name || ''}
                onChange={(e) => {
                  const pool = pools?.find(p => p.name === e.target.value) || null;
                setSelectedPool(pool);
              }}
                label="Select a pool"
            >
              {pools?.map(pool => (
                  <MenuItem key={pool.id} value={pool.name}>{pool.name}</MenuItem>
              ))}
            </Select>
            </FormControl>
            <Button 
              variant="outlined"
              startIcon={isDatasetsLoading ? <CircularProgress size={20} /> : <ReloadIcon />}
              onClick={() => refetchDatasets()}
              disabled={isDatasetsLoading}
            >
              Refresh
            </Button>
          </Stack>
          
          {isDatasetsLoading ? (
            <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', py: 4 }}>
              <CircularProgress />
              <Typography variant="body2" color="text.secondary" sx={{ mt: 2 }}>
                Loading datasets...
              </Typography>
            </Box>
          ) : !selectedPool ? (
            <Box sx={{ textAlign: 'center', py: 4 }}>
              <Typography color="text.secondary">Please select a pool to view its datasets</Typography>
            </Box>
          ) : (
            <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>Name</TableCell>
                    <TableCell>Mountpoint</TableCell>
                    <TableCell>Used</TableCell>
                    <TableCell>Available</TableCell>
                    <TableCell>Compression</TableCell>
                    <TableCell>Record Size</TableCell>
                    <TableCell>Read Only</TableCell>
                    <TableCell>Actions</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {!datasets || datasets.length === 0 ? (
                    <TableRow>
                      <TableCell colSpan={8} sx={{ textAlign: 'center', py: 4 }}>
                        <Typography color="text.secondary">No datasets found in this pool</Typography>
                      </TableCell>
                    </TableRow>
                  ) : (
                    datasets.map((dataset) => (
                      <TableRow key={dataset.id}>
                        <TableCell>
                          <Typography variant="body2" sx={{ fontWeight: 'medium' }}>
                            {dataset.name}
                          </Typography>
                        </TableCell>
                        <TableCell>{dataset.mountpoint}</TableCell>
                        <TableCell>{formatCapacity(Number(dataset.used) || 0)}</TableCell>
                        <TableCell>{formatCapacity(Number(dataset.available) || 0)}</TableCell>
                        <TableCell>{dataset.compression}</TableCell>
                        <TableCell>{dataset.recordsize}</TableCell>
                        <TableCell>
                          <StatusChip
                            status={dataset.readonly ? 'error' : 'success'}
                            label={dataset.readonly ? 'Yes' : 'No'}
                            size="small"
                          />
                        </TableCell>
                        <TableCell>
                          <IconButton size="small">
                            <SettingsIcon />
                          </IconButton>
                        </TableCell>
                      </TableRow>
                    ))
                  )}
                </TableBody>
              </Table>
            </TableContainer>
          )}
        </Stack>
      </TabPanel>

      {/* Pool Actions Menu */}
      <Menu
        anchorEl={menuAnchorEl}
        open={Boolean(menuAnchorEl)}
        onClose={() => {
          setMenuAnchorEl(null);
          setMenuPool(null);
        }}
      >
        <MenuItem onClick={() => menuPool && handlePoolAction('export', menuPool)}>
          <Typography color="error">Export Pool</Typography>
        </MenuItem>
        <MenuItem onClick={() => menuPool && handlePoolAction('scrub', menuPool)}>
          Start Scrub
        </MenuItem>
      </Menu>

      {/* Create Pool Modal */}
      <Dialog
        open={isCreatePoolModalOpen}
        onClose={() => setIsCreatePoolModalOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create Storage Pool</DialogTitle>
        <DialogContent>
          <Stack sx={{ paddingTop: 1 }}>
            <TextField
            label="Pool Name"
              value={createPoolForm.name}
              onChange={(e) => setCreatePoolForm(prev => ({ ...prev, name: e.target.value }))}
              error={!!poolFormErrors.name}
              helperText={poolFormErrors.name}
              placeholder="Enter pool name"
              required
              fullWidth
            />
            
            <FormControl fullWidth required>
              <InputLabel>RAID Configuration</InputLabel>
              <Select
                value={createPoolForm.type}
                onChange={(e) => setCreatePoolForm(prev => ({ ...prev, type: e.target.value as any }))}
            label="RAID Configuration"
                error={!!poolFormErrors.type}
              >
                <MenuItem value="stripe">Stripe (RAID 0)</MenuItem>
                <MenuItem value="mirror">Mirror (RAID 1)</MenuItem>
                <MenuItem value="raidz1">RAIDZ1 (RAID 5)</MenuItem>
                <MenuItem value="raidz2">RAIDZ2 (RAID 6)</MenuItem>
                <MenuItem value="raidz3">RAIDZ3</MenuItem>
            </Select>
              {poolFormErrors.type && (
                <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                  {poolFormErrors.type}
                </Typography>
              )}
            </FormControl>
            
            <FormControl fullWidth required>
              <InputLabel>Disks</InputLabel>
              <Select
                multiple
                value={createPoolForm.disks}
                onChange={(e) => setCreatePoolForm(prev => ({ ...prev, disks: e.target.value as string[] }))}
            label="Disks"
                error={!!poolFormErrors.disks}
              >
                <MenuItem value="disk1">/dev/sda (1TB)</MenuItem>
                <MenuItem value="disk2">/dev/sdb (1TB)</MenuItem>
                <MenuItem value="disk3">/dev/sdc (1TB)</MenuItem>
                <MenuItem value="disk4">/dev/sdd (1TB)</MenuItem>
              </Select>
              {poolFormErrors.disks && (
                <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                  {poolFormErrors.disks}
                </Typography>
              )}
            </FormControl>
          </Stack>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setIsCreatePoolModalOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleCreatePool}
            variant="contained"
          >
            Create
          </Button>
        </DialogActions>
      </Dialog>

      {/* Create Dataset Modal */}
      <Dialog
        open={isCreateDatasetModalOpen}
        onClose={() => setIsCreateDatasetModalOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create Dataset in {selectedPool?.name || 'pool'}</DialogTitle>
        <DialogContent>
          <Stack sx={{ paddingTop: 1 }}>
            <TextField
            label="Dataset Name"
              value={createDatasetForm.name}
              onChange={(e) => setCreateDatasetForm(prev => ({ ...prev, name: e.target.value }))}
              error={!!datasetFormErrors.name}
              helperText={datasetFormErrors.name}
              placeholder="Enter dataset name"
              required
              fullWidth
            />
            
            <TextField
            label="Mount Point"
              value={createDatasetForm.mountpoint}
              onChange={(e) => setCreateDatasetForm(prev => ({ ...prev, mountpoint: e.target.value }))}
              error={!!datasetFormErrors.mountpoint}
              helperText={datasetFormErrors.mountpoint}
              placeholder="Enter mount point"
              required
              fullWidth
            />
            
            <FormControl fullWidth>
              <InputLabel>Record Size</InputLabel>
              <Select
                value={createDatasetForm.recordsize}
                onChange={(e) => setCreateDatasetForm(prev => ({ ...prev, recordsize: e.target.value }))}
            label="Record Size"
              >
                <MenuItem value="4K">4K</MenuItem>
                <MenuItem value="8K">8K</MenuItem>
                <MenuItem value="16K">16K</MenuItem>
                <MenuItem value="32K">32K</MenuItem>
                <MenuItem value="64K">64K</MenuItem>
                <MenuItem value="128K">128K</MenuItem>
                <MenuItem value="1M">1M</MenuItem>
            </Select>
            </FormControl>
            
            <FormControl fullWidth>
              <InputLabel>Compression</InputLabel>
              <Select
                value={createDatasetForm.compression}
                onChange={(e) => setCreateDatasetForm(prev => ({ ...prev, compression: e.target.value }))}
            label="Compression"
              >
                <MenuItem value="off">off</MenuItem>
                <MenuItem value="lz4">lz4</MenuItem>
                <MenuItem value="gzip">gzip</MenuItem>
                <MenuItem value="zstd">zstd</MenuItem>
              </Select>
            </FormControl>
          </Stack>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setIsCreateDatasetModalOpen(false)}>
            Cancel
          </Button>
          <Button
            onClick={handleCreateDatasetSubmit}
            variant="contained"
          >
            Create
          </Button>
        </DialogActions>
      </Dialog>
      
      {/* Import Pool Modal */}
      <Dialog
        open={isImportPoolModalOpen}
        onClose={() => setIsImportPoolModalOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Import Pool</DialogTitle>
        <DialogContent>
          <Stack sx={{ paddingTop: 1 }}>
            <TextField
              label="Pool Name"
              value={poolNameToImport}
              onChange={(e) => setPoolNameToImport(e.target.value)}
              placeholder="Enter pool name to import"
              required
              fullWidth
            />
            <Typography variant="body2" color="text.secondary">
              This will import an existing ZFS pool that was previously exported or is from another system.
            </Typography>
          </Stack>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setIsImportPoolModalOpen(false)}>
            Cancel
          </Button>
          <Button 
            onClick={handleImportPool}
            variant="contained"
            disabled={importPoolMutation.isLoading}
          >
            {importPoolMutation.isLoading ? <CircularProgress size={20} /> : 'Import'}
          </Button>
        </DialogActions>
      </Dialog>

      {/* Snackbar for notifications */}
      <Snackbar
        open={snackbar.open}
        autoHideDuration={6000}
        onClose={handleCloseSnackbar}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
      >
        <Alert 
          onClose={handleCloseSnackbar} 
          severity={snackbar.severity}
          sx={{ width: '100%' }}
        >
          {snackbar.message}
        </Alert>
      </Snackbar>
    </Box>
  );
};

export default StoragePage; 