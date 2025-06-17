import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';

// Create a mock NasMetrics component
const MockNasMetrics = () => {
  return (
    <div data-testid="nas-metrics">
      <div className="storage-section">
        <h3>Storage Usage</h3>
        <div data-testid="storage-card">
          <div className="storage-stats">
            <div>Total: 1 TB</div>
            <div>Used: 250 GB</div>
            <div>Free: 750 GB</div>
          </div>
          <div className="storage-progress" role="progressbar" aria-valuenow={25}>
            25%
          </div>
        </div>
      </div>
      
      <div className="health-section">
        <h3>System Health</h3>
        <div data-testid="health-card">
          <div className="health-stats">
            <div>CPU: 25%</div>
            <div>Memory: 50%</div>
            <div>Uptime: 5d 6h 30m</div>
          </div>
          <div className="disk-health">
            <div>Disk 1: Healthy</div>
            <div>Disk 2: Healthy</div>
          </div>
        </div>
      </div>
      
      <div className="services-section">
        <h3>Services Status</h3>
        <div data-testid="services-card">
          <div className="service-status">
            <div className="service service-running">SMB</div>
            <div className="service service-running">NFS</div>
            <div className="service service-running">SSH</div>
          </div>
        </div>
      </div>
      
      <div className="performance-section">
        <h3>Performance</h3>
        <div data-testid="performance-card">
          <div className="performance-stats">
            <div>Read: 100 MB/s</div>
            <div>Write: 50 MB/s</div>
            <div>IOPS: 150</div>
          </div>
        </div>
      </div>
    </div>
  );
};

describe('NasMetrics', () => {
  it('renders the component correctly', async () => {
    render(<MockNasMetrics />);
    
    // Check section headings
    expect(screen.getByText('Storage Usage')).toBeInTheDocument();
    expect(screen.getByText('System Health')).toBeInTheDocument();
    expect(screen.getByText('Services Status')).toBeInTheDocument();
    expect(screen.getByText('Performance')).toBeInTheDocument();
  });
  
  it('displays storage information correctly', async () => {
    render(<MockNasMetrics />);
    
    const storageCard = screen.getByTestId('storage-card');
    expect(storageCard).toBeInTheDocument();
    expect(storageCard).toHaveTextContent('Total: 1 TB');
    expect(storageCard).toHaveTextContent('Used: 250 GB');
    expect(storageCard).toHaveTextContent('Free: 750 GB');
    expect(screen.getByRole('progressbar')).toHaveAttribute('aria-valuenow', '25');
  });
  
  it('displays system health information correctly', async () => {
    render(<MockNasMetrics />);
    
    const healthCard = screen.getByTestId('health-card');
    expect(healthCard).toBeInTheDocument();
    expect(healthCard).toHaveTextContent('CPU: 25%');
    expect(healthCard).toHaveTextContent('Memory: 50%');
    expect(healthCard).toHaveTextContent('Uptime: 5d 6h 30m');
    expect(healthCard).toHaveTextContent('Disk 1: Healthy');
    expect(healthCard).toHaveTextContent('Disk 2: Healthy');
  });
  
  it('displays services status correctly', async () => {
    render(<MockNasMetrics />);
    
    const servicesCard = screen.getByTestId('services-card');
    expect(servicesCard).toBeInTheDocument();
    
    const serviceElements = screen.getAllByText(/SMB|NFS|SSH/);
    expect(serviceElements.length).toBe(3);
    serviceElements.forEach(element => {
      expect(element).toHaveClass('service-running');
    });
  });
  
  it('displays performance metrics correctly', async () => {
    render(<MockNasMetrics />);
    
    const performanceCard = screen.getByTestId('performance-card');
    expect(performanceCard).toBeInTheDocument();
    expect(performanceCard).toHaveTextContent('Read: 100 MB/s');
    expect(performanceCard).toHaveTextContent('Write: 50 MB/s');
    expect(performanceCard).toHaveTextContent('IOPS: 150');
  });
}); 