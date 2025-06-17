# Network Configuration Component

## Overview

The Network Configuration component provides a comprehensive interface for managing network settings in the NestGate system. It includes tools for configuring interfaces, DNS settings, firewall rules, and network diagnostics.

## Features

### Network Interfaces Management
- View all network interfaces with their status and details
- Enable/disable network interfaces
- Configure interface settings (DHCP/Static IP)
- Update IP addresses, subnet masks, and gateway settings

### DNS Settings
- Configure primary and secondary DNS servers
- Update DNS settings with validation

### Firewall Rules Management
- View all firewall rules with their status
- Add new firewall rules with protocol, port, source, and destination settings
- Enable/disable existing rules
- Delete firewall rules when no longer needed

### Network Diagnostics Tools
- **Ping**: Test connectivity to hosts with configurable packet count and timeout
- **Traceroute**: Track network path to destination with configurable max hops
- **DNS Lookup**: Resolve domain names using various record types (A, AAAA, MX, etc.)
- **Port Scan**: Check which ports are open on a target host

## Usage

### Network Diagnostics

The Network Diagnostics tools provide essential capabilities for troubleshooting network issues:

#### Ping Tool
Ping tests basic connectivity to a host by sending ICMP echo requests and waiting for replies. It helps verify if a host is reachable and measures round-trip time.

Options:
- Target Host (required): The hostname or IP address to ping
- Packet Count: Number of ping packets to send (default: 4)
- Timeout: Time to wait for each response in seconds (default: 1)
- Network Interface: Specific interface to use for the ping (optional)

#### Traceroute Tool
Traceroute shows the path packets take to reach a destination and measures transit delays at each hop. It helps identify where network issues might be occurring along a connection path.

Options:
- Target Host (required): The hostname or IP address to trace
- Max Hops: Maximum number of hops to trace (default: 30)
- Timeout: Time to wait for each hop response in seconds (default: 2)
- Network Interface: Specific interface to use for the traceroute (optional)

#### DNS Lookup Tool
DNS lookup queries DNS servers to resolve domain names and retrieve various DNS record types. It helps troubleshoot DNS resolution issues.

Options:
- Domain Name (required): The domain to look up
- Record Type: Type of DNS record to query (A, AAAA, MX, NS, TXT, CNAME)
- DNS Server: Specific DNS server to query (optional)

#### Port Scan Tool
Port scan checks which ports are open, closed, or filtered on a target host. It helps verify firewall configurations and service availability.

Options:
- Target Host (required): The hostname or IP address to scan
- Ports (required): Ports to scan (single port, range, or comma-separated list)
- Timeout: Time to wait for each port response in seconds (default: 2)

## API

The component uses the NetworkService for all backend communication, which provides the following methods:

- getNetworkInterfaces()
- updateNetworkInterface(interfaceUpdate)
- getDNSSettings()
- updateDNSSettings(dnsSettings)
- getFirewallRules()
- createFirewallRule(rule)
- updateFirewallRule(rule)
- deleteFirewallRule(ruleId)
- ping(options)
- traceroute(options)
- dnsLookup(options)
- portScan(options)

## Best Practices

1. Network interfaces should only be disabled if you understand the consequences (may lose connectivity)
2. Use proper CIDR notation for IP addresses and subnets
3. Exercise caution when using port scan tools - only scan hosts you have permission to scan
4. Firewall rules are processed in order, from top to bottom
5. DNS settings apply system-wide and may affect all network communications 