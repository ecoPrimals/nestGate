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
  PlayArrow as PlayIcon,
  ExpandMore as ExpandMoreIcon,
  ExpandLess as ExpandLessIcon,
  Save as SaveIcon,
} from '@mui/icons-material';
import BackupService from '../../services/BackupService';
import { BackupJob, BackupTarget, BackupJobStatus } from '../../types/backup';
import StatusChip from '../common/StatusChip';

const BackupJobManagement: React.FC = () => {
  const [jobs, setJobs] = useState<BackupJob[]>([]);
  const [targets, setTargets] = useState<BackupTarget[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalVisible, setModalVisible] = useState(false);
  const [confirmLoading, setConfirmLoading] = useState(false);
  const [editingJob, setEditingJob] = useState<BackupJob | null>(null);
  const [expandedRow, setExpandedRow] = useState<string | null>(null);
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [deleteConfirmOpen, setDeleteConfirmOpen] = useState(false);
  const [jobToDelete, setJobToDelete] = useState<string | null>(null);
  const [runningJobs, setRunningJobs] = useState<Set<string>>(new Set());

  // Form state
  const [formData, setFormData] = useState({
    name: '',
    description: '',
    source: '',
    targetId: '',
    schedule: '',
    hourlyRetention: 0,
    dailyRetention: 0,
    weeklyRetention: 0,
    monthlyRetention: 0,
    yearlyRetention: 0,
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

  const fetchData = async () => {
    setLoading(true);
    try {
      const [jobsData, targetsData] = await Promise.all([
        backupService.getJobs(),
        backupService.getTargets()
      ]);
      setJobs(jobsData);
      setTargets(targetsData);
    } catch (error) {
      showNotification('Failed to fetch data', 'error');
      console.error('Failed to fetch data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const resetForm = () => {
    setFormData({
      name: '',
      description: '',
      source: '',
      targetId: '',
      schedule: '',
      hourlyRetention: 0,
      dailyRetention: 0,
      weeklyRetention: 0,
      monthlyRetention: 0,
      yearlyRetention: 0,
    });
    setFormErrors({});
  };

  const showAddModal = () => {
    setEditingJob(null);
    resetForm();
    setModalVisible(true);
  };

  const showEditModal = (record: BackupJob) => {
    setEditingJob(record);
    
    // Prepare form data
    setFormData({
      name: record.name,
      description: record.description || '',
      source: record.source,
      targetId: record.targetId,
      schedule: record.schedule,
      hourlyRetention: record.retention.hourly || 0,
      dailyRetention: record.retention.daily || 0,
      weeklyRetention: record.retention.weekly || 0,
      monthlyRetention: record.retention.monthly || 0,
      yearlyRetention: record.retention.yearly || 0,
    });
    
    setModalVisible(true);
  };

  const handleCancel = () => {
    setModalVisible(false);
    resetForm();
  };

  const validateForm = (): boolean => {
    const errors: Record<string, string> = {};

    if (!formData.name.trim()) {
      errors.name = 'Please enter a job name';
    }

    if (!formData.source.trim()) {
      errors.source = 'Please enter a source dataset or path';
    }

    if (!formData.targetId) {
      errors.targetId = 'Please select a backup target';
    }

    if (!formData.schedule.trim()) {
      errors.schedule = 'Please enter a cron schedule';
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
      const jobData = {
        name: formData.name,
        description: formData.description,
        source: formData.source,
        targetId: formData.targetId,
        schedule: formData.schedule,
        retention: {
          hourly: formData.hourlyRetention || null,
          daily: formData.dailyRetention || null,
          weekly: formData.weeklyRetention || null,
          monthly: formData.monthlyRetention || null,
          yearly: formData.yearlyRetention || null,
        }
      };
      
      if (editingJob) {
        // Update existing job
        const updatedJob = await backupService.updateJob({
          ...jobData,
          id: editingJob.id,
          status: editingJob.status,
          lastRun: editingJob.lastRun,
          nextRun: editingJob.nextRun,
          created: editingJob.created,
          modified: new Date().toISOString()
        });
        setJobs(jobs.map(j => j.id === updatedJob.id ? updatedJob : j));
        showNotification(`Job ${updatedJob.name} updated successfully`);
      } else {
        // Create new job
        const newJob = await backupService.createJob(jobData);
        setJobs([...jobs, newJob]);
        showNotification(`Job ${newJob.name} created successfully`);
      }
      
      setModalVisible(false);
      resetForm();
    } catch (error) {
      console.error('Error submitting form:', error);
      showNotification('Failed to save backup job', 'error');
    } finally {
      setConfirmLoading(false);
    }
  };

  const handleDeleteClick = (id: string) => {
    setJobToDelete(id);
    setDeleteConfirmOpen(true);
  };

  const handleDeleteConfirm = async () => {
    if (!jobToDelete) return;

    try {
      await backupService.deleteJob(jobToDelete);
      setJobs(jobs.filter(j => j.id !== jobToDelete));
      showNotification('Job deleted successfully');
    } catch (error) {
      console.error('Error deleting job:', error);
      showNotification('Failed to delete job', 'error');
    } finally {
      setDeleteConfirmOpen(false);
      setJobToDelete(null);
    }
  };

  const handleDeleteCancel = () => {
    setDeleteConfirmOpen(false);
    setJobToDelete(null);
  };

  const runJob = async (id: string) => {
    setRunningJobs(prev => new Set(prev).add(id));
    try {
      await backupService.runJob(id);
      // Update job status in the list
      setJobs(jobs.map(job => {
        if (job.id === id) {
          return { ...job, status: 'Running' as BackupJobStatus, lastRun: new Date().toISOString() };
        }
        return job;
      }));
      showNotification('Backup job started');
      
      // Poll for updates (in a real implementation this would be handled by WebSockets)
      setTimeout(() => {
        fetchData();
        setRunningJobs(prev => {
          const newSet = new Set(prev);
          newSet.delete(id);
          return newSet;
        });
      }, 3000);
    } catch (error) {
      console.error('Error running job:', error);
      showNotification('Failed to run backup job', 'error');
      setRunningJobs(prev => {
        const newSet = new Set(prev);
        newSet.delete(id);
        return newSet;
      });
    }
  };

  const getTargetName = (targetId: string) => {
    const target = targets.find(t => t.id === targetId);
    return target ? target.name : 'Unknown';
  };

  const handleFormChange = (field: string, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    
    // Clear error when user starts typing
    if (formErrors[field]) {
      setFormErrors(prev => ({ ...prev, [field]: '' }));
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

  const renderJobDetails = (record: BackupJob) => {
              return (
      <Box sx={{ padding: 2, backgroundColor: 'grey.50' }}>
        <Typography variant="body2" sx={{ marginBottom: 1 }}>
          {record.description || 'No description provided'}
        </Typography>
        <Typography variant="body2" sx={{ marginBottom: 1 }}>
          <strong>Created:</strong> {new Date(record.created).toLocaleString()}
        </Typography>
        <Typography variant="body2" sx={{ marginBottom: 1 }}>
          <strong>Last Modified:</strong> {new Date(record.modified).toLocaleString()}
        </Typography>
        <Typography variant="body2" sx={{ marginBottom: 1 }}>
          <strong>Retention Policy:</strong>
        </Typography>
        <Box component="ul" sx={{ margin: 0, paddingLeft: 2 }}>
                    {record.retention.hourly && <li><strong>Hourly:</strong> {record.retention.hourly} hours</li>}
                    {record.retention.daily && <li><strong>Daily:</strong> {record.retention.daily} days</li>}
                    {record.retention.weekly && <li><strong>Weekly:</strong> {record.retention.weekly} weeks</li>}
                    {record.retention.monthly && <li><strong>Monthly:</strong> {record.retention.monthly} months</li>}
                    {record.retention.yearly && <li><strong>Yearly:</strong> {record.retention.yearly} years</li>}
                    {!record.retention.hourly && !record.retention.daily && !record.retention.weekly && 
                     !record.retention.monthly && !record.retention.yearly && <li>No retention policy specified</li>}
        </Box>
      </Box>
    );
  };

  const paginatedJobs = jobs.slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage);

  return (
    <Box>
      <Card>
        <CardHeader
          title="Backup Jobs"
          action={
            <Button 
              variant="contained"
              startIcon={<AddIcon />}
              onClick={showAddModal}
            >
              Add Job
            </Button>
          }
        />
        <CardContent>
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Source</TableCell>
                  <TableCell>Target</TableCell>
                  <TableCell>Schedule</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Last Run</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {loading ? (
                  <TableRow>
                    <TableCell colSpan={7} sx={{ textAlign: 'center', padding: 4 }}>
                      <CircularProgress />
                    </TableCell>
                  </TableRow>
                ) : paginatedJobs.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={7} sx={{ textAlign: 'center', padding: 4 }}>
                      <Typography color="text.secondary">No backup jobs found</Typography>
                    </TableCell>
                  </TableRow>
                ) : (
                  paginatedJobs.map((job) => (
                    <React.Fragment key={job.id}>
                      <TableRow>
                        <TableCell>
                          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                            <IconButton
                              size="small"
                              onClick={() => handleRowExpand(job.id)}
                            >
                              {expandedRow === job.id ? <ExpandLessIcon /> : <ExpandMoreIcon />}
                            </IconButton>
                            <Typography variant="body2" fontWeight="bold">
                              {job.name}
                            </Typography>
                          </Box>
                        </TableCell>
                        <TableCell>{job.source}</TableCell>
                        <TableCell>{getTargetName(job.targetId)}</TableCell>
                        <TableCell>{job.schedule}</TableCell>
                        <TableCell>
                          <StatusChip 
                            status={job.status}
                            variant="outlined"
                          />
                        </TableCell>
                        <TableCell>
                          {job.lastRun ? new Date(job.lastRun).toLocaleString() : 'Never'}
                        </TableCell>
                        <TableCell>
                          <Stack direction="row">
                            <IconButton
                              size="small"
                              color="success"
                              disabled={job.status === 'Running' || runningJobs.has(job.id)}
                              onClick={() => runJob(job.id)}
                              title="Run backup job"
                            >
                              {runningJobs.has(job.id) ? <CircularProgress size={16} /> : <PlayIcon />}
                            </IconButton>
                            <IconButton
                              size="small"
                              color="primary"
                              onClick={() => showEditModal(job)}
                              title="Edit job"
                            >
                              <EditIcon />
                            </IconButton>
                            <IconButton
                              size="small"
                              color="error"
                              onClick={() => handleDeleteClick(job.id)}
                              title="Delete job"
                            >
                              <DeleteIcon />
                            </IconButton>
                          </Stack>
                        </TableCell>
                      </TableRow>
                      <TableRow>
                        <TableCell colSpan={7} sx={{ padding: 0 }}>
                          <Collapse in={expandedRow === job.id} timeout="auto" unmountOnExit>
                            {renderJobDetails(job)}
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
              count={jobs.length}
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
          {editingJob ? 'Edit Backup Job' : 'Add Backup Job'}
        </DialogTitle>
        <DialogContent>
          <Box sx={{ paddingTop: 1 }}>
            <Stack>
              <TextField
            label="Job Name"
                value={formData.name}
                onChange={(e) => handleFormChange('name', e.target.value)}
                error={!!formErrors.name}
                helperText={formErrors.name}
                placeholder="Daily Backup"
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
              
              <TextField
            label="Source Dataset/Path"
                value={formData.source}
                onChange={(e) => handleFormChange('source', e.target.value)}
                error={!!formErrors.source}
                helperText={formErrors.source}
                placeholder="tank/data"
                required
                fullWidth
              />
              
              <FormControl fullWidth required>
                <InputLabel>Backup Target</InputLabel>
                <Select
                  value={formData.targetId}
                  onChange={(e) => handleFormChange('targetId', e.target.value)}
            label="Backup Target"
                  error={!!formErrors.targetId}
          >
              {targets.map(target => (
                    <MenuItem key={target.id} value={target.id}>
                  {target.name} ({target.targetType.type})
                    </MenuItem>
              ))}
            </Select>
                {formErrors.targetId && (
                  <Typography variant="caption" color="error" sx={{ ml: 1.75, mt: 0.5 }}>
                    {formErrors.targetId}
                  </Typography>
                )}
              </FormControl>
              
              <TextField
            label="Schedule (Cron Expression)"
                value={formData.schedule}
                onChange={(e) => handleFormChange('schedule', e.target.value)}
                error={!!formErrors.schedule}
                helperText={formErrors.schedule}
                placeholder="0 0 * * *"
                required
                fullWidth
              />
              
              <Typography variant="h6" sx={{ mt: 2 }}>Retention Policy</Typography>
              
              <TextField
            label="Hourly Backups (hours)"
                type="number"
                value={formData.hourlyRetention}
                onChange={(e) => handleFormChange('hourlyRetention', parseInt(e.target.value) || 0)}
                inputProps={{ min: 0 }}
                placeholder="24"
                fullWidth
              />
              
              <TextField
            label="Daily Backups (days)"
                type="number"
                value={formData.dailyRetention}
                onChange={(e) => handleFormChange('dailyRetention', parseInt(e.target.value) || 0)}
                inputProps={{ min: 0 }}
                placeholder="7"
                fullWidth
              />
              
              <TextField
            label="Weekly Backups (weeks)"
                type="number"
                value={formData.weeklyRetention}
                onChange={(e) => handleFormChange('weeklyRetention', parseInt(e.target.value) || 0)}
                inputProps={{ min: 0 }}
                placeholder="4"
                fullWidth
              />
              
              <TextField
                label="Monthly Backups (months)"
                type="number"
                value={formData.monthlyRetention}
                onChange={(e) => handleFormChange('monthlyRetention', parseInt(e.target.value) || 0)}
                inputProps={{ min: 0 }}
                placeholder="12"
                fullWidth
              />
              
              <TextField
                label="Yearly Backups (years)"
                type="number"
                value={formData.yearlyRetention}
                onChange={(e) => handleFormChange('yearlyRetention', parseInt(e.target.value) || 0)}
                inputProps={{ min: 0 }}
                placeholder="5"
                fullWidth
              />
            </Stack>
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
            {editingJob ? 'Update' : 'Create'}
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
            Are you sure you want to delete this backup job? This action cannot be undone.
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

export default BackupJobManagement; 