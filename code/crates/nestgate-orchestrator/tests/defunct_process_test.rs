/*!
 * Defunct Process Test
 * 
 * This test specifically addresses the original issue where services would start
 * but immediately become defunct (zombie processes). This test validates that
 * our process management correctly handles process lifecycle and cleanup.
 */

use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;
use nestgate_port_manager::{
    ProcessManager, 
    service::{ServiceDefinition, ServiceInstance, ServiceType, ServiceStatus},
    errors::Result,
};

/// Helper function to create a test service definition
fn create_test_service_definition(
    id: &str,
    name: &str,
    service_type: ServiceType,
    startup_command: &str
) -> ServiceDefinition {
    ServiceDefinition {
        id: id.to_string(),
        name: name.to_string(),
        service_type,
        startup_command: startup_command.to_string(),
        shutdown_command: None,
        working_directory: None,
        environment: HashMap::new(),
        preferred_port: None,
        port_range: None,
        dependencies: Vec::new(),
        health_checks: Vec::new(),
        auto_restart: false,
        max_restart_attempts: 3,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

/// Helper function to create a test service instance
fn create_test_service_instance(definition: ServiceDefinition, port: Option<u16>) -> ServiceInstance {
    ServiceInstance {
        definition,
        status: ServiceStatus::Stopped,
        port,
        pid: None,
        urls: HashMap::new(),
        started_at: None,
        stopped_at: None,
        last_error: None,
        restart_count: 0,
        is_healthy: false,
        last_health_check: None,
        health_details: None,
    }
}

/// Test: Reproduce the original defunct process scenario
#[tokio::test]
async fn test_nodejs_api_server_no_defunct() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    // Create a Node.js API server that mimics the original failing scenario
    let definition = create_test_service_definition(
        "test-api-server-defunct",
        "Test API Server (Defunct Check)",
        ServiceType::API,
        r#"node -e "
        const http = require('http');
        const port = process.env.API_PORT || 3051;
        
        console.log('Starting API server on port', port);
        console.log('Environment variables:');
        console.log('  API_PORT:', process.env.API_PORT);
        console.log('  PORT:', process.env.PORT);
        
        const server = http.createServer((req, res) => {
            res.writeHead(200, { 'Content-Type': 'text/plain' });
            res.end('API Server Running');
        });
        
        server.on('error', (err) => {
            console.error('Server error:', err);
            process.exit(1);
        });
        
        server.listen(port, '127.0.0.1', () => {
            console.log('Server listening on port', port);
            // Run for 2 seconds then gracefully exit
            setTimeout(() => {
                console.log('Shutting down server');
                server.close(() => {
                    console.log('Server closed');
                    process.exit(0);
                });
            }, 2000);
        });
        ""#
    );
    
    let instance = create_test_service_instance(definition, Some(3051));
    
    println!("Starting Node.js API server to test for defunct processes...");
    let pid = manager.start_service(&instance).await?;
    assert!(pid > 0, "Should get a valid PID");
    
    // Wait for the server to start and run
    sleep(Duration::from_millis(500)).await;
    
    // Check that the process is tracked and running
    let process_info = manager.get_process_info(pid);
    assert!(process_info.is_some(), "Process should be tracked initially");
    
    // Wait for the server to complete its 2-second run
    sleep(Duration::from_secs(3)).await;
    
    // Check if the process is still running (it shouldn't be)
    let is_still_running = std::process::Command::new("ps")
        .args(["-p", &pid.to_string()])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    
    if is_still_running {
        // Check if it's a zombie/defunct process
        let ps_output = std::process::Command::new("ps")
            .args(["-o", "pid,stat,comm", "-p", &pid.to_string()])
            .output()
            .expect("Failed to run ps command");
        
        let output_str = String::from_utf8_lossy(&ps_output.stdout);
        println!("Process status: {}", output_str);
        
        // Check for zombie status (Z or <defunct>)
        if output_str.contains("Z") || output_str.contains("defunct") {
            panic!("Process {} became a zombie/defunct process! This is the bug we're trying to fix.", pid);
        }
    }
    
    println!("✓ Node.js API server completed without becoming defunct");
    
    Ok(())
}

/// Test: WebSocket server scenario
#[tokio::test]
async fn test_websocket_server_no_defunct() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    // Create a WebSocket server that mimics the original failing scenario
    let definition = create_test_service_definition(
        "test-websocket-server-defunct",
        "Test WebSocket Server (Defunct Check)",
        ServiceType::WebSocket,
        r#"node -e "
        const http = require('http');
        const port = process.env.WEBSOCKET_PORT || 3101;
        
        console.log('Starting WebSocket server on port', port);
        console.log('Environment variables:');
        console.log('  WEBSOCKET_PORT:', process.env.WEBSOCKET_PORT);
        console.log('  SERVER_PORT:', process.env.SERVER_PORT);
        console.log('  PORT:', process.env.PORT);
        
        const server = http.createServer();
        
        server.on('error', (err) => {
            console.error('Server error:', err);
            process.exit(1);
        });
        
        server.listen(port, '127.0.0.1', () => {
            console.log('WebSocket server listening on port', port);
            // Run for 2 seconds then gracefully exit
            setTimeout(() => {
                console.log('Shutting down WebSocket server');
                server.close(() => {
                    console.log('WebSocket server closed');
                    process.exit(0);
                });
            }, 2000);
        });
        ""#
    );
    
    let instance = create_test_service_instance(definition, Some(3101));
    
    println!("Starting WebSocket server to test for defunct processes...");
    let pid = manager.start_service(&instance).await?;
    assert!(pid > 0, "Should get a valid PID");
    
    // Wait for the server to start and run
    sleep(Duration::from_millis(500)).await;
    
    // Check that the process is tracked and running
    let process_info = manager.get_process_info(pid);
    assert!(process_info.is_some(), "Process should be tracked initially");
    
    // Wait for the server to complete its 2-second run
    sleep(Duration::from_secs(3)).await;
    
    // Check if the process is still running (it shouldn't be)
    let is_still_running = std::process::Command::new("ps")
        .args(["-p", &pid.to_string()])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false);
    
    if is_still_running {
        // Check if it's a zombie/defunct process
        let ps_output = std::process::Command::new("ps")
            .args(["-o", "pid,stat,comm", "-p", &pid.to_string()])
            .output()
            .expect("Failed to run ps command");
        
        let output_str = String::from_utf8_lossy(&ps_output.stdout);
        println!("Process status: {}", output_str);
        
        // Check for zombie status (Z or <defunct>)
        if output_str.contains("Z") || output_str.contains("defunct") {
            panic!("Process {} became a zombie/defunct process! This is the bug we're trying to fix.", pid);
        }
    }
    
    println!("✓ WebSocket server completed without becoming defunct");
    
    Ok(())
}

