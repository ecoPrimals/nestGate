import React from 'react';
import ReactDOM from 'react-dom/client';
import { ThemeProvider, createTheme, CssBaseline, Box, Typography, AppBar, Toolbar, Tabs, Tab } from '@mui/material';
import { NasMetrics } from './components/dashboard/NasMetrics';
import { PerformanceOptimizer } from './components/storage/PerformanceOptimizer';
import DemoDescription from './demo-description';
import TabPanel from './components/common/TabPanel';

const theme = createTheme({
  palette: {
    primary: {
      main: '#1677ff',
      dark: '#0958d9',
    },
    background: {
      default: '#f5f5f5',
    },
  },
});

export const Demo: React.FC = () => {
  const [activeTab, setActiveTab] = React.useState(0);

  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(newValue);
  };

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ 
        minHeight: '100vh',
        display: 'flex',
        flexDirection: 'column'
      }}>
        <AppBar position="static" sx={{
          background: 'linear-gradient(90deg, #1677ff, #0958d9)',
        }}>
          <Toolbar sx={{ padding: '0 24px' }}>
            <Typography variant="h5" component="h1" sx={{ color: 'white', margin: 0 }}>
              NestGate ZFS Dashboard
            </Typography>
          </Toolbar>
        </AppBar>
        
        <Box sx={{ 
          flex: 1,
          padding: { xs: 2, md: 3 },
          backgroundColor: 'background.default'
        }}>
          <Box sx={{
            backgroundColor: 'white',
            padding: { xs: 1, md: 2 },
            borderRadius: 1,
            boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
          }}>
            <Tabs 
              value={activeTab} 
              onChange={handleTabChange}
              sx={{ borderBottom: 1, borderColor: 'divider' }}
            >
              <Tab label="Demo Description" />
              <Tab label="System Metrics" />
              <Tab label="Performance Optimizer" />
            </Tabs>
            
            <TabPanel value={activeTab} index={0}>
              <DemoDescription />
            </TabPanel>
            <TabPanel value={activeTab} index={1}>
              <NasMetrics />
            </TabPanel>
            <TabPanel value={activeTab} index={2}>
              <PerformanceOptimizer />
            </TabPanel>
          </Box>
        </Box>
      </Box>
    </ThemeProvider>
  );
};

// Create root element
const rootElement = document.createElement('div');
rootElement.id = 'root';
document.body.appendChild(rootElement);

// Render the demo
const root = ReactDOM.createRoot(rootElement);
root.render(<Demo />); 