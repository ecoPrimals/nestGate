import React, { useState } from 'react';
import {
  Card,
  CardHeader,
  CardContent,
  Button,
  Box,
  Typography,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Paper,
  Tab,
  Tabs,
  CircularProgress,
  Divider
} from '@mui/material';
import {
  PlayArrow as RunIcon,
  Clear as ClearIcon
} from '@mui/icons-material';
import { 
  PingOptions, 
  TracerouteOptions, 
  DNSLookupOptions, 
  PortScanOptions,
  DiagnosticResult,
  NetworkInterface
} from '../types';

interface DiagnosticsTabProps {
  interfaces: NetworkInterface[];
  diagnosticsRunning: boolean;
  diagnosticResults: DiagnosticResult | null;
  onRunPing: (options: PingOptions) => Promise<boolean>;
  onRunTraceroute: (options: TracerouteOptions) => Promise<boolean>;
  onRunDNSLookup: (options: DNSLookupOptions) => Promise<boolean>;
  onRunPortScan: (options: PortScanOptions) => Promise<boolean>;
}

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`diagnostics-tabpanel-${index}`}
      aria-labelledby={`diagnostics-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ pt: 2 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

const DiagnosticsTab: React.FC<DiagnosticsTabProps> = ({
  interfaces,
  diagnosticsRunning,
  diagnosticResults,
  onRunPing,
  onRunTraceroute,
  onRunDNSLookup,
  onRunPortScan
}) => {
  const [diagType, setDiagType] = useState(0);
  
  // Ping options
  const [pingOptions, setPingOptions] = useState<PingOptions>({
    target: '',
    count: 4,
    interval: 1,
    interface: ''
  });
  
  // Traceroute options
  const [tracerouteOptions, setTracerouteOptions] = useState<TracerouteOptions>({
    target: '',
    maxHops: 30,
    timeout: 5,
    interface: ''
  });
  
  // DNS lookup options
  const [dnsOptions, setDnsOptions] = useState<DNSLookupOptions>({
    domain: '',
    recordType: 'A',
    server: ''
  });
  
  // Port scan options
  const [portScanOptions, setPortScanOptions] = useState<PortScanOptions>({
    target: '',
    ports: '1-1024',
    timeout: 2
  });
  
  // Handle tab change
  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setDiagType(newValue);
  };
  
  // Handle Ping options change
  const handlePingChange = (e: React.ChangeEvent<HTMLInputElement> | { target: { name: string; value: unknown } }) => {
    const { name, value } = e.target;
    if (!name) return;
    setPingOptions(prev => ({ ...prev, [name]: value }));
  };
  
  // Handle Traceroute options change
  const handleTracerouteChange = (e: React.ChangeEvent<HTMLInputElement> | { target: { name: string; value: unknown } }) => {
    const { name, value } = e.target;
    if (!name) return;
    setTracerouteOptions(prev => ({ ...prev, [name]: value }));
  };
  
  // Handle DNS options change
  const handleDNSChange = (e: React.ChangeEvent<HTMLInputElement> | { target: { name: string; value: unknown } }) => {
    const { name, value } = e.target;
    if (!name) return;
    setDnsOptions(prev => ({ ...prev, [name]: value }));
  };
  
  // Handle Port scan options change
  const handlePortScanChange = (e: React.ChangeEvent<HTMLInputElement> | { target: { name: string; value: unknown } }) => {
    const { name, value } = e.target;
    if (!name) return;
    setPortScanOptions(prev => ({ ...prev, [name]: value }));
  };
  
  // Run the selected diagnostic
  const runDiagnostic = async () => {
    switch (diagType) {
      case 0:
        return onRunPing(pingOptions);
      case 1:
        return onRunTraceroute(tracerouteOptions);
      case 2:
        return onRunDNSLookup(dnsOptions);
      case 3:
        return onRunPortScan(portScanOptions);
      default:
        return false;
    }
  };
  
  // Format diagnostic results
  const formatResults = (results: DiagnosticResult) => {
    // Simple pre-formatted display for now
    return (
      <Paper 
        elevation={0} 
        sx={{ 
          p: 2, 
          backgroundColor: 'grey.100', 
          fontFamily: 'monospace',
          whiteSpace: 'pre-wrap',
          maxHeight: '400px',
          overflow: 'auto'
        }}
      >
        <Typography variant="body2" component="pre" sx={{ fontFamily: 'monospace' }}>
          {results.output || 'No output received'}
          {results.error && `\nError: ${results.error}`}
        </Typography>
      </Paper>
    );
  };
  
  // DNS record type options
  const recordTypes = [
    { value: 'A', label: 'A (IPv4 Address)' },
    { value: 'AAAA', label: 'AAAA (IPv6 Address)' },
    { value: 'MX', label: 'MX (Mail Exchange)' },
    { value: 'NS', label: 'NS (Name Server)' },
    { value: 'TXT', label: 'TXT (Text Record)' },
    { value: 'SOA', label: 'SOA (Start of Authority)' },
    { value: 'CNAME', label: 'CNAME (Canonical Name)' },
    { value: 'PTR', label: 'PTR (Pointer)' }
  ];
  
  // Form validation for each type
  const isPingValid = pingOptions.target.trim() !== '';
  const isTracerouteValid = tracerouteOptions.target.trim() !== '';
  const isDNSValid = dnsOptions.domain.trim() !== '';
  const isPortScanValid = portScanOptions.target.trim() !== '' && portScanOptions.ports.trim() !== '';
  
  // Get current form validity based on diagnostic type
  const isFormValid = () => {
    switch (diagType) {
      case 0: return isPingValid;
      case 1: return isTracerouteValid;
      case 2: return isDNSValid;
      case 3: return isPortScanValid;
      default: return false;
    }
  };

  return (
    <Card>
      <CardHeader title="Network Diagnostics" />
      <CardContent>
        <Typography variant="body2" color="text.secondary" paragraph>
          Run network diagnostic tools to troubleshoot connectivity issues.
        </Typography>
        
        <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
          <Tabs 
            value={diagType} 
            onChange={handleTabChange}
            aria-label="diagnostic tools tabs"
          >
            <Tab label="Ping" id="diag-tab-0" aria-controls="diag-tabpanel-0" />
            <Tab label="Traceroute" id="diag-tab-1" aria-controls="diag-tabpanel-1" />
            <Tab label="DNS Lookup" id="diag-tab-2" aria-controls="diag-tabpanel-2" />
            <Tab label="Port Scan" id="diag-tab-3" aria-controls="diag-tabpanel-3" />
          </Tabs>
        </Box>
        
        {/* Ping Tab */}
        <TabPanel value={diagType} index={0}>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              label="Target Host or IP"
              name="target"
              value={pingOptions.target}
              onChange={handlePingChange}
              fullWidth
              placeholder="e.g., google.com or 8.8.8.8"
              required
              error={!isPingValid}
            />
            
            <Box sx={{ display: 'flex', gap: 2 }}>
              <TextField
                label="Ping Count"
                name="count"
                type="number"
                value={pingOptions.count}
                onChange={handlePingChange}
                fullWidth
                InputProps={{ inputProps: { min: 1, max: 20 } }}
              />
              
              <TextField
                label="Interval (seconds)"
                name="interval"
                type="number"
                value={pingOptions.interval}
                onChange={handlePingChange}
                fullWidth
                InputProps={{ inputProps: { min: 0.1, max: 5, step: 0.1 } }}
              />
            </Box>
            
            <FormControl fullWidth>
              <InputLabel id="ping-interface-label">Interface</InputLabel>
              <Select
                labelId="ping-interface-label"
                name="interface"
                value={pingOptions.interface}
                onChange={handlePingChange}
                label="Interface"
              >
                <MenuItem value="">Default Interface</MenuItem>
                {interfaces.map(iface => (
                  <MenuItem key={iface.name} value={iface.name}>
                    {iface.name} ({iface.ipv4 || 'No IPv4'})
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Box>
        </TabPanel>
        
        {/* Traceroute Tab */}
        <TabPanel value={diagType} index={1}>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              label="Target Host or IP"
              name="target"
              value={tracerouteOptions.target}
              onChange={handleTracerouteChange}
              fullWidth
              placeholder="e.g., google.com or 8.8.8.8"
              required
              error={!isTracerouteValid}
            />
            
            <Box sx={{ display: 'flex', gap: 2 }}>
              <TextField
                label="Max Hops"
                name="maxHops"
                type="number"
                value={tracerouteOptions.maxHops}
                onChange={handleTracerouteChange}
                fullWidth
                InputProps={{ inputProps: { min: 1, max: 64 } }}
              />
              
              <TextField
                label="Timeout (seconds)"
                name="timeout"
                type="number"
                value={tracerouteOptions.timeout}
                onChange={handleTracerouteChange}
                fullWidth
                InputProps={{ inputProps: { min: 1, max: 30 } }}
              />
            </Box>
            
            <FormControl fullWidth>
              <InputLabel id="traceroute-interface-label">Interface</InputLabel>
              <Select
                labelId="traceroute-interface-label"
                name="interface"
                value={tracerouteOptions.interface}
                onChange={handleTracerouteChange}
                label="Interface"
              >
                <MenuItem value="">Default Interface</MenuItem>
                {interfaces.map(iface => (
                  <MenuItem key={iface.name} value={iface.name}>
                    {iface.name} ({iface.ipv4 || 'No IPv4'})
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Box>
        </TabPanel>
        
        {/* DNS Lookup Tab */}
        <TabPanel value={diagType} index={2}>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              label="Domain Name"
              name="domain"
              value={dnsOptions.domain}
              onChange={handleDNSChange}
              fullWidth
              placeholder="e.g., example.com"
              required
              error={!isDNSValid}
            />
            
            <FormControl fullWidth>
              <InputLabel id="dns-record-label">Record Type</InputLabel>
              <Select
                labelId="dns-record-label"
                name="recordType"
                value={dnsOptions.recordType}
                onChange={handleDNSChange}
                label="Record Type"
              >
                {recordTypes.map(option => (
                  <MenuItem key={option.value} value={option.value}>
                    {option.label}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
            
            <TextField
              label="DNS Server (optional)"
              name="server"
              value={dnsOptions.server}
              onChange={handleDNSChange}
              fullWidth
              placeholder="e.g., 8.8.8.8"
              helperText="Leave empty to use system default"
            />
          </Box>
        </TabPanel>
        
        {/* Port Scan Tab */}
        <TabPanel value={diagType} index={3}>
          <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
            <TextField
              label="Target Host or IP"
              name="target"
              value={portScanOptions.target}
              onChange={handlePortScanChange}
              fullWidth
              placeholder="e.g., example.com or 192.168.1.10"
              required
              error={!isPortScanValid}
            />
            
            <TextField
              label="Ports"
              name="ports"
              value={portScanOptions.ports}
              onChange={handlePortScanChange}
              fullWidth
              placeholder="e.g., 22,80,443 or 1-1024"
              helperText="Comma-separated list or range (e.g., 80,443 or 1-1024)"
              required
              error={!isPortScanValid}
            />
            
            <TextField
              label="Timeout (seconds)"
              name="timeout"
              type="number"
              value={portScanOptions.timeout}
              onChange={handlePortScanChange}
              fullWidth
              InputProps={{ inputProps: { min: 0.1, max: 10, step: 0.1 } }}
            />
          </Box>
        </TabPanel>
        
        {/* Action Buttons */}
        <Box sx={{ display: 'flex', justifyContent: 'flex-end', mt: 3, gap: 1 }}>
          <Button
            variant="outlined"
            startIcon={<ClearIcon />}
            onClick={() => {
              // Clear results
              switch (diagType) {
                case 0:
                  setPingOptions({ target: '', count: 4, interval: 1, interface: '' });
                  break;
                case 1:
                  setTracerouteOptions({ target: '', maxHops: 30, timeout: 5, interface: '' });
                  break;
                case 2:
                  setDnsOptions({ domain: '', recordType: 'A', server: '' });
                  break;
                case 3:
                  setPortScanOptions({ target: '', ports: '1-1024', timeout: 2 });
                  break;
              }
            }}
          >
            Clear Form
          </Button>
          <Button
            variant="contained"
            color="primary"
            startIcon={diagnosticsRunning ? <CircularProgress size={20} color="inherit" /> : <RunIcon />}
            onClick={runDiagnostic}
            disabled={diagnosticsRunning || !isFormValid()}
          >
            Run Diagnostic
          </Button>
        </Box>
        
        {/* Results Section */}
        {diagnosticResults && (
          <Box sx={{ mt: 3 }}>
            <Divider sx={{ my: 2 }} />
            <Typography variant="h6" gutterBottom>
              {`${diagnosticResults.type} Results`}
            </Typography>
            <Typography variant="subtitle2" color="text.secondary" gutterBottom>
              Command: {diagnosticResults.command}
            </Typography>
            {formatResults(diagnosticResults)}
          </Box>
        )}
      </CardContent>
    </Card>
  );
};

export default DiagnosticsTab; 