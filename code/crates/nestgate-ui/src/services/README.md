# NestGate UI Services

This directory contains service classes that handle API communication between the NestGate UI and backend services.

## NetworkService

The NetworkService provides methods for managing network interfaces, DNS settings, firewall rules, and network diagnostics.

### Methods

#### Network Interfaces
- `getNetworkInterfaces()`: Retrieves all network interfaces with their status and configuration
- `updateNetworkInterface(interface)`: Updates a network interface's configuration (enable/disable, IP settings)

#### DNS Settings
- `getDNSSettings()`: Retrieves the current DNS server configuration
- `updateDNSSettings(settings)`: Updates the DNS server configuration

#### Firewall Rules
- `getFirewallRules()`: Retrieves all firewall rules
- `createFirewallRule(rule)`: Creates a new firewall rule
- `updateFirewallRule(rule)`: Updates an existing firewall rule
- `deleteFirewallRule(ruleId)`: Deletes a firewall rule

#### Network Diagnostics
- `ping(options)`: Executes a ping command to test connectivity
  - Parameters:
    - `host` (string): Target hostname or IP address
    - `count` (number, optional): Number of packets to send (default: 4)
    - `timeout` (number, optional): Timeout in seconds (default: 1)
    - `interface` (string, optional): Network interface to use
  - Returns: Promise with ping results (success, command, output, execution time)

- `traceroute(options)`: Executes a traceroute to show network path
  - Parameters:
    - `host` (string): Target hostname or IP address
    - `maxHops` (number, optional): Maximum number of hops (default: 30)
    - `timeout` (number, optional): Timeout in seconds (default: 2)
    - `interface` (string, optional): Network interface to use
  - Returns: Promise with traceroute results (success, command, output, execution time)

- `dnsLookup(options)`: Performs DNS lookup for a domain
  - Parameters:
    - `domain` (string): Domain name to look up
    - `recordType` (string, optional): DNS record type (A, AAAA, MX, etc.)
    - `server` (string, optional): DNS server to query
  - Returns: Promise with DNS lookup results (success, command, output, execution time)

- `portScan(options)`: Scans for open ports on a target host
  - Parameters:
    - `host` (string): Target hostname or IP address
    - `ports` (string): Ports to scan (e.g., "80,443" or "1000-2000")
    - `timeout` (number, optional): Timeout in seconds (default: 2)
  - Returns: Promise with port scan results (success, command, output, execution time)

## Usage Example

```typescript
import { NetworkService } from '../services/NetworkService';

// Create service instance
const networkService = new NetworkService();

// Get all network interfaces
networkService.getNetworkInterfaces()
  .then(interfaces => {
    console.log('Network interfaces:', interfaces);
  })
  .catch(error => {
    console.error('Error fetching interfaces:', error);
  });

// Ping a host
networkService.ping({ host: 'example.com', count: 5 })
  .then(result => {
    console.log('Ping successful:', result.success);
    console.log('Command executed:', result.command);
    console.log('Output:', result.output);
    console.log('Execution time:', result.executionTime);
  })
  .catch(error => {
    console.error('Ping error:', error);
  });
```

## Error Handling

All service methods return Promises that can be caught using standard Promise error handling.
Service methods include proper error handling to ensure API errors are correctly formatted and returned to the calling component.

## Security Considerations

- Network diagnostic tools should be used responsibly
- Port scanning should only be performed on authorized systems
- The API ensures proper permission checks before executing commands 