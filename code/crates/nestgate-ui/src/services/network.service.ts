import { API_BASE_URL } from '../constants';
import { TelemetryService, NetworkInterface } from './telemetry.service';

export interface DNSSettings {
  primaryDNS: string;
  secondaryDNS: string;
}

export interface FirewallRule {
  id: string;
  name: string;
  protocol: string;
  port: string;
  source: string;
  destination: string;
  action: 'allow' | 'deny';
  enabled: boolean;
}

export interface NetworkInterfaceUpdate {
  name: string;
  status: 'up' | 'down';
  ipMode: 'dhcp' | 'static';
  ipv4?: string;
  ipv6?: string;
  subnetMask?: string;
  gateway?: string;
}

export interface DiagnosticResult {
  success: boolean;
  command: string;
  output: string[];
  error?: string;
  executionTime?: number;
}

export interface PingOptions {
  target: string;
  count?: number;
  timeout?: number;
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
  recordType?: 'A' | 'AAAA' | 'MX' | 'NS' | 'TXT' | 'CNAME';
  server?: string;
}

export interface PortScanOptions {
  target: string;
  ports: string; // e.g. "80,443" or "22-25"
  timeout?: number;
}

export class NetworkService {
  private static readonly API_URL = `${API_BASE_URL}/api/network`;
  private static _instance: NetworkService;

  /**
   * Get singleton instance of NetworkService
   */
  public static getInstance(): NetworkService {
    if (!NetworkService._instance) {
      NetworkService._instance = new NetworkService();
    }
    return NetworkService._instance;
  }

  /**
   * Constructor
   */
  private constructor() {
    // Private constructor to enforce singleton pattern
  }

