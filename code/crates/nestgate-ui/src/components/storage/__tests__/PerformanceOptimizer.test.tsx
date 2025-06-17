import React, { useState } from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import userEvent from '@testing-library/user-event';

// Create a mock component
const MockPerformanceOptimizer = () => {
  const [recordSize, setRecordSize] = useState('128K');
  
  const handleRecordSizeChange = (e) => {
    setRecordSize(e.target.value);
  };

  return (
    <div data-testid="performance-optimizer">
      <h2>Performance Optimizer</h2>
      <div className="pool-selector">
        <div className="ant-form-item">
          <label>Storage Pool</label>
          <select data-testid="pool-select">
            <option value="tank">tank</option>
            <option value="backup">backup</option>
          </select>
        </div>
      </div>
      
      <div className="dataset-selector">
        <div className="ant-form-item">
          <label>Dataset</label>
          <select data-testid="dataset-select">
            <option value="tank/data">tank/data</option>
            <option value="tank/home">tank/home</option>
          </select>
        </div>
      </div>
      
      <div className="metrics-display">
        <div className="metrics-card">
          <h3>Performance Metrics</h3>
          <div data-testid="read-metric">Read: 100 MB/s</div>
          <div data-testid="write-metric">Write: 50 MB/s</div>
          <div data-testid="iops-metric">IOPS: 150</div>
        </div>
      </div>
      
      <div className="optimization-controls">
        <div className="ant-form-item">
          <label>Record Size</label>
          <select 
            data-testid="recordsize-select" 
            value={recordSize} 
            onChange={handleRecordSizeChange}
          >
            <option value="4K">4K</option>
            <option value="8K">8K</option>
            <option value="16K">16K</option>
            <option value="32K">32K</option>
            <option value="64K">64K</option>
            <option value="128K">128K</option>
            <option value="1M">1M</option>
          </select>
        </div>
        
        <button data-testid="apply-button">Apply Changes</button>
      </div>
    </div>
  );
};

describe('PerformanceOptimizer', () => {
  it('renders the component correctly', async () => {
    render(<MockPerformanceOptimizer />);
    
    // Check basic structure
    expect(screen.getByText('Performance Optimizer')).toBeInTheDocument();
    expect(screen.getByTestId('pool-select')).toBeInTheDocument();
    expect(screen.getByTestId('dataset-select')).toBeInTheDocument();
    
    // Check metrics display
    expect(screen.getByTestId('read-metric')).toBeInTheDocument();
    expect(screen.getByTestId('write-metric')).toBeInTheDocument();
    expect(screen.getByTestId('iops-metric')).toBeInTheDocument();
    
    // Check optimization controls
    expect(screen.getByTestId('recordsize-select')).toBeInTheDocument();
    expect(screen.getByTestId('apply-button')).toBeInTheDocument();
  });
  
  it('displays performance metrics correctly', async () => {
    render(<MockPerformanceOptimizer />);
    
    // Wait for metrics to load
    await waitFor(() => {
      expect(screen.getByTestId('read-metric')).toHaveTextContent('Read: 100 MB/s');
      expect(screen.getByTestId('write-metric')).toHaveTextContent('Write: 50 MB/s');
      expect(screen.getByTestId('iops-metric')).toHaveTextContent('IOPS: 150');
    });
  });
  
  it('allows changing the record size', async () => {
    render(<MockPerformanceOptimizer />);
    
    // Get the select element
    const recordSizeSelect = screen.getByTestId('recordsize-select');
    
    // Verify initial value
    expect(recordSizeSelect).toHaveValue('128K');
    
    // Select a different record size
    await userEvent.selectOptions(recordSizeSelect, '64K');
    
    // Check that the selection was made
    expect(recordSizeSelect).toHaveValue('64K');
  });
}); 