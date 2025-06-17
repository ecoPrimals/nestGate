import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import MicroTestDefault, { MicroTest } from '../MicroTest';

describe('MicroTest Default Import', () => {
  it('renders the component via default import', () => {
    render(<MicroTestDefault />);
    expect(screen.getByText('Micro Test')).toBeInTheDocument();
  });
});

describe('MicroTest Named Import', () => {
  it('renders the component via named import', () => {
    render(<MicroTest />);
    expect(screen.getByText('Micro Test')).toBeInTheDocument();
  });
}); 