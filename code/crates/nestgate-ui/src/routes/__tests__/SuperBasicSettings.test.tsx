import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import SuperBasicSettings from '../SuperBasicSettings';

// Mock Ant Design's Tab component since it might be causing issues in tests
jest.mock('antd', () => {
  const originalModule = jest.requireActual('antd');
  
  return {
    ...originalModule,
    Tabs: ({ items, defaultActiveKey }: any) => {
      return (
        <div data-testid="mock-tabs">
          <div data-testid="mock-tabs-nav">
            {items.map((item: any) => (
              <button key={item.key} data-tab-key={item.key}>
                {item.label}
              </button>
            ))}
          </div>
          <div data-testid="mock-tabs-content">
            {items.find((item: any) => item.key === defaultActiveKey)?.children}
          </div>
        </div>
      );
    },
  };
});

describe('SuperBasicSettings', () => {
  it('renders the title and description', () => {
    render(<SuperBasicSettings />);
    
    expect(screen.getByText('System Settings')).toBeInTheDocument();
    expect(screen.getByText('Configure system-wide settings for your NestGate storage system.')).toBeInTheDocument();
  });

  it('renders the tabs navigation', () => {
    render(<SuperBasicSettings />);
    
    const tabsNav = screen.getByTestId('mock-tabs-nav');
    expect(tabsNav).toBeInTheDocument();
    
    // Check for tab buttons
    expect(screen.getByText('General')).toBeInTheDocument();
    expect(screen.getByText('Network')).toBeInTheDocument();
    expect(screen.getByText('Backup & Replication')).toBeInTheDocument();
    expect(screen.getByText('User Management')).toBeInTheDocument();
  });

  it('shows the general settings form by default', () => {
    render(<SuperBasicSettings />);
    
    expect(screen.getByText('System Configuration')).toBeInTheDocument();
    expect(screen.getByText('Setting Name:')).toBeInTheDocument();
    expect(screen.getByText('Save Changes')).toBeInTheDocument();
  });
}); 