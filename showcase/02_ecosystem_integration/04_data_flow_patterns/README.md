# Demo 2.4: Data Flow Patterns

**Level**: 2 (Ecosystem Integration)  
**Time**: 35 minutes  
**Complexity**: Medium-High  
**Status**: 🚧 Building (Week 2)

---

## 🎯 **WHAT THIS DEMO SHOWS**

This demo demonstrates common data flow patterns across the ecoPrimals ecosystem:

1. **Producer/Consumer** - Data generation and consumption
2. **Request/Response** - Synchronous data exchange
3. **Event-Driven** - Asynchronous event propagation
4. **Batch Processing** - Large-scale data processing
5. **Stream Processing** - Real-time data streams

**Key Concept**: Real-world patterns for building distributed systems

---

## 🚀 **QUICK RUN**

```bash
# Make sure ecosystem services are running
../../scripts/start_ecosystem.sh --all

# Run the demo
./demo.sh

# Expected runtime: ~7 minutes
```

---

## 📋 **PATTERN 1: PRODUCER/CONSUMER**

### Overview
One service produces data, another consumes it. Classic decoupling pattern.

### Example: Log Aggregation
```yaml
# Producer: Application generates logs
producer:
  name: web-application
  outputs:
    - path: nestgate://logs/app.log
      format: json
      rotation: hourly

# Consumer: Log processor reads and analyzes
consumer:
  name: log-analyzer
  inputs:
    - path: nestgate://logs/app.log
      poll_interval: 60s
  processing:
    - parse_json
    - extract_metrics
    - detect_errors
  outputs:
    - alerts: songbird://notifications
    - metrics: prometheus://metrics
```

### Benefits
- **Decoupling**: Producer and consumer independent
- **Buffering**: NestGate buffers during consumer downtime
- **Scalability**: Multiple consumers can read same data

### Use Cases
- Log aggregation
- Metrics collection
- Event processing
- Data pipelines

---

## 📋 **PATTERN 2: REQUEST/RESPONSE**

### Overview
Synchronous data exchange - request sent, response received.

### Example: ML Model Serving
```yaml
# Client requests inference
request:
  model: nestgate://models/resnet50.pth
  input: image.jpg
  
# NestGate loads model, ToadStool runs inference
workflow:
  - load_model:
      service: nestgate
      operation: retrieve
      path: /models/resnet50.pth
      
  - run_inference:
      service: toadstool
      operation: execute
      model: "${load_model.data}"
      input: "${request.input}"
      
  - return_result:
      service: response
      data: "${run_inference.output}"

# Response returned
response:
  predictions: [
    {class: "dog", confidence: 0.95},
    {class: "cat", confidence: 0.03},
    {class: "bird", confidence: 0.02}
  ]
  latency_ms: 45
```

### Benefits
- **Immediate feedback**: Results returned synchronously
- **Simple**: Easy to understand and implement
- **Reliable**: Clear success/failure

### Use Cases
- API endpoints
- Model inference
- Data queries
- RPC calls

---

## 📋 **PATTERN 3: EVENT-DRIVEN**

### Overview
Asynchronous event propagation - events published, subscribers notified.

### Example: Data Pipeline Triggers
```yaml
# Event: New data arrives in NestGate
event:
  type: data.created
  source: nestgate
  path: /raw-data/dataset-2024-12.csv
  size: 2.4GB
  timestamp: 2024-12-17T14:30:00Z

# Subscriber 1: Songbird orchestrates processing
subscriber_1:
  service: songbird
  on_event: data.created
  action: workflow.trigger
  workflow: data-processing-pipeline
  params:
    input: "${event.path}"
    
# Subscriber 2: BearDog creates encrypted backup
subscriber_2:
  service: beardog
  on_event: data.created
  action: backup.create
  params:
    source: "${event.path}"
    encrypt: true
    destination: s3://backup/

# Subscriber 3: Notification sent
subscriber_3:
  service: notifications
  on_event: data.created
  action: notify
  message: "New dataset available: ${event.path}"
```

### Benefits
- **Loose coupling**: Publishers don't know subscribers
- **Scalability**: Add subscribers without changing publisher
- **Flexibility**: Different actions for same event

### Use Cases
- Pipeline automation
- Monitoring alerts
- Audit logging
- System integration

---

## 📋 **PATTERN 4: BATCH PROCESSING**

### Overview
Process large amounts of data in batches, typically on schedule.

### Example: Nightly ETL Job
```yaml
# Batch job definition
job:
  name: nightly-etl
  schedule: "0 2 * * *"  # 2 AM daily
  
  stages:
    - extract:
        source: nestgate://production/transactions
        filter: date = yesterday
        output: nestgate://staging/raw-transactions
        
    - transform:
        service: toadstool
        script: transform-data.py
        input: nestgate://staging/raw-transactions
        operations:
          - clean_nulls
          - normalize_values
          - enrich_metadata
        output: nestgate://staging/clean-transactions
        
    - load:
        source: nestgate://staging/clean-transactions
        destination: database://warehouse/transactions
        mode: append
        
    - verify:
        checks:
          - row_count_matches
          - no_duplicates
          - data_quality_rules
          
    - cleanup:
        delete: nestgate://staging/*
        create_snapshot: nestgate://production/transactions
        
  on_success:
    - notify: team@company.com
    - metrics: update_dashboard
    
  on_failure:
    - alert: oncall@company.com
    - rollback: restore_snapshot
```

### Benefits
- **Efficiency**: Process data when resources available
- **Reliability**: Atomic operations with rollback
- **Monitoring**: Clear success/failure metrics