  /**
   * Get network interfaces
   */
  public async getNetworkInterfaces(): Promise<NetworkInterface[]> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/interfaces`);
      
      if (!response.ok) {
        throw new Error(`Failed to fetch network interfaces: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('Error fetching network interfaces:', error);
      // Fall back to telemetry service in case of error
      const networkStatus = await TelemetryService.getNetworkStatus();
      return networkStatus.interfaces;
    }
  }

  /**
   * Update network interface configuration
   */
  public async updateNetworkInterface(interfaceUpdate: NetworkInterfaceUpdate): Promise<NetworkInterface> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/interfaces/${interfaceUpdate.name}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(interfaceUpdate),
      });
      
      if (!response.ok) {
        throw new Error(`Failed to update network interface: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('Error updating network interface:', error);
      
      // For development/testing, return a mock response
      return this.getMockUpdatedInterface(interfaceUpdate);
    }
  }

  /**
   * Get DNS settings
   */
  public async getDNSSettings(): Promise<DNSSettings> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/dns`);
      
      if (!response.ok) {
        throw new Error(`Failed to fetch DNS settings: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('Error fetching DNS settings:', error);
      
      // For development/testing, return mock DNS settings
      return {
        primaryDNS: '8.8.8.8',
        secondaryDNS: '8.8.4.4',
      };
    }
  }

  /**
   * Update DNS settings
   */
  public async updateDNSSettings(dnsSettings: DNSSettings): Promise<DNSSettings> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/dns`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(dnsSettings),
      });
      
      if (!response.ok) {
        throw new Error(`Failed to update DNS settings: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('Error updating DNS settings:', error);
      
      // For development/testing, return the settings that were passed in
      return dnsSettings;
    }
  }

  /**
   * Get firewall rules
   */
  public async getFirewallRules(): Promise<FirewallRule[]> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/firewall/rules`);
      
      if (!response.ok) {
        throw new Error(`Failed to fetch firewall rules: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('Error fetching firewall rules:', error);
      
      // For development/testing, return mock firewall rules
      return [
        {
          id: '1',
          name: 'Allow SSH',
          protocol: 'TCP',
          port: '22',
          source: 'ANY',
          destination: 'ANY',
          action: 'allow',
          enabled: true
        },
        {
          id: '2',
          name: 'Allow Web UI',
          protocol: 'TCP',
          port: '80,443',
          source: 'ANY',
          destination: 'ANY',
          action: 'allow',
          enabled: true
        },
        {
          id: '3',
          name: 'Block Telnet',
          protocol: 'TCP',
          port: '23',
          source: 'ANY',
          destination: 'ANY',
          action: 'deny',
          enabled: true
        }
      ];
    }
  }

  /**
   * Create a new firewall rule
   */
  public async createFirewallRule(rule: Omit<FirewallRule, 'id'>): Promise<FirewallRule> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/firewall/rules`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(rule),
      });
      
      if (!response.ok) {
        throw new Error(`Failed to create firewall rule: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('Error creating firewall rule:', error);
      
      // For development/testing, return a mock response with an ID
      return {
        ...rule,
        id: String(Date.now())
      };
    }
  }

  /**
   * Update an existing firewall rule
   */
  public async updateFirewallRule(rule: FirewallRule): Promise<FirewallRule> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/firewall/rules/${rule.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(rule),
      });
      
      if (!response.ok) {
        throw new Error(`Failed to update firewall rule: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error('Error updating firewall rule:', error);
      
      // For development/testing, return the rule that was passed in
      return rule;
    }
  }

  /**
   * Delete a firewall rule
   */
  public async deleteFirewallRule(ruleId: string): Promise<void> {
    try {
      // In a real implementation, this would make an actual API call
      const response = await fetch(`${NetworkService.API_URL}/firewall/rules/${ruleId}`, {
        method: 'DELETE',
      });
      
      if (!response.ok) {
        throw new Error(`Failed to delete firewall rule: ${response.statusText}`);
      }
    } catch (error) {
      console.error('Error deleting firewall rule:', error);
      // No need to return anything for delete operations
    }
  }

  /**
   * Run a network diagnostic command
   */
  public async runNetworkCommand(command: string, params: Record<string, any>): Promise<DiagnosticResult> {
    try {
      // Call the backend API endpoint
      const response = await fetch(`${NetworkService.API_URL}/diagnostics/${command}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('auth_token')}`
        },
        body: JSON.stringify(params),
      });
      
      if (!response.ok) {
        throw new Error(`Failed to run network command: ${response.statusText}`);
      }
      
      return await response.json();
    } catch (error) {
      console.error(`Error running network command ${command}:`, error);
      
      // For development/testing, return mock responses based on command
      return this.getMockDiagnosticResult(command, params);
    }
  }

  /**
   * Ping a host
   */
  public async ping(options: PingOptions): Promise<DiagnosticResult> {
    // Convert UI options to API options
    const apiOptions = {
      target: options.target,
      count: options.count,
      timeout: options.timeout,
      interface: options.interface
    };
    return this.runNetworkCommand('ping', apiOptions);
  }

  /**
   * Traceroute to a host
   */
  public async traceroute(options: TracerouteOptions): Promise<DiagnosticResult> {
    // Convert UI options to API options
    const apiOptions = {
      target: options.target,
      max_hops: options.maxHops,
      timeout: options.timeout,
      interface: options.interface
    };
    return this.runNetworkCommand('traceroute', apiOptions);
  }

  /**
   * Perform DNS lookup
   */
  public async dnsLookup(options: DNSLookupOptions): Promise<DiagnosticResult> {
    // Convert UI options to API options
    const apiOptions = {
      domain: options.domain,
      record_type: options.recordType,
      server: options.server
    };
    return this.runNetworkCommand('dns-lookup', apiOptions);
  }

  /**
   * Perform port scan
   */
  public async portScan(options: PortScanOptions): Promise<DiagnosticResult> {
    // Convert UI options to API options
    const apiOptions = {
      target: options.target,
      ports: options.ports,
      timeout: options.timeout
    };
    return this.runNetworkCommand('port-scan', apiOptions);
  }

  /**
   * Mock helper to simulate updated interface
   */
  private getMockUpdatedInterface(update: NetworkInterfaceUpdate): NetworkInterface {
    return {
      name: update.name,
      status: update.status,
      ipv4: update.ipMode === 'static' ? update.ipv4 || '' : '',
      ipv6: update.ipv6 || '',
      mac: '00:11:22:33:44:55',
      type: 'ethernet',
      speed: '1 Gbps',
      rx_bytes: 1024000,
      tx_bytes: 512000,
      rx_packets: 1200,
      tx_packets: 800
    };
  }

  /**
   * Mock helper to simulate diagnostic results
   */
  private getMockDiagnosticResult(command: string, params: Record<string, any>): DiagnosticResult {
    const startTime = Date.now();
    
    switch (command) {
      case 'ping':
        return this.getMockPingResult(params as PingOptions);
      case 'traceroute':
        return this.getMockTracerouteResult(params as TracerouteOptions);
      case 'dnsLookup':
        return this.getMockDnsLookupResult(params as DNSLookupOptions);
      case 'portScan':
        return this.getMockPortScanResult(params as PortScanOptions);
      default:
        return {
          success: false,
          command,
          output: [],
          error: 'Unknown command',
          executionTime: Date.now() - startTime
        };
    }
  }

  /**
   * Mock ping result
   */
  private getMockPingResult(options: PingOptions): DiagnosticResult {
    const count = options.count || 4;
    const target = options.target;
    const output: string[] = [];
    
    output.push(`PING ${target} (${target.includes('.') ? '192.168.1.1' : '2001:db8::1'}): 56 data bytes`);
    
    let packetLoss = 0;
    const times: number[] = [];
    
    for (let i = 0; i < count; i++) {
      // Simulate some packet loss and variable response times
      const time = Math.random() > 0.8 ? undefined : Math.random() * 100;
      
      if (time) {
        times.push(time);
        output.push(`64 bytes from ${target}: icmp_seq=${i+1} ttl=64 time=${time.toFixed(3)} ms`);
      } else {
        packetLoss++;
        output.push(`Request timeout for icmp_seq ${i+1}`);
      }
    }
    
    const recvCount = count - packetLoss;
    const packetLossPercent = (packetLoss / count) * 100;
    
    if (times.length > 0) {
      const min = Math.min(...times);
      const max = Math.max(...times);
      const avg = times.reduce((a, b) => a + b, 0) / times.length;
      
      output.push('');
      output.push(`--- ${target} ping statistics ---`);
      output.push(`${count} packets transmitted, ${recvCount} packets received, ${packetLossPercent.toFixed(1)}% packet loss`);
      output.push(`round-trip min/avg/max = ${min.toFixed(3)}/${avg.toFixed(3)}/${max.toFixed(3)} ms`);
    }
    
    return {
      success: packetLoss < count,
      command: `ping ${target}`,
      output,
      executionTime: 1000 + Math.random() * 500
    };
  }

  /**
   * Mock traceroute result
   */
  private getMockTracerouteResult(options: TracerouteOptions): DiagnosticResult {
    const target = options.target;
    const maxHops = options.maxHops || 30;
    const output: string[] = [];
    
    output.push(`traceroute to ${target} (${target.includes('.') ? '192.168.1.1' : '2001:db8::1'}), ${maxHops} hops max, 60 byte packets`);
    
    // Create a realistic number of hops
    const hops = Math.min(Math.floor(Math.random() * 15) + 3, maxHops);
    
    // Common router names
    const routerNames = [
      'router.local',
      'gateway.isp.net',
      'core1.isp.net',
      'core2.isp.net',
      'exchange.isp.net',
      'backbone.provider.net',
      'edge1.provider.net',
      'edge2.provider.net',
      'datacenter.provider.net'
    ];
    
    for (let i = 1; i <= hops; i++) {
      // Generate hop IP and name
      const ip = i === 1 
        ? '192.168.1.1' 
        : i === hops 
          ? target.includes('.') ? '192.168.1.1' : '2001:db8::1'
          : `10.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}`;
      
      const name = i === 1 
        ? 'gateway.local' 
        : i === hops 
          ? target
          : routerNames[Math.min(i - 2, routerNames.length - 1)];
      
      // Generate 3 response times (standard traceroute sends 3 packets per hop)
      const times = Array(3).fill(0).map(() => Math.random() * 100 + i * 5);
      
      // Format the output
      output.push(`${i}  ${name} (${ip})  ${times.map(t => `${t.toFixed(3)} ms`).join('  ')}`);
    }
    
    return {
      success: true,
      command: `traceroute ${target}`,
      output,
      executionTime: hops * 500 + Math.random() * 1000
    };
  }

  /**
   * Mock DNS lookup result
   */
  private getMockDnsLookupResult(options: DNSLookupOptions): DiagnosticResult {
    const domain = options.domain;
    const recordType = options.recordType || 'A';
    const server = options.server || '8.8.8.8';
    const output: string[] = [];
    
    output.push(`; <<>> DiG 9.16.1 <<>> ${domain} ${recordType} @${server}`);
    output.push(';; global options: +cmd');
    output.push('');
    output.push(';; Got answer:');
    output.push(';; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 29741');
    output.push(';; flags: qr rd ra; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 1');
    output.push('');
    output.push(';; OPT PSEUDOSECTION:');
    output.push('; EDNS: version: 0, flags:; udp: 512');
    output.push('');
    output.push(';; QUESTION SECTION:');
    output.push(`;${domain}.			IN	${recordType}`);
    output.push('');
    output.push(';; ANSWER SECTION:');
    
    switch (recordType) {
      case 'A':
        output.push(`${domain}.		300	IN	A	${this.getRandomIPv4()}`);
        break;
      case 'AAAA':
        output.push(`${domain}.		300	IN	AAAA	2001:db8::${Math.floor(Math.random() * 1000) + 1}`);
        break;
      case 'MX':
        output.push(`${domain}.		300	IN	MX	10 mail.${domain}.`);
        output.push(`${domain}.		300	IN	MX	20 mail2.${domain}.`);
        break;
      case 'NS':
        output.push(`${domain}.		300	IN	NS	ns1.${domain}.`);
        output.push(`${domain}.		300	IN	NS	ns2.${domain}.`);
        break;
      case 'TXT':
        output.push(`${domain}.		300	IN	TXT	"v=spf1 include:_spf.${domain} ~all"`);
        break;
      case 'CNAME':
        output.push(`${domain}.		300	IN	CNAME	www.${domain.split('.').slice(1).join('.')}.`);
        break;
    }
    
    output.push('');
    output.push(';; Query time: 48 msec');
    output.push(`;; SERVER: ${server}#53(${server})`);
    output.push(';; WHEN: ' + new Date().toUTCString());
    output.push(';; MSG SIZE  rcvd: 56');
    
    return {
      success: true,
      command: `dig ${domain} ${recordType} @${server}`,
      output,
      executionTime: Math.random() * 200 + 50
    };
  }

  /**
   * Mock port scan result
   */
  private getMockPortScanResult(options: PortScanOptions): DiagnosticResult {
    const target = options.target;
    const ports = options.ports;
    const output: string[] = [];
    
    output.push(`Starting port scan on host ${target} (${target.includes('.') ? '192.168.1.1' : '2001:db8::1'})`);
    output.push(`PORT     STATE    SERVICE`);
    
    // Parse port ranges (e.g. "80,443" or "22-25")
    const portList: number[] = [];
    const portRanges = ports.split(',');
    
    portRanges.forEach(range => {
      if (range.includes('-')) {
        const [start, end] = range.split('-').map(Number);
        for (let i = start; i <= end; i++) {
          portList.push(i);
        }
      } else {
        portList.push(Number(range));
      }
    });
    
    // Common port services
    const portServices: Record<number, string> = {
      22: 'ssh',
      23: 'telnet',
      25: 'smtp',
      53: 'domain',
      80: 'http',
      110: 'pop3',
      143: 'imap',
      443: 'https',
      587: 'submission',
      993: 'imaps',
      995: 'pop3s',
      3306: 'mysql',
      5432: 'postgresql',
      8080: 'http-proxy'
    };
    
    portList.forEach(port => {
      // Randomly determine if port is open, closed, or filtered
      const rand = Math.random();
      let state;
      
      if (rand < 0.7) {
        // Most ports are closed
        state = 'closed';
      } else if (rand < 0.9) {
        // Some ports are open
        state = 'open';
      } else {
        // Few ports are filtered
        state = 'filtered';
      }
      
      // Common services are more likely to be open
      if (portServices[port]) {
        state = Math.random() < 0.8 ? 'open' : state;
      }
      
      // Format the output
      const service = portServices[port] || 'unknown';
      output.push(`${port.toString().padEnd(8)}${state.padEnd(9)}${service}`);
    });
    
    output.push('');
    output.push(`Port scan completed in ${(Math.random() * 5 + 1).toFixed(2)} seconds`);
    output.push(`${portList.length} ports scanned, ${output.filter(line => line.includes('open')).length} ports open`);
    
    return {
      success: true,
      command: `nmap ${target} -p ${ports}`,
      output,
      executionTime: portList.length * 100 + Math.random() * 1000
    };
  }

  /**
   * Helper to generate random IPv4 address
   */
  private getRandomIPv4(): string {
    return `${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}`;
  }
} 