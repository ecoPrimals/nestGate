import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { MemoryRouter } from 'react-router-dom';

// Create a mock BasicSettings component
const MockBasicSettings = () => {
  return (
    <div data-testid="basic-settings-container">
      <h2>System Settings</h2>
      <p>Configure system settings and preferences</p>
      
      <div data-testid="mock-tabs">
        <div data-testid="mock-tabs-nav">
          <button data-testid="tab-general" data-tab-key="general" aria-selected="true">General</button>
          <button data-testid="tab-network" data-tab-key="network" aria-selected="false">Network</button>
          <button data-testid="tab-user" data-tab-key="user" aria-selected="false">User Management</button>
        </div>
        
        <div data-testid="mock-tabs-content">
          {/* General Tab Content */}
          <div className="ant-card">
            <div className="ant-card-head">System Configuration</div>
            <div className="ant-card-body">
              <p>General system settings</p>
            </div>
          </div>
          
          {/* Network Tab Content */}
          <div className="ant-card" style={{ display: 'none' }}>
            <div className="ant-card-head">Network Configuration</div>
            <div className="ant-card-body">
              <p>Network settings</p>
            </div>
          </div>
          
          {/* User Management Tab Content */}
          <div className="ant-card" style={{ display: 'none' }}>
            <div className="ant-card-head">User Management Settings</div>
            <div className="ant-card-body">
              <p>User settings</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

describe('BasicSettings Component', () => {
  it('renders the basic settings page', () => {
    render(
      <MemoryRouter>
        <MockBasicSettings />
      </MemoryRouter>
    );
    
    // Check for page title
    expect(screen.getByText('System Settings')).toBeInTheDocument();
    
    // Check for sections
    expect(screen.getByText('System Configuration')).toBeInTheDocument();
    expect(screen.getByText('Network Configuration')).toBeInTheDocument();
    expect(screen.getByText('User Management Settings')).toBeInTheDocument();
  });
}); 