### Use Cases
- ETL pipelines
- Report generation
- Data warehousing
- Backup jobs

---

## 📋 **PATTERN 5: STREAM PROCESSING**

### Overview
Process data as it arrives in real-time continuous stream.

### Example: Real-Time Analytics
```yaml
# Stream source: IoT sensors
stream:
  source: nestgate://streams/sensor-data
  format: json
  rate: 1000 events/sec
  
# Processing topology
topology:
  - parse:
      operation: parse_json
      output: parsed_stream
      
  - filter:
      input: parsed_stream
      condition: temperature > 80
      output: high_temp_stream
      
  - aggregate:
      input: high_temp_stream
      window: 5 minutes
      operation: average
      output: avg_temp_stream
      
  - detect:
      input: avg_temp_stream
      condition: value > 85
      action: alert
      
  - store:
      input: parsed_stream
      destination: nestgate://timeseries/sensors
      batch_size: 1000
      flush_interval: 10s

# Alerts published
alerts:
  destination: songbird://notifications
  format: urgent
```

### Benefits
- **Real-time**: Process data as it arrives
- **Low latency**: Sub-second response times
- **Scalable**: Handle high throughput

### Use Cases
- IoT data processing
- Real-time analytics
- Fraud detection
- Monitoring systems

---

## 🏗️ **ARCHITECTURE PATTERNS**

### Pattern: Data Lake
```
Sources → NestGate (Raw) → Processing → NestGate (Curated)
                                              ↓
                                        Analytics/ML
```

**Use**: Store all data, process as needed

### Pattern: Lambda Architecture
```
Real-time: Stream → Processing → Results
Batch: Historical → Processing → Results
                        ↓
                  Combined View
```

**Use**: Real-time + historical analysis

### Pattern: Microservices Data
```
Service A → NestGate → Service B
              ↓
         Shared Data
              ↓
Service C → NestGate → Service D
```

**Use**: Decouple microservices via shared storage

---

## 🧪 **EXPERIMENTS TO TRY**

### Experiment 1: Build a Data Pipeline
```yaml
# Complete ETL pipeline
pipeline:
  - extract:
      service: nestgate
      path: /source-data
      
  - validate:
      service: toadstool
      checks: data-quality-rules.py
      
  - transform:
      service: toadstool
      script: transform.py
      
  - load:
      service: nestgate
      path: /processed-data
      
  - backup:
      service: beardog
      encrypt: true
```

### Experiment 2: Event-Driven Workflow
```yaml
# Trigger workflow on file arrival
trigger:
  event: file.created
  path: /uploads/*
  
actions:
  - validate_file
  - process_contents
  - notify_completion
```

### Experiment 3: Real-Time Dashboard
```yaml
# Stream sensor data to dashboard
stream:
  source: nestgate://sensors
  processing:
    - aggregate_metrics
    - detect_anomalies
  output: dashboard
  update_interval: 1s
```

---

## 📚 **BEST PRACTICES**

### 1. Error Handling
```yaml
# Always include error handling
workflow:
  steps:
    - try:
        operation: process_data
      catch:
        - log_error
        - notify_team
        - rollback_changes
      finally:
        - cleanup_temp_files
```

### 2. Idempotency
```yaml
# Make operations idempotent
operation:
  check_if_exists: true
  skip_if_done: true
  use_transaction: true
```

### 3. Monitoring
```yaml
# Monitor everything
monitoring:
  metrics:
    - latency
    - throughput
    - error_rate
  alerts:
    - threshold_exceeded
    - service_down
    - data_quality_issues
```

### 4. Data Validation
```yaml
# Validate at boundaries
validation:
  input:
    - check_schema
    - verify_types
    - validate_ranges
  output:
    - confirm_completeness
    - check_quality
```

---

## 🆘 **TROUBLESHOOTING**

### "Event not received"
**Fix**: Check event subscriptions and routing

### "Batch job failed"
**Fix**: Review logs, check dependencies, verify data

### "Stream lag increasing"
**Fix**: Scale processing, optimize pipeline, check bottlenecks

---

## 📚 **LEARN MORE**

**Pattern Guides**:
- Producer/Consumer: `../../../docs/patterns/PRODUCER_CONSUMER.md`
- Event-Driven: `../../../docs/patterns/EVENT_DRIVEN.md`
- Stream Processing: `../../../docs/patterns/STREAM_PROCESSING.md`

**Best Practices**:
- Data Pipelines: `../../../docs/guides/DATA_PIPELINES.md`
- Error Handling: `../../../docs/guides/ERROR_HANDLING.md`
- Monitoring: `../../../docs/guides/MONITORING.md`

---

## ⏭️ **WHAT'S NEXT**

**Completed Level 2?** 🎉 Congratulations!

You now understand:
- ✅ BearDog crypto integration
- ✅ Songbird orchestration
- ✅ ToadStool storage integration
- ✅ Data flow patterns

**Ready for Level 3?** (`../../03_federation/`)
- Multi-node NestGate clusters
- Distributed consensus
- Data replication
- Network resilience

**Or review**:
- Practice patterns in your use case
- Build custom workflows
- Optimize performance

---

**Status**: 🚧 Building (Week 2)  
**Estimated time**: 35 minutes  
**Prerequisites**: Understanding of Demos 2.1-2.3

**Key Takeaway**: Patterns enable scalable, maintainable distributed systems! 🎯

---

*Demo 2.4 - Data Flow Patterns*  
*Part of Level 2: Ecosystem Integration*  
*🎊 LEVEL 2 COMPLETE! 🎊*

