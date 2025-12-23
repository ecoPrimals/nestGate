#!/usr/bin/env bash
# Demo 2.4: Data Flow Patterns
# Shows common integration patterns across the ecosystem

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

log_step() {
    echo -e "\n${BLUE}==>${NC} ${1}"
}

log_success() {
    echo -e "${GREEN}✓${NC} ${1}"
}

log_info() {
    echo -e "${CYAN}→${NC} ${1}"
}

log_feature() {
    echo -e "${MAGENTA}🔄${NC} ${1}"
}

pattern1_producer_consumer() {
    log_step "Pattern 1: Producer/Consumer"
    
    log_feature "Decoupled data generation and consumption"
    
    echo ""
    log_info "Example: Log Aggregation System"
    
    cat <<'EOF'

Producer/Consumer Pattern
════════════════════════════════════════════════════════

Setup:
  Producer: Web application generating logs
  Buffer: NestGate storage
  Consumer: Log analyzer processing logs
  
Flow:
  Web App → NestGate → Log Analyzer → Metrics/Alerts

────────────────────────────────────────────────────────

Time: 14:30:00 | Producer: Writing logs...
  ✓ 1,247 log entries written to nestgate://logs/app.log
  ✓ Buffered in NestGate (consumer offline)

Time: 14:30:30 | Producer: Writing logs...
  ✓ 1,398 log entries written
  ✓ Total buffered: 2,645 entries

Time: 14:31:00 | Consumer: Starting...
  → Connected to NestGate
  → Reading buffered logs (2,645 entries)
  → Processing at 500 entries/sec
  
Time: 14:31:05 | Consumer: Processing...
  ✓ Parsed 2,645 log entries
  ✓ Extracted 45 errors
  ✓ Extracted 234 warnings
  ✓ Published metrics to Prometheus
  ✓ Sent 3 alerts for critical errors

────────────────────────────────────────────────────────

Benefits:
  ✓ Decoupling: Producer/consumer run independently
  ✓ Buffering: Data preserved during consumer downtime
  ✓ Scalability: Multiple consumers can read same data
  ✓ Reliability: No data loss even if consumer fails

Use Cases:
  • Log aggregation
  • Metrics collection
  • Event processing
  • Data pipelines

════════════════════════════════════════════════════════

EOF
    
    log_success "Producer/Consumer pattern demonstrated!"
}

pattern2_request_response() {
    log_step "Pattern 2: Request/Response"
    
    log_feature "Synchronous data exchange"
    
    echo ""
    log_info "Example: ML Model Serving"
    
    cat <<'EOF'

Request/Response Pattern
════════════════════════════════════════════════════════

Setup:
  Client: Sends inference request
  Storage: NestGate (stores models)
  Compute: ToadStool (runs inference)
  
Flow:
  Client → Request → Load Model → Inference → Response

────────────────────────────────────────────────────────

Request Received:
  Timestamp: 2024-12-17T14:32:15.123Z
  Model: resnet50.pth
  Input: image_cat_001.jpg
  Request ID: req_abc123

Processing:
  [0.000s] Step 1: Load model from NestGate
            Path: nestgate://models/resnet50.pth
            Size: 98 MB
            Status: ✓ Loaded (12ms)
            
  [0.012s] Step 2: Load input image
            Path: image_cat_001.jpg
            Size: 2.3 MB
            Status: ✓ Loaded (3ms)
            
  [0.015s] Step 3: Run inference (ToadStool)
            Model: ResNet-50
            Input shape: [1, 3, 224, 224]
            Status: ✓ Complete (28ms)
            
  [0.043s] Step 4: Format response
            Top-3 predictions ready
            Status: ✓ Done (2ms)

Response Sent:
  Request ID: req_abc123
  Status: SUCCESS
  Latency: 45ms
  Predictions:
    1. cat (95.3% confidence)
    2. dog (2.8% confidence)
    3. bird (1.1% confidence)

────────────────────────────────────────────────────────

Benefits:
  ✓ Immediate feedback: Results returned synchronously
  ✓ Simple: Easy to understand and implement
  ✓ Reliable: Clear success/failure status
  ✓ Measurable: Latency tracking built-in

Use Cases:
  • API endpoints
  • Model inference
  • Data queries
  • RPC calls

════════════════════════════════════════════════════════

EOF
    
    log_success "Request/Response pattern demonstrated!"
}

