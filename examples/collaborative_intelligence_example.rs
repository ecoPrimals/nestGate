#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! # 🤝 Collaborative Intelligence Example
//!
//! Demonstrates NestGate's template storage and audit trail capabilities
//! for AI-human collaboration in graph editing and deployment.

use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// Send JSON-RPC request to NestGate
async fn send_jsonrpc(
    stream: &mut UnixStream,
    method: &str,
    params: serde_json::Value,
    id: i32,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": id
    });

    // Write request
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    let request_json = serde_json::to_string(&request)?;
    writer.write_all(request_json.as_bytes()).await?;
    writer.write_all(b"\n").await?;

    // Read response
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await?;
    let response: serde_json::Value = serde_json::from_str(&response_line)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤝 Collaborative Intelligence Example\n");

    // Connect to NestGate
    let family_id =
        std::env::var("NESTGATE_FAMILY_ID").unwrap_or_else(|_| "example_ci".to_string());
    let uid = nestgate_core::platform::get_current_uid();
    let socket_path = format!("/run/user/{}/nestgate-{}.sock", uid, family_id);

    println!("📡 Connecting to: {}", socket_path);
    let mut stream = UnixStream::connect(&socket_path).await?;
    println!("✅ Connected!\n");

    // ========================================================================
    // PART 1: Template Storage
    // ========================================================================

    println!("📝 Part 1: Storing a Graph Template");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let template_params = json!({
        "name": "FastAPI CRUD Service",
        "description": "Production-ready REST API with PostgreSQL and Redis",
        "graph_data": {
            "nodes": [
                {
                    "id": "api",
                    "type": "fastapi",
                    "config": {
                        "port": 8000,
                        "workers": 4
                    }
                },
                {
                    "id": "db",
                    "type": "postgres",
                    "config": {
                        "host": "localhost",
                        "pool_size": 20
                    }
                },
                {
                    "id": "cache",
                    "type": "redis",
                    "config": {
                        "host": "localhost",
                        "ttl": 300
                    }
                }
            ],
            "edges": [
                {"from": "api", "to": "db"},
                {"from": "api", "to": "cache"}
            ]
        },
        "user_id": "developer_alice",
        "family_id": family_id,
        "metadata": {
            "tags": ["api", "rest", "database", "cache", "production"],
            "niche_type": "web_service",
            "is_community": true
        }
    });

    let response = send_jsonrpc(&mut stream, "templates.store", template_params, 1).await?;

    if let Some(result) = response.get("result") {
        let template_id = result["template_id"].as_str().unwrap();
        println!("✅ Template stored!");
        println!("   ID: {}", template_id);
        println!("   Version: {}", result["version"]);
        println!("   Created: {}\n", result["created_at"]);

        // ====================================================================
        // PART 2: Retrieve Template
        // ====================================================================

        println!("🔍 Part 2: Retrieving Template");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let retrieve_params = json!({
            "template_id": template_id,
            "family_id": family_id
        });

        let response = send_jsonrpc(&mut stream, "templates.retrieve", retrieve_params, 2).await?;

        if let Some(result) = response.get("result") {
            println!("✅ Template retrieved!");
            println!("   Name: {}", result["name"]);
            println!("   Description: {}", result["description"]);
            println!(
                "   Nodes: {}",
                result["graph_data"]["nodes"].as_array().unwrap().len()
            );
            println!("   Tags: {:?}\n", result["metadata"]["tags"]);
        }

        // ====================================================================
        // PART 3: List Templates
        // ====================================================================

        println!("📋 Part 3: Listing Templates");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let list_params = json!({
            "family_id": family_id,
            "tags": ["api"]
        });

        let response = send_jsonrpc(&mut stream, "templates.list", list_params, 3).await?;

        if let Some(result) = response.get("result") {
            let total = result["total"].as_u64().unwrap();
            println!("✅ Found {} template(s)", total);

            if let Some(templates) = result["templates"].as_array() {
                for template in templates {
                    println!("   • {} ({})", template["name"], template["id"]);
                }
            }
            println!();
        }

        // ====================================================================
        // PART 4: Execution Audit
        // ====================================================================

        println!("📊 Part 4: Storing Execution Audit");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        let audit_params = json!({
            "id": "",
            "execution_id": format!("exec_{}", uuid::Uuid::new_v4().simple()),
            "graph_id": "graph_test_123",
            "template_id": template_id,
            "user_id": "developer_alice",
            "family_id": family_id,
            "started_at": chrono::Utc::now().to_rfc3339(),
            "completed_at": chrono::Utc::now().to_rfc3339(),
            "status": "completed",
            "modifications": [
                {
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "modification_type": "add_node",
                    "node_id": "monitoring",
                    "data": {
                        "type": "prometheus",
                        "reason": "User added monitoring for production readiness"
                    }
                }
            ],
            "outcomes": [
                {
                    "node_id": "api",
                    "status": "success",
                    "started_at": chrono::Utc::now().to_rfc3339(),
                    "completed_at": chrono::Utc::now().to_rfc3339(),
                    "duration_ms": 5000,
                    "metrics": {
                        "requests_handled": 10000,
                        "avg_response_time_ms": 45
                    }
                },
                {
                    "node_id": "db",
                    "status": "success",
                    "started_at": chrono::Utc::now().to_rfc3339(),
                    "completed_at": chrono::Utc::now().to_rfc3339(),
                    "duration_ms": 2000,
                    "metrics": {
                        "queries_executed": 5000,
                        "pool_utilization": 0.75
                    }
                }
            ],
            "metadata": {
                "environment": "production",
                "deployment_type": "rolling"
            }
        });

        let response = send_jsonrpc(&mut stream, "audit.store_execution", audit_params, 4).await?;

        if let Some(result) = response.get("result") {
            println!("✅ Audit stored!");
            println!("   Audit ID: {}", result["audit_id"]);
            println!("   Stored At: {}\n", result["stored_at"]);
        }

        // ====================================================================
        // PART 5: Community Top Templates
        // ====================================================================

        println!("🏆 Part 5: Discovering Top Community Templates");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // First, make our template community-visible by storing it with high stats
        let popular_template = json!({
            "name": "Battle-Tested API",
            "description": "Production template with 100+ successful deployments",
            "graph_data": {
                "nodes": [],
                "edges": []
            },
            "user_id": "community",
            "family_id": family_id,
            "metadata": {
                "tags": ["api", "production"],
                "niche_type": "web_service",
                "usage_count": 100,
                "success_rate": 0.98,
                "is_community": true,
                "community_rating": 4.9,
                "rating_count": 50
            }
        });

        send_jsonrpc(&mut stream, "templates.store", popular_template, 5).await?;

        let top_params = json!({
            "niche_type": "web_service",
            "limit": 5,
            "min_usage": 0
        });

        let response = send_jsonrpc(&mut stream, "templates.community_top", top_params, 6).await?;

        if let Some(result) = response.get("result") {
            if let Some(templates) = result["templates"].as_array() {
                println!("✅ Found {} top template(s):", templates.len());
                for (i, template) in templates.iter().enumerate() {
                    println!(
                        "   {}. {} (Score: {:.2})",
                        i + 1,
                        template["name"],
                        template["score"].as_f64().unwrap_or(0.0)
                    );
                    println!(
                        "      Usage: {}, Success Rate: {:.0}%",
                        template["usage_count"],
                        template["success_rate"].as_f64().unwrap_or(0.0) * 100.0
                    );
                }
            }
            println!();
        }
    }

    println!("✅ Collaborative Intelligence Demo Complete!");
    println!("\n💡 Key Takeaways:");
    println!("   • Templates enable quick bootstrapping of new deployments");
    println!("   • Community sharing accelerates learning across teams");
    println!("   • Audit trails capture human modifications for AI learning");
    println!("   • Smart ranking surfaces the most successful patterns");

    Ok(())
}
