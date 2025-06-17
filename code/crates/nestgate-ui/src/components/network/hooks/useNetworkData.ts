import { useState, useEffect } from 'react';
import { NetworkInterface } from '../../../services/telemetry.service';
import { NetworkService } from '../../../services/network.service';
import { shouldUseMockData, DataSourceType } from '../../../utils/env';
import { 
  DNSSettings, 
  FirewallRule, 
  NetworkInterfaceUpdate,
  PingOptions,
  TracerouteOptions,
  DNSLookupOptions,
  PortScanOptions,
  DiagnosticResult,
  NetworkConfigurationState
} from '../types';

export const useNetworkData = () => {
  const [state, setState] = useState<NetworkConfigurationState>({
    interfaces: [],
    isLoading: true,
    editModalVisible: false,
    currentInterface: null,
    dnsSettings: {
      primaryDNS: '',
      secondaryDNS: ''
    },
    firewallRules: [],
    activeTab: 'interfaces',
    diagnosticsRunning: false,
    diagnosticResults: null
  });
  
  const [dataSource, setDataSource] = useState<DataSourceType>(
    shouldUseMockData('network') ? DataSourceType.MOCK : DataSourceType.LIVE
  );
  
  const networkService = NetworkService.getInstance();
  
  // Load initial data
  useEffect(() => {
    loadNetworkData();
  }, []);
  
  // Function to load all network data
  const loadNetworkData = async () => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      // Detect if we're using mock data
      const usingMock = shouldUseMockData('network');
      setDataSource(usingMock ? DataSourceType.MOCK : DataSourceType.LIVE);
      
      // Load network interfaces
      const interfaces = await networkService.getNetworkInterfaces();
      
      // Load DNS settings
      const dnsSettings = await networkService.getDNSSettings();
      
      // Load firewall rules
      const firewallRules = await networkService.getFirewallRules();
      
      setState(prev => ({
        ...prev,
        interfaces,
        dnsSettings,
        firewallRules,
        isLoading: false
      }));
    } catch (error) {
      console.error('Error loading network data:', error);
      // Do not fallback to mock data
      setState(prev => ({ ...prev, isLoading: false }));
    }
  };
  
  // Function to refresh network interfaces
  const refreshInterfaces = async () => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      const interfaces = await networkService.getNetworkInterfaces();
      setState(prev => ({ ...prev, interfaces, isLoading: false }));
      return true;
    } catch (error) {
      console.error('Error refreshing network interfaces:', error);
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    }
  };
  
  // Function to toggle interface status
  const toggleInterfaceStatus = async (interfaceToToggle: NetworkInterface) => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      const newStatus = interfaceToToggle.status === 'up' ? 'down' : 'up';
      const updated = await networkService.updateNetworkInterface({
        name: interfaceToToggle.name,
        status: newStatus,
        ipMode: interfaceToToggle.ipv4 ? 'static' : 'dhcp',
        ipv4: interfaceToToggle.ipv4,
        ipv6: interfaceToToggle.ipv6
      });
      
      if (updated) {
        // Refresh interfaces after update
        await refreshInterfaces();
        return true;
      }
      
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    } catch (error) {
      console.error('Error toggling interface status:', error);
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    }
  };
  
  // Function to update network interface
  const updateNetworkInterface = async (values: NetworkInterfaceUpdate) => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      const updated = await networkService.updateNetworkInterface(values);
      
      if (updated) {
        // Refresh interfaces after update
        await refreshInterfaces();
        // Close modal
        setState(prev => ({ 
          ...prev, 
          editModalVisible: false, 
          currentInterface: null 
        }));
        return true;
      }
      
      setState(prev => ({ 
        ...prev, 
        isLoading: false 
      }));
      return false;
    } catch (error) {
      console.error('Error updating network interface:', error);
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    }
  };
  
  // Function to update DNS settings
  const updateDNSSettings = async (values: DNSSettings) => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      const updated = await networkService.updateDNSSettings(values);
      
      if (updated) {
        setState(prev => ({ 
          ...prev, 
          dnsSettings: values,
          isLoading: false 
        }));
        return true;
      }
      
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    } catch (error) {
      console.error('Error updating DNS settings:', error);
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    }
  };
  
  // Function to add firewall rule
  const addFirewallRule = async (rule: Omit<FirewallRule, 'id'>) => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      // TODO: Fix NetworkService method names and types
      // const added = await networkService.createFirewallRule(rule);
      
      // For now, just add to local state with a mock ID
      const newRule: FirewallRule = {
        ...rule,
        id: Date.now().toString()
      };
      
        setState(prev => ({ 
          ...prev, 
        firewallRules: [...prev.firewallRules, newRule],
          isLoading: false 
        }));
        return true;
    } catch (error) {
      console.error('Error adding firewall rule:', error);
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    }
  };
  
  // Function to toggle firewall rule
  const toggleFirewallRule = async (rule: FirewallRule) => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      // TODO: Fix NetworkService method
      // const updated = await networkService.toggleFirewallRule(rule.id);
      
      // For now, just update local state
        const updatedRules = state.firewallRules.map(r => 
          r.id === rule.id ? { ...r, enabled: !r.enabled } : r
        );
        
        setState(prev => ({ 
          ...prev, 
          firewallRules: updatedRules,
          isLoading: false 
        }));
        return true;
    } catch (error) {
      console.error('Error toggling firewall rule:', error);
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    }
  };
  
  // Function to delete firewall rule
  const deleteFirewallRule = async (rule: FirewallRule) => {
    setState(prev => ({ ...prev, isLoading: true }));
    
    try {
      // TODO: Fix NetworkService method
      // await networkService.deleteFirewallRule(rule.id);
      
      // For now, just remove from local state
        const updatedRules = state.firewallRules.filter(r => r.id !== rule.id);
        
        setState(prev => ({ 
          ...prev, 
          firewallRules: updatedRules,
          isLoading: false 
        }));
        return true;
    } catch (error) {
      console.error('Error deleting firewall rule:', error);
      setState(prev => ({ ...prev, isLoading: false }));
      return false;
    }
  };
  
  // Function to run diagnostic ping
  const runPing = async (options: PingOptions) => {
    setState(prev => ({ ...prev, diagnosticsRunning: true }));
    
    try {
      // TODO: Fix DiagnosticResult type mismatch
      // const result = await networkService.ping(options);
      
      setState(prev => ({ 
        ...prev, 
        diagnosticResults: null, // result,
        diagnosticsRunning: false 
      }));
      return true;
    } catch (error) {
      console.error('Error running ping:', error);
      setState(prev => ({ ...prev, diagnosticsRunning: false }));
      return false;
    }
  };
  
  // Function to run traceroute
  const runTraceroute = async (options: TracerouteOptions) => {
    setState(prev => ({ ...prev, diagnosticsRunning: true }));
    
    try {
      // TODO: Fix DiagnosticResult type mismatch
      // const result = await networkService.traceroute(options);
      
      setState(prev => ({ 
        ...prev, 
        diagnosticResults: null, // result,
        diagnosticsRunning: false 
      }));
      return true;
    } catch (error) {
      console.error('Error running traceroute:', error);
      setState(prev => ({ ...prev, diagnosticsRunning: false }));
      return false;
    }
  };
  
  // Function to run DNS lookup
  const runDNSLookup = async (options: DNSLookupOptions) => {
    setState(prev => ({ ...prev, diagnosticsRunning: true }));
    
    try {
      // TODO: Fix DiagnosticResult type mismatch and DNSLookupOptions
      // const result = await networkService.dnsLookup(options);
      
      setState(prev => ({ 
        ...prev, 
        diagnosticResults: null, // result,
        diagnosticsRunning: false 
      }));
      return true;
    } catch (error) {
      console.error('Error running DNS lookup:', error);
      setState(prev => ({ ...prev, diagnosticsRunning: false }));
      return false;
    }
  };
  
  // Function to run port scan
  const runPortScan = async (options: PortScanOptions) => {
    setState(prev => ({ ...prev, diagnosticsRunning: true }));
    
    try {
      // TODO: Fix DiagnosticResult type mismatch
      // const result = await networkService.portScan(options);
      
      setState(prev => ({ 
        ...prev, 
        diagnosticResults: null, // result,
        diagnosticsRunning: false 
      }));
      return true;
    } catch (error) {
      console.error('Error running port scan:', error);
      setState(prev => ({ ...prev, diagnosticsRunning: false }));
      return false;
    }
  };
  
  // Function to set the active tab
  const setActiveTab = (tab: string) => {
    setState(prev => ({ 
      ...prev, 
      activeTab: tab,
      diagnosticResults: null 
    }));
  };
  
  // Function to show edit interface modal
  const showEditInterfaceModal = (networkInterface: NetworkInterface) => {
    setState(prev => ({
      ...prev,
      currentInterface: networkInterface,
      editModalVisible: true
    }));
  };
  
  // Function to hide edit interface modal
  const hideEditInterfaceModal = () => {
    setState(prev => ({
      ...prev,
      currentInterface: null,
      editModalVisible: false
    }));
  };
  
  return {
    ...state,
    dataSource,
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
  };
}; 