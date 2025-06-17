import React, { useState, useEffect } from 'react';
import { 
  Card, 
  CardContent,
  CardHeader,
  Stepper,
  Step,
  StepLabel,
  Button, 
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
  Checkbox, 
  FormControlLabel,
  Alert, 
  Typography, 
  Box,
  Stack,
  CircularProgress,
  LinearProgress,
  TablePagination,
  List,
  ListItem,
  ListItemText,
  IconButton,
  Snackbar,
} from '@mui/material';
import { 
  PlayArrow as PlayIcon,
  Stop as StopIcon,
  Delete as DeleteIcon,
  Description as FileIcon,
  Restore as RestoreIcon,
} from '@mui/icons-material';
import BackupService, { BackupSnapshot, RestoreJob } from '../../services/BackupService';
import StatusChip from '../common/StatusChip';

const RestoreWorkflow: React.FC = () => {
  // State for current step
  const [currentStep, setCurrentStep] = useState<number>(0);
  
  // States for snapshots
  const [snapshots, setSnapshots] = useState<BackupSnapshot[]>([]);
  const [loadingSnapshots, setLoadingSnapshots] = useState<boolean>(false);
  const [selectedSnapshot, setSelectedSnapshot] = useState<BackupSnapshot | null>(null);
  
  // States for restore jobs
  const [restoreJobs, setRestoreJobs] = useState<RestoreJob[]>([]);
  const [loadingJobs, setLoadingJobs] = useState<boolean>(false);
  const [selectedRestoreJob, setSelectedRestoreJob] = useState<RestoreJob | null>(null);
  
  // States for form
  const [formVisible, setFormVisible] = useState<boolean>(false);
  const [formData, setFormData] = useState({
    snapshotId: '',
    destination: '',
    overwrite: false,
    description: '',
  });
  const [formErrors, setFormErrors] = useState<Record<string, string>>({});
  
  // States for job progress
  const [jobProgress, setJobProgress] = useState<number | null>(null);
  const [jobStatus, setJobStatus] = useState<string | null>(null);
  
  // States for job details modal
  const [jobDetailsVisible, setJobDetailsVisible] = useState<boolean>(false);
  
  // States for pagination
  const [snapshotPage, setSnapshotPage] = useState(0);
  const [snapshotRowsPerPage, setSnapshotRowsPerPage] = useState(10);
  const [jobPage, setJobPage] = useState(0);
  const [jobRowsPerPage, setJobRowsPerPage] = useState(10);
  
  // States for notifications
  const [snackbar, setSnackbar] = useState<{
    open: boolean;
    message: string;
    severity: 'success' | 'error';
  }>({
    open: false,
    message: '',
    severity: 'success',
  });

  const showNotification = (message: string, severity: 'success' | 'error' = 'success') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar(prev => ({ ...prev, open: false }));
  };
  
  // Fetch snapshots and restore jobs on component mount
  useEffect(() => {
    fetchData();
  }, []);
  
  // Function to fetch data
  const fetchData = async () => {
    // Fetch snapshots
    setLoadingSnapshots(true);
    try {
      const snapshotsData = await BackupService.getSnapshots();
      setSnapshots(snapshotsData);
    } catch (error) {
      console.error('Error fetching snapshots:', error);
      showNotification('Failed to fetch snapshots. Please try again later.', 'error');
    } finally {
      setLoadingSnapshots(false);
    }
    
    // Fetch restore jobs
    setLoadingJobs(true);
    try {
      const jobsData = await BackupService.getRestoreJobs();
      setRestoreJobs(jobsData);
    } catch (error) {
      console.error('Error fetching restore jobs:', error);
      showNotification('Failed to fetch restore jobs. Please try again later.', 'error');
    } finally {
      setLoadingJobs(false);
    }
  };
  
  // Function to handle next step
  const handleNextStep = () => {
    setCurrentStep(currentStep + 1);
  };
  
  // Function to handle previous step
  const handlePreviousStep = () => {
    setCurrentStep(currentStep - 1);
  };
  
  // Function to select a snapshot
  const handleSelectSnapshot = (snapshot: BackupSnapshot) => {
    setSelectedSnapshot(snapshot);
    handleNextStep();
  };
  
  // Function to show restore form
  const showRestoreForm = () => {
    if (selectedSnapshot) {
      setFormData({
        snapshotId: selectedSnapshot.id,
        destination: `/restore/${selectedSnapshot.source}`,
        overwrite: false,
        description: `Restore of ${selectedSnapshot.name}`
      });
    }
    setFormVisible(true);
  };
  
  // Function to hide restore form
  const hideRestoreForm = () => {
    setFormVisible(false);
    setFormErrors({});
  };

  // Form validation
  const validateForm = (): boolean => {
    const errors: Record<string, string> = {};

    if (!formData.destination.trim()) {
      errors.destination = 'Please enter a destination path';
    }

    setFormErrors(errors);
    return Object.keys(errors).length === 0;
  };
  
  // Function to handle form submission
  const handleFormSubmit = async () => {
    if (!validateForm()) {
      return;
    }

    try {
      setLoadingJobs(true);
      
      // Create restore job
      const newJob = await BackupService.createRestoreJob(formData);
      
      // Update restore jobs list
      setRestoreJobs([...restoreJobs, newJob]);
      
      // Select the newly created job
      setSelectedRestoreJob(newJob);
      
      // Hide form
      hideRestoreForm();
      
      // Move to next step
      handleNextStep();
      
      showNotification('Restore job created successfully');
    } catch (error) {
      console.error('Form submission failed:', error);
      showNotification('Failed to create restore job', 'error');
    } finally {
      setLoadingJobs(false);
    }
  };
  
  // Function to run a restore job
  const runRestoreJob = async (id: string) => {
    try {
      setJobStatus('running');
      setJobProgress(0);
      
      // Run job
      await BackupService.runRestoreJob(id);
      
      // Simulate progress updates
      const interval = setInterval(() => {
        setJobProgress((prev) => {
          if (prev === null) return 0;
          if (prev >= 100) {
            clearInterval(interval);
            setJobStatus('completed');
            return 100;
          }
          return prev + 10;
        });
      }, 1000);
      
      showNotification('Restore job started successfully');
      
      // Refresh jobs after a delay
      setTimeout(() => {
        fetchData();
      }, 12000);
    } catch (error) {
      console.error(`Error running restore job ${id}:`, error);
      showNotification(`Failed to run restore job. Please try again later.`, 'error');
      setJobStatus('failed');
    }
  };
  
  // Function to cancel a restore job
  const cancelRestoreJob = async (id: string) => {
    try {
      await BackupService.cancelRestoreJob(id);
      setJobStatus('failed');
      
      // Refresh jobs
      fetchData();
      
      showNotification('Restore job canceled');
    } catch (error) {
      console.error(`Error canceling restore job ${id}:`, error);
      showNotification(`Failed to cancel restore job. Please try again later.`, 'error');
    }
  };
  
  // Function to delete a restore job
  const deleteRestoreJob = async (id: string) => {
    try {
      await BackupService.deleteRestoreJob(id);
      
      // Remove job from list
      setRestoreJobs(restoreJobs.filter(job => job.id !== id));
      
      // If the deleted job was selected, clear selection
      if (selectedRestoreJob && selectedRestoreJob.id === id) {
        setSelectedRestoreJob(null);
      }
      
      showNotification('Restore job deleted successfully');
    } catch (error) {
      console.error(`Error deleting restore job ${id}:`, error);
      showNotification(`Failed to delete restore job. Please try again later.`, 'error');
    }
  };
  
  // Function to show job details
  const showJobDetails = (job: RestoreJob) => {
    setSelectedRestoreJob(job);
    setJobDetailsVisible(true);
  };
  
  // Function to hide job details
  const hideJobDetails = () => {
    setJobDetailsVisible(false);
  };
  
  // Format file size
  const formatSize = (size: number) => {
        const units = ['B', 'KB', 'MB', 'GB', 'TB'];
        let formattedSize = size;
        let unitIndex = 0;
        
        while (formattedSize >= 1024 && unitIndex < units.length - 1) {
          formattedSize /= 1024;
          unitIndex++;
        }
        
        return `${formattedSize.toFixed(2)} ${units[unitIndex]}`;
  };

  const handleFormChange = (field: string, value: any) => {
    setFormData(prev => ({ ...prev, [field]: value }));
    
    // Clear error when user starts typing
    if (formErrors[field]) {
      setFormErrors(prev => ({ ...prev, [field]: '' }));
    }
  };
  
  // Steps content
  const steps = [
    {
      title: 'Select Snapshot',
      content: (
        <Box>
          <Typography variant="body1" sx={{ marginBottom: 2 }}>
            Select a backup snapshot to restore from. This will be the source of the restored data.
          </Typography>
          
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Source</TableCell>
                  <TableCell>Size</TableCell>
                  <TableCell>Created</TableCell>
                  <TableCell>Type</TableCell>
                  <TableCell>Action</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {loadingSnapshots ? (
                  <TableRow>
                    <TableCell colSpan={6} sx={{ textAlign: 'center', padding: 4 }}>
                      <CircularProgress />
                    </TableCell>
                  </TableRow>
                ) : snapshots.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={6} sx={{ textAlign: 'center', padding: 4 }}>
                      <Typography color="text.secondary">No snapshots found</Typography>
                    </TableCell>
                  </TableRow>
                ) : (
                  snapshots
                    .slice(snapshotPage * snapshotRowsPerPage, snapshotPage * snapshotRowsPerPage + snapshotRowsPerPage)
                    .map((snapshot) => (
                      <TableRow key={snapshot.id}>
                        <TableCell>{snapshot.name}</TableCell>
                        <TableCell>{snapshot.source}</TableCell>
                        <TableCell>{formatSize(snapshot.size)}</TableCell>
                        <TableCell>{new Date(snapshot.created).toLocaleString()}</TableCell>
                        <TableCell>{snapshot.snapshotType}</TableCell>
                        <TableCell>
                          <Button 
                            variant="contained"
                            size="small"
                            onClick={() => handleSelectSnapshot(snapshot)}
                          >
                            Select
                          </Button>
                        </TableCell>
                      </TableRow>
                    ))
                )}
              </TableBody>
            </Table>
            <TablePagination
              rowsPerPageOptions={[5, 10, 25]}
              component="div"
              count={snapshots.length}
              rowsPerPage={snapshotRowsPerPage}
              page={snapshotPage}
              onPageChange={(_, newPage) => setSnapshotPage(newPage)}
              onRowsPerPageChange={(e) => {
                setSnapshotRowsPerPage(parseInt(e.target.value, 10));
                setSnapshotPage(0);
              }}
            />
          </TableContainer>
        </Box>
      )
    },
    {
      title: 'Configure Restore',
      content: (
        <Box>
          <Typography variant="body1" sx={{ marginBottom: 2 }}>
            Configure the restore operation. Specify the destination path and other options.
          </Typography>
          
          {selectedSnapshot && (
            <Card sx={{ marginBottom: 2 }}>
              <CardHeader title="Selected Snapshot" />
              <CardContent>
                <List>
                  <ListItem>
                    <ListItemText primary="Name" secondary={selectedSnapshot.name} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Source" secondary={selectedSnapshot.source} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Size" secondary={formatSize(selectedSnapshot.size)} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Created" secondary={new Date(selectedSnapshot.created).toLocaleString()} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Type" secondary={selectedSnapshot.snapshotType} />
                  </ListItem>
                </List>
              </CardContent>
            </Card>
          )}
          
          <Button variant="contained" onClick={showRestoreForm}>
            Configure Restore
          </Button>
          
          <Dialog
            open={formVisible}
            onClose={hideRestoreForm}
            maxWidth="sm"
            fullWidth
          >
            <DialogTitle>Configure Restore</DialogTitle>
            <DialogContent>
              <Stack spacing={3} sx={{ paddingTop: 1 }}>
                <TextField
                label="Destination Path"
                  value={formData.destination}
                  onChange={(e) => handleFormChange('destination', e.target.value)}
                  error={!!formErrors.destination}
                  helperText={formErrors.destination}
                  placeholder="/path/to/restore"
                  required
                  fullWidth
                />
                
                <FormControlLabel
                  control={
                    <Checkbox
                      checked={formData.overwrite}
                      onChange={(e) => handleFormChange('overwrite', e.target.checked)}
                    />
                  }
                  label="Overwrite existing files"
                />
                
                <TextField
                label="Description"
                  value={formData.description}
                  onChange={(e) => handleFormChange('description', e.target.value)}
                  placeholder="Description of the restore job"
                  multiline
                  rows={3}
                  fullWidth
                />
              </Stack>
            </DialogContent>
            <DialogActions>
              <Button onClick={hideRestoreForm}>Cancel</Button>
              <Button 
                onClick={handleFormSubmit} 
                variant="contained"
                disabled={loadingJobs}
              >
                {loadingJobs ? <CircularProgress size={20} /> : 'Create Job'}
              </Button>
            </DialogActions>
          </Dialog>
        </Box>
      )
    },
    {
      title: 'Start Restore',
      content: (
        <Box>
          <Typography variant="body1" sx={{ marginBottom: 2 }}>
            Review the restore job and start the restore process.
          </Typography>
          
          {selectedRestoreJob && (
            <Card sx={{ marginBottom: 2 }}>
              <CardHeader title="Restore Job" />
              <CardContent>
                <List>
                  <ListItem>
                    <ListItemText primary="ID" secondary={selectedRestoreJob.id} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Destination" secondary={selectedRestoreJob.destination} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Overwrite" secondary={selectedRestoreJob.overwrite ? 'Yes' : 'No'} />
                  </ListItem>
                  <ListItem>
                    <ListItemText 
                      primary="Status" 
                      secondary={<StatusChip status={selectedRestoreJob.status} />} 
                    />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Description" secondary={selectedRestoreJob.description || '-'} />
                  </ListItem>
                </List>
                
                <Box sx={{ marginTop: 2 }}>
                {selectedRestoreJob.status === 'idle' && (
                  <Button 
                      variant="contained"
                      startIcon={<PlayIcon />}
                    onClick={() => runRestoreJob(selectedRestoreJob.id)}
                      disabled={jobStatus === 'running'}
                  >
                    Start Restore
                  </Button>
                )}
                
                {jobStatus === 'running' && (
                    <Box>
                      <Typography variant="body2" sx={{ marginBottom: 1 }}>
                        Progress: {jobProgress}%
                      </Typography>
                      <LinearProgress 
                        variant="determinate" 
                        value={jobProgress || 0} 
                        sx={{ marginBottom: 2 }}
                      />
                    
                    <Button 
                        variant="outlined"
                        color="error"
                        startIcon={<StopIcon />}
                      onClick={() => cancelRestoreJob(selectedRestoreJob.id)}
                    >
                      Cancel Restore
                    </Button>
                    </Box>
                )}
                
                {jobStatus === 'completed' && (
                    <Alert severity="success">
                      Restore completed successfully
                    </Alert>
                )}
                
                {jobStatus === 'failed' && (
                    <Alert severity="error">
                      Restore failed or was canceled
                    </Alert>
                  )}
                </Box>
              </CardContent>
            </Card>
          )}
        </Box>
      )
    },
    {
      title: 'Restore History',
      content: (
        <Box>
          <Typography variant="body1" sx={{ marginBottom: 2 }}>
            View and manage restore job history. You can see the status of current and past restore jobs.
          </Typography>
          
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>ID</TableCell>
                  <TableCell>Destination</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Start Time</TableCell>
                  <TableCell>End Time</TableCell>
                  <TableCell>Actions</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {loadingJobs ? (
                  <TableRow>
                    <TableCell colSpan={6} sx={{ textAlign: 'center', padding: 4 }}>
                      <CircularProgress />
                    </TableCell>
                  </TableRow>
                ) : restoreJobs.length === 0 ? (
                  <TableRow>
                    <TableCell colSpan={6} sx={{ textAlign: 'center', padding: 4 }}>
                      <Typography color="text.secondary">No restore jobs found</Typography>
                    </TableCell>
                  </TableRow>
                ) : (
                  restoreJobs
                    .slice(jobPage * jobRowsPerPage, jobPage * jobRowsPerPage + jobRowsPerPage)
                    .map((job) => (
                      <TableRow key={job.id}>
                        <TableCell>{job.id}</TableCell>
                        <TableCell>{job.destination}</TableCell>
                        <TableCell>
                          <StatusChip status={job.status} />
                        </TableCell>
                        <TableCell>
                          {job.startTime ? new Date(job.startTime).toLocaleString() : '-'}
                        </TableCell>
                        <TableCell>
                          {job.endTime ? new Date(job.endTime).toLocaleString() : '-'}
                        </TableCell>
                        <TableCell>
                          <Stack direction="row" spacing={1}>
                            {job.status === 'idle' && (
                              <IconButton
                                size="small"
                                color="primary"
                                onClick={() => runRestoreJob(job.id)}
                                title="Run job"
                              >
                                <PlayIcon />
                              </IconButton>
                            )}
                            
                            {job.status === 'running' && (
                              <IconButton
                                size="small"
                                color="error"
                                onClick={() => cancelRestoreJob(job.id)}
                                title="Cancel job"
                              >
                                <StopIcon />
                              </IconButton>
                            )}
                            
                            {job.status !== 'running' && (
                              <IconButton
                                size="small"
                                color="error"
                                onClick={() => deleteRestoreJob(job.id)}
                                title="Delete job"
                              >
                                <DeleteIcon />
                              </IconButton>
                            )}
                            
                            <IconButton
                              size="small"
                              onClick={() => showJobDetails(job)}
                              title="View details"
                            >
                              <FileIcon />
                            </IconButton>
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
              count={restoreJobs.length}
              rowsPerPage={jobRowsPerPage}
              page={jobPage}
              onPageChange={(_, newPage) => setJobPage(newPage)}
              onRowsPerPageChange={(e) => {
                setJobRowsPerPage(parseInt(e.target.value, 10));
                setJobPage(0);
              }}
            />
          </TableContainer>
          
          <Dialog
            open={jobDetailsVisible}
            onClose={hideJobDetails}
            maxWidth="md"
            fullWidth
          >
            <DialogTitle>Restore Job Details</DialogTitle>
            <DialogContent>
            {selectedRestoreJob && (
                <List>
                  <ListItem>
                    <ListItemText primary="ID" secondary={selectedRestoreJob.id} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Snapshot ID" secondary={selectedRestoreJob.snapshotId} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Destination" secondary={selectedRestoreJob.destination} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Overwrite" secondary={selectedRestoreJob.overwrite ? 'Yes' : 'No'} />
                  </ListItem>
                  <ListItem>
                    <ListItemText 
                      primary="Status" 
                      secondary={<StatusChip status={selectedRestoreJob.status} />} 
                    />
                  </ListItem>
                  <ListItem>
                    <ListItemText 
                      primary="Start Time" 
                      secondary={selectedRestoreJob.startTime ? new Date(selectedRestoreJob.startTime).toLocaleString() : '-'} 
                    />
                  </ListItem>
                  <ListItem>
                    <ListItemText 
                      primary="End Time" 
                      secondary={selectedRestoreJob.endTime ? new Date(selectedRestoreJob.endTime).toLocaleString() : '-'} 
                    />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Created" secondary={new Date(selectedRestoreJob.created).toLocaleString()} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Modified" secondary={new Date(selectedRestoreJob.modified).toLocaleString()} />
                  </ListItem>
                  <ListItem>
                    <ListItemText primary="Description" secondary={selectedRestoreJob.description || '-'} />
                  </ListItem>
                </List>
              )}
            </DialogContent>
            <DialogActions>
              <Button onClick={hideJobDetails}>Close</Button>
            </DialogActions>
          </Dialog>
        </Box>
      )
    }
  ];
  
  return (
    <Card>
      <CardHeader title="Backup Restoration" />
      <CardContent>
        <Box sx={{ marginBottom: 3 }}>
          <Stepper activeStep={currentStep}>
            {steps.map((step, index) => (
              <Step key={index}>
                <StepLabel>{step.title}</StepLabel>
              </Step>
            ))}
          </Stepper>
        </Box>
        
        <Box sx={{ marginBottom: 3 }}>
        {steps[currentStep].content}
        </Box>
      
        <Stack direction="row" spacing={1}>
        {currentStep > 0 && (
            <Button variant="outlined" onClick={handlePreviousStep}>
            Previous
          </Button>
        )}
        
        {currentStep < steps.length - 1 && (
          <Button 
              variant="contained"
            onClick={handleNextStep}
            disabled={
              (currentStep === 0 && !selectedSnapshot) || 
              (currentStep === 1 && !selectedRestoreJob)
            }
          >
            Next
          </Button>
        )}
        
        {currentStep === steps.length - 1 && (
          <Button 
              variant="contained"
              startIcon={<RestoreIcon />}
            onClick={() => {
              setCurrentStep(0);
              setSelectedSnapshot(null);
              setSelectedRestoreJob(null);
              setJobStatus(null);
              setJobProgress(null);
            }}
          >
            New Restore
          </Button>
        )}
        </Stack>
      </CardContent>

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
    </Card>
  );
};

export default RestoreWorkflow; 