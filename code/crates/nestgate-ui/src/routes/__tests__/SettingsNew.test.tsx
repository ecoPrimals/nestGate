import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { MemoryRouter } from 'react-router-dom';
import SettingsNew from '../SettingsNew';

describe('SettingsNew Page', () => {
  // Helper function to render component
  const renderComponent = () => render(
    <MemoryRouter>
      <SettingsNew />
    </MemoryRouter>
  );

  it('renders the settings page with all sections', () => {
    renderComponent();
    
    // Check for page title
    expect(screen.getByText('System Settings')).toBeInTheDocument();
    
    // Check for some settings sections
    expect(screen.getByText('System Configuration')).toBeInTheDocument();
    expect(screen.getByText('Network Configuration')).toBeInTheDocument();
  });
}); 