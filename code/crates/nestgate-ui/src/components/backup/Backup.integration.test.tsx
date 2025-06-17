import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import BackupManagement from './BackupManagement';
import BackupService from '../../services/BackupService';

// Mock the BackupService
jest.mock('../../services/BackupService', () => ({
  // Targets
  getTargets: jest.fn(),
  getTarget: jest.fn(),
  createTarget: jest.fn(),
  updateTarget: jest.fn(),
  deleteTarget: jest.fn(),
  
  // Jobs
  getJobs: jest.fn(),
  getJob: jest.fn(),
  createJob: jest.fn(),
  updateJob: jest.fn(),
  deleteJob: jest.fn(),
  runJob: jest.fn(),
  
  // Snapshots
  getSnapshots: jest.fn(),
  getSnapshot: jest.fn(),
  getSnapshotsForJob: jest.fn(),
  deleteSnapshot: jest.fn(),
  
  // Restore jobs
  getRestoreJobs: jest.fn(),
  getRestoreJob: jest.fn(),
  createRestoreJob: jest.fn(),
  runRestoreJob: jest.fn(),
  cancelRestoreJob: jest.fn(),
  deleteRestoreJob: jest.fn(),
  
  __esModule: true,
  default: {
    // Targets
    getTargets: jest.fn(),
    getTarget: jest.fn(),
    createTarget: jest.fn(),
    updateTarget: jest.fn(),
    deleteTarget: jest.fn(),
    
    // Jobs
    getJobs: jest.fn(),
    getJob: jest.fn(),
    createJob: jest.fn(),
    updateJob: jest.fn(),
    deleteJob: jest.fn(),
    runJob: jest.fn(),
    
    // Snapshots
    getSnapshots: jest.fn(),
    getSnapshot: jest.fn(),
    getSnapshotsForJob: jest.fn(),
    deleteSnapshot: jest.fn(),
    
    // Restore jobs
    getRestoreJobs: jest.fn(),
    getRestoreJob: jest.fn(),
    createRestoreJob: jest.fn(),
    runRestoreJob: jest.fn(),
    cancelRestoreJob: jest.fn(),
    deleteRestoreJob: jest.fn(),
  }
}));

// Mock timing functions
jest.useFakeTimers();

// Mock Ant Design components to simplify testing
jest.mock('antd', () => {
  const antd = jest.requireActual('antd');
  
  const mockModal = ({ title, open, onOk, onCancel, children }: any) => {
    if (!open) return null;
    return (
      <div data-testid="modal" onClick={onOk}>
        <div data-testid="modal-title">{title}</div>
        <div data-testid="modal-content">{children}</div>
        <button data-testid="modal-ok" onClick={onOk}>OK</button>
        <button data-testid="modal-cancel" onClick={onCancel}>Cancel</button>
      </div>
    );
  };
  
  const mockTabs = ({ activeKey, onChange, children }: any) => {
    return (
      <div data-testid="tabs">
        <div data-testid="tab-buttons">
          <button onClick={() => onChange('jobs')} data-active={activeKey === 'jobs'}>Backup Jobs</button>
          <button onClick={() => onChange('targets')} data-active={activeKey === 'targets'}>Backup Targets</button>
          <button onClick={() => onChange('restore')} data-active={activeKey === 'restore'}>Restore Data</button>
        </div>
        <div data-testid="tab-content">
          {React.Children.toArray(children).find(
            (child: any) => child.props.tab && child.props.tab.includes(
              activeKey === 'jobs' ? 'Backup Jobs' : 
              activeKey === 'targets' ? 'Backup Targets' : 'Restore Data'
            )
          )}
        </div>
      </div>
    );
  };
  
  return {
    ...antd,
    Modal: mockModal,
    Tabs: mockTabs,
    TabPane: ({ children }: any) => children,
  };
});

