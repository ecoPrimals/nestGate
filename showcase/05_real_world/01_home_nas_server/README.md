# Home NAS Server with NestGate

**Level**: 5 - Real-World Scenarios  
**Complexity**: Beginner-Friendly  
**Time**: 10 minutes  
**Prerequisites**: None (works standalone)

---

## 🎯 Purpose

Demonstrate NestGate as a complete home NAS (Network-Attached Storage) solution:
- Family media server (photos, videos, music)
- Automatic backups for all devices
- User-friendly web interface
- Zero-configuration setup
- Snapshot-based versioning

---

## 🏠 Real-World Scenario

**The Problem**: Family needs reliable storage for:
- **Mom**: 50GB of photos from iPhone
- **Dad**: 200GB of work documents
- **Kids**: 100GB of videos and games
- **Shared**: 500GB of family movies and music

**The Solution**: NestGate Home NAS
- One-command setup
- Automatic device discovery
- Web UI for everyone
- Automatic daily snapshots
- No IT expertise required

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                   Home Network                           │
│                                                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐│
│  │  iPhone  │  │  Laptop  │  │  Desktop │  │  Tablet ││
│  │  (Mom)   │  │  (Dad)   │  │  (Kids)  │  │ (Shared)││
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬────┘│
│       │             │              │              │     │
│       └─────────────┴──────────────┴──────────────┘     │
│                          │                              │
│                    WiFi/Ethernet                        │
│                          │                              │
│       ┌──────────────────▼───────────────────┐         │
│       │       NestGate Home NAS              │         │
│       │     http://nestgate.local:8080       │         │
│       │                                      │         │
│       │  • Web UI (drag & drop uploads)     │         │
│       │  • Automatic device backup           │         │
│       │  • Daily snapshots                   │         │
│       │  • Photo thumbnails                  │         │
│       │  • Video streaming                   │         │
│       └──────────────────────────────────────┘         │
└─────────────────────────────────────────────────────────┘
```

---

## 🚀 What This Demo Shows

### 1. One-Command Setup
- Start NestGate as a home NAS
- No configuration files to edit
- Automatic network discovery

### 2. Family-Friendly Features
- Web UI for uploads/downloads
- Drag-and-drop file management
- Photo and video previews
- Mobile-friendly interface

### 3. Automatic Protection
- Daily snapshots (keep 30 days)
- Ransomware protection (immutable snapshots)
- Easy file recovery
- No data loss

### 4. Multi-Device Support
- iOS/Android apps (planned)
- SMB/NFS file sharing
- WebDAV for remote access
- Time Machine support (macOS)

---

## 📋 Prerequisites

### Hardware (Recommended)
- Raspberry Pi 4 (4GB+) or any Linux box
- External USB drive (1TB+)
- Ethernet connection (recommended)

### Software
- Linux (Ubuntu, Debian, Raspberry Pi OS)
- NestGate installed (cargo install nestgate)

---

## 🏃 Running the Demo

### Quick Start
```bash
./demo.sh
```

### What Happens

1. **Setup Phase** (10 seconds)
   - Initialize storage pool
   - Create family shares (photos, videos, documents, music)
   - Configure user accounts
   - Start web UI

2. **Usage Simulation** (30 seconds)
   - Upload family photos (50 files)
   - Store documents (20 files)
   - Add videos (5 large files)
   - Create daily snapshot

3. **Recovery Demo** (20 seconds)
   - Simulate accidental deletion
   - Restore from snapshot
   - Verify data integrity

4. **Results**
   - Web UI available at http://localhost:8080
   - All data protected with snapshots
   - Ready for family use

---

## 📊 Expected Results

### Storage Layout
```
nestgate-home/
├── photos/         # Mom's iPhone photos
├── documents/      # Dad's work files
├── videos/         # Kids' content
├── music/          # Shared family music
└── backups/        # Device backups
```

### Snapshot Schedule
```
Daily snapshots:     Kept for 30 days
Weekly snapshots:    Kept for 6 months
Monthly snapshots:   Kept for 1 year
Total overhead:      ~5% of used space
```

### Performance
```
Upload Speed:   100 MB/s (Gigabit Ethernet)
Download Speed: 100 MB/s
Web UI:         < 50ms response time
Snapshot Time:  < 1 second
Recovery Time:  Instant (ZFS clone)
```

---

## 🔍 Key Concepts

### 1. Zero Configuration
- No `/etc` files to edit
- Automatic network discovery (mDNS)
- Works out of the box

### 2. User-Friendly
- Web UI (no command line required)
- Drag-and-drop uploads
- Photo/video previews
- Mobile responsive

### 3. Data Protection
- **Snapshots**: Instant, zero-cost backups
- **Immutable**: Ransomware can't delete old versions
- **Versioned**: Go back to any point in time
- **Efficient**: Only changed data stored

### 4. Family Sharing
- Per-user accounts with quotas
- Shared folders for family content
- Privacy controls (private folders)
- Guest access for temporary sharing

---

## 💡 Real-World Usage

### Daily Workflow

**Mom** (iPhone backup):
1. Open Photos app
2. Enable automatic backup to NestGate
3. Photos uploaded over WiFi when charging
4. All photos safe, accessible from web

**Dad** (Work documents):
1. Map network drive: `\\nestgate.local\documents`
2. Work normally (Word, Excel, etc.)
3. Files automatically synced
4. Previous versions always available

**Kids** (Videos and games):
1. Open web UI: `http://nestgate.local:8080`
2. Drag & drop video files
3. Stream videos from any device
4. Never lose saved games