pattern3_event_driven() {
    log_step "Pattern 3: Event-Driven"
    
    log_feature "Asynchronous event propagation"
    
    echo ""
    log_info "Example: Data Pipeline Automation"
    
    cat <<'EOF'

Event-Driven Pattern
════════════════════════════════════════════════════════

Setup:
  Publisher: NestGate (emits events)
  Subscribers: Multiple services react to events
  
Flow:
  Data Arrives → Event Published → Subscribers Notified

────────────────────────────────────────────────────────

Event Published:
  Type: data.created
  Source: nestgate
  Path: /raw-data/dataset-2024-12.csv
  Size: 2.4 GB
  Timestamp: 2024-12-17T14:35:00Z
  Event ID: evt_xyz789

Subscriber 1: Songbird (Orchestration)
  Status: Event received (3ms latency)
  Action: Trigger workflow "data-processing-pipeline"
  ✓ Workflow started: wf_proc_001
  ✓ Steps: validate → clean → transform → load
  ✓ ETA: 15 minutes

Subscriber 2: BearDog (Security)
  Status: Event received (5ms latency)
  Action: Create encrypted backup
  ✓ Backup started: backup_sec_002
  ✓ Encryption: AES-256-GCM
  ✓ Destination: s3://backup/2024/12/17/
  ✓ ETA: 8 minutes

Subscriber 3: Notification Service
  Status: Event received (2ms latency)
  Action: Send notification
  ✓ Email sent to: data-team@company.com
  ✓ Slack message posted: #data-pipeline
  ✓ Dashboard updated: /datasets/new

Subscriber 4: Audit Logger
  Status: Event received (1ms latency)
  Action: Log event to audit trail
  ✓ Entry created in audit log
  ✓ Compliance record updated
  ✓ Retention policy applied

────────────────────────────────────────────────────────

Benefits:
  ✓ Loose coupling: Publisher doesn't know subscribers
  ✓ Scalability: Add subscribers without changing publisher
  ✓ Flexibility: Different actions for same event
  ✓ Asynchronous: Non-blocking operations

Use Cases:
  • Pipeline automation
  • Monitoring alerts
  • Audit logging
  • System integration

════════════════════════════════════════════════════════

EOF
    
    log_success "Event-Driven pattern demonstrated!"
}

pattern4_batch_processing() {
    log_step "Pattern 4: Batch Processing"
    
    log_feature "Scheduled large-scale data processing"
    
    echo ""
    log_info "Example: Nightly ETL Job"
    
    cat <<'EOF'

Batch Processing Pattern
════════════════════════════════════════════════════════

Setup:
  Schedule: Daily at 2:00 AM
  Source: NestGate production data
  Processing: ToadStool ETL scripts
  Destination: Data warehouse
  
Flow:
  Extract → Transform → Load → Verify → Cleanup

────────────────────────────────────────────────────────

Job Started: 2024-12-17 02:00:00

Stage 1: Extract [02:00:00 - 02:03:45]
  Source: nestgate://production/transactions
  Filter: date = 2024-12-16
  Rows: 2,847,392
  Size: 4.2 GB
  Output: nestgate://staging/raw-transactions
  Status: ✓ Complete (3m 45s)

Stage 2: Transform [02:03:45 - 02:18:23]
  Input: nestgate://staging/raw-transactions
  Service: ToadStool
  Script: transform-data.py
  
  Operations:
    ✓ Clean nulls (12,456 rows affected)
    ✓ Normalize values (all columns)
    ✓ Enrich metadata (external API calls)
    ✓ Validate business rules (99.8% pass rate)
  
  Output: nestgate://staging/clean-transactions
  Rows: 2,834,936 (12,456 removed)
  Status: ✓ Complete (14m 38s)

Stage 3: Load [02:18:23 - 02:24:12]
  Source: nestgate://staging/clean-transactions
  Destination: postgres://warehouse/transactions
  Mode: Append
  Batch size: 10,000 rows
  
  Progress:
    Batch 1-100: ████████████████████████████ 100%
    Batch 101-200: ████████████████████████████ 100%
    Batch 201-283: ████████████████████████████ 100%
  
  Rows loaded: 2,834,936
  Status: ✓ Complete (5m 49s)

Stage 4: Verify [02:24:12 - 02:25:34]
  Checks:
    ✓ Row count matches (2,834,936 = 2,834,936)
    ✓ No duplicates found
    ✓ Data quality rules passed (100%)
    ✓ Referential integrity maintained
  
  Status: ✓ All checks passed (1m 22s)

Stage 5: Cleanup [02:25:34 - 02:26:45]
  Actions:
    ✓ Deleted: nestgate://staging/*
    ✓ Created snapshot: snap_prod_20241217
    ✓ Updated metadata catalog
    ✓ Archived logs
  
  Status: ✓ Complete (1m 11s)

────────────────────────────────────────────────────────

Job Complete: 2024-12-17 02:26:45
  Total duration: 26m 45s
  Records processed: 2,847,392 → 2,834,936
  Success rate: 99.6%
  Next run: 2024-12-18 02:00:00
  
Notifications:
  ✓ Email sent to: data-team@company.com
  ✓ Dashboard updated: /etl/status
  ✓ Metrics published to monitoring

════════════════════════════════════════════════════════

EOF
    
    log_success "Batch Processing pattern demonstrated!"
}