describe('Backup Integration Tests', () => {
  // Mock data for all tests
  const mockTargets = [
    {
      id: '1',
      name: 'Local Backup',
      targetType: 'local',
      path: '/mnt/backup',
      description: 'Local backup storage',
      created: new Date().toISOString(),
      modified: new Date().toISOString()
    },
    {
      id: '2',
      name: 'Remote SSH',
      targetType: 'remoteSsh',
      host: 'backup.example.com',
      port: 22,
      user: 'backup',
      path: '/backup',
      description: 'Remote SSH backup server',
      created: new Date().toISOString(),
      modified: new Date().toISOString()
    }
  ];
  
  const mockJobs = [
    {
      id: '1',
      name: 'Daily Backup',
      source: '/data',
      targetId: '1',
      schedule: '0 0 * * *',
      retention: {
        daily: 7,
        weekly: 4,
        monthly: 12
      },
      status: 'idle',
      description: 'Daily backup of all data',
      created: new Date().toISOString(),
      modified: new Date().toISOString()
    }
  ];
  
  const mockSnapshots = [
    {
      id: '1',
      name: 'Daily Backup-20230101-000000',
      jobId: '1',
      targetId: '1',
      source: '/data',
      size: 1024 * 1024 * 100, // 100MB
      created: new Date().toISOString(),
      snapshotType: 'daily'
    }
  ];
  
  interface MockRestoreJob {
    id: string;
    snapshotId: string;
    destination: string;
    overwrite: boolean;
    status: string;
    description: string;
    startTime?: string;
    endTime?: string;
    created: string;
    modified: string;
  }
  
  const mockRestoreJobs: MockRestoreJob[] = [
    {
      id: '1',
      snapshotId: '1',
      destination: '/tmp/restore',
      overwrite: true,
      status: 'idle',
      description: 'Test restore job',
      created: new Date().toISOString(),
      modified: new Date().toISOString()
    }
  ];
  
  beforeEach(() => {
    jest.clearAllMocks();
    
    // Setup default mock implementations
    (BackupService.getTargets as jest.Mock).mockResolvedValue(mockTargets);
    (BackupService.getJobs as jest.Mock).mockResolvedValue(mockJobs);
    (BackupService.getSnapshots as jest.Mock).mockResolvedValue(mockSnapshots);
    (BackupService.getRestoreJobs as jest.Mock).mockResolvedValue(mockRestoreJobs);
    
    // Implementation for creating new items
    (BackupService.createTarget as jest.Mock).mockImplementation((target) => 
      Promise.resolve({
        ...target,
        id: Date.now().toString(),
        created: new Date().toISOString(),
        modified: new Date().toISOString()
      })
    );
    
    (BackupService.createJob as jest.Mock).mockImplementation((job) => 
      Promise.resolve({
        ...job,
        id: Date.now().toString(),
        status: 'idle',
        created: new Date().toISOString(),
        modified: new Date().toISOString()
      })
    );
    
    (BackupService.createRestoreJob as jest.Mock).mockImplementation((job) => 
      Promise.resolve({
        ...job,
        id: Date.now().toString(),
        status: 'idle',
        created: new Date().toISOString(),
        modified: new Date().toISOString()
      })
    );
    
    // Run job implementation - simulates creating a snapshot
    (BackupService.runJob as jest.Mock).mockImplementation((id) => {
      const job = mockJobs.find(j => j.id === id);
      if (!job) return Promise.reject(new Error('Job not found'));
      
      // Create a new snapshot
      const newSnapshot = {
        id: Date.now().toString(),
        name: `${job.name}-${new Date().toISOString().replace(/[-:]/g, '').slice(0, 15)}`,
        jobId: job.id,
        targetId: job.targetId,
        source: job.source,
        size: Math.floor(Math.random() * 1024 * 1024 * 500), // Random size up to 500MB
        created: new Date().toISOString(),
        snapshotType: 'manual'
      };
      
      // Add to mock snapshots
      mockSnapshots.push(newSnapshot);
      
      return Promise.resolve();
    });
    
    // Run restore job implementation
    (BackupService.runRestoreJob as jest.Mock).mockImplementation((id) => {
      const job = mockRestoreJobs.find(j => j.id === id);
      if (!job) return Promise.reject(new Error('Restore job not found'));
      
      // Update job in mock data
      job.status = 'running';
      job.startTime = new Date().toISOString();
      
      return Promise.resolve();
    });
  });
  
  afterEach(() => {
    jest.clearAllTimers();
  });
  
  test('End-to-end backup and restore workflow', async () => {
    render(<BackupManagement />);
    
    // STEP 1: First check that the component loads
    expect(screen.getByText('Backup Management')).toBeInTheDocument();
    
    // Initial data should be fetched
    expect(BackupService.getTargets).toHaveBeenCalled();
    expect(BackupService.getJobs).toHaveBeenCalled();
    
    // STEP 2: Create a new backup target
    const tabButtons = screen.getByTestId('tab-buttons');
    fireEvent.click(tabButtons.querySelectorAll('button')[1]); // Click on Targets tab
    
    // Check that we're on the targets tab
    await waitFor(() => {
      expect(tabButtons.querySelectorAll('button')[1].getAttribute('data-active')).toBe('true');
    });
    
    // Now we should have a New Target button, click it
    const newTargetButton = screen.getByText(/New Target/i);
    fireEvent.click(newTargetButton);
    
    // Wait for the modal to appear
    await waitFor(() => {
      expect(screen.getByTestId('modal-title')).toHaveTextContent(/Add Backup Target/i);
    });
    
    // Submit the form
    const okButton = screen.getByTestId('modal-ok');
    fireEvent.click(okButton);
    
    // Check that createTarget was called
    expect(BackupService.createTarget).toHaveBeenCalled();
    
    // STEP 3: Create a new backup job
    fireEvent.click(tabButtons.querySelectorAll('button')[0]); // Click on Jobs tab
    
    // Check that we're on the jobs tab
    await waitFor(() => {
      expect(tabButtons.querySelectorAll('button')[0].getAttribute('data-active')).toBe('true');
    });
    
    // Now we should have a New Job button, click it
    const newJobButton = screen.getByText(/New Job/i);
    fireEvent.click(newJobButton);
    
    // Wait for the modal to appear
    await waitFor(() => {
      expect(screen.getByTestId('modal-title')).toHaveTextContent(/Add Backup Job/i);
    });
    
    // Submit the form
    const jobOkButton = screen.getByTestId('modal-ok');
    fireEvent.click(jobOkButton);
    
    // Check that createJob was called
    expect(BackupService.createJob).toHaveBeenCalled();
    
    // STEP 4: Run a backup job
    // Find and click Run button for a job
    const runButtons = screen.getAllByText('Run');
    fireEvent.click(runButtons[0]);
    
    // Check that runJob was called
    expect(BackupService.runJob).toHaveBeenCalled();
    
    // STEP 5: Go to Restore tab and verify snapshots
    fireEvent.click(tabButtons.querySelectorAll('button')[2]); // Click on Restore tab
    
    // Check that we're on the restore tab
    await waitFor(() => {
      expect(tabButtons.querySelectorAll('button')[2].getAttribute('data-active')).toBe('true');
    });
    
    // Check that snapshots were fetched
    expect(BackupService.getSnapshots).toHaveBeenCalled();
    
    // STEP 6: Select a snapshot and create a restore job
    // Find and click Select button for a snapshot
    const selectButtons = screen.getAllByText('Select');
    if (selectButtons.length > 0) {
      fireEvent.click(selectButtons[0]);
      
      // Now we should have moved to step 2 (Configure Restore)
      await waitFor(() => {
        expect(screen.getByText(/Configure the restore operation/i)).toBeInTheDocument();
      });
      
      // Click Configure Restore button
      const configureButton = screen.getByText('Configure Restore');
      fireEvent.click(configureButton);
      
      // Wait for the modal to appear
      await waitFor(() => {
        expect(screen.getByTestId('modal-title')).toHaveTextContent(/Configure Restore/i);
      });
      
      // Submit the form
      const restoreOkButton = screen.getByTestId('modal-ok');
      fireEvent.click(restoreOkButton);
      
      // Check that createRestoreJob was called
      expect(BackupService.createRestoreJob).toHaveBeenCalled();
      
      // Now we should have moved to step 3 (Start Restore)
      await waitFor(() => {
        expect(screen.getByText(/Review the restore job/i)).toBeInTheDocument();
      });
      
      // STEP 7: Run the restore job
      const startRestoreButton = screen.getByText('Start Restore');
      fireEvent.click(startRestoreButton);
      
      // Check that runRestoreJob was called
      expect(BackupService.runRestoreJob).toHaveBeenCalled();
      
      // Wait for progress bar to appear
      await waitFor(() => {
        expect(screen.getByText(/Progress:/i)).toBeInTheDocument();
      });
      
      // Fast-forward timers to complete the restore
      act(() => {
        jest.advanceTimersByTime(12000);
      });
      
      // STEP 8: Go to restore history and verify
      const nextButton = screen.getByText('Next');
      fireEvent.click(nextButton);
      
      // We should be at the Restore History step
      await waitFor(() => {
        expect(screen.getByText(/View and manage restore job history/i)).toBeInTheDocument();
      });
      
      // Check that getRestoreJobs was called
      expect(BackupService.getRestoreJobs).toHaveBeenCalled();
    }
  });

  test('Error handling in backup workflow', async () => {
    // Mock the getJobs function to return an error
    (BackupService.getJobs as jest.Mock).mockRejectedValue(new Error('Failed to fetch jobs'));
    
    render(<BackupManagement />);
    
    // Check for error message
    await waitFor(() => {
      expect(screen.getByText(/Failed to fetch jobs/i)).toBeInTheDocument();
    });
    
    // Try creating a job when getTargets fails
    (BackupService.getTargets as jest.Mock).mockRejectedValue(new Error('Failed to fetch targets'));
    
    // Go to Jobs tab
    const tabButtons = screen.getByTestId('tab-buttons');
    fireEvent.click(tabButtons.querySelectorAll('button')[0]); // Click on Jobs tab
    
    // Try to create a new job
    const newJobButton = screen.getByText(/New Job/i);
    fireEvent.click(newJobButton);
    
    // Wait for the modal to appear
    await waitFor(() => {
      expect(screen.getByTestId('modal-title')).toHaveTextContent(/Add Backup Job/i);
    });
    
    // Submit the form - this should fail because targets couldn't be fetched
    const jobOkButton = screen.getByTestId('modal-ok');
    fireEvent.click(jobOkButton);
    
    // Check for error message
    await waitFor(() => {
      expect(screen.getByText(/Failed to fetch targets/i)).toBeInTheDocument();
    });
  });
  
  test('Error handling in restore workflow', async () => {
    // Mock getSnapshots to fail
    (BackupService.getSnapshots as jest.Mock).mockRejectedValue(new Error('Failed to fetch snapshots'));
    
    render(<BackupManagement />);
    
    // Go to Restore tab
    const tabButtons = screen.getByTestId('tab-buttons');
    fireEvent.click(tabButtons.querySelectorAll('button')[2]); // Click on Restore tab
    
    // Check for error message
    await waitFor(() => {
      expect(screen.getByText(/Failed to fetch snapshots/i)).toBeInTheDocument();
    });
    
    // Now make getSnapshots succeed but runRestoreJob fail
    (BackupService.getSnapshots as jest.Mock).mockResolvedValue(mockSnapshots);
    (BackupService.runRestoreJob as jest.Mock).mockRejectedValue(new Error('Failed to run restore job'));
    
    // Reload the component to clear error
    await act(async () => {
      render(<BackupManagement />);
    });
    
    // Go to Restore tab
    fireEvent.click(tabButtons.querySelectorAll('button')[2]); // Click on Restore tab
    
    // Select a snapshot
    await waitFor(() => {
      const selectButtons = screen.getAllByText('Select');
      if (selectButtons.length > 0) {
        fireEvent.click(selectButtons[0]);
      }
    });
    
    // Configure restore
    await waitFor(() => {
      const configureButton = screen.getByText('Configure Restore');
      fireEvent.click(configureButton);
    });
    
    // Submit form
    await waitFor(() => {
      const restoreOkButton = screen.getByTestId('modal-ok');
      fireEvent.click(restoreOkButton);
    });
    
    // Try to run restore job
    await waitFor(() => {
      const startRestoreButton = screen.getByText('Start Restore');
      fireEvent.click(startRestoreButton);
    });
    
    // Check for error message
    await waitFor(() => {
      expect(screen.getByText(/Failed to run restore job/i)).toBeInTheDocument();
    });
  });
}); 