---

## 🎓 Learning Outcomes

After this demo, you'll understand:

1. ✅ How NestGate works as a home NAS
2. ✅ Zero-configuration setup process
3. ✅ Snapshot-based data protection
4. ✅ Multi-user family sharing
5. ✅ Recovery from accidental deletion

---

## 🔗 Related Demos

### Previous
- **../04_inter_primal_mesh/**: Advanced inter-primal coordination
- **../03_federation/**: Multi-node setups

### Next
- **02_edge_computing**: IoT edge deployment
- **03_media_production**: Professional video editing

---

## 🛠️ Customization

### Storage Pool Size
```bash
# Edit demo.sh
POOL_SIZE="1TB"  # Adjust based on your drive
```

### User Accounts
```bash
# Edit demo.sh
USERS=("mom" "dad" "alice" "bob")
QUOTAS=(100G 200G 50G 50G)  # Per-user limits
```

### Snapshot Retention
```bash
# Edit demo.sh
DAILY_SNAPSHOTS=30    # Keep 30 days
WEEKLY_SNAPSHOTS=24   # Keep 6 months
MONTHLY_SNAPSHOTS=12  # Keep 1 year
```

---

## 📈 Cost Comparison

### Commercial NAS (Synology DS220+)
```
Hardware:      $300
Storage (2TB): $150
Total:         $450
Features:      Good, but locked to vendor
```

### Cloud Storage (2TB)
```
Monthly Cost:  $10/month
Annual Cost:   $120/year
5-Year Cost:   $600
Privacy:       Your data, their servers
```

### NestGate Home NAS
```
Hardware (RPi4): $75
Storage (2TB):   $50
Total:           $125
Features:        Full control, privacy, extensible
Savings:         $325 vs. Synology, $475 over 5 years vs. cloud
```

---

## 🐛 Troubleshooting

### Issue: Can't access web UI
**Solution**: Check firewall, ensure port 8080 is open

### Issue: Slow uploads
**Solution**: Use Ethernet instead of WiFi, check network speed

### Issue: Drive full
**Solution**: Add more drives, enable compression, remove old snapshots

---

## 🔮 Advanced Topics

### 1. Remote Access
- Port forwarding (8080)
- VPN for secure access
- Dynamic DNS (no-ip.com, duckdns.org)

### 2. Mobile Apps
- iOS app (planned)
- Android app (planned)
- Photo auto-upload

### 3. Advanced Features
- Plex media server integration
- Time Machine backups (macOS)
- Docker container hosting
- Home automation hub

---

## 📖 References

- NestGate Home Setup Guide
- Family NAS Best Practices
- ZFS for Home Users
- Network Storage Protocols

---

## 🎯 Use Cases

### Small Families (2-4 people)
- 500GB - 2TB storage
- Raspberry Pi 4 sufficient
- WiFi or Ethernet
- $100-150 total cost

### Large Families (5+ people)
- 2TB - 10TB storage
- Mini PC or desktop
- Gigabit Ethernet
- $200-400 total cost

### Home Office
- Work document backup
- Client file sharing
- Version control for projects
- Time Machine for Macs

---

**Next**: Try **02_edge_computing** to see NestGate on IoT devices!

