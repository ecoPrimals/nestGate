import React, { useState, useEffect } from 'react';
import {
  Card,
  CardContent,
  CardHeader,
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
  Box,
  Stack,
  Typography,
  IconButton,
  Collapse,
  Alert,
  Snackbar,
  CircularProgress,
  TablePagination,
} from '@mui/material';
import {
  Add as AddIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  ExpandMore as ExpandMoreIcon,
  ExpandLess as ExpandLessIcon,
  Save as SaveIcon,
} from '@mui/icons-material';
import BackupService from '../../services/BackupService';
import { BackupTarget, BackupTargetType } from '../../types/backup';

const BackupTargetManagement: React.FC = () => {
  const [targets, setTargets] = useState<BackupTarget[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [editingTarget, setEditingTarget] = useState<BackupTarget | null>(null);
  const [targetType, setTargetType] = useState<'Local' | 'RemoteSsh' | 'Nfs'>('Local');
  const [expandedRow, setExpandedRow] = useState<string | null>(null);
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [deleteConfirmOpen, setDeleteConfirmOpen] = useState(false);
  const [targetToDelete, setTargetToDelete] = useState<string | null>(null);

  // Form state
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    targetType: 'Local' as 'Local' | 'RemoteSsh' | 'Nfs',
    path: '',
    host: '',
    port: 22,
    user: '',
    keyFile: '',
    server: '',
    export: '',
    mountPoint: '',
  });

  const [formErrors, setFormErrors] = useState<Record<string, string>>({});

  const [snackbar, setSnackbar] = useState<{
    open: boolean;
    message: string;
    severity: 'success' | 'error';
  }>({
    open: false,
    message: '',
    severity: 'success',
  });

  const backupService = BackupService;

  const showNotification = (message: string, severity: 'success' | 'error' = 'success') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar(prev => ({ ...prev, open: false }));
  };

  const fetchTargets = async () => {
    setLoading(true);
    try {
      const data = await backupService.getTargets();
      setTargets(data);
    } catch (error) {
      showNotification('Failed to fetch backup targets', 'error');
      console.error('Failed to fetch backup targets:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTargets();
  }, []);

  const resetForm = () => {
    setFormData({
      name: '',
      description: '',
      targetType: 'Local',
      path: '',
      host: '',
      port: 22,
      user: '',
      keyFile: '',
      server: '',
      export: '',
      mountPoint: '',
    });
    setFormErrors({});
    setTargetType('Local');
  };

  const showAddModal = () => {
    setEditingTarget(null);
    resetForm();
    setModalVisible(true);
  };

  const showEditModal = (record: BackupTarget) => {
    setEditingTarget(record);
    setTargetType(record.targetType.type);
    
    // Prepare form data
    const flattenedData = flattenTargetType(record.targetType);
    setFormData({
      name: record.name,
      description: record.description || '',
      targetType: record.targetType.type,
      ...flattenedData,
    });
    
    setModalVisible(true);
  };

  const flattenTargetType = (targetType: BackupTargetType) => {
    const defaultData = {
      path: '',
      host: '',
      port: 22,
      user: '',
      keyFile: '',
      server: '',
      export: '',
      mountPoint: '',
    };

    switch (targetType.type) {
      case 'Local':
        return { ...defaultData, path: targetType.path };
      case 'RemoteSsh':
        return {
          ...defaultData,
          host: targetType.host,
          port: targetType.port,
          user: targetType.user,
          path: targetType.path,
          keyFile: targetType.keyFile || '',
        };
      case 'Nfs':
        return {
          ...defaultData,
          server: targetType.server,
          export: targetType.export,
          mountPoint: targetType.mountPoint,
        };
      default:
        return defaultData;
    }
  };

  const handleCancel = () => {
    setModalVisible(false);
    resetForm();
  };

  const validateForm = (): boolean => {
    const errors: Record<string, string> = {};

    if (!formData.name.trim()) {
      errors.name = 'Please enter a target name';
    }

    switch (formData.targetType) {
      case 'Local':
        if (!formData.path.trim()) {
          errors.path = 'Please enter a local path';
        }
        break;
      case 'RemoteSsh':
        if (!formData.host.trim()) {
          errors.host = 'Please enter a hostname or IP address';
        }
        if (!formData.user.trim()) {
          errors.user = 'Please enter a username';
        }
        if (!formData.path.trim()) {
          errors.path = 'Please enter a remote path';
        }
        if (formData.port < 1 || formData.port > 65535) {
          errors.port = 'Port must be between 1 and 65535';
        }
        break;
      case 'Nfs':
        if (!formData.server.trim()) {
          errors.server = 'Please enter an NFS server hostname or IP';
        }
        if (!formData.export.trim()) {
          errors.export = 'Please enter an NFS export path';
        }
        if (!formData.mountPoint.trim()) {
          errors.mountPoint = 'Please enter a mount point';
        }
        break;
    }

    setFormErrors(errors);
    return Object.keys(errors).length === 0;
  };

  const handleSubmit = async () => {
    if (!validateForm()) {
      return;
    }

    setConfirmLoading(true);
    try {
      let targetTypeData: BackupTargetType;
      
      switch (formData.targetType) {
        case 'Local':
          targetTypeData = {
            type: 'Local',
            path: formData.path
          };
          break;
        case 'RemoteSsh':
          targetTypeData = {
            type: 'RemoteSsh',
            host: formData.host,
            port: formData.port,
            user: formData.user,
            path: formData.path,
            keyFile: formData.keyFile || undefined
          };
          break;
        case 'Nfs':
          targetTypeData = {
            type: 'Nfs',
            server: formData.server,
            export: formData.export,
            mountPoint: formData.mountPoint
          };
          break;
        default:
          throw new Error(`Invalid target type: ${formData.targetType}`);
      }
      
      const targetData = {
        name: formData.name,
        targetType: targetTypeData,
        description: formData.description
      };
      
      if (editingTarget) {
        // Update existing target
        const updatedTarget = await backupService.updateTarget({
          ...targetData,
          id: editingTarget.id,
          created: editingTarget.created,
          modified: new Date().toISOString()
        });
        setTargets(targets.map(t => t.id === updatedTarget.id ? updatedTarget : t));
        showNotification(`Target ${updatedTarget.name} updated successfully`);
      } else {
        // Create new target
        const newTarget = await backupService.createTarget(targetData);
        setTargets([...targets, newTarget]);
        showNotification(`Target ${newTarget.name} created successfully`);
      }
      
      setModalVisible(false);
      resetForm();
    } catch (error) {
      console.error('Error submitting form:', error);
      showNotification('Failed to save backup target', 'error');
    } finally {
      setConfirmLoading(false);
    }
  };

  const handleDeleteClick = (id: string) => {
    setTargetToDelete(id);
    setDeleteConfirmOpen(true);
  };

  const handleDeleteConfirm = async () => {
    if (!targetToDelete) return;

    try {
      await backupService.deleteTarget(targetToDelete);
      setTargets(targets.filter(t => t.id !== targetToDelete));
      showNotification('Target deleted successfully');
    } catch (error) {
      console.error('Error deleting target:', error);
      showNotification('Failed to delete target', 'error');
    } finally {
      setDeleteConfirmOpen(false);
      setTargetToDelete(null);
    }
  };

  const handleDeleteCancel = () => {
    setDeleteConfirmOpen(false);
    setTargetToDelete(null);
  };

  const handleFormChange = (field: string, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    
    // Clear error when user starts typing
    if (formErrors[field]) {
      setFormErrors(prev => ({ ...prev, [field]: '' }));
    }

    // Update targetType state when form targetType changes
    if (field === 'targetType') {
      setTargetType(value);
    }
  };

  const handleRowExpand = (id: string) => {
    setExpandedRow(expandedRow === id ? null : id);
  };

  const handleChangePage = (event: unknown, newPage: number) => {
    setPage(newPage);
  };

  const handleChangeRowsPerPage = (event: React.ChangeEvent<HTMLInputElement>) => {
    setRowsPerPage(parseInt(event.target.value, 10));
    setPage(0);
  };

  const getLocationDisplay = (targetType: BackupTargetType) => {
        switch (targetType.type) {
          case 'Local':
            return targetType.path;
          case 'RemoteSsh':
            return `${targetType.user}@${targetType.host}:${targetType.path}`;
          case 'Nfs':
            return `${targetType.server}:${targetType.export} → ${targetType.mountPoint}`;
          default:
            return 'Unknown';
        }
  };

  const renderTargetDetails = (record: BackupTarget) => {
    const targetDetails = (() => {
      switch (record.targetType.type) {
        case 'Local':
          return (
            <Box component="ul" sx={{ margin: 0, paddingLeft: 2 }}>
              <li><strong>Path:</strong> {record.targetType.path}</li>
            </Box>
          );
        case 'RemoteSsh':
          return (
            <Box component="ul" sx={{ margin: 0, paddingLeft: 2 }}>
              <li><strong>Host:</strong> {record.targetType.host}</li>
              <li><strong>Port:</strong> {record.targetType.port}</li>
              <li><strong>User:</strong> {record.targetType.user}</li>
              <li><strong>Path:</strong> {record.targetType.path}</li>
              {record.targetType.keyFile && <li><strong>Key File:</strong> {record.targetType.keyFile}</li>}
            </Box>
          );
        case 'Nfs':
          return (
            <Box component="ul" sx={{ margin: 0, paddingLeft: 2 }}>
              <li><strong>Server:</strong> {record.targetType.server}</li>
              <li><strong>Export:</strong> {record.targetType.export}</li>
              <li><strong>Mount Point:</strong> {record.targetType.mountPoint}</li>
            </Box>
          );
        default:
          return <Typography>Unknown target type</Typography>;
      }
    })();

    return (
      <Box sx={{ padding: 2, backgroundColor: 'grey.50' }}>
        <Typography variant="body2" sx={{ marginBottom: 1 }}>
          <strong>Created:</strong> {new Date(record.created).toLocaleString()}
        </Typography>
        <Typography variant="body2" sx={{ marginBottom: 1 }}>
          <strong>Last Modified:</strong> {new Date(record.modified).toLocaleString()}
        </Typography>
        <Typography variant="body2" sx={{ marginBottom: 1 }}>
          <strong>Target Details:</strong>
        </Typography>
        {targetDetails}
      </Box>
    );
  };

  const renderFormFields = () => {
    return (
      <Stack spacing={3}>
        <TextField
          label="Name"
          value={formData.name}
          onChange={(e) => handleFormChange('name', e.target.value)}
          error={!!formErrors.name}
          helperText={formErrors.name}
          placeholder="Backup Target Name"
          required
          fullWidth
        />
        
        <TextField
          label="Description"
          value={formData.description}
          onChange={(e) => handleFormChange('description', e.target.value)}
          placeholder="Description (optional)"
          multiline
          rows={3}
          fullWidth
        />
        
        <FormControl fullWidth required>
          <InputLabel>Target Type</InputLabel>
          <Select
            value={formData.targetType}
            onChange={(e) => handleFormChange('targetType', e.target.value)}
            label="Target Type"
          >
            <MenuItem value="Local">Local Storage</MenuItem>
            <MenuItem value="RemoteSsh">Remote SSH</MenuItem>
            <MenuItem value="Nfs">NFS Share</MenuItem>
          </Select>
        </FormControl>

        {/* Type-specific fields */}
        {formData.targetType === 'Local' && (
          <TextField
            label="Path"
            value={formData.path}
            onChange={(e) => handleFormChange('path', e.target.value)}
            error={!!formErrors.path}
            helperText={formErrors.path}
            placeholder="/path/to/backup/location"
            required
            fullWidth
          />
        )}

        {formData.targetType === 'RemoteSsh' && (
          <>
            <TextField
              label="Host"
              value={formData.host}
              onChange={(e) => handleFormChange('host', e.target.value)}
              error={!!formErrors.host}
              helperText={formErrors.host}
              placeholder="backup.example.com"
              required
              fullWidth
            />
            
            <TextField
              label="Port"
              type="number"
              value={formData.port}
              onChange={(e) => handleFormChange('port', parseInt(e.target.value) || 22)}
              error={!!formErrors.port}
              helperText={formErrors.port}
              inputProps={{ min: 1, max: 65535 }}
              required
              fullWidth
            />
            
            <TextField
              label="Username"
              value={formData.user}
              onChange={(e) => handleFormChange('user', e.target.value)}
              error={!!formErrors.user}
              helperText={formErrors.user}
              placeholder="backup"
              required
              fullWidth
            />
            
            <TextField
              label="Remote Path"
              value={formData.path}
              onChange={(e) => handleFormChange('path', e.target.value)}
              error={!!formErrors.path}
              helperText={formErrors.path}
              placeholder="/path/to/backup/directory"
              required
              fullWidth
            />
            
            <TextField
              label="SSH Key File (optional)"
              value={formData.keyFile}
              onChange={(e) => handleFormChange('keyFile', e.target.value)}
              placeholder="/home/user/.ssh/id_rsa"
              fullWidth
            />
          </>
        )}

        {formData.targetType === 'Nfs' && (
          <>
            <TextField
              label="NFS Server"
              value={formData.server}
              onChange={(e) => handleFormChange('server', e.target.value)}
              error={!!formErrors.server}
              helperText={formErrors.server}
              placeholder="nfs.example.com"
              required
              fullWidth
            />
            
            <TextField
              label="NFS Export"
              value={formData.export}
              onChange={(e) => handleFormChange('export', e.target.value)}
              error={!!formErrors.export}
              helperText={formErrors.export}
              placeholder="/export/backup"
              required
              fullWidth
            />
            
            <TextField
              label="Mount Point"
              value={formData.mountPoint}
              onChange={(e) => handleFormChange('mountPoint', e.target.value)}
              error={!!formErrors.mountPoint}
              helperText={formErrors.mountPoint}
              placeholder="/mnt/nfs-backup"
              required
              fullWidth
            />
          </>
        )}
      </Stack>
    );
  };

  const paginatedTargets = targets.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage);

  return (
    <Box>
      <Card>
        <CardHeader
          title="Backup Targets"
          action={
          <Button 
              variant="contained"
              startIcon={<AddIcon />}
            onClick={showAddModal}
          >
            Add Target
          </Button>
          }
        />
        <CardContent>
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Type</TableCell>
                  <TableCell>Location</TableCell>
                  <TableCell>Description</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {loading ? (
                  <TableRow>
                    <TableCell colSpan={5} sx={{ textAlign: 'center', padding: 4 }}>
                      <CircularProgress />
                    </TableCell>
                  </TableRow>
                ) : paginatedTargets.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={5} sx={{ textAlign: 'center', padding: 4 }}>
                      <Typography color="text.secondary">No backup targets found</Typography>
                    </TableCell>
                  </TableRow>
                ) : (
                  paginatedTargets.map((target) => (
                    <React.Fragment key={target.id}>
                      <TableRow>
                        <TableCell>
                          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                            <IconButton
                              size="small"
                              onClick={() => handleRowExpand(target.id)}
                            >
                              {expandedRow === target.id ? <ExpandLessIcon /> : <ExpandMoreIcon />}
                            </IconButton>
                            <Typography variant="body2" fontWeight="bold">
                              {target.name}
                            </Typography>
                          </Box>
                        </TableCell>
                        <TableCell>{target.targetType.type}</TableCell>
                        <TableCell>{getLocationDisplay(target.targetType)}</TableCell>
                        <TableCell>{target.description || '-'}</TableCell>
                        <TableCell>
                          <Stack direction="row" spacing={1}>
                            <IconButton
                              size="small"
                              color="primary"
                              onClick={() => showEditModal(target)}
                            >
                              <EditIcon />
                            </IconButton>
                            <IconButton
                              size="small"
                              color="error"
                              onClick={() => handleDeleteClick(target.id)}
                            >
                              <DeleteIcon />
                            </IconButton>
                          </Stack>
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell colSpan={5} sx={{ padding: 0 }}>
                          <Collapse in={expandedRow === target.id} timeout="auto" unmountOnExit>
                            {renderTargetDetails(target)}
                          </Collapse>
                        </TableCell>
                      </TableRow>
                    </React.Fragment>
                  ))
                )}
              </TableBody>
            </Table>
            <TablePagination
              rowsPerPageOptions={[5, 10, 25]}
              component="div"
              count={targets.length}
              rowsPerPage={rowsPerPage}
              page={page}
              onPageChange={handleChangePage}
              onRowsPerPageChange={handleChangeRowsPerPage}
            />
          </TableContainer>
        </CardContent>
      </Card>

      {/* Add/Edit Modal */}
      <Dialog
        open={modalVisible}
        onClose={handleCancel}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>
          {editingTarget ? 'Edit Backup Target' : 'Add Backup Target'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ paddingTop: 1 }}>
            {renderFormFields()}
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCancel}>
            Cancel
          </Button>
          <Button
            onClick={handleSubmit}
            variant="contained"
            disabled={confirmLoading}
            startIcon={confirmLoading ? <CircularProgress size={20} /> : <SaveIcon />}
          >
            {editingTarget ? 'Update' : 'Create'}
          </Button>
        </DialogActions>
      </Dialog>

      {/* Delete Confirmation Dialog */}
      <Dialog
        open={deleteConfirmOpen}
        onClose={handleDeleteCancel}
      >
        <DialogTitle>Confirm Delete</DialogTitle>
        <DialogContent>
          <Typography>
            Are you sure you want to delete this backup target? This action cannot be undone.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleDeleteCancel}>Cancel</Button>
          <Button onClick={handleDeleteConfirm} color="error" variant="contained">
            Delete
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

export default BackupTargetManagement; 