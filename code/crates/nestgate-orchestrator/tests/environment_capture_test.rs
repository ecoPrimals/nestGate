/*!
 * Environment Variable Capture Test
 * 
 * This test specifically captures and validates environment variable injection
 * and process output to detect issues with service startup.
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

/// Test: Capture and validate environment variables are actually injected
#[tokio::test]
async fn test_api_service_environment_validation() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    // Create a test file to capture environment output
    let output_file = "/tmp/nestgate_test_api_env.txt";
    std::fs::write(output_file, "").expect("Failed to create test file");
    
    // Create an API service that captures environment variables
    let definition = create_test_service_definition(
        "test-api-env-capture",
        "Test API Environment Capture",
        ServiceType::API,
        &format!("echo 'API_PORT='$API_PORT >> {} && echo 'PORT='$PORT >> {} && echo 'Test complete' >> {}", 
                output_file, output_file, output_file)
    );
    
    let instance = create_test_service_instance(definition, Some(3051));
    
    println!("Starting API service to capture environment variables...");
    let pid = manager.start_service(&instance).await?;
    assert!(pid > 0, "Should get a valid PID");
    
    // Wait for the command to complete
    sleep(Duration::from_millis(1000)).await;
    
    // Read the captured output
    let output = std::fs::read_to_string(output_file).expect("Failed to read test file");
    println!("Captured environment output:\n{}", output);
    
    // Validate that environment variables were set
    assert!(output.contains("API_PORT=3051"), "API_PORT should be set to 3051");
    assert!(output.contains("PORT=3051"), "PORT should be set to 3051");
    assert!(output.contains("Test complete"), "Command should have completed");
    
    // Clean up
    std::fs::remove_file(output_file).ok();
    
    Ok(())
}

/// Test: Validate WebSocket service environment variables
#[tokio::test]
async fn test_websocket_service_environment_validation() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    let output_file = "/tmp/nestgate_test_ws_env.txt";
    std::fs::write(output_file, "").expect("Failed to create test file");
    
    let definition = create_test_service_definition(
        "test-ws-env-capture",
        "Test WebSocket Environment Capture",
        ServiceType::WebSocket,
        &format!("echo 'WEBSOCKET_PORT='$WEBSOCKET_PORT >> {} && echo 'SERVER_PORT='$SERVER_PORT >> {} && echo 'PORT='$PORT >> {} && echo 'Test complete' >> {}", 
                output_file, output_file, output_file, output_file)
    );
    
    let instance = create_test_service_instance(definition, Some(3101));
    
    println!("Starting WebSocket service to capture environment variables...");
    let pid = manager.start_service(&instance).await?;
    assert!(pid > 0, "Should get a valid PID");
    
    sleep(Duration::from_millis(1000)).await;
    
    let output = std::fs::read_to_string(output_file).expect("Failed to read test file");
    println!("Captured WebSocket environment output:\n{}", output);
    
    // Validate WebSocket-specific environment variables
    assert!(output.contains("WEBSOCKET_PORT=3101"), "WEBSOCKET_PORT should be set to 3101");
    assert!(output.contains("SERVER_PORT=3101"), "SERVER_PORT should be set to 3101");
    assert!(output.contains("PORT=3101"), "PORT should be set to 3101");
    assert!(output.contains("Test complete"), "Command should have completed");
    
    std::fs::remove_file(output_file).ok();
    
    Ok(())
}

/// Test: Validate UI service environment variables
#[tokio::test]
async fn test_ui_service_environment_validation() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    let output_file = "/tmp/nestgate_test_ui_env.txt";
    std::fs::write(output_file, "").expect("Failed to create test file");
    
    let definition = create_test_service_definition(
        "test-ui-env-capture",
        "Test UI Environment Capture",
        ServiceType::UI,
        &format!("echo 'UI_PORT='$UI_PORT >> {} && echo 'REACT_APP_PORT='$REACT_APP_PORT >> {} && echo 'PORT='$PORT >> {} && echo 'Test complete' >> {}", 
                output_file, output_file, output_file, output_file)
    );
    
    let instance = create_test_service_instance(definition, Some(3000));
    
    println!("Starting UI service to capture environment variables...");
    let pid = manager.start_service(&instance).await?;
    assert!(pid > 0, "Should get a valid PID");
    
    sleep(Duration::from_millis(1000)).await;
    
    let output = std::fs::read_to_string(output_file).expect("Failed to read test file");
    println!("Captured UI environment output:\n{}", output);
    
    // Validate UI-specific environment variables
    assert!(output.contains("UI_PORT=3000"), "UI_PORT should be set to 3000");
    assert!(output.contains("REACT_APP_PORT=3000"), "REACT_APP_PORT should be set to 3000");
    assert!(output.contains("PORT=3000"), "PORT should be set to 3000");
    assert!(output.contains("Test complete"), "Command should have completed");
    
    std::fs::remove_file(output_file).ok();
    
    Ok(())
}

/// Test: Reproduce the exact Node.js startup scenario
#[tokio::test]
async fn test_nodejs_api_server_reproduction() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    let output_file = "/tmp/nestgate_test_nodejs_api.txt";
    std::fs::write(output_file, "").expect("Failed to create test file");
    
    // Create a Node.js command that mimics the API server startup
    let nodejs_command = format!(
        r#"node -e "
        const fs = require('fs');
        const output = '{}';
        
        console.log('=== Node.js API Server Test ===');
        fs.appendFileSync(output, 'Node.js version: ' + process.version + '\n');
        fs.appendFileSync(output, 'API_PORT from env: ' + process.env.API_PORT + '\n');
        fs.appendFileSync(output, 'PORT from env: ' + process.env.PORT + '\n');
        
        if (!process.env.API_PORT) {{
            fs.appendFileSync(output, 'ERROR: API_PORT not set\n');
            process.exit(1);
        }}
        
        const port = parseInt(process.env.API_PORT, 10);
        if (isNaN(port)) {{
            fs.appendFileSync(output, 'ERROR: Invalid API_PORT value\n');
            process.exit(1);
        }}
        
        fs.appendFileSync(output, 'Port validation passed: ' + port + '\n');
        
        // Try to create HTTP server
        const http = require('http');
        const server = http.createServer((req, res) => {{
            res.end('OK');
        }});
        
        server.on('error', (err) => {{
            fs.appendFileSync(output, 'Server error: ' + err.message + '\n');
            process.exit(1);
        }});
        
        server.listen(port, '127.0.0.1', () => {{
            fs.appendFileSync(output, 'Server listening on port ' + port + '\n');
            fs.appendFileSync(output, 'Test completed successfully\n');
            server.close();
            process.exit(0);
        }});
        ""#,
        output_file
    );
    
    let definition = create_test_service_definition(
        "test-nodejs-api-reproduction",
        "Test Node.js API Reproduction",
        ServiceType::API,
        &nodejs_command
    );
    
    let instance = create_test_service_instance(definition, Some(3052));
    
    println!("Starting Node.js API server reproduction test...");
    let pid = manager.start_service(&instance).await?;
    assert!(pid > 0, "Should get a valid PID");
    
    // Wait longer for Node.js to start and complete
    sleep(Duration::from_millis(2000)).await;
    
    let output = std::fs::read_to_string(output_file).expect("Failed to read test file");
    println!("Node.js API server output:\n{}", output);
    
    // Validate the Node.js service ran correctly
    assert!(output.contains("Node.js version:"), "Should show Node.js version");
    assert!(output.contains("API_PORT from env: 3052"), "Should have API_PORT set");
    assert!(output.contains("PORT from env: 3052"), "Should have PORT set");
    assert!(output.contains("Port validation passed: 3052"), "Should pass port validation");
    
    // Check if server started successfully or if there was an error
    if output.contains("ERROR:") {
        panic!("Node.js service encountered an error: {}", output);
    }
    
    // This test should reveal if the server actually starts or becomes defunct
    if !output.contains("Test completed successfully") {
        println!("WARNING: Node.js service did not complete successfully");
        println!("This may indicate the process became defunct or crashed");
        
        // Let's check the process status
        let process_info = manager.get_process_info(pid);
        if process_info.is_some() {
            println!("Process is still tracked by manager");
        } else {
            println!("Process is no longer tracked (may have become defunct)");
        }
    }
    
    std::fs::remove_file(output_file).ok();
    
    Ok(())
}

/// Test: Check if commands are executed correctly with shell wrapper
#[tokio::test]
async fn test_shell_command_execution() -> Result<()> {
    let manager = ProcessManager::new();
    manager.initialize().await?;
    
    let output_file = "/tmp/nestgate_test_shell.txt";
    std::fs::write(output_file, "").expect("Failed to create test file");
    
    // Test that shell command execution works correctly
    let definition = create_test_service_definition(
        "test-shell-execution",
        "Test Shell Execution",
        ServiceType::Other("test".to_string()),
        &format!("echo 'Shell execution works' > {} && echo 'PWD:'$(pwd) >> {} && echo 'USER:'$USER >> {} && echo 'SHELL:'$SHELL >> {}", 
                output_file, output_file, output_file, output_file)
    );
    
    let instance = create_test_service_instance(definition, None);
    
    println!("Testing shell command execution...");
    let pid = manager.start_service(&instance).await?;
    assert!(pid > 0, "Should get a valid PID");
    
    sleep(Duration::from_millis(1000)).await;
    
    let output = std::fs::read_to_string(output_file).expect("Failed to read test file");
    println!("Shell execution output:\n{}", output);
    
    assert!(output.contains("Shell execution works"), "Shell command should execute");
    assert!(output.contains("PWD:"), "Should capture working directory");
    
    std::fs::remove_file(output_file).ok();
    
    Ok(())
} 