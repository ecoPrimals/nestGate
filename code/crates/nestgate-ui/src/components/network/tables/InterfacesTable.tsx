import React from 'react';
import { 
  TableContainer,
  Table,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
  Box,
  IconButton,
  Tooltip,
  Typography,
  Paper
} from '@mui/material';
import {
  Edit as EditIcon,
  PowerSettingsNew as PowerIcon
} from '@mui/icons-material';
import { NetworkInterface } from '../../../services/telemetry.service';
import StatusChip from '../../common/StatusChip';

interface InterfacesTableProps {
  interfaces: NetworkInterface[];
  isLoading: boolean;
  onEditInterface: (networkInterface: NetworkInterface) => void;
  onToggleInterface: (networkInterface: NetworkInterface) => void;
}

const InterfacesTable: React.FC<InterfacesTableProps> = ({
  interfaces,
  isLoading,
  onEditInterface,
  onToggleInterface
}) => {
  // Get type color for visualization
  const getTypeColor = (type: string): string => {
    const typeColors: Record<string, string> = {
      ethernet: 'primary.main',
      wireless: 'secondary.main',
      bond: 'warning.main',
      virtual: 'info.main'
    };
    return typeColors[type] || 'text.secondary';
  };

  return (
    <TableContainer component={Paper} elevation={0} sx={{ mb: 3 }}>
      <Table size="medium">
        <TableHead>
          <TableRow>
            <TableCell>Name</TableCell>
            <TableCell>Status</TableCell>
            <TableCell>Type</TableCell>
            <TableCell>MAC Address</TableCell>
            <TableCell>IPv4 Address</TableCell>
            <TableCell>IPv6 Address</TableCell>
            <TableCell>Speed</TableCell>
            <TableCell align="right">Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {interfaces.map((iface) => (
            <TableRow key={iface.name} hover>
              <TableCell>
                <Typography fontWeight="bold">{iface.name}</Typography>
              </TableCell>
              <TableCell>
                <StatusChip status={iface.status as 'online' | 'error'} />
              </TableCell>
              <TableCell>
                <Box sx={{ display: 'flex', alignItems: 'center' }}>
                  <StatusChip 
                    label={iface.type.toUpperCase()} 
                    color={getTypeColor(iface.type)}
                  />
                </Box>
              </TableCell>
              <TableCell>{iface.mac}</TableCell>
              <TableCell>{iface.ipv4 || '-'}</TableCell>
              <TableCell sx={{ fontSize: '0.9em' }}>{iface.ipv6 || '-'}</TableCell>
              <TableCell>{iface.speed > 0 ? `${iface.speed} Mbps` : '-'}</TableCell>
              <TableCell align="right">
                <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
                  <Tooltip title="Edit interface">
                    <IconButton
                      size="small"
                      onClick={() => onEditInterface(iface)}
                      sx={{ mr: 1 }}
                    >
                      <EditIcon fontSize="small" />
                    </IconButton>
                  </Tooltip>
                  <Tooltip title={iface.status === 'up' ? 'Disable interface' : 'Enable interface'}>
                    <IconButton
                      size="small"
                      color={iface.status === 'up' ? 'error' : 'success'}
                      onClick={() => onToggleInterface(iface)}
                    >
                      <PowerIcon fontSize="small" />
                    </IconButton>
                  </Tooltip>
                </Box>
              </TableCell>
            </TableRow>
          ))}
          {interfaces.length === 0 && (
            <TableRow>
              <TableCell colSpan={8} align="center" sx={{ py: 3 }}>
                <Typography color="text.secondary">
                  No network interfaces found
                </Typography>
              </TableCell>
            </TableRow>
          )}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default InterfacesTable; 