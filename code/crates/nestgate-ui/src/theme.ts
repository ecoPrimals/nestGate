import { createTheme } from '@mui/material/styles';

const theme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#005f73',
      light: '#3d8184',
      dark: '#003f4f',
      contrastText: '#ffffff',
    },
    secondary: {
      main: '#0a9396',
      light: '#3da8aa',
      dark: '#006669',
      contrastText: '#ffffff',
    },
    error: {
      main: '#ae2012',
      light: '#bf4d3d',
      dark: '#79160c',
      contrastText: '#ffffff',
    },
    warning: {
      main: '#ee9b00',
      light: '#f1af33',
      dark: '#a66c00',
      contrastText: '#000000',
    },
    info: {
      main: '#94d2bd',
      light: '#a9daca',
      dark: '#679384',
      contrastText: '#000000',
    },
    success: {
      main: '#2b9348',
      light: '#55a96c',
      dark: '#1e6631',
      contrastText: '#ffffff',
    },
    background: {
      default: '#f5f5f5',
      paper: '#ffffff',
    },
    text: {
      primary: '#333333',
      secondary: '#555555',
      disabled: '#999999',
    },
  },
  typography: {
    fontFamily: [
      '-apple-system',
      'BlinkMacSystemFont',
      '"Segoe UI"',
      'Roboto',
      '"Helvetica Neue"',
      'Arial',
      'sans-serif',
    ].join(','),
    h1: {
      fontSize: '2.5rem',
      fontWeight: 500,
    },
    h2: {
      fontSize: '2rem',
      fontWeight: 500,
    },
    h3: {
      fontSize: '1.75rem',
      fontWeight: 500,
    },
    h4: {
      fontSize: '1.5rem',
      fontWeight: 500,
    },
    h5: {
      fontSize: '1.25rem',
      fontWeight: 500,
    },
    h6: {
      fontSize: '1rem',
      fontWeight: 500,
    },
    body1: {
      fontSize: '1rem',
    },
    body2: {
      fontSize: '0.875rem',
    },
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none',
          borderRadius: 4,
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          boxShadow: '0 2px 8px rgba(0, 0, 0, 0.08)',
        },
      },
    },
    MuiChip: {
      styleOverrides: {
        root: {
          borderRadius: 4,
        },
      },
    },
    MuiDivider: {
      styleOverrides: {
        root: {
          margin: '16px 0',
        },
      },
    },
  },
});

export default theme; 