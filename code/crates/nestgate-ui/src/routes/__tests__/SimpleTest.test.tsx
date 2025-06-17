import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import SimpleTest from '../SimpleTest';

describe('SimpleTest Component', () => {
  it('renders the simple test component', () => {
    render(<SimpleTest />);
    
    expect(screen.getByText('Simple Test Component')).toBeInTheDocument();
    expect(screen.getByText(/This is a simple test component/)).toBeInTheDocument();
  });
}); 