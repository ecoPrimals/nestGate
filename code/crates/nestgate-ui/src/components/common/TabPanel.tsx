import React from 'react';
import { Box, SxProps, Theme } from '@mui/material';

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
  sx?: SxProps<Theme>;
  padding?: number | string;
}

/**
 * TabPanel component provides a standardized way to handle tab content.
 * It shows the children only when the tab is active (value === index).
 */
const TabPanel: React.FC<TabPanelProps> = ({
  children,
  value,
  index,
  sx = {},
  padding = 2,
  ...other
}) => {
  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`tabpanel-${index}`}
      aria-labelledby={`tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ p: padding, ...sx }}>
          {children}
        </Box>
      )}
    </div>
  );
};

/**
 * Helper function to generate aria-props for tabs
 */
export const a11yProps = (index: number) => {
  return {
    id: `tab-${index}`,
    'aria-controls': `tabpanel-${index}`,
  };
};

export default TabPanel; 