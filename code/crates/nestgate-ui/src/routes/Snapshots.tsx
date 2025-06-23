import React, { useState } from 'react';
import { 
  Typography, 
  Table, 
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Button, 
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Checkbox, 
  FormControlLabel,
  Alert,
  Box,
  Stack,
  IconButton,
  Tooltip,
  CircularProgress,
  TablePagination,
  Snackbar,
} from '@mui/material';
import {
  Refresh as RefreshIcon,
  Add as AddIcon,
  Delete as DeleteIcon,
  Restore as RollbackIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
  Sync as SyncIcon,
} from '@mui/icons-material';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Snapshot, SnapshotService } from '../services/snapshot.service';
import { ZfsPool, ZfsPoolService, ZfsDataset } from '../services/zfs-pool.service';
import dayjs from 'dayjs';
import { formatDate } from '../utils/format';
import StatusChip from '../components/common/StatusChip';

const SnapshotsPage: React.FC = () => {
  const [createModalVisible, setCreateModalVisible] = useState(false);
  const [rollbackModalVisible, setRollbackModalVisible] = useState(false);
  const [deleteConfirmOpen, setDeleteConfirmOpen] = useState(false);
  const [selectedSnapshot, setSelectedSnapshot] = useState<Snapshot | null>(null);
  const [snapshotToDelete, setSnapshotToDelete] = useState<string | null>(null);
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  
  // Form state for create snapshot
  const [createFormData, setCreateFormData] = useState({
    dataset: '',
    name: '',
    recursive: false,
  });
  const [createFormErrors, setCreateFormErrors] = useState<Record<string, string>>({});
  
  // Form state for rollback
  const [rollbackFormData, setRollbackFormData] = useState({
    force: false,
  });
  
  const [snackbar, setSnackbar] = useState<{
    open: boolean;
    message: string;
    severity: 'success' | 'error';
  }>({
    open: false,
    message: '',
    severity: 'success',
  });
  
  const queryClient = useQueryClient();
  const snapshotService = SnapshotService.getInstance();
  const zfsPoolService = ZfsPoolService.getInstance();

  const showNotification = (message: string, severity: 'success' | 'error' = 'success') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar(prev => ({ ...prev, open: false }));
  };
  
  // Query for fetching snapshots
  const { 
    data: snapshots, 
    isLoading, 
    isError, 
    error, 
    refetch 
  } = useQuery({
    queryKey: ['snapshots'],
    queryFn: () => snapshotService.getSnapshots(),
  });
  
  // Query for fetching datasets (for snapshot creation)
  const { 
    data: poolsData 
  } = useQuery({
    queryKey: ['zfsPools'],
    queryFn: () => ZfsPoolService.getPools(),
  });
  
  // Datasets derived from pools for dropdown selection
  const datasets = React.useMemo(() => {
    if (!poolsData) return [];
    
    const datasetList: string[] = [];
    let poolsArray: ZfsPool[] = [];
    
    // Safely extract pools array from response
    try {
      if (Array.isArray(poolsData)) {
        poolsArray = poolsData;
      } else if (poolsData && typeof poolsData === 'object' && 'pools' in poolsData) {
        const typedData = poolsData as { pools: ZfsPool[] };
        poolsArray = typedData.pools;
      }
    } catch (error) {
      console.error('Error parsing pools data:', error);
    }
    
    // Process the pool array to get dataset names
    if (poolsArray.length > 0) {
      poolsArray.forEach((pool: ZfsPool) => {
        datasetList.push(pool.name);
      });
    }
    
    return datasetList;
  }, [poolsData]);
  
  // Mutation for creating snapshots
  const createMutation = useMutation({
    mutationFn: (values: { dataset: string; name: string; recursive: boolean }) => 
      snapshotService.createSnapshot(values.dataset, values.name, values.recursive),
    onSuccess: () => {
      showNotification('Snapshot was created successfully.');
      resetCreateForm();
      setCreateModalVisible(false);
      queryClient.invalidateQueries({ queryKey: ['snapshots'] });
    },
    onError: (error: Error) => {
      showNotification(`Failed to create snapshot: ${error.message}`, 'error');
    }
  });
  
  // Mutation for deleting snapshots
  const deleteMutation = useMutation({
    mutationFn: (id: string) => snapshotService.deleteSnapshot(id),
    onSuccess: () => {
      showNotification('Snapshot was deleted successfully.');
      queryClient.invalidateQueries({ queryKey: ['snapshots'] });
    },
    onError: (error: Error) => {
      showNotification(`Failed to delete snapshot: ${error.message}`, 'error');
    }
  });
  
  // Mutation for rolling back to a snapshot
  const rollbackMutation = useMutation({
    mutationFn: ({ id, force }: { id: string; force: boolean }) => 
      snapshotService.rollbackSnapshot(id, force),
    onSuccess: () => {
      showNotification('System has been rolled back to the selected snapshot.');
      resetRollbackForm();
      setRollbackModalVisible(false);
      setSelectedSnapshot(null);
      queryClient.invalidateQueries({ queryKey: ['snapshots'] });
    },
    onError: (error: Error) => {
      showNotification(`Rollback failed: ${error.message}`, 'error');
    }
  });

  const resetCreateForm = () => {
    setCreateFormData({
      dataset: '',
      name: '',
      recursive: false,
    });
    setCreateFormErrors({});
  };

  const resetRollbackForm = () => {
    setRollbackFormData({
      force: false,
    });
  };

  const validateCreateForm = (): boolean => {
    const errors: Record<string, string> = {};

    if (!createFormData.dataset.trim()) {
      errors.dataset = 'Please select a dataset';
    }

    if (!createFormData.name.trim()) {
      errors.name = 'Please enter a name for the snapshot';
    }

    setCreateFormErrors(errors);
    return Object.keys(errors).length === 0;
  };

  // Handle create snapshot form submission
  const handleCreateSnapshot = () => {
    if (!validateCreateForm()) {
      return;
    }

    createMutation.mutate({
      dataset: createFormData.dataset,
      name: createFormData.name,
      recursive: createFormData.recursive
    });
  };
  
  // Handle rollback form submission
  const handleRollbackSnapshot = () => {
    if (selectedSnapshot) {
      rollbackMutation.mutate({
        id: selectedSnapshot.id,
        force: rollbackFormData.force
      });
    }
  };
  
  // Handle snapshot refresh
  const handleRefresh = () => {
    refetch();
  };
  
  // Handle opening the rollback modal
  const openRollbackModal = (snapshot: Snapshot) => {
    setSelectedSnapshot(snapshot);
    resetRollbackForm();
    setRollbackModalVisible(true);
  };
  
  // Handle delete click
  const handleDeleteClick = (id: string) => {
    setSnapshotToDelete(id);
    setDeleteConfirmOpen(true);
  };

  // Handle delete confirm
  const handleDeleteConfirm = () => {
    if (snapshotToDelete) {
      deleteMutation.mutate(snapshotToDelete);
      setDeleteConfirmOpen(false);
      setSnapshotToDelete(null);
    }
  };

  // Handle delete cancel
  const handleDeleteCancel = () => {
    setDeleteConfirmOpen(false);
    setSnapshotToDelete(null);
  };

  const handleCreateFormChange = (field: string, value: any) => {
    setCreateFormData(prev => ({ ...prev, [field]: value }));
    
    // Clear error when user starts typing
    if (createFormErrors[field]) {
      setCreateFormErrors(prev => ({ ...prev, [field]: '' }));
    }
  };

  const handleChangePage = (event: unknown, newPage: number) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  const paginatedSnapshots = snapshots?.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage) || [];

  return (
    <Box sx={{ padding: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 3 }}>
        <Typography variant="h4" component="h1">
          ZFS Snapshots
        </Typography>
        <Stack direction="row">
          <Button
            variant="outlined"
            startIcon={<RefreshIcon />}
            onClick={handleRefresh}
            disabled={isLoading}
          >
            {isLoading ? <CircularProgress size={20} /> : 'Refresh'}
          </Button>
          <Button 
            variant="contained"
            startIcon={<AddIcon />}
            onClick={() => {
              resetCreateForm();
              setCreateModalVisible(true);
            }}
          >
            Create Snapshot
          </Button>
        </Stack>
      </Box>
      
      {isError && (
        <Alert severity="error" sx={{ marginBottom: 2 }}>
            Error loading snapshots: {(error as Error).message}
        </Alert>
      )}
      
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Name</TableCell>
              <TableCell>Dataset</TableCell>
              <TableCell>Created</TableCell>
              <TableCell>Size</TableCell>
              <TableCell>Replication</TableCell>
              <TableCell>Actions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {isLoading ? (
              <TableRow>
                <TableCell colSpan={6} sx={{ textAlign: 'center', padding: 4 }}>
                  <CircularProgress />
                </TableCell>
              </TableRow>
            ) : paginatedSnapshots.length === 0 ? (
              <TableRow>
                <TableCell colSpan={6} sx={{ textAlign: 'center', padding: 4 }}>
                  <Typography color="text.secondary">No snapshots found</Typography>
                </TableCell>
              </TableRow>
            ) : (
              paginatedSnapshots.map((snapshot) => (
                <TableRow key={snapshot.id}>
                  <TableCell>
                    <Tooltip title={snapshot.id}>
                      <Typography variant="body2">{snapshot.name}</Typography>
                    </Tooltip>
                  </TableCell>
                  <TableCell>{snapshot.dataset}</TableCell>
                  <TableCell>{formatDate(snapshot.created)}</TableCell>
                  <TableCell>{snapshot.size}</TableCell>
                  <TableCell>
                    <StatusChip 
                      status={snapshot.isReplicated ? 'success' : 'warning'}
                      label={snapshot.isReplicated ? 'Replicated' : 'Pending'}
                      icon={snapshot.isReplicated ? <CheckCircleIcon /> : <SyncIcon />}
                      size="small"
                    />
                  </TableCell>
                  <TableCell>
                    <Stack direction="row">
                      <Tooltip title="Rollback to this snapshot">
                        <IconButton 
                          size="small"
                          color="primary"
                          onClick={() => openRollbackModal(snapshot)}
                        >
                          <RollbackIcon />
                        </IconButton>
                      </Tooltip>
                      <Tooltip title="Delete snapshot">
                        <IconButton 
                          size="small"
                          color="error"
                          onClick={() => handleDeleteClick(snapshot.id)}
                        >
                          <DeleteIcon />
                        </IconButton>
                      </Tooltip>
                    </Stack>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
        <TablePagination
          rowsPerPageOptions={[5, 10, 25]}
          component="div"
          count={snapshots?.length || 0}
          rowsPerPage={rowsPerPage}
          page={page}
          onPageChange={handleChangePage}
          onRowsPerPageChange={handleChangeRowsPerPage}
        />
      </TableContainer>
      
      {/* Create Snapshot Modal */}
      <Dialog
        open={createModalVisible}
        onClose={() => setCreateModalVisible(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Create New Snapshot</DialogTitle>
        <DialogContent>
          <Stack sx={{ paddingTop: 1 }}>
            <FormControl fullWidth required>
              <InputLabel>Dataset</InputLabel>
              <Select
                value={createFormData.dataset}
                onChange={(e) => handleCreateFormChange('dataset', e.target.value)}
            label="Dataset"
                error={!!createFormErrors.dataset}
          >
              {datasets.map(dataset => (
                  <MenuItem key={dataset} value={dataset}>{dataset}</MenuItem>
              ))}
            </Select>
              {createFormErrors.dataset && (
                <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                  {createFormErrors.dataset}
                </Typography>
              )}
            </FormControl>
            
            <TextField
            label="Snapshot Name"
              value={createFormData.name}
              onChange={(e) => handleCreateFormChange('name', e.target.value)}
              error={!!createFormErrors.name}
              helperText={createFormErrors.name}
              placeholder="e.g., backup-2023-07-15"
              required
              fullWidth
            />
            
            <FormControlLabel
              control={
                <Checkbox
                  checked={createFormData.recursive}
                  onChange={(e) => handleCreateFormChange('recursive', e.target.checked)}
                />
              }
              label="Recursive (include child datasets)"
            />
          </Stack>
        </DialogContent>
        <DialogActions>
              <Button onClick={() => setCreateModalVisible(false)}>
                Cancel
              </Button>
          <Button 
            onClick={handleCreateSnapshot}
            variant="contained"
            disabled={createMutation.isLoading}
          >
            {createMutation.isLoading ? <CircularProgress size={20} /> : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>
      
      {/* Rollback Modal */}
      <Dialog
        open={rollbackModalVisible}
        onClose={() => {
          setRollbackModalVisible(false);
          setSelectedSnapshot(null);
        }}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Rollback to Snapshot</DialogTitle>
        <DialogContent>
          <Stack>
            <Typography variant="body1">
            You are about to roll back to snapshot: <strong>{selectedSnapshot?.name}</strong> from dataset <strong>{selectedSnapshot?.dataset}</strong>
            </Typography>
            
            <Alert severity="warning" icon={<WarningIcon />}>
              Warning: Rolling back will revert your system to the state when this snapshot was taken. Any data changes made after the snapshot will be lost.
            </Alert>
            
            <FormControlLabel
              control={
                <Checkbox
                  checked={rollbackFormData.force}
                  onChange={(e) => setRollbackFormData(prev => ({ ...prev, force: e.target.checked }))}
                />
              }
              label="Force rollback (may destroy later snapshots)"
            />
          </Stack>
        </DialogContent>
        <DialogActions>
              <Button onClick={() => {
                setRollbackModalVisible(false);
                setSelectedSnapshot(null);
              }}>
                Cancel
              </Button>
          <Button 
            onClick={handleRollbackSnapshot}
            variant="contained"
            color="error"
            disabled={rollbackMutation.isLoading}
          >
            {rollbackMutation.isLoading ? <CircularProgress size={20} /> : 'Rollback'}
          </Button>
        </DialogActions>
      </Dialog>

      {/* Delete Confirmation Dialog */}
      <Dialog
        open={deleteConfirmOpen}
        onClose={handleDeleteCancel}
      >
        <DialogTitle>Delete Snapshot</DialogTitle>
        <DialogContent>
          <Typography>
            Are you sure you want to delete this snapshot? This action cannot be undone.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDeleteCancel}>Cancel</Button>
          <Button 
            onClick={handleDeleteConfirm} 
            color="error" 
            variant="contained"
            disabled={deleteMutation.isLoading}
          >
            {deleteMutation.isLoading ? <CircularProgress size={20} /> : 'Delete'}
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

export default SnapshotsPage; 