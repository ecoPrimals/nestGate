import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import TierCard, { StorageTier } from './TierCard';

// Mock data
const mockTier: StorageTier = {
  id: 'test-tier',
  name: 'hot',
  path: '/nestpool/hot',
  properties: {
    compression: 'lz4',
    recordsize: '128K',
    atime: 'off',
    primarycache: 'all',
  },
  usage: {
    available: 8 * 1024 * 1024 * 1024, // 8GB
    used: 2 * 1024 * 1024 * 1024, // 2GB
    total: 10 * 1024 * 1024 * 1024, // 10GB
    compressionRatio: 1.8,
  },
  monitoring: {
    enabled: true,
    activeEvents: 5,
    recentEvents: 43,
  },
};

describe('TierCard', () => {
  test('renders correctly with provided props', () => {
    render(<TierCard tier={mockTier} />);
    
    // Check that the title is displayed correctly
    expect(screen.getByText('Hot Tier')).toBeInTheDocument();
    
    // Check that the path is displayed
    expect(screen.getByText('/nestpool/hot')).toBeInTheDocument();
    
    // Check that compression is displayed
    expect(screen.getByText('Compression: lz4')).toBeInTheDocument();
    
    // Check that recordsize is displayed
    expect(screen.getByText('Recordsize: 128K')).toBeInTheDocument();
    
    // Check that the compression ratio is displayed (1.8x)
    expect(screen.getByText('1.80')).toBeInTheDocument();
    expect(screen.getByText('x')).toBeInTheDocument();
    
    // Check that activity is displayed (43 events)
    expect(screen.getByText('43')).toBeInTheDocument();
    expect(screen.getByText('events')).toBeInTheDocument();
  });
  
  test('applies correct class when selected', () => {
    const { container } = render(<TierCard tier={mockTier} isSelected={true} />);
    
    // Check that the selected class is applied
    expect(container.querySelector('.tier-card--selected')).toBeInTheDocument();
  });
  
  test('calls onSelect when clicked', () => {
    const handleSelect = jest.fn();
    render(<TierCard tier={mockTier} onSelect={handleSelect} />);
    
    // Click the card
    fireEvent.click(screen.getByText('Hot Tier'));
    
    // Check that the onSelect callback was called
    expect(handleSelect).toHaveBeenCalledTimes(1);
  });
  
  test('calls onConfigClick when config icon is clicked', () => {
    const handleConfigClick = jest.fn();
    render(<TierCard tier={mockTier} onConfigClick={handleConfigClick} />);
    
    // Find and click the config icon (SettingOutlined)
    const configIcon = document.querySelector('.tier-card__config-icon');
    if (configIcon) {
      fireEvent.click(configIcon);
    }
    
    // Check that the onConfigClick callback was called
    expect(handleConfigClick).toHaveBeenCalledTimes(1);
  });
  
  test('formats storage values correctly', () => {
    render(<TierCard tier={mockTier} />);
    
    // The usage text should show "2.00 GB / 10.00 GB"
    expect(screen.getByText('2.00 GB / 10.00 GB')).toBeInTheDocument();
  });
  
  test('applies correct tier-specific styling', () => {
    const { container, rerender } = render(<TierCard tier={mockTier} />);
    
    // Check that the hot tier class is applied
    expect(container.querySelector('.tier-card--hot')).toBeInTheDocument();
    
    // Test with warm tier
    const warmTier = { ...mockTier, name: 'warm' };
    rerender(<TierCard tier={warmTier} />);
    expect(container.querySelector('.tier-card--warm')).toBeInTheDocument();
    
    // Test with cold tier
    const coldTier = { ...mockTier, name: 'cold' };
    rerender(<TierCard tier={coldTier} />);
    expect(container.querySelector('.tier-card--cold')).toBeInTheDocument();
  });
}); 