import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import RecentActivityCard from '../RecentActivityCard';

describe('RecentActivityCard', () => {
  beforeEach(() => {
    // Mock Date.now() to return a fixed timestamp
    const mockDate = new Date('2024-12-01T12:00:00Z');
    jest.spyOn(Date, 'now').mockImplementation(() => mockDate.getTime());
  });

  afterEach(() => {
    // Reset all mocks
    jest.restoreAllMocks();
  });

  it('renders the card with title', () => {
    render(<RecentActivityCard />);
    
    // Check for title
    expect(screen.getByText('Recent Activity')).toBeInTheDocument();
  });

  it('displays activity items with correct information', () => {
    render(<RecentActivityCard />);
    
    // Check for activity messages
    expect(screen.getByText('Snapshot created')).toBeInTheDocument();
    expect(screen.getByText('Scrub started')).toBeInTheDocument();
    expect(screen.getByText('Disk temperature warning')).toBeInTheDocument();
    expect(screen.getByText('Resilver completed')).toBeInTheDocument();
    expect(screen.getByText('Dataset mount failed')).toBeInTheDocument();
    
    // Check for activity details
    expect(screen.getByText('pool/dataset@auto-2024-11-30-1200')).toBeInTheDocument();
    expect(screen.getByText('mainpool')).toBeInTheDocument();
    expect(screen.getByText('ada3 (WDC WD1002FAEX)')).toBeInTheDocument();
    expect(screen.getByText('backup_pool')).toBeInTheDocument();
    expect(screen.getByText('pool/dataset2')).toBeInTheDocument();
  });

  it('displays status tags correctly', () => {
    render(<RecentActivityCard />);
    
    // Check for status tags
    expect(screen.getAllByText('Completed').length).toBe(2);
    expect(screen.getByText('In Progress')).toBeInTheDocument();
    expect(screen.getByText('Warning')).toBeInTheDocument();
    expect(screen.getByText('Failed')).toBeInTheDocument();
  });

  it('formats timestamps correctly', () => {
    render(<RecentActivityCard />);
    
    // Since we mocked Date.now() to a fixed time, we should see these relative times
    expect(screen.getByText('5m ago')).toBeInTheDocument();
    expect(screen.getByText('30m ago')).toBeInTheDocument();
    expect(screen.getByText('2h ago')).toBeInTheDocument();
    expect(screen.getByText('12h ago')).toBeInTheDocument();
    expect(screen.getByText('1d ago')).toBeInTheDocument();
  });
}); 