/// Test: Multiple concurrent services to stress test the process management
#[tokio::test]
async fn test_multiple_services_no_defunct() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    let mut handles = vec![];
    let mut pids = vec![];
    
    // Start multiple services concurrently
    for i in 0..3 {
        let service_type = match i {
            0 => ServiceType::API,
            1 => ServiceType::WebSocket,
            _ => ServiceType::UI,
        };
        
        let port = 3050 + i as u16;
        
        let definition = create_test_service_definition(
            &format!("test-multi-service-{}", i),
            &format!("Test Multi Service {}", i),
            service_type,
            &format!(r#"node -e "
            const port = process.env.PORT || {};
            console.log('Service {} starting on port', port);
            setTimeout(() => {{
                console.log('Service {} exiting');
                process.exit(0);
            }}, 1500);
            ""#, port, i, i)
        );
        
        let instance = create_test_service_instance(definition, Some(port));
        
        let manager_clone = manager.clone();
        let handle = tokio::spawn(async move {
            manager_clone.start_service(&instance).await
        });
        handles.push(handle);
    }
    
    // Wait for all services to start
    for handle in handles {
        let result = handle.await.unwrap()?;
        pids.push(result);
    }
    
    println!("Started {} services with PIDs: {:?}", pids.len(), pids);
    
    // Wait for all services to complete
    sleep(Duration::from_secs(3)).await;
    
    // Check for any defunct processes
    for pid in &pids {
        let ps_output = std::process::Command::new("ps")
            .args(["-o", "pid,stat,comm", "-p", &pid.to_string()])
            .output();
        
        if let Ok(output) = ps_output {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("Z") || output_str.contains("defunct") {
                    panic!("Process {} became a zombie/defunct process!", pid);
                }
            }
        }
    }
    
    println!("✓ All {} services completed without becoming defunct", pids.len());
    
    Ok(())
} 