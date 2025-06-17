import React, { useEffect } from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  TextField,
  FormControl,
  FormLabel,
  RadioGroup,
  FormControlLabel,
  Radio,
  Box,
  CircularProgress
} from '@mui/material';
import { NetworkInterface } from '../../../services/telemetry.service';
import { NetworkInterfaceUpdate } from '../types';

interface EditInterfaceModalProps {
  open: boolean;
  interface?: NetworkInterface | null;
  isLoading: boolean;
  onClose: () => void;
  onSave: (values: NetworkInterfaceUpdate) => Promise<boolean>;
}

const EditInterfaceModal: React.FC<EditInterfaceModalProps> = ({
  open,
  interface: networkInterface,
  isLoading,
  onClose,
  onSave
}) => {
  const [values, setValues] = React.useState<NetworkInterfaceUpdate>({
    name: '',
    ipMode: 'dhcp',
    ipv4: '',
    ipv6: '',
    status: 'up'
  });
  
  // Update values when interface changes
  useEffect(() => {
    if (networkInterface) {
      setValues({
        name: networkInterface.name,
        ipMode: networkInterface.ipv4 ? 'static' : 'dhcp',
        ipv4: networkInterface.ipv4 || '',
        ipv6: networkInterface.ipv6 || '',
        status: networkInterface.status
      });
    }
  }, [networkInterface]);
  
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setValues(prev => ({ ...prev, [name]: value }));
  };
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (await onSave(values)) {
      onClose();
    }
  };
  
  return (
    <Dialog 
      open={open} 
      onClose={onClose}
      maxWidth="sm"
      fullWidth
    >
      <DialogTitle>
        {networkInterface ? `Edit Interface: ${networkInterface.name}` : 'Edit Interface'}
      </DialogTitle>
      
      <form onSubmit={handleSubmit}>
        <DialogContent>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              label="Interface Name"
              name="name"
              value={values.name}
              onChange={handleChange}
              fullWidth
              disabled
              required
              margin="normal"
            />
            
            <TextField
              label="Interface Type"
              value={networkInterface?.type || ''}
              fullWidth
              disabled
              margin="normal"
            />
            
            <TextField
              label="MAC Address"
              value={networkInterface?.mac || ''}
              fullWidth
              disabled
              margin="normal"
            />
            
            <FormControl component="fieldset" margin="normal">
              <FormLabel component="legend">IP Configuration</FormLabel>
              <RadioGroup
                name="ipMode"
                value={values.ipMode}
                onChange={handleChange}
              >
                <FormControlLabel 
                  value="dhcp" 
                  control={<Radio />} 
                  label="DHCP (Automatic)" 
                />
                <FormControlLabel 
                  value="static" 
                  control={<Radio />} 
                  label="Static IP" 
                />
              </RadioGroup>
            </FormControl>
            
            <TextField
              label="IPv4 Address"
              name="ipv4"
              value={values.ipv4}
              onChange={handleChange}
              fullWidth
              disabled={values.ipMode !== 'static'}
              placeholder="e.g., 192.168.1.100"
              margin="normal"
            />
            
            <TextField
              label="IPv6 Address"
              name="ipv6"
              value={values.ipv6}
              onChange={handleChange}
              fullWidth
              disabled={values.ipMode !== 'static'}
              placeholder="e.g., 2001:db8::1"
              margin="normal"
            />
            
            <FormControl component="fieldset" margin="normal">
              <FormLabel component="legend">Status</FormLabel>
              <RadioGroup
                name="status"
                value={values.status}
                onChange={handleChange}
              >
                <FormControlLabel 
                  value="up" 
                  control={<Radio color="success" />} 
                  label="Enabled" 
                />
                <FormControlLabel 
                  value="down" 
                  control={<Radio color="error" />} 
                  label="Disabled" 
                />
              </RadioGroup>
            </FormControl>
          </Box>
        </DialogContent>
        
        <DialogActions>
          <Button onClick={onClose} disabled={isLoading}>
            Cancel
          </Button>
          <Button 
            type="submit" 
            variant="contained" 
            color="primary"
            disabled={isLoading}
            startIcon={isLoading ? <CircularProgress size={20} /> : undefined}
          >
            Save Changes
          </Button>
        </DialogActions>
      </form>
    </Dialog>
  );
};

export default EditInterfaceModal; 