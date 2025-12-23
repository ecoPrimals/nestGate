# Edge Computing Node with NestGate

**Level**: 5 - Real-World Scenarios  
**Complexity**: Intermediate  
**Time**: 10 minutes  
**Prerequisites**: None (works standalone)

---

## 🎯 Purpose

Demonstrate NestGate as an edge computing storage node:
- IoT data collection and storage
- Local-first processing (no cloud required)
- Resource-constrained deployment (Raspberry Pi)
- Offline-capable operation
- Efficient data synchronization

---

## 🌐 Real-World Scenario

**The Problem**: Smart farm needs reliable IoT storage:
- **Sensors**: 50 temperature/humidity sensors
- **Cameras**: 10 security cameras (24/7 recording)
- **Data Volume**: 100GB/day
- **Constraints**: Limited internet, power outages common
- **Requirements**: Must work offline, sync when online

**The Solution**: NestGate Edge Node
- Runs on Raspberry Pi 4 (low power)
- Stores data locally (fast, reliable)
- Syncs to central server when online
- Continues working during outages
- No cloud dependencies

---

## 🏗️ Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                    Smart Farm Edge Network                    │
│                                                               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐    │
│  │ Temp     │  │ Humidity │  │ Soil     │  │ Security │    │
│  │ Sensors  │  │ Sensors  │  │ Moisture │  │ Cameras  │    │
│  │ (x20)    │  │ (x20)    │  │ (x10)    │  │ (x10)    │    │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘    │
│       │             │              │              │          │
│       └─────────────┴──────────────┴──────────────┘          │
│                          │                                   │
│                   WiFi/Ethernet                              │
│                          │                                   │
│       ┌──────────────────▼───────────────────────┐          │
│       │      NestGate Edge Node (RPi4)           │          │
│       │                                           │          │
│       │  • Local storage (2TB SSD)                │          │
│       │  • Real-time data ingestion               │          │
│       │  • Local processing/analytics             │          │
│       │  • Offline-capable                        │          │
│       │  • Auto-sync to central                   │          │
│       └───────────────┬───────────────────────────┘          │
└───────────────────────┼──────────────────────────────────────┘
                        │
                  (When online)
                        │
                        ▼
            ┌───────────────────────┐
            │   Central Server      │
            │   (Cloud/Datacenter)  │
            └───────────────────────┘
```

---

## 🚀 What This Demo Shows

### 1. Resource-Constrained Deployment
- Runs on Raspberry Pi 4 (4GB RAM)
- Low power consumption (5W)
- Works on ARM architecture
- Compact form factor

### 2. IoT Data Collection
- Real-time sensor data ingestion
- Camera footage storage
- Efficient time-series storage
- Automatic data retention

### 3. Local-First Architecture
- Works without internet
- Fast local queries
- No latency to cloud
- Privacy-preserving

### 4. Smart Synchronization
- Automatic sync when online
- Bandwidth-efficient (compression)
- Resume after interruption
- Deduplication

---

## 📋 Prerequisites

### Hardware (Recommended)
- Raspberry Pi 4 (4GB+) or similar SBC
- USB SSD (256GB+) - faster than SD card
- Ethernet connection (reliable)
- Power supply with backup (optional)

### Software
- Raspberry Pi OS Lite (minimal)
- NestGate ARM build
- No GUI required

---

## 🏃 Running the Demo

### Quick Start
```bash
./demo.sh
```

### What Happens

1. **Deployment Phase** (10 seconds)
   - Initialize edge storage
   - Configure retention policies
   - Set up sensor data streams
   - Start ingestion service

2. **Data Collection** (30 seconds)
   - Simulate 50 sensors (1 reading/sec)
   - Store camera footage (10 streams)
   - Local analytics processing
   - Storage metrics

3. **Offline Scenario** (20 seconds)
   - Simulate internet outage
   - Continue collecting data
   - Local buffering
   - Automatic recovery

4. **Sync Phase** (20 seconds)
   - Reconnect to central server
   - Sync queued data
   - Bandwidth optimization
   - Verification

---

## 📊 Expected Results

### Storage Layout
```
nestgate-edge/
├── sensors/
│   ├── temperature/    # Time-series data
│   ├── humidity/       # Time-series data
│   └── soil/           # Time-series data
├── cameras/
│   ├── cam01-10/       # Video streams
│   └── snapshots/      # Still images
└── analytics/
    ├── hourly/         # Aggregated data
    └── daily/          # Reports
