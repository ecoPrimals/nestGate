import React from 'react';
import { render, screen } from '@testing-library/react';
import VeryBasicSettings from '../VeryBasicSettings';

describe('VeryBasicSettings', () => {
  it('renders the main title and description', () => {
    render(<VeryBasicSettings />);
    
    const titleElement = screen.getByText('System Settings');
    const descriptionElement = screen.getByText(/Configure system-wide settings/);
    
    expect(titleElement).toBeInTheDocument();
    expect(descriptionElement).toBeInTheDocument();
  });
  
  it('renders all section headings', () => {
    render(<VeryBasicSettings />);
    
    const headings = screen.getAllByRole('heading', { level: 2 });
    
    expect(headings).toHaveLength(3);
    expect(headings[0]).toHaveTextContent('General Settings');
    expect(headings[1]).toHaveTextContent('Network Settings');
    expect(headings[2]).toHaveTextContent('User Management');
  });
}); 