import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import BackupJobManagement from './BackupJobManagement';
import BackupService from '../../services/BackupService';

// Mock message from antd
jest.mock('antd', () => {
  const antd = jest.requireActual('antd');
  return {
    ...antd,
    message: {
      success: jest.fn(),
      error: jest.fn(),
    },
  };
});

// Mock the BackupService
jest.mock('../../services/BackupService', () => ({
  getTargets: jest.fn(),
  getJobs: jest.fn(),
  createJob: jest.fn(),
  updateJob: jest.fn(),
  deleteJob: jest.fn(),
  runJob: jest.fn(),
  __esModule: true,
  default: {
    getTargets: jest.fn(),
    getJobs: jest.fn(),
    createJob: jest.fn(),
    updateJob: jest.fn(),
    deleteJob: jest.fn(),
    runJob: jest.fn(),
  }
}));

describe('BackupJobManagement Component', () => {
  // Mock data
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
      lastRun: null,
      nextRun: null,
      created: new Date().toISOString(),
      modified: new Date().toISOString()
    },
    {
      id: '2',
      name: 'Weekly Backup',
      source: '/home',
      targetId: '2',
      schedule: '0 0 * * 0',
      retention: {
        weekly: 4,
        monthly: 6
      },
      status: 'completed',
      description: 'Weekly backup of home directory',
      lastRun: new Date(Date.now() - (24 * 60 * 60 * 1000)).toISOString(), // 1 day ago
      nextRun: new Date(Date.now() + (6 * 24 * 60 * 60 * 1000)).toISOString(), // 6 days from now
      created: new Date().toISOString(),
      modified: new Date().toISOString()
    }
  ];

  beforeEach(() => {
    jest.clearAllMocks();
    
    // Setup default mocks
    (BackupService.getTargets as jest.Mock).mockResolvedValue(mockTargets);
    (BackupService.getJobs as jest.Mock).mockResolvedValue(mockJobs);
    (BackupService.createJob as jest.Mock).mockImplementation((job) => 
      Promise.resolve({
        ...job,
        id: '3',
        status: 'idle',
        created: new Date().toISOString(),
        modified: new Date().toISOString()
      })
    );
    (BackupService.updateJob as jest.Mock).mockImplementation((job) => 
      Promise.resolve({
        ...job,
        modified: new Date().toISOString()
      })
    );
    (BackupService.deleteJob as jest.Mock).mockResolvedValue(undefined);
    (BackupService.runJob as jest.Mock).mockResolvedValue(undefined);
  });

  test('renders the component and loads backup jobs', async () => {
    render(<BackupJobManagement />);
    
    // Wait for services to be called and data to load
    await waitFor(() => {
      expect(BackupService.getJobs).toHaveBeenCalled();
      expect(BackupService.getTargets).toHaveBeenCalled();
    });
    
    // Check if job data is displayed
    await waitFor(() => {
      expect(screen.getByText('Daily Backup')).toBeInTheDocument();
      expect(screen.getByText('Weekly Backup')).toBeInTheDocument();
      expect(screen.getByText('/data')).toBeInTheDocument();
      expect(screen.getByText('/home')).toBeInTheDocument();
    });
    
    // Check if the "New Job" button is rendered
    expect(screen.getByText('New Job')).toBeInTheDocument();
  });

  test('handles creating a new backup job', async () => {
    render(<BackupJobManagement />);
    
    // Wait for component to load
    await waitFor(() => {
      expect(screen.getByText('New Job')).toBeInTheDocument();
    });
    
    // Click the "New Job" button
    fireEvent.click(screen.getByText('New Job'));
    
    // Wait for modal to appear
    await waitFor(() => {
      expect(screen.getByText('Add Backup Job')).toBeInTheDocument();
    });
    
    // Fill out the form
    // Name field
    const nameInput = screen.getByLabelText('Job Name');
    fireEvent.change(nameInput, { target: { value: 'Test Backup Job' } });
    
    // Description field
    const descriptionInput = screen.getByLabelText('Description');
    fireEvent.change(descriptionInput, { target: { value: 'Test job description' } });
    
    // Source field
    const sourceInput = screen.getByLabelText('Source Dataset/Path');
    fireEvent.change(sourceInput, { target: { value: '/var/test' } });
    
    // Target field
    const targetSelect = screen.getByLabelText('Backup Target');
    fireEvent.change(targetSelect, { target: { value: '1' } });
    
    // Schedule field
    const scheduleInput = screen.getByLabelText('Schedule (Cron Expression)');
    fireEvent.change(scheduleInput, { target: { value: '0 0 * * 1-5' } });
    
    // Submit the form
    const submitButton = screen.getByRole('button', { name: 'OK' });
    fireEvent.click(submitButton);
    
    // Verify the service was called with the correct data
    await waitFor(() => {
      expect(BackupService.createJob).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'Test Backup Job',
          description: 'Test job description',
          source: '/var/test',
          targetId: '1',
          schedule: '0 0 * * 1-5'
        })
      );
    });
  });

  test('handles editing an existing backup job', async () => {
    render(<BackupJobManagement />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Daily Backup')).toBeInTheDocument();
    });
    
    // Find and click the Edit button for the first job
    const editButtons = screen.getAllByText('Edit');
    fireEvent.click(editButtons[0]);
    
    // Wait for modal to appear with existing data
    await waitFor(() => {
      expect(screen.getByText('Edit Backup Job')).toBeInTheDocument();
      const nameInput = screen.getByLabelText('Job Name') as HTMLInputElement;
      expect(nameInput.value).toBe('Daily Backup');
    });
    
    // Modify the job name
    const nameInput = screen.getByLabelText('Job Name');
    fireEvent.change(nameInput, { target: { value: 'Updated Daily Backup' } });
    
    // Submit the form
    const submitButton = screen.getByRole('button', { name: 'OK' });
    fireEvent.click(submitButton);
    
    // Verify the service was called with the correct data
    await waitFor(() => {
      expect(BackupService.updateJob).toHaveBeenCalledWith(
        expect.objectContaining({
          id: '1',
          name: 'Updated Daily Backup'
        })
      );
    });
  });

  test('handles running a backup job', async () => {
    render(<BackupJobManagement />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Daily Backup')).toBeInTheDocument();
    });
    
    // Find and click the Run button for the first job
    const runButtons = screen.getAllByText('Run');
    fireEvent.click(runButtons[0]);
    
    // Verify the service was called
    await waitFor(() => {
      expect(BackupService.runJob).toHaveBeenCalledWith('1');
    });
  });

  test('handles deleting a backup job', async () => {
    render(<BackupJobManagement />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Daily Backup')).toBeInTheDocument();
    });
    
    // Find and click the Delete button for the first job
    const deleteButtons = screen.getAllByText('Delete');
    fireEvent.click(deleteButtons[0]);
    
    // Wait for confirmation dialog
    await waitFor(() => {
      expect(screen.getByText('Are you sure you want to delete this backup job?')).toBeInTheDocument();
    });
    
    // Confirm deletion
    const confirmButton = screen.getByText('Yes');
    fireEvent.click(confirmButton);
    
    // Verify the service was called
    await waitFor(() => {
      expect(BackupService.deleteJob).toHaveBeenCalledWith('1');
    });
  });

  test('handles error scenarios', async () => {
    // Mock an error for getJobs
    (BackupService.getJobs as jest.Mock).mockRejectedValue(new Error('Failed to fetch jobs'));
    
    render(<BackupJobManagement />);
    
    // Verify error handling
    await waitFor(() => {
      expect(screen.getByText('Failed to fetch backup jobs')).toBeInTheDocument();
    });
    
    // Reset mock and rerender
    (BackupService.getJobs as jest.Mock).mockResolvedValue(mockJobs);
    (BackupService.createJob as jest.Mock).mockRejectedValue(new Error('Failed to create job'));
    
    // Unmount and remount to reload component
    await act(async () => {
      render(<BackupJobManagement />);
    });
    
    // Try to create a new job
    await waitFor(() => {
      expect(screen.getByText('New Job')).toBeInTheDocument();
    });
    
    fireEvent.click(screen.getByText('New Job'));
    
    // Fill out form minimally and submit
    await waitFor(() => {
      const nameInput = screen.getByLabelText('Job Name');
      fireEvent.change(nameInput, { target: { value: 'Test Job' } });
      
      const sourceInput = screen.getByLabelText('Source Dataset/Path');
      fireEvent.change(sourceInput, { target: { value: '/test' } });
      
      const targetSelect = screen.getByLabelText('Backup Target');
      fireEvent.change(targetSelect, { target: { value: '1' } });
      
      const scheduleInput = screen.getByLabelText('Schedule (Cron Expression)');
      fireEvent.change(scheduleInput, { target: { value: '0 0 * * *' } });
    });
    
    // Submit the form
    const submitButton = screen.getByRole('button', { name: 'OK' });
    fireEvent.click(submitButton);
    
    // Verify error handling
    await waitFor(() => {
      expect(BackupService.createJob).toHaveBeenCalled();
    });
  });
}); 