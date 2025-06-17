import React from 'react';
import { Box, Container, Typography, Paper, ThemeProvider, createTheme, CssBaseline } from '@mui/material';
import SystemDashboard from './components/Dashboard/SystemDashboard';

// Create a dark theme for the NAS interface
const theme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#00bcd4',
    },
    secondary: {
      main: '#ff9800',
    },
    background: {
      default: '#121212',
      paper: '#1e1e1e',
    },
  },
  typography: {
    fontFamily: '"Roboto", "Helvetica", "Arial", sans-serif',
  },
});

const App: React.FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ minHeight: '100vh', bgcolor: 'background.default' }}>
        <SystemDashboard />
      </Box>
    </ThemeProvider>
  );
};

export default App; 