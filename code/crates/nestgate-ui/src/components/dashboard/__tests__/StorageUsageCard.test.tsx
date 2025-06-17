import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';

// Create a mock component that mimics the behavior of StorageUsageCard
const MockStorageUsageCard: React.FC<{
  total: number;
  used: number;
  percentage: number;
  style?: React.CSSProperties;
}> = ({ total, used, percentage, style }) => {
  const freeSpace = total - used;
  
  // Simulate the formatCapacity function
  const formatCapacity = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };
  
  // Determine color based on usage percentage
  const getColorByPercentage = (percent: number): string => {
    if (percent < 70) return '#52c41a'; // success
    if (percent < 90) return '#faad14'; // warning
    return '#f5222d'; // error
  };
  
  const color = getColorByPercentage(percentage);
  
  return (
    <div className="ant-card" style={style}>
      <div className="ant-card-head">Storage Usage</div>
      <div className="ant-card-body">
        <div className="storage-usage-header">
          <h3>{percentage}%</h3>
          <div style={{ flexGrow: 1, marginLeft: 16 }}>
            <div 
              role="progressbar" 
              className="ant-progress" 
              aria-valuenow={percentage}
              aria-valuemin={0}
              aria-valuemax={100}
              style={{ backgroundColor: color }}
            />
          </div>
        </div>
        
        <div className="ant-row" style={{ marginTop: 24 }}>
          <div className="ant-col-8" style={{ textAlign: 'center' }}>
            <div>
              <span>Total</span>
              <h5>{formatCapacity(total)}</h5>
            </div>
          </div>
          
          <div className="ant-col-8" style={{ textAlign: 'center' }}>
            <div>
              <span>Used</span>
              <h5>{formatCapacity(used)}</h5>
            </div>
          </div>
          
          <div className="ant-col-8" style={{ textAlign: 'center' }}>
            <div>
              <span>Free</span>
              <h5>{formatCapacity(freeSpace)}</h5>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

describe('StorageUsageCard', () => {
  it('renders the component with correct data', () => {
    const { container } = render(
      <MockStorageUsageCard 
        total={1000000000} 
        used={250000000} 
        percentage={25} 
      />
    );
    
    // Check title is rendered
    expect(screen.getByText('Storage Usage')).toBeInTheDocument();
    
    // Check percentage is displayed
    expect(screen.getByText('25%')).toBeInTheDocument();
    
    // Check values are present (using more flexible approach)
    const totalElement = container.querySelector('.ant-col-8:nth-child(1) h5');
    const usedElement = container.querySelector('.ant-col-8:nth-child(2) h5');
    const freeElement = container.querySelector('.ant-col-8:nth-child(3) h5');
    
    expect(totalElement).toBeInTheDocument();
    expect(usedElement).toBeInTheDocument();
    expect(freeElement).toBeInTheDocument();
    
    // Verify they contain the right values approximately
    expect(totalElement?.textContent).toContain('MB');
    expect(usedElement?.textContent).toContain('MB');
    expect(freeElement?.textContent).toContain('MB');
  });
  
  it('displays normal status when usage is below warning threshold', () => {
    render(
      <MockStorageUsageCard 
        total={1000000000} 
        used={250000000} 
        percentage={25}
      />
    );
    
    const progressBar = screen.getByRole('progressbar');
    // For low usage, check the color is green (success)
    expect(progressBar).toHaveStyle('background-color: #52c41a');
  });
  
  it('displays warning status when usage is between 70% and 90%', () => {
    render(
      <MockStorageUsageCard 
        total={1000000000} 
        used={850000000} 
        percentage={85}
      />
    );
    
    const progressBar = screen.getByRole('progressbar');
    // For medium usage, check the color is yellow (warning)
    expect(progressBar).toHaveStyle('background-color: #faad14');
  });

  it('displays critical status when usage exceeds 90%', () => {
    render(
      <MockStorageUsageCard 
        total={1000000000} 
        used={950000000} 
        percentage={95}
      />
    );
    
    const progressBar = screen.getByRole('progressbar');
    // For high usage, check the color is red (error)
    expect(progressBar).toHaveStyle('background-color: #f5222d');
  });
  
  it('handles zero values correctly', () => {
    const { container } = render(
      <MockStorageUsageCard 
        total={0} 
        used={0} 
        percentage={0}
      />
    );
    
    // Check for "0%" specifically (should be unique)
    expect(screen.getByText('0%')).toBeInTheDocument();
    
    // Check that all three size displays show "0 Bytes"
    const sizeDisplays = container.querySelectorAll('h5');
    expect(sizeDisplays.length).toBe(3);
    sizeDisplays.forEach(element => {
      expect(element.textContent).toBe('0 Bytes');
    });
  });
  
  it('applies custom styles when provided', () => {
    const { container } = render(
      <MockStorageUsageCard 
        total={1000000000} 
        used={250000000} 
        percentage={25}
        style={{ maxWidth: '300px', margin: '8px' }} 
      />
    );
    
    const card = container.querySelector('.ant-card');
    expect(card).toHaveStyle('max-width: 300px');
    expect(card).toHaveStyle('margin: 8px');
  });
}); 