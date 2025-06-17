import { NetworkInterface } from '../../../services/telemetry.service';

// Re-export NetworkInterface for use in network components
export type { NetworkInterface };

// Exported from the network service
export interface DNSSettings {
  primaryDNS: string;
  secondaryDNS: string;
}

export interface FirewallRule {
  id: string;
  name: string;
  enabled: boolean;
  protocol: string;
  port: string;
  source: string;
  destination: string;
  action: 'allow' | 'deny';
}

export interface NetworkInterfaceUpdate {
  name: string;
  ipMode: 'dhcp' | 'static';
  ipv4?: string;
  ipv6?: string;
  status: 'up' | 'down';
}

export interface PingOptions {
  target: string;
  count?: number;
  interval?: number;
  interface?: string;
}

export interface TracerouteOptions {
  target: string;
  maxHops?: number;
  timeout?: number;
  interface?: string;
}

export interface DNSLookupOptions {
  domain: string;
  recordType: string;
  server?: string;
}

export interface PortScanOptions {
  target: string;
  ports: string;
  timeout: number;
}

export interface DiagnosticResult {
  type: string;
  command: string;
  output: string;
  error?: string;
  details?: {
    [key: string]: any;
  };
}

export interface NetworkConfigurationState {
  interfaces: NetworkInterface[];
  isLoading: boolean;
  editModalVisible: boolean;
  currentInterface: NetworkInterface | null;
  dnsSettings: DNSSettings;
  firewallRules: FirewallRule[];
  activeTab: string;
  diagnosticsRunning: boolean;
  diagnosticResults: DiagnosticResult | null;
} 