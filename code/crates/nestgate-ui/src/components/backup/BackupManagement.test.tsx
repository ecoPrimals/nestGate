import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import BackupManagement from './BackupManagement';
import BackupTargetManagement from './BackupTargetManagement';
import BackupJobManagement from './BackupJobManagement';
import BackupService from '../../services/BackupService';
import { BrowserRouter } from 'react-router-dom';

// Mock BackupService
jest.mock('../../services/BackupService', () => {
  return {
    getInstance: jest.fn(() => ({
      getTargets: jest.fn().mockResolvedValue([
        {
          id: '1',
          name: 'Local Backup',
          targetType: {
            type: 'Local',
            path: '/mnt/backup'
          },
          description: 'Local backup directory',
          created: new Date().toISOString(),
          modified: new Date().toISOString()
        }
      ]),
      getJobs: jest.fn().mockResolvedValue([
        {
          id: '1',
          name: 'Daily Backup',
          source: 'tank/data',
          targetId: '1',
          schedule: '0 0 * * *',
          retention: {
            hourly: 24,
            daily: 7,
            weekly: 4,
            monthly: 12,
            yearly: 5
          },
          status: 'Idle',
          description: 'Daily backup of data',
          lastRun: null,
          nextRun: null,
          created: new Date().toISOString(),
          modified: new Date().toISOString()
        }
      ]),
      createTarget: jest.fn().mockImplementation((target) => {
        return Promise.resolve({
          ...target,
          id: '2',
          created: new Date().toISOString(),
          modified: new Date().toISOString()
        });
      }),
      createJob: jest.fn().mockImplementation((job) => {
        return Promise.resolve({
          ...job,
          id: '2',
          status: 'Idle',
          lastRun: null,
          nextRun: null,
          created: new Date().toISOString(),
          modified: new Date().toISOString()
        });
      }),
      updateTarget: jest.fn().mockImplementation((target) => Promise.resolve(target)),
      updateJob: jest.fn().mockImplementation((job) => Promise.resolve(job)),
      deleteTarget: jest.fn().mockResolvedValue(undefined),
      deleteJob: jest.fn().mockResolvedValue(undefined),
      runJob: jest.fn().mockResolvedValue(undefined)
    }))
  };
});

// Mock antd components
jest.mock('antd', () => {
  const originalModule = jest.requireActual('antd');
  
  return {
    ...originalModule,
    Table: ({ dataSource, expandable }) => (
      <div data-testid="mock-table">
        <div data-testid="table-data-count">{dataSource?.length || 0}</div>
        {dataSource?.map((item: any) => (
          <div key={item.id} data-testid={`table-row-${item.id}`}>
            {item.name}
          </div>
        ))}
      </div>
    ),
    Card: ({ children }) => <div data-testid="mock-card">{children}</div>,
    Tabs: ({ children }) => <div data-testid="mock-tabs">{children}</div>,
    TabPane: ({ children }) => <div data-testid="mock-tab-pane">{children}</div>,
    Button: originalModule.Button,
    Typography: {
      ...originalModule.Typography,
      Title: ({ children }) => <h1 data-testid="mock-title">{children}</h1>
    },
    Form: {
      ...originalModule.Form,
      useForm: () => [
        {
          resetFields: jest.fn(),
          setFieldsValue: jest.fn(),
          validateFields: jest.fn().mockResolvedValue({
            name: 'Test Backup',
            description: 'Test description',
            targetType: 'Local',
            path: '/path/to/backup',
            source: 'tank/data',
            targetId: '1',
            schedule: '0 0 * * *',
            hourlyRetention: 24,
            dailyRetention: 7,
            weeklyRetention: 4,
            monthlyRetention: 12,
            yearlyRetention: 5
          })
        }
      ],
      Item: ({ children }) => <div>{children}</div>
    },
    Modal: ({ title, children, onOk, onCancel }) => (
      <div data-testid="mock-modal">
        <h2>{title}</h2>
        <div>{children}</div>
        <button onClick={onOk} data-testid="modal-ok">OK</button>
        <button onClick={onCancel} data-testid="modal-cancel">Cancel</button>
      </div>
    ),
    message: {
      success: jest.fn(),
      error: jest.fn()
    }
  };
});

describe('BackupManagement Component', () => {
  it('renders without crashing', () => {
    render(
      <BrowserRouter>
        <BackupManagement />
      </BrowserRouter>
    );
    
    expect(screen.getByText('Backup Management')).toBeInTheDocument();
  });
});

describe('BackupTargetManagement Component', () => {
  it('fetches and displays targets', async () => {
    render(
      <BrowserRouter>
        <BackupTargetManagement />
      </BrowserRouter>
    );
    
    expect(screen.getByTestId('mock-title')).toHaveTextContent('Backup Targets');
    
    await waitFor(() => {
      expect(screen.getByTestId('table-row-1')).toBeInTheDocument();
    });
    
    expect(screen.getByTestId('table-row-1')).toHaveTextContent('Local Backup');
  });
  
  it('allows adding a new target', async () => {
    render(
      <BrowserRouter>
        <BackupTargetManagement />
      </BrowserRouter>
    );
    
    const addButton = screen.getByText('Add Target');
    fireEvent.click(addButton);
    
    expect(screen.getByTestId('mock-modal')).toBeInTheDocument();
    
    const okButton = screen.getByTestId('modal-ok');
    fireEvent.click(okButton);
    
    await waitFor(() => {
      expect(BackupService.getInstance().createTarget).toHaveBeenCalled();
    });
  });
});

describe('BackupJobManagement Component', () => {
  it('fetches and displays jobs', async () => {
    render(
      <BrowserRouter>
        <BackupJobManagement />
      </BrowserRouter>
    );
    
    expect(screen.getByTestId('mock-title')).toHaveTextContent('Backup Jobs');
    
    await waitFor(() => {
      expect(screen.getByTestId('table-row-1')).toBeInTheDocument();
    });
    
    expect(screen.getByTestId('table-row-1')).toHaveTextContent('Daily Backup');
  });
  
  it('allows adding a new job', async () => {
    render(
      <BrowserRouter>
        <BackupJobManagement />
      </BrowserRouter>
    );
    
    const addButton = screen.getByText('Add Job');
    fireEvent.click(addButton);
    
    expect(screen.getByTestId('mock-modal')).toBeInTheDocument();
    
    const okButton = screen.getByTestId('modal-ok');
    fireEvent.click(okButton);
    
    await waitFor(() => {
      expect(BackupService.getInstance().createJob).toHaveBeenCalled();
    });
  });
}); 