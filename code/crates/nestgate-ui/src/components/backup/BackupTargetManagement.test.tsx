import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import BackupTargetManagement from './BackupTargetManagement';
import BackupService from '../../services/BackupService';
import { BackupTarget, BackupTargetType } from '../../types/backup';

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
  getInstance: jest.fn(() => ({
    getTargets: jest.fn(),
    createTarget: jest.fn(),
    updateTarget: jest.fn(),
    deleteTarget: jest.fn(),
  })),
}));

describe('BackupTargetManagement Component', () => {
  // Mock targets data
  const mockTargets: BackupTarget[] = [
    {
      id: '1',
      name: 'Local Backup',
      targetType: {
        type: 'Local',
        path: '/mnt/backup',
      },
      description: 'Local backup storage',
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
    },
    {
      id: '2',
      name: 'SSH Backup',
      targetType: {
        type: 'RemoteSsh',
        host: 'server.example.com',
        port: 22,
        user: 'backup',
        path: '/backup',
        keyFile: '/home/user/.ssh/id_rsa',
      },
      description: 'Remote backup server',
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
    },
    {
      id: '3',
      name: 'NFS Backup',
      targetType: {
        type: 'Nfs',
        server: 'nas.example.com',
        export: '/exports/backup',
        mountPoint: '/mnt/nfs/backup',
      },
      description: 'Network storage',
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
    },
  ];

  // Mock service instances
  const mockBackupService = {
    getTargets: jest.fn().mockResolvedValue(mockTargets),
    createTarget: jest.fn().mockImplementation((target) => 
      Promise.resolve({
        ...target,
        id: '4',
        created: new Date().toISOString(),
        modified: new Date().toISOString(),
      })
    ),
    updateTarget: jest.fn().mockImplementation((target) => 
      Promise.resolve({
        ...target,
        modified: new Date().toISOString(),
      })
    ),
    deleteTarget: jest.fn().mockResolvedValue(undefined),
  };

  beforeEach(() => {
    jest.clearAllMocks();
    (BackupService.getInstance as jest.Mock).mockReturnValue(mockBackupService);
  });

  test('renders the component and loads targets', async () => {
    render(<BackupTargetManagement />);
    
    // Check if the component title is rendered
    expect(screen.getByText('Backup Targets')).toBeInTheDocument();
    
    // Check if the service was called to fetch targets
    expect(mockBackupService.getTargets).toHaveBeenCalled();
    
    // Wait for target data to be loaded and displayed
    await waitFor(() => {
      expect(screen.getByText('Local Backup')).toBeInTheDocument();
      expect(screen.getByText('SSH Backup')).toBeInTheDocument();
      expect(screen.getByText('NFS Backup')).toBeInTheDocument();
    });
    
    // Check if target details are displayed correctly
    expect(screen.getByText('Local')).toBeInTheDocument();
    expect(screen.getByText('RemoteSsh')).toBeInTheDocument();
    expect(screen.getByText('Nfs')).toBeInTheDocument();
    
    // Check for action buttons
    const editButtons = screen.getAllByText('Edit');
    const deleteButtons = screen.getAllByText('Delete');
    expect(editButtons.length).toBe(3);
    expect(deleteButtons.length).toBe(3);
  });

  test('handles creating a new target', async () => {
    render(<BackupTargetManagement />);
    
    // Wait for component to load
    await waitFor(() => {
      expect(screen.getByText('Add Target')).toBeInTheDocument();
    });
    
    // Click the Add Target button
    fireEvent.click(screen.getByText('Add Target'));
    
    // Wait for modal to appear
    await waitFor(() => {
      expect(screen.getByText('Target Type')).toBeInTheDocument();
    });
    
    // Fill out the form
    // Name field
    const nameInput = screen.getByLabelText('Name');
    fireEvent.change(nameInput, { target: { value: 'New Local Backup' } });
    
    // Description field
    const descriptionInput = screen.getByLabelText('Description');
    fireEvent.change(descriptionInput, { target: { value: 'Test backup target' } });
    
    // Type field (Local is default)
    // Path field
    const pathInput = screen.getByLabelText('Path');
    fireEvent.change(pathInput, { target: { value: '/mnt/new-backup' } });
    
    // Submit the form
    const submitButton = screen.getByRole('button', { name: 'OK' });
    fireEvent.click(submitButton);
    
    // Check if service was called with correct data
    await waitFor(() => {
      expect(mockBackupService.createTarget).toHaveBeenCalledWith(
        expect.objectContaining({
          name: 'New Local Backup',
          description: 'Test backup target',
          targetType: {
            type: 'Local',
            path: '/mnt/new-backup',
          },
        })
      );
    });
  });

  test('handles editing an existing target', async () => {
    render(<BackupTargetManagement />);
    
    // Wait for component to load
    await waitFor(() => {
      expect(screen.getAllByText('Edit')[0]).toBeInTheDocument();
    });
    
    // Click the Edit button for the first target
    const editButtons = screen.getAllByText('Edit');
    fireEvent.click(editButtons[0]);
    
    // Wait for modal to appear with prefilled data
    await waitFor(() => {
      const nameInput = screen.getByLabelText('Name') as HTMLInputElement;
      expect(nameInput.value).toBe('Local Backup');
    });
    
    // Modify the name
    const nameInput = screen.getByLabelText('Name');
    fireEvent.change(nameInput, { target: { value: 'Updated Local Backup' } });
    
    // Submit the form
    const submitButton = screen.getByRole('button', { name: 'OK' });
    fireEvent.click(submitButton);
    
    // Check if service was called with correct data
    await waitFor(() => {
      expect(mockBackupService.updateTarget).toHaveBeenCalledWith(
        expect.objectContaining({
          id: '1',
          name: 'Updated Local Backup',
          targetType: expect.objectContaining({
            type: 'Local',
          }),
        })
      );
    });
  });

  test('handles deleting a target', async () => {
    render(<BackupTargetManagement />);
    
    // Wait for component to load
    await waitFor(() => {
      expect(screen.getAllByText('Delete')[0]).toBeInTheDocument();
    });
    
    // Click the Delete button for the first target
    const deleteButtons = screen.getAllByText('Delete');
    fireEvent.click(deleteButtons[0]);
    
    // Wait for confirmation dialog
    await waitFor(() => {
      expect(screen.getByText('Are you sure you want to delete this target?')).toBeInTheDocument();
    });
    
    // Confirm deletion
    const confirmButton = screen.getByText('Yes');
    fireEvent.click(confirmButton);
    
    // Check if service was called with correct ID
    await waitFor(() => {
      expect(mockBackupService.deleteTarget).toHaveBeenCalledWith('1');
    });
  });

  test('handles form validation', async () => {
    render(<BackupTargetManagement />);
    
    // Wait for component to load
    await waitFor(() => {
      expect(screen.getByText('Add Target')).toBeInTheDocument();
    });
    
    // Click the Add Target button
    fireEvent.click(screen.getByText('Add Target'));
    
    // Wait for modal to appear
    await waitFor(() => {
      expect(screen.getByText('Target Type')).toBeInTheDocument();
    });
    
    // Submit form without required fields
    const submitButton = screen.getByRole('button', { name: 'OK' });
    fireEvent.click(submitButton);
    
    // Check for validation messages
    await waitFor(() => {
      expect(screen.getByText('Please enter a target name')).toBeInTheDocument();
      expect(screen.getByText('Please enter a local path')).toBeInTheDocument();
    });
    
    // Service should not have been called
    expect(mockBackupService.createTarget).not.toHaveBeenCalled();
  });

  test('handles different target types in form', async () => {
    render(<BackupTargetManagement />);
    
    // Click the Add Target button
    await waitFor(() => {
      fireEvent.click(screen.getByText('Add Target'));
    });
    
    // Test Remote SSH type
    await waitFor(() => {
      const typeSelect = screen.getByLabelText('Target Type');
      fireEvent.change(typeSelect, { target: { value: 'RemoteSsh' } });
      // Simulate the Select onChange
      fireEvent.click(screen.getByText('Remote SSH'));
    });
    
    // Check if SSH-specific fields are displayed
    await waitFor(() => {
      expect(screen.getByLabelText('Host')).toBeInTheDocument();
      expect(screen.getByLabelText('Port')).toBeInTheDocument();
      expect(screen.getByLabelText('Username')).toBeInTheDocument();
      expect(screen.getByLabelText('Remote Path')).toBeInTheDocument();
      expect(screen.getByLabelText('SSH Key File (optional)')).toBeInTheDocument();
    });
    
    // Test NFS type
    await waitFor(() => {
      const typeSelect = screen.getByLabelText('Target Type');
      fireEvent.change(typeSelect, { target: { value: 'Nfs' } });
      // Simulate the Select onChange
      fireEvent.click(screen.getByText('NFS Share'));
    });
    
    // Check if NFS-specific fields are displayed
    await waitFor(() => {
      expect(screen.getByLabelText('Server')).toBeInTheDocument();
      expect(screen.getByLabelText('Export')).toBeInTheDocument();
      expect(screen.getByLabelText('Mount Point')).toBeInTheDocument();
    });
  });
}); 