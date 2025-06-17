import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { MemoryRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import Storage from '../Storage';
import { ZfsPoolService } from '../../services/zfs-pool.service';

// Mock the service
jest.mock('../../services/zfs-pool.service');

// Setup mock data
const mockPools = [
  {
    id: 'tank',
    name: 'tank',
    status: 'ONLINE',
    size: 10000000000,
    used: 3000000000,
    available: 7000000000,
    datasets: [
      { id: 'tank/data', name: 'data', size: 5000000000, used: 1500000000, available: 3500000000 },
      { id: 'tank/apps', name: 'apps', size: 5000000000, used: 1500000000, available: 3500000000 }
    ]
  },
  {
    id: 'backup',
    name: 'backup',
    status: 'ONLINE',
    size: 20000000000,
    used: 5000000000,
    available: 15000000000,
    datasets: []
  }
];

// Setup mock service implementation
const MockZfsPoolService = {
  getInstance: jest.fn().mockReturnValue({
    getZfsPools: jest.fn().mockResolvedValue(mockPools),
    createZfsPool: jest.fn().mockImplementation((name, devices) => 
      Promise.resolve({
        id: name,
        name,
        status: 'ONLINE',
        size: 5000000000,
        used: 0,
        available: 5000000000,
        datasets: []
      })
    ),
    createDataset: jest.fn().mockImplementation((pool, name) => 
      Promise.resolve({
        id: `${pool}/${name}`,
        name,
        size: 5000000000,
        used: 0,
        available: 5000000000
      })
    ),
    importPool: jest.fn().mockResolvedValue(undefined),
    exportPool: jest.fn().mockResolvedValue(undefined),
    scrubPool: jest.fn().mockResolvedValue(undefined)
  })
};

// Mock the implementation
(ZfsPoolService.getInstance as jest.Mock).mockImplementation(MockZfsPoolService.getInstance);

describe('Storage Page', () => {
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
        <Storage />
      </MemoryRouter>
    </QueryClientProvider>
  );

  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders the storage page with title and action buttons', async () => {
    renderComponent();
    
    // Check for page title
    expect(screen.getByText('ZFS Storage')).toBeInTheDocument();
    
    // Check for action buttons
    expect(screen.getByText('Create Pool')).toBeInTheDocument();
    expect(screen.getByText('Import Pool')).toBeInTheDocument();
    expect(screen.getByText('Export Pool')).toBeInTheDocument();
    expect(screen.getByText('Scrub Pool')).toBeInTheDocument();
    
    // Wait for pools to load
    await waitFor(() => {
      expect(ZfsPoolService.getInstance().getZfsPools).toHaveBeenCalled();
    });
  });

  it('displays pools in a table', async () => {
    renderComponent();
    
    // Wait for pools to load and be displayed
    await waitFor(() => {
      expect(screen.getByText('tank')).toBeInTheDocument();
      expect(screen.getByText('backup')).toBeInTheDocument();
    });
    
    // Check for pool status
    const onlineStatuses = screen.getAllByText('ONLINE');
    expect(onlineStatuses.length).toBeGreaterThanOrEqual(2);
    
    // Check for size information
    expect(screen.getByText('9.3 GB')).toBeInTheDocument(); // 10GB formatted
    expect(screen.getByText('18.6 GB')).toBeInTheDocument(); // 20GB formatted
  });

  it('displays datasets for each pool when expanded', async () => {
    renderComponent();
    
    // Wait for pools to load
    await waitFor(() => {
      expect(screen.getByText('tank')).toBeInTheDocument();
    });
    
    // Find and click the expand button for the first pool
    const expandButtons = screen.getAllByRole('button', { name: 'Expand row' });
    fireEvent.click(expandButtons[0]);
    
    // Check if datasets are displayed
    await waitFor(() => {
      expect(screen.getByText('data')).toBeInTheDocument();
      expect(screen.getByText('apps')).toBeInTheDocument();
    });
  });

  it('opens the create pool modal when clicking create pool button', async () => {
    renderComponent();
    
    // Click create pool button
    fireEvent.click(screen.getByText('Create Pool'));
    
    // Check if modal is displayed
    await waitFor(() => {
      expect(screen.getByText('Create ZFS Pool')).toBeInTheDocument();
      expect(screen.getByText('Pool Name')).toBeInTheDocument();
      expect(screen.getByText('Devices')).toBeInTheDocument();
    });
  });

  it('creates a new pool when submitting the form', async () => {
    renderComponent();
    
    // Open create pool modal
    fireEvent.click(screen.getByText('Create Pool'));
    
    // Wait for modal to open
    await waitFor(() => {
      expect(screen.getByText('Create ZFS Pool')).toBeInTheDocument();
    });
    
    // Enter pool name
    const nameInput = screen.getByLabelText('Pool Name');
    fireEvent.change(nameInput, { target: { value: 'newpool' } });
    
    // Enter devices
    const devicesInput = screen.getByLabelText('Devices');
    fireEvent.change(devicesInput, { target: { value: '/dev/sda,/dev/sdb' } });
    
    // Submit the form
    fireEvent.click(screen.getByRole('button', { name: 'Create' }));
    
    // Check if create function was called with correct params
    await waitFor(() => {
      expect(ZfsPoolService.getInstance().createZfsPool).toHaveBeenCalledWith(
        'newpool',
        ['/dev/sda', '/dev/sdb']
      );
    });
  });

  it('opens create dataset modal when clicking create dataset button', async () => {
    renderComponent();
    
    // Wait for pools to load
    await waitFor(() => {
      expect(screen.getByText('tank')).toBeInTheDocument();
    });
    
    // Find and click the create dataset button for the first pool
    const actionMenus = screen.getAllByRole('button', { name: 'more' });
    fireEvent.click(actionMenus[0]);
    
    // Wait for dropdown menu and click Create Dataset
    await waitFor(() => {
      const createDatasetMenuItem = screen.getByText('Create Dataset');
      fireEvent.click(createDatasetMenuItem);
    });
    
    // Check if dataset modal is displayed
    await waitFor(() => {
      expect(screen.getByText('Create ZFS Dataset')).toBeInTheDocument();
      expect(screen.getByText('Dataset Name')).toBeInTheDocument();
      expect(screen.getByText('Record Size')).toBeInTheDocument();
    });
  });

  it('creates a new dataset when submitting the form', async () => {
    renderComponent();
    
    // Wait for pools to load
    await waitFor(() => {
      expect(screen.getByText('tank')).toBeInTheDocument();
    });
    
    // Find and click the create dataset button for the first pool
    const actionMenus = screen.getAllByRole('button', { name: 'more' });
    fireEvent.click(actionMenus[0]);
    
    // Wait for dropdown menu and click Create Dataset
    await waitFor(() => {
      const createDatasetMenuItem = screen.getByText('Create Dataset');
      fireEvent.click(createDatasetMenuItem);
    });
    
    // Wait for modal to open
    await waitFor(() => {
      expect(screen.getByText('Create ZFS Dataset')).toBeInTheDocument();
    });
    
    // Enter dataset name
    const nameInput = screen.getByLabelText('Dataset Name');
    fireEvent.change(nameInput, { target: { value: 'newdata' } });
    
    // Submit the form
    fireEvent.click(screen.getByRole('button', { name: 'Create' }));
    
    // Check if create function was called with correct params
    await waitFor(() => {
      expect(ZfsPoolService.getInstance().createDataset).toHaveBeenCalledWith(
        'tank',
        'newdata',
        expect.any(Object)
      );
    });
  });

  it('calls importPool when clicking Import Pool button', async () => {
    renderComponent();
    
    // Click import pool button
    fireEvent.click(screen.getByText('Import Pool'));
    
    // Check if import function was called
    await waitFor(() => {
      expect(ZfsPoolService.getInstance().importPool).toHaveBeenCalled();
    });
  });

  it('calls exportPool when clicking Export Pool button', async () => {
    renderComponent();
    
    // Wait for pools to load
    await waitFor(() => {
      expect(screen.getByText('tank')).toBeInTheDocument();
    });
    
    // Find and click the action menu for the first pool
    const actionMenus = screen.getAllByRole('button', { name: 'more' });
    fireEvent.click(actionMenus[0]);
    
    // Wait for dropdown menu and click Export Pool
    await waitFor(() => {
      const exportMenuItem = screen.getByText('Export');
      fireEvent.click(exportMenuItem);
    });
    
    // Check if confirmation dialog is shown and confirm
    await waitFor(() => {
      expect(screen.getByText('Are you sure you want to export this pool?')).toBeInTheDocument();
    });
    
    // Confirm export
    fireEvent.click(screen.getByRole('button', { name: 'Yes' }));
    
    // Check if export function was called with correct pool name
    await waitFor(() => {
      expect(ZfsPoolService.getInstance().exportPool).toHaveBeenCalledWith('tank');
    });
  });

  it('calls scrubPool when clicking Scrub Pool button', async () => {
    renderComponent();
    
    // Wait for pools to load
    await waitFor(() => {
      expect(screen.getByText('tank')).toBeInTheDocument();
    });
    
    // Find and click the action menu for the first pool
    const actionMenus = screen.getAllByRole('button', { name: 'more' });
    fireEvent.click(actionMenus[0]);
    
    // Wait for dropdown menu and click Scrub
    await waitFor(() => {
      const scrubMenuItem = screen.getByText('Scrub');
      fireEvent.click(scrubMenuItem);
    });
    
    // Check if confirmation dialog is shown and confirm
    await waitFor(() => {
      expect(screen.getByText('Are you sure you want to scrub this pool?')).toBeInTheDocument();
    });
    
    // Confirm scrub
    fireEvent.click(screen.getByRole('button', { name: 'Yes' }));
    
    // Check if scrub function was called with correct pool name
    await waitFor(() => {
      expect(ZfsPoolService.getInstance().scrubPool).toHaveBeenCalledWith('tank');
    });
  });
}); 