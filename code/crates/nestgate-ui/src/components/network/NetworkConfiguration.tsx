import React from 'react';
import {
  Box,
  Typography,
  Tabs,
  Tab,
  Alert
} from '@mui/material';
import { DataSourceBanner } from '../common';
import { MockModeReason } from '../../services/websocket.service';
import { useNetworkData } from './hooks/useNetworkData';
import InterfacesTab from './components/InterfacesTab';
import DNSTab from './components/DNSTab';
import FirewallTab from './components/FirewallTab';
import DiagnosticsTab from './components/DiagnosticsTab';

// Custom TabPanel component
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
      id={`network-tabpanel-${index}`}
      aria-labelledby={`network-tab-${index}`}
      {...other}
    >
      {value === index && (
        <Box sx={{ py: 2 }}>
          {children}
        </Box>
      )}
    </div>
  );
}

const NetworkConfiguration: React.FC = () => {
  // Use our custom hook to manage network data
  const {
    interfaces,
    isLoading,
    dataSource,
    editModalVisible,
    currentInterface,
    dnsSettings,
    firewallRules,
    activeTab,
    diagnosticsRunning,
    diagnosticResults,
    setActiveTab,
    refreshInterfaces,
    toggleInterfaceStatus,
    updateNetworkInterface,
    updateDNSSettings,
    addFirewallRule,
    toggleFirewallRule,
    deleteFirewallRule,
    runPing,
    runTraceroute,
    runDNSLookup,
    runPortScan,
    showEditInterfaceModal,
    hideEditInterfaceModal
  } = useNetworkData();

  // Map activeTab from string to number for Material UI Tabs
  const getTabIndex = (tab: string): number => {
    switch(tab) {
      case 'interfaces': return 0;
      case 'dns': return 1;
      case 'firewall': return 2;
      case 'diagnostics': return 3;
      default: return 0;
    }
  };

  // Map tab index to string for state
  const getTabName = (index: number): string => {
    switch(index) {
      case 0: return 'interfaces';
      case 1: return 'dns';
      case 2: return 'firewall';
      case 3: return 'diagnostics';
      default: return 'interfaces';
    }
  };

  // Handle tab change
  const handleTabChange = (event: React.SyntheticEvent, newValue: number) => {
    setActiveTab(getTabName(newValue));
  };

  return (
    <Box className="network-configuration">
      {/* Show mock data banner if using mock data */}
      {dataSource !== 'live' && (
        <Alert severity="info" sx={{ mb: 2 }}>
          <DataSourceBanner
            dataSource={dataSource}
            serviceName="Network Configuration"
            mockReason={MockModeReason.DELIBERATE}
          />
        </Alert>
      )}
      
      <Typography variant="h4" gutterBottom>Network Configuration</Typography>
      <Typography variant="body1" color="text.secondary" paragraph>
        Configure and manage network interfaces, DNS settings, and firewall rules.
      </Typography>
      
      <Box sx={{ borderBottom: 1, borderColor: 'divider' }}>
        <Tabs 
          value={getTabIndex(activeTab)} 
          onChange={handleTabChange}
        >
          <Tab label="Network Interfaces" id="network-tab-0" aria-controls="network-tabpanel-0" />
          <Tab label="DNS Settings" id="network-tab-1" aria-controls="network-tabpanel-1" />
          <Tab label="Firewall Rules" id="network-tab-2" aria-controls="network-tabpanel-2" />
          <Tab label="Network Diagnostics" id="network-tab-3" aria-controls="network-tabpanel-3" />
        </Tabs>
      </Box>
      
      <TabPanel value={getTabIndex(activeTab)} index={0}>
        <InterfacesTab 
          interfaces={interfaces}
          isLoading={isLoading}
          editModalVisible={editModalVisible}
          currentInterface={currentInterface}
          onRefreshInterfaces={refreshInterfaces}
          onEditInterface={showEditInterfaceModal}
          onToggleInterface={toggleInterfaceStatus}
          onSaveInterface={updateNetworkInterface}
          onCloseEditModal={hideEditInterfaceModal}
        />
      </TabPanel>
      
      <TabPanel value={getTabIndex(activeTab)} index={1}>
        <DNSTab 
          dnsSettings={dnsSettings}
          isLoading={isLoading}
          onSaveDNSSettings={updateDNSSettings}
        />
      </TabPanel>
      
      <TabPanel value={getTabIndex(activeTab)} index={2}>
        <FirewallTab 
          firewallRules={firewallRules}
          isLoading={isLoading}
          onRefreshRules={refreshInterfaces}
          onAddRule={addFirewallRule}
          onToggleRule={toggleFirewallRule}
          onDeleteRule={deleteFirewallRule}
        />
      </TabPanel>
      
      <TabPanel value={getTabIndex(activeTab)} index={3}>
        <DiagnosticsTab 
          interfaces={interfaces}
          diagnosticsRunning={diagnosticsRunning}
          diagnosticResults={diagnosticResults}
          onRunPing={runPing}
          onRunTraceroute={runTraceroute}
          onRunDNSLookup={runDNSLookup}
          onRunPortScan={runPortScan}
        />
      </TabPanel>
    </Box>
  );
};

export default NetworkConfiguration; 