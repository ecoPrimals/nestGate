import React from 'react';
import { Box, Typography, Button } from '@mui/material';
import { Link } from 'react-router-dom';
import { Home as HomeIcon } from '@mui/icons-material';

/**
 * 404 Not Found page component
 */
const NotFound: React.FC = () => {
  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        minHeight: '60vh',
        textAlign: 'center',
        padding: 3,
      }}
    >
      <Typography
        variant="h1"
        sx={{
          fontSize: '6rem',
          fontWeight: 'bold',
          color: 'text.secondary',
          marginBottom: 2,
        }}
      >
        404
      </Typography>
      
      <Typography
        variant="h4"
        sx={{
          marginBottom: 1,
          color: 'text.primary',
        }}
      >
        Page Not Found
      </Typography>
      
      <Typography
        variant="body1"
        sx={{
          marginBottom: 4,
          color: 'text.secondary',
          maxWidth: 400,
        }}
      >
        Sorry, the page you visited does not exist. Please check the URL or navigate back to the dashboard.
      </Typography>
      
      <Button
        component={Link}
        to="/dashboard"
        variant="contained"
        color="primary"
        startIcon={<HomeIcon />}
        size="large"
      >
        Back to Dashboard
      </Button>
    </Box>
  );
};

export default NotFound; 