pattern5_stream_processing() {
    log_step "Pattern 5: Stream Processing"
    
    log_feature "Real-time continuous data processing"
    
    echo ""
    log_info "Example: IoT Sensor Analytics"
    
    cat <<'EOF'

Stream Processing Pattern
════════════════════════════════════════════════════════

Setup:
  Source: IoT sensors (1000 events/sec)
  Storage: NestGate streams
  Processing: Real-time analytics
  Output: Alerts + Dashboard
  
Flow:
  Sensors → Parse → Filter → Aggregate → Detect → Alert

────────────────────────────────────────────────────────

Stream Started: 14:40:00

Window 1: [14:40:00 - 14:40:05] 5-second window
  
  Incoming events: 5,234 events
  Parse rate: 1,046 events/sec
  
  Parsed events:
    Sensor readings: 5,234
    Valid: 5,189 (99.1%)
    Invalid: 45 (0.9% - format errors)
  
  Filtered events (temperature > 80°F):
    Count: 234 events (4.5%)
    Sensors affected: 18 unique sensors
  
  Aggregated (5-sec average):
    Average temperature: 76.3°F
    Max temperature: 94.2°F
    Min temperature: 68.1°F
    High temp sensors: 18

Window 2: [14:40:05 - 14:40:10] 5-second window
  
  Incoming events: 5,456 events
  Parse rate: 1,091 events/sec
  
  Filtered events (temperature > 80°F):
    Count: 312 events (5.7%)
    Sensors affected: 24 unique sensors
  
  Aggregated (5-sec average):
    Average temperature: 78.9°F  ⚠ Trending up
    Max temperature: 96.8°F      ⚠ Above threshold!
    Min temperature: 67.8°F
    High temp sensors: 24        ⚠ Increasing

Window 3: [14:40:10 - 14:40:15] 5-second window
  
  Incoming events: 5,123 events
  
  Filtered events (temperature > 80°F):
    Count: 428 events (8.4%)  ⚠ Rising fast!
    Sensors affected: 31 unique sensors
  
  Aggregated (5-sec average):
    Average temperature: 82.4°F  🚨 Above 80°F!
    Max temperature: 98.3°F      🚨 Critical!
    
  🚨 ALERT TRIGGERED 🚨
    Condition: Average temp > 85°F for 3 consecutive windows
    Severity: URGENT
    Action: Notify operations team
    
  Alert sent:
    ✓ SMS to: +1-555-ONCALL
    ✓ Email to: ops-team@company.com
    ✓ PagerDuty incident created: #INC-2847
    ✓ Dashboard updated (red status)

────────────────────────────────────────────────────────

Stream Statistics (last 60 seconds):
  Total events: 62,456
  Average rate: 1,041 events/sec
  Processing latency: 23ms (p99)
  Alerts triggered: 3
  
Storage:
  Raw events: nestgate://timeseries/sensors (batched)
  Aggregates: nestgate://timeseries/aggregates
  Batch size: 1,000 events
  Flush interval: 10 seconds
  Compression: Enabled (3.2x ratio)

════════════════════════════════════════════════════════

EOF
    
    log_success "Stream Processing pattern demonstrated!"
}

