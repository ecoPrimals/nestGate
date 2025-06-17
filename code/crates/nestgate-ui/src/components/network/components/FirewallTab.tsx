import React, { useState } from 'react';
import {
  Card,
  CardHeader,
  CardContent,
  Button,
  Box,
  Typography,
  Table,
  TableContainer,
  TableHead,
  TableBody,
  TableRow,
  TableCell,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Switch,
  Paper,
  Tooltip
} from '@mui/material';
import {
  Add as AddIcon,
  Delete as DeleteIcon,
  PowerSettingsNew as PowerIcon,
  Refresh as RefreshIcon
} from '@mui/icons-material';
import { FirewallRule } from '../types';

interface FirewallTabProps {
  firewallRules: FirewallRule[];
  isLoading: boolean;
  onRefreshRules: () => Promise<boolean>;
  onAddRule: (rule: Omit<FirewallRule, 'id'>) => Promise<boolean>;
  onToggleRule: (rule: FirewallRule) => Promise<boolean>;
  onDeleteRule: (rule: FirewallRule) => Promise<boolean>;
}

const FirewallTab: React.FC<FirewallTabProps> = ({
  firewallRules,
  isLoading,
  onRefreshRules,
  onAddRule,
  onToggleRule,
  onDeleteRule
}) => {
  const [dialogOpen, setDialogOpen] = useState(false);
  const [newRule, setNewRule] = useState<Omit<FirewallRule, 'id'>>({
    name: '',
    enabled: true,
    protocol: 'tcp',
    port: '',
    source: '',
    destination: '',
    action: 'allow'
  });

  const handleOpenDialog = () => {
    setDialogOpen(true);
  };

  const handleCloseDialog = () => {
    setDialogOpen(false);
  };

  const handleRuleChange = (e: React.ChangeEvent<HTMLInputElement> | { target: { name: string; value: unknown } }) => {
    const { name, value } = e.target;
    if (!name) return;
    
    if (name === 'enabled') {
      // Special handling for the switch component
      const checked = (e.target as HTMLInputElement).checked;
      setNewRule(prev => ({ ...prev, [name]: checked }));
    } else {
      setNewRule(prev => ({ ...prev, [name]: value }));
    }
  };

  const handleAddRule = async () => {
    if (await onAddRule(newRule)) {
      handleCloseDialog();
      // Reset form
      setNewRule({
        name: '',
        enabled: true,
        protocol: 'tcp',
        port: '',
        source: '',
        destination: '',
        action: 'allow'
      });
    }
  };

  // Protocol options
  const protocolOptions = [
    { value: 'tcp', label: 'TCP' },
    { value: 'udp', label: 'UDP' },
    { value: 'icmp', label: 'ICMP' },
    { value: 'any', label: 'Any' }
  ];

  // Action options
  const actionOptions = [
    { value: 'allow', label: 'Allow' },
    { value: 'deny', label: 'Deny' }
  ];

  // Form validation
  const isFormValid = () => {
    return (
      newRule.name.trim() !== '' &&
      newRule.port.trim() !== '' &&
      (newRule.source.trim() !== '' || newRule.destination.trim() !== '')
    );
  };

  return (
    <>
      <Card>
        <CardHeader
          title="Firewall Rules"
          action={
            <Box sx={{ display: 'flex', gap: 1 }}>
              <Button
                variant="outlined"
                startIcon={<RefreshIcon />}
                onClick={onRefreshRules}
                disabled={isLoading}
              >
                Refresh
              </Button>
              <Button
                variant="contained"
                color="primary"
                startIcon={<AddIcon />}
                onClick={handleOpenDialog}
                disabled={isLoading}
              >
                Add Rule
              </Button>
            </Box>
          }
        />
        <CardContent>
          <Typography variant="body2" color="text.secondary" paragraph>
            Manage firewall rules to control network traffic to and from your NAS.
          </Typography>
          
          <TableContainer component={Paper} elevation={0}>
            <Table size="medium">
              <TableHead>
                <TableRow>
                  <TableCell>Name</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Protocol</TableCell>
                  <TableCell>Port</TableCell>
                  <TableCell>Source</TableCell>
                  <TableCell>Destination</TableCell>
                  <TableCell>Action</TableCell>
                  <TableCell align="right">Controls</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {firewallRules.map((rule) => (
                  <TableRow key={rule.id} hover>
                    <TableCell>{rule.name}</TableCell>
                    <TableCell>
                      <Typography color={rule.enabled ? 'success.main' : 'text.disabled'}>
                        {rule.enabled ? 'Enabled' : 'Disabled'}
                      </Typography>
                    </TableCell>
                    <TableCell>{rule.protocol.toUpperCase()}</TableCell>
                    <TableCell>{rule.port || 'Any'}</TableCell>
                    <TableCell>{rule.source || 'Any'}</TableCell>
                    <TableCell>{rule.destination || 'Any'}</TableCell>
                    <TableCell>
                      <Typography 
                        color={rule.action === 'allow' ? 'success.main' : 'error.main'}
                        fontWeight="medium"
                      >
                        {rule.action.toUpperCase()}
                      </Typography>
                    </TableCell>
                    <TableCell align="right">
                      <Box sx={{ display: 'flex', justifyContent: 'flex-end' }}>
                        <Tooltip title={rule.enabled ? 'Disable rule' : 'Enable rule'}>
                          <IconButton
                            color={rule.enabled ? 'default' : 'success'}
                            onClick={() => onToggleRule(rule)}
                            size="small"
                            sx={{ mr: 1 }}
                          >
                            <PowerIcon fontSize="small" />
                          </IconButton>
                        </Tooltip>
                        <Tooltip title="Delete rule">
                          <IconButton
                            color="error"
                            onClick={() => onDeleteRule(rule)}
                            size="small"
                          >
                            <DeleteIcon fontSize="small" />
                          </IconButton>
                        </Tooltip>
                      </Box>
                    </TableCell>
                  </TableRow>
                ))}
                {firewallRules.length === 0 && (
                  <TableRow>
                    <TableCell colSpan={8} align="center" sx={{ py: 3 }}>
                      <Typography color="text.secondary">
                        No firewall rules configured
                      </Typography>
                    </TableCell>
                  </TableRow>
                )}
              </TableBody>
            </Table>
          </TableContainer>
        </CardContent>
      </Card>

      {/* Add Firewall Rule Dialog */}
      <Dialog open={dialogOpen} onClose={handleCloseDialog} maxWidth="sm" fullWidth>
        <DialogTitle>Add Firewall Rule</DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2, pt: 1 }}>
            <TextField
              label="Rule Name"
              name="name"
              value={newRule.name}
              onChange={handleRuleChange}
              fullWidth
              required
            />
            
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              <Typography>Enabled</Typography>
              <Switch
                name="enabled"
                checked={newRule.enabled}
                onChange={handleRuleChange}
                color="success"
              />
            </Box>
            
            <FormControl fullWidth>
              <InputLabel id="protocol-label">Protocol</InputLabel>
              <Select
                labelId="protocol-label"
                name="protocol"
                value={newRule.protocol}
                onChange={handleRuleChange}
                label="Protocol"
              >
                {protocolOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
            
            <TextField
              label="Port"
              name="port"
              value={newRule.port}
              onChange={handleRuleChange}
              fullWidth
              placeholder="e.g., 80, 443, 22-25"
              helperText="Specify port numbers or ranges (e.g., 80-443)"
            />
            
            <TextField
              label="Source"
              name="source"
              value={newRule.source}
              onChange={handleRuleChange}
              fullWidth
              placeholder="e.g., 192.168.1.0/24, any"
              helperText="Specify IP address, subnet, or 'any'"
            />
            
            <TextField
              label="Destination"
              name="destination"
              value={newRule.destination}
              onChange={handleRuleChange}
              fullWidth
              placeholder="e.g., 192.168.1.10, any"
              helperText="Specify IP address, subnet, or 'any'"
            />
            
            <FormControl fullWidth>
              <InputLabel id="action-label">Action</InputLabel>
              <Select
                labelId="action-label"
                name="action"
                value={newRule.action}
                onChange={handleRuleChange}
                label="Action"
              >
                {actionOptions.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancel</Button>
          <Button 
            onClick={handleAddRule} 
            variant="contained" 
            color="primary"
            disabled={!isFormValid()}
          >
            Add Rule
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

export default FirewallTab; 