```

### Data Retention
```
Sensor readings:     30 days (raw), 1 year (hourly avg)
Camera footage:      7 days (full), 30 days (snapshots)
Analytics:           1 year
Compression:         3-5x for sensor data, 10x for video
```

### Performance
```
Sensor Ingestion:    50 sensors × 1/sec = 50 writes/sec
Camera Ingestion:    10 cameras × 2Mbps = 20Mbps = 2.5MB/s
Total Write Rate:    ~3MB/s (well within Pi4 capability)
Query Latency:       <10ms (local SSD)
Power Consumption:   5W (Pi4 + SSD)
```

---

## 🔍 Key Concepts

### 1. Edge Computing
- **Compute near data source**: Faster, lower latency
- **Local processing**: No cloud round-trip
- **Reduced bandwidth**: Only send summaries to cloud
- **Privacy**: Sensitive data stays on-site

### 2. Offline-First
- **Continue working**: Internet not required
- **Local buffer**: Queue data for later sync
- **Graceful degradation**: Full functionality offline
- **Auto-recovery**: Sync when reconnected

### 3. Resource Efficiency
- **Low power**: 5W vs. 200W for server
- **Compact**: Fits in hand
- **Fanless**: No moving parts
- **Reliable**: Solid-state storage

### 4. Time-Series Optimization
- **Compression**: 3-5x for sensor data
- **Downsampling**: Keep high-res recent, low-res old
- **Efficient queries**: Fast range scans
- **Automatic rollups**: Hourly/daily aggregates

---

## 💡 Real-World Usage

### Daily Operation

**7:00 AM** - Data collection starts
- Sensors: Temperature, humidity, soil moisture
- Cameras: Motion-activated recording
- Storage: Local buffering

**9:00 AM** - Internet available
- Sync overnight data to central server
- Compressed: 50GB → 10GB (5x compression)
- Transfer time: 10 minutes @ 20Mbps

**12:00 PM** - Internet outage (power failure)
- Edge node continues on battery backup
- Buffer data locally
- No data loss

**3:00 PM** - Internet restored
- Auto-reconnect to central server
- Resume sync where left off
- Verify data integrity

**11:59 PM** - Daily rollup
- Generate daily analytics
- Archive old camera footage
- Free up space

---

## 🎓 Learning Outcomes

After this demo, you'll understand:

1. ✅ How NestGate works on resource-constrained devices
2. ✅ Edge computing storage patterns
3. ✅ Offline-first architecture benefits
4. ✅ Time-series data optimization
5. ✅ IoT data collection at scale

---

## 🔗 Related Demos

### Previous
- **01_home_nas_server**: Family NAS use case
- **../04_inter_primal_mesh/**: Multi-node coordination

### Similar Use Cases
- **Industrial IoT**: Factory sensor networks
- **Retail**: Point-of-sale + inventory
- **Healthcare**: Medical device data
- **Smart Cities**: Traffic/environmental monitoring

---

## 🛠️ Customization

### Storage Configuration
```bash
# Edit demo.sh
STORAGE_SIZE="256GB"  # Adjust for your SSD
RETENTION_DAYS=30     # Sensor data retention
CAMERA_DAYS=7         # Video retention
```

### Sensor Configuration
```bash
# Edit demo.sh
SENSOR_COUNT=50       # Number of sensors
SAMPLE_RATE=1         # Readings per second
CAMERA_COUNT=10       # Number of cameras
```

### Sync Configuration
```bash
# Edit demo.sh
SYNC_INTERVAL=3600    # Sync every hour (seconds)
BANDWIDTH_LIMIT=20    # Mbps upload limit
```

---

## 📈 Cost Comparison

### Cloud-Only Solution
```
Monthly Data: 3TB (100GB/day × 30)
AWS S3 Cost:  $70/month ($840/year)
Bandwidth:    $270/month ($3,240/year)
Total:        $4,080/year
Lock-in:      High (AWS-specific APIs)
```

### Edge + Cloud Hybrid
```
Hardware:     $150 (Raspberry Pi 4 + SSD, one-time)
Power:        $5/year (5W × $0.12/kWh)
Cloud:        $10/month ($120/year) - only summaries
Total Year 1: $275
Total Year 5: $615
Savings:      $19,665 over 5 years!
Lock-in:      None (standard protocols)
```

---

## 🐛 Troubleshooting

### Issue: High CPU usage
**Solution**: Reduce sample rate, enable compression, use SSD not SD card

### Issue: Sync failures
**Solution**: Check network, increase retry timeout, verify credentials

### Issue: Storage full
**Solution**: Reduce retention days, enable compression, add more storage

---

## 🔮 Advanced Topics

### 1. Multi-Edge Deployment
- 10+ edge nodes per site
- Hierarchical sync (edge → site → central)
- Load balancing across edges

### 2. Edge Analytics
- Local ML inference
- Anomaly detection
- Alerting without cloud

### 3. Failover & HA
- Multiple edge nodes (redundancy)
- Automatic failover
- Data replication

---

## 📖 References

- Edge Computing Best Practices
- IoT Storage Patterns
- Time-Series Database Design
- Raspberry Pi Optimization

---

## 🎯 Use Cases

### Smart Agriculture
- Soil monitoring (moisture, pH, nutrients)
- Weather stations
- Irrigation control
- Pest detection (cameras + ML)

### Industrial IoT
- Equipment monitoring
- Predictive maintenance
- Quality control
- Safety sensors

### Retail
- Point-of-sale data
- Inventory tracking
- Customer analytics
- Security cameras

### Smart Home/Building
- Energy monitoring
- HVAC optimization
- Security system
- Access control

---

**Next**: Congratulations! You've completed all showcase demos! 🎉

