import React from 'react';
import { MemoryRouter } from 'react-router-dom';
import Settings from './Settings';

const TestSettings: React.FC = () => {
  return (
    <MemoryRouter>
      <Settings />
    </MemoryRouter>
  );
};

export default TestSettings; 