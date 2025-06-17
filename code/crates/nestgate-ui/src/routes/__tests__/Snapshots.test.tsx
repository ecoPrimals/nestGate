import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { MemoryRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import SnapshotsPage from '../Snapshots';
import { SnapshotService } from '../../services/snapshot.service';
import { ZfsPoolService } from '../../services/zfs-pool.service';

// Mock the services
jest.mock('../../services/snapshot.service');
jest.mock('../../services/zfs-pool.service');

// Setup mock data
const mockSnapshots = [
  {
    id: 'tank/data@snapshot-1',
    name: 'snapshot-1',
    dataset: 'tank/data',
    created: '2023-12-01T10:00:00Z',
    size: '1.5 GB',
    isReplicated: true
  },
  {
    id: 'tank/apps@snapshot-2',
    name: 'snapshot-2',
    dataset: 'tank/apps',
    created: '2023-12-02T14:30:00Z',
    size: '500 MB',
    isReplicated: false
  }
];

const mockPools = [
  {
    id: 'tank',
    name: 'tank',
    status: 'ONLINE',
    size: 10000000000,
    used: 3000000000,
    datasets: [
      { id: 'data', name: 'data' },
      { id: 'apps', name: 'apps' }
    ]
  }
];

// Setup mock service implementations
const MockSnapshotService = {
  getInstance: jest.fn().mockReturnValue({
    getSnapshots: jest.fn().mockResolvedValue(mockSnapshots),
    createSnapshot: jest.fn().mockImplementation((dataset, name) => 
      Promise.resolve({
        id: `${dataset}@${name}`,
        name,
        dataset,
        created: new Date().toISOString(),
        size: '100 MB',
        isReplicated: false
      })
    ),
    deleteSnapshot: jest.fn().mockResolvedValue(undefined),
    rollbackSnapshot: jest.fn().mockResolvedValue(undefined)
  })
};

const MockZfsPoolService = {
  getInstance: jest.fn().mockReturnValue({
    getZfsPools: jest.fn().mockResolvedValue(mockPools)
  })
};

// Mock the implementations
(SnapshotService.getInstance as jest.Mock).mockImplementation(MockSnapshotService.getInstance);
(ZfsPoolService.getInstance as jest.Mock).mockImplementation(MockZfsPoolService.getInstance);

describe('SnapshotsPage', () => {
  // Setup QueryClient with default options
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
      },
    },
  });

  // Helper function to render component with providers
  const renderComponent = () => render(
    <QueryClientProvider client={queryClient}>
      <MemoryRouter>
        <SnapshotsPage />
      </MemoryRouter>
    </QueryClientProvider>
  );

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders the snapshots page with title and actions', async () => {
    renderComponent();
    
    // Check for page title
    expect(screen.getByText('ZFS Snapshots')).toBeInTheDocument();
    
    // Check for action buttons
    expect(screen.getByText('Refresh')).toBeInTheDocument();
    expect(screen.getByText('Create Snapshot')).toBeInTheDocument();
    
    // Wait for snapshots to load
    await waitFor(() => {
      expect(SnapshotService.getInstance().getSnapshots).toHaveBeenCalled();
    });
  });

  it('displays snapshots in a table', async () => {
    renderComponent();
    
    // Wait for snapshots to load and be displayed
    await waitFor(() => {
      expect(screen.getByText('snapshot-1')).toBeInTheDocument();
      expect(screen.getByText('snapshot-2')).toBeInTheDocument();
      expect(screen.getByText('tank/data')).toBeInTheDocument();
      expect(screen.getByText('tank/apps')).toBeInTheDocument();
      expect(screen.getByText('1.5 GB')).toBeInTheDocument();
      expect(screen.getByText('500 MB')).toBeInTheDocument();
    });
    
    // Check for replication status
    expect(screen.getByText('Replicated')).toBeInTheDocument();
    expect(screen.getByText('Pending')).toBeInTheDocument();
  });

  it('opens the create snapshot modal when clicking create button', async () => {
    renderComponent();
    
    // Click create snapshot button
    fireEvent.click(screen.getByText('Create Snapshot'));
    
    // Check if modal is displayed
    await waitFor(() => {
      expect(screen.getByText('Create New Snapshot')).toBeInTheDocument();
      expect(screen.getByText('Dataset')).toBeInTheDocument();
      expect(screen.getByText('Snapshot Name')).toBeInTheDocument();
      expect(screen.getByText('Recursive (include child datasets)')).toBeInTheDocument();
    });
  });

  it('creates a new snapshot when submitting the form', async () => {
    renderComponent();
    
    // Open create modal
    fireEvent.click(screen.getByText('Create Snapshot'));
    
    // Wait for modal to open and pools to load
    await waitFor(() => {
      expect(screen.getByText('Create New Snapshot')).toBeInTheDocument();
    });
    
    // Select dataset from dropdown
    const datasetSelect = screen.getByLabelText('Dataset');
    fireEvent.mouseDown(datasetSelect);
    
    // Wait for dropdown options to appear
    await waitFor(() => {
      // Select the first dataset option from dropdown
      fireEvent.click(screen.getByText('tank/data'));
    });
    
    // Enter snapshot name
    const nameInput = screen.getByLabelText('Snapshot Name');
    fireEvent.change(nameInput, { target: { value: 'test-snapshot' } });
    
    // Submit the form
    fireEvent.click(screen.getByRole('button', { name: 'Create' }));
    
    // Check if create function was called with correct params
    await waitFor(() => {
      expect(SnapshotService.getInstance().createSnapshot).toHaveBeenCalledWith(
        'tank/data',
        'test-snapshot',
        false
      );
    });
    
    // Check for success notification (requires mocking notification system)
    // This would normally test that a success message appears
  });

  it('opens rollback modal when clicking rollback button', async () => {
    renderComponent();
    
    // Wait for snapshots to load
    await waitFor(() => {
      expect(screen.getByText('snapshot-1')).toBeInTheDocument();
    });
    
    // Find and click the rollback button (first one)
    const rollbackButtons = screen.getAllByRole('button', { 
      name: 'Rollback to this snapshot' 
    });
    fireEvent.click(rollbackButtons[0]);
    
    // Check if rollback modal is displayed
    await waitFor(() => {
      expect(screen.getByText('Rollback to Snapshot')).toBeInTheDocument();
      expect(screen.getByText(/You are about to roll back to snapshot/)).toBeInTheDocument();
      expect(screen.getByText('Force rollback (may destroy later snapshots)')).toBeInTheDocument();
    });
    
    // Click rollback
    fireEvent.click(screen.getByRole('button', { name: 'Rollback' }));
    
    // Check if rollback function was called
    await waitFor(() => {
      expect(SnapshotService.getInstance().rollbackSnapshot).toHaveBeenCalled();
    });
  });

  it('deletes a snapshot when delete button is clicked and confirmed', async () => {
    renderComponent();
    
    // Wait for snapshots to load
    await waitFor(() => {
      expect(screen.getByText('snapshot-1')).toBeInTheDocument();
    });
    
    // Find and click the delete button (first one)
    const deleteButtons = screen.getAllByRole('button', { 
      name: 'Delete snapshot' 
    });
    fireEvent.click(deleteButtons[0]);
    
    // Check if confirmation popover is displayed
    await waitFor(() => {
      expect(screen.getByText('Delete Snapshot')).toBeInTheDocument();
      expect(screen.getByText('Are you sure you want to delete this snapshot?')).toBeInTheDocument();
    });
    
    // Confirm deletion
    fireEvent.click(screen.getByText('Yes'));
    
    // Check if delete function was called with correct ID
    await waitFor(() => {
      expect(SnapshotService.getInstance().deleteSnapshot).toHaveBeenCalledWith('tank/data@snapshot-1');
    });
  });

  it('refreshes snapshot list when refresh button is clicked', async () => {
    renderComponent();
    
    // Wait for initial snapshot load
    await waitFor(() => {
      expect(SnapshotService.getInstance().getSnapshots).toHaveBeenCalledTimes(1);
    });
    
    // Click refresh button
    fireEvent.click(screen.getByText('Refresh'));
    
    // Check if getSnapshots was called again
    await waitFor(() => {
      expect(SnapshotService.getInstance().getSnapshots).toHaveBeenCalledTimes(2);
    });
  });
}); 