show_pattern_comparison() {
    log_step "Pattern Comparison"
    
    cat <<'EOF'

Pattern Selection Guide
════════════════════════════════════════════════════════

Pattern              | Latency | Throughput | Complexity | Use When
────────────────────────────────────────────────────────────────────
Producer/Consumer    | Medium  | High       | Low        | Decouple services
Request/Response     | Low     | Medium     | Low        | Need immediate result
Event-Driven         | High    | High       | Medium     | Async workflows
Batch Processing     | High    | Very High  | Medium     | Large datasets
Stream Processing    | Very Low| Very High  | High       | Real-time data

────────────────────────────────────────────────────────

Pattern Combinations:
  • Request/Response + Event-Driven
    → Immediate ack, async processing
    
  • Producer/Consumer + Batch Processing
    → Buffer real-time, process in batches
    
  • Event-Driven + Stream Processing
    → Events trigger real-time analysis
    
  • All patterns together
    → Lambda architecture (real-time + batch)

════════════════════════════════════════════════════════

EOF
}

show_summary() {
    log_step "Demo Summary"
    
    echo "You've learned 5 essential data flow patterns:"
    echo ""
    echo "  ✓ Producer/Consumer - Decoupled data flow"
    echo "  ✓ Request/Response - Synchronous exchange"
    echo "  ✓ Event-Driven - Asynchronous propagation"
    echo "  ✓ Batch Processing - Scheduled large-scale"
    echo "  ✓ Stream Processing - Real-time continuous"
    echo ""
    echo "Key Takeaways:"
    echo "  • Choose pattern based on requirements"
    echo "  • Combine patterns for complex systems"
    echo "  • NestGate enables all patterns"
    echo "  • Patterns are building blocks"
    echo ""
    echo "Apply These Patterns To:"
    echo "  • Data pipelines (ETL, ELT)"
    echo "  • Event processing (real-time, batch)"
    echo "  • Microservices (decoupling)"
    echo "  • Analytics (streaming, batch)"
    echo "  • Any distributed system"
}

show_level2_complete() {
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║           🎊 LEVEL 2 COMPLETE! 🎊                         ║"
    echo "║                                                            ║"
    echo "║  Congratulations! You've mastered Ecosystem Integration   ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    echo "You completed all 4 Level 2 demos:"
    echo "  ✓ BearDog Crypto Integration"
    echo "  ✓ Songbird Orchestration"
    echo "  ✓ ToadStool Storage Integration"
    echo "  ✓ Data Flow Patterns"
    echo ""
    echo "You now understand:"
    echo "  • Cross-primal integration"
    echo "  • Service discovery"
    echo "  • Workflow orchestration"
    echo "  • Compute + storage"
    echo "  • Data flow patterns"
    echo ""
    echo "Ready for Level 3?"
    echo "  cd ../../03_federation"
    echo "  cat README.md"
    echo ""
    echo "Level 3 covers:"
    echo "  • Multi-node NestGate clusters"
    echo "  • Distributed consensus"
    echo "  • Data replication"
    echo "  • Network resilience"
    echo ""
}

main() {
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║      NestGate Demo 2.4: Data Flow Patterns                ║"
    echo "║                                                            ║"
    echo "║  Common integration patterns for distributed systems      ║"
    echo "║  • Producer/Consumer                                      ║"
    echo "║  • Request/Response                                       ║"
    echo "║  • Event-Driven                                           ║"
    echo "║  • Batch Processing                                       ║"
    echo "║  • Stream Processing                                      ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    
    pattern1_producer_consumer
    pattern2_request_response
    pattern3_event_driven
    pattern4_batch_processing
    pattern5_stream_processing
    show_pattern_comparison
    show_summary
    show_level2_complete
}

main "$@"

