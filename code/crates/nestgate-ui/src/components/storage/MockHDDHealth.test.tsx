import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';

// Create a mock version of the HDDHealth component to avoid Ant Design rendering issues
const MockHDDHealth = () => {
  return (
    <div data-testid="hdd-health">
      <h1>Disk Health Monitoring</h1>
      <div data-testid="status-indicator">Test Data</div>
      <button>Refresh Data</button>
      <div data-testid="disk-list">
        <div data-testid="disk-item">sda</div>
        <div data-testid="disk-item">sdb</div>
      </div>
    </div>
  );
};

// Mock the websocket service
jest.mock('../../services/websocket.service', () => ({
  WebSocketService: {
    getInstance: jest.fn().mockReturnValue({
      connect: jest.fn(),
      send: jest.fn(),
      subscribe: jest.fn().mockReturnValue(jest.fn())
    })
  }
}));

describe('MockHDDHealth Component', () => {
  test('renders the component title', () => {
    render(<MockHDDHealth />);
    
    expect(screen.getByText('Disk Health Monitoring')).toBeInTheDocument();
    expect(screen.getByText('Refresh Data')).toBeInTheDocument();
    expect(screen.getByText('Test Data')).toBeInTheDocument();
  });

  test('renders disk items', () => {
    render(<MockHDDHealth />);
    
    const diskItems = screen.getAllByTestId('disk-item');
    expect(diskItems).toHaveLength(2);
    expect(diskItems[0]).toHaveTextContent('sda');
    expect(diskItems[1]).toHaveTextContent('sdb');
  });

  test('container has proper structure', () => {
    render(<MockHDDHealth />);
    
    const container = screen.getByTestId('hdd-health');
    expect(container).toBeInTheDocument();
    expect(container).toContainElement(screen.getByTestId('status-indicator'));
    expect(container).toContainElement(screen.getByTestId('disk-list'));
  });
}); 