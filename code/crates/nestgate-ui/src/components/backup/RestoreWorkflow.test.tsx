import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import RestoreWorkflow from './RestoreWorkflow';
import BackupService from '../../services/BackupService';

// Mock the BackupService
jest.mock('../../services/BackupService', () => ({
  getSnapshots: jest.fn(),
  getRestoreJobs: jest.fn(),
  createRestoreJob: jest.fn(),
  runRestoreJob: jest.fn(),
  cancelRestoreJob: jest.fn(),
  deleteRestoreJob: jest.fn(),
  __esModule: true,
  default: {
    getSnapshots: jest.fn(),
    getRestoreJobs: jest.fn(),
    createRestoreJob: jest.fn(),
    runRestoreJob: jest.fn(),
    cancelRestoreJob: jest.fn(),
    deleteRestoreJob: jest.fn(),
  }
}));

// Mock Ant Design components to avoid test complexity
jest.mock('antd', () => {
  const antd = jest.requireActual('antd');
  
  const mockModal = ({ title, open, onOk, onCancel, children }: any) => {
    if (!open) return null;
    return (
      <div data-testid="modal" onClick={onOk}>
        <div data-testid="modal-title">{title}</div>
        <div data-testid="modal-content">{children}</div>
        <button data-testid="modal-ok" onClick={onOk}>
          OK
        </button>
        <button data-testid="modal-cancel" onClick={onCancel}>
          Cancel
        </button>
      </div>
    );
  };
  
  return {
    ...antd,
    Modal: mockModal,
  };
});

describe('RestoreWorkflow Component', () => {
  const mockSnapshots = [
    {
      id: '1',
      name: 'Snapshot 1',
      jobId: 'job1',
      targetId: 'target1',
      source: '/data',
      size: 1024 * 1024 * 100, // 100MB
      created: new Date().toISOString(),
      snapshotType: 'manual'
    },
    {
      id: '2',
      name: 'Snapshot 2',
      jobId: 'job2',
      targetId: 'target2',
      source: '/var',
      size: 1024 * 1024 * 200, // 200MB
      created: new Date().toISOString(),
      snapshotType: 'daily'
    }
  ];
  
  const mockRestoreJobs = [
    {
      id: '1',
      snapshotId: '1',
      destination: '/restore/data',
      overwrite: true,
      status: 'idle',
      description: 'Test restore job',
      created: new Date().toISOString(),
      modified: new Date().toISOString()
    },
    {
      id: '2',
      snapshotId: '2',
      destination: '/restore/var',
      overwrite: false,
      status: 'completed',
      description: 'Another test restore job',
      startTime: new Date(Date.now() - 3600000).toISOString(),
      endTime: new Date().toISOString(),
      created: new Date(Date.now() - 3600000).toISOString(),
      modified: new Date().toISOString()
    }
  ];
  
  beforeEach(() => {
    jest.clearAllMocks();
    
    // Setup mocks
    (BackupService.getSnapshots as jest.Mock).mockResolvedValue(mockSnapshots);
    (BackupService.getRestoreJobs as jest.Mock).mockResolvedValue(mockRestoreJobs);
    (BackupService.createRestoreJob as jest.Mock).mockImplementation((job) => 
      Promise.resolve({
        ...job,
        id: '3',
        status: 'idle',
        created: new Date().toISOString(),
        modified: new Date().toISOString()
      })
    );
    (BackupService.runRestoreJob as jest.Mock).mockResolvedValue(undefined);
    (BackupService.cancelRestoreJob as jest.Mock).mockResolvedValue(undefined);
    (BackupService.deleteRestoreJob as jest.Mock).mockResolvedValue(undefined);
  });
  
  test('renders the component without crashing', () => {
    render(<RestoreWorkflow />);
    expect(screen.getByText('Backup Restoration')).toBeInTheDocument();
  });
  
  test('fetches and displays snapshots on mount', async () => {
    render(<RestoreWorkflow />);
    
    // Check if the service was called
    expect(BackupService.getSnapshots).toHaveBeenCalled();
    expect(BackupService.getRestoreJobs).toHaveBeenCalled();
    
    // Wait for snapshots to be displayed
    await waitFor(() => {
      expect(screen.getByText('Snapshot 1')).toBeInTheDocument();
      expect(screen.getByText('Snapshot 2')).toBeInTheDocument();
    });
  });
  
  test('navigates through the restore workflow steps', async () => {
    render(<RestoreWorkflow />);
    
    // Wait for snapshots to load
    await waitFor(() => {
      expect(screen.getByText('Snapshot 1')).toBeInTheDocument();
    });
    
    // Step 1: Select snapshot
    const selectButtons = screen.getAllByText('Select');
    fireEvent.click(selectButtons[0]);
    
    // Step 2: Configure restore
    await waitFor(() => {
      expect(screen.getByText('Configure the restore operation.')).toBeInTheDocument();
    });
    
    // Open configure modal
    const configureButton = screen.getByText('Configure Restore');
    fireEvent.click(configureButton);
    
    // Wait for modal
    await waitFor(() => {
      expect(screen.getByTestId('modal-title')).toHaveTextContent('Configure Restore');
    });
    
    // Submit the form
    const okButton = screen.getByTestId('modal-ok');
    fireEvent.click(okButton);
    
    // Step 3: Start restore
    await waitFor(() => {
      expect(screen.getByText('Review the restore job and start the restore process.')).toBeInTheDocument();
    });
    
    // Step 4: Navigate to history
    const nextButton = screen.getByText('Next');
    fireEvent.click(nextButton);
    
    await waitFor(() => {
      expect(screen.getByText('View and manage restore job history.')).toBeInTheDocument();
    });
  });
  
  test('handles restore job operations', async () => {
    render(<RestoreWorkflow />);
    
    // Wait for data to load
    await waitFor(() => {
      expect(screen.getByText('Restore History')).toBeInTheDocument();
    });
    
    // Go directly to restore history tab
    const steps = screen.getAllByRole('button');
    const historyStepButton = Array.from(steps).find(step => 
      step.textContent?.includes('Restore History')
    );
    
    if (historyStepButton) {
      fireEvent.click(historyStepButton);
    }
    
    await waitFor(() => {
      expect(screen.getByText('View and manage restore job history.')).toBeInTheDocument();
    });
    
    // Find and click Run button for idle job
    const runButtons = screen.getAllByText('Run');
    fireEvent.click(runButtons[0]);
    
    expect(BackupService.runRestoreJob).toHaveBeenCalledWith('1');
    
    // Find and click Delete button
    const deleteButtons = screen.getAllByText('Delete');
    fireEvent.click(deleteButtons[0]);
    
    expect(BackupService.deleteRestoreJob).toHaveBeenCalled();
  });
  
  test('handles error scenarios', async () => {
    // Setup error scenario
    (BackupService.getSnapshots as jest.Mock).mockRejectedValue(new Error('Failed to fetch snapshots'));
    
    render(<RestoreWorkflow />);
    
    // Check for error message
    await waitFor(() => {
      expect(screen.getByText('Failed to fetch snapshots. Please try again later.')).toBeInTheDocument();
    });
  });
}); 