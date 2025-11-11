# 🗺️ **NestGate Documentation Map**

**Quick navigation to all NestGate documentation**

---

## 🎯 **Start Here**

**New to NestGate?** Start with these:

1. **[README.md](README.md)** - Project overview & key features
2. **[START_HERE.md](START_HERE.md)** - 5-minute quick start guide
3. **[showcase/](showcase/)** - See NestGate in action (8 demos!)

---

## 📚 **Core Documentation**

### **Architecture & Design**
- **[NESTGATE_ROLE_CLARIFICATION.md](NESTGATE_ROLE_CLARIFICATION.md)** - What NestGate IS and IS NOT ⭐
- **[ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md)** - System architecture & design
- **[docs/ZERO_COST_ARCHITECTURE_GUIDE.md](docs/ZERO_COST_ARCHITECTURE_GUIDE.md)** - Performance details

### **Status & Planning**
- **[CURRENT_STATUS.md](CURRENT_STATUS.md)** - Current state (updated daily)
- **[PROJECT_STATUS_MASTER.md](PROJECT_STATUS_MASTER.md)** - Complete metrics
- **[CHANGELOG.md](CHANGELOG.md)** - Version history

### **Setup & Deployment**
- **[QUICK_START.md](QUICK_START.md)** - Quick setup guide
- **[LOCAL_INSTANCE_SETUP.md](LOCAL_INSTANCE_SETUP.md)** - Local installation
- **[CONNECT_TO_SONGBIRD.md](CONNECT_TO_SONGBIRD.md)** - Service mesh integration
- **[docs/DEPLOYMENT_GUIDE.md](docs/DEPLOYMENT_GUIDE.md)** - Production deployment

### **Usage & CLI**
- **[CLI_COMMANDS_WORKING.md](CLI_COMMANDS_WORKING.md)** - All CLI commands
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Command quick reference
- **[QUICK_TEST_COMMANDS.md](QUICK_TEST_COMMANDS.md)** - Testing commands

### **Development**
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - How to contribute
- **[docs/current/](docs/current/)** - Current development work
- **[specs/](specs/)** - Technical specifications

---

## 🎬 **Showcase & Demos**

### **Main Showcase**
- **[showcase/README.md](showcase/README.md)** - Showcase overview
- **[showcase/START_HERE.md](showcase/START_HERE.md)** - Showcase navigation

### **8 Production Demos**
All located in **[showcase/demos/](showcase/demos/)**:

1. **[07_connected_live/](showcase/demos/07_connected_live/)** - Service mesh integration (5 min)
2. **[08_bioinformatics_live/](showcase/demos/08_bioinformatics_live/)** - NCBI → Protein prediction (10 min) ⭐
3. **[09_ml_model_serving/](showcase/demos/09_ml_model_serving/)** - AI/ML model storage (10 min)
4. **[10_scientific_computing/](showcase/demos/10_scientific_computing/)** - HPC workflows (12 min)
5. **[11_raw_photo_workflow/](showcase/demos/11_raw_photo_workflow/)** - Professional photography (10 min)
6. **[12_container_registry/](showcase/demos/12_container_registry/)** - Docker/OCI management (10 min)
7. **[13_git_lfs_alternative/](showcase/demos/13_git_lfs_alternative/)** - Binary versioning (10 min)
8. **[14_media_server/](showcase/demos/14_media_server/)** - Plex/Jellyfin backend (12 min)

---

## 📖 **Learning Paths**

### **For Users** (30 minutes)
1. Read [START_HERE.md](START_HERE.md)
2. Run `cd showcase && ./demos/07_connected_live/demo.sh`
3. Browse [showcase/README.md](showcase/README.md)
4. Try demos in your domain

### **For Architects** (1 hour)
1. Read [NESTGATE_ROLE_CLARIFICATION.md](NESTGATE_ROLE_CLARIFICATION.md) ⭐
2. Read [ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md)
3. Study [showcase/demos/08_bioinformatics_live/](showcase/demos/08_bioinformatics_live/)
4. Review [specs/](specs/)

### **For Operators** (1 hour)
1. Follow [LOCAL_INSTANCE_SETUP.md](LOCAL_INSTANCE_SETUP.md)
2. Read [CLI_COMMANDS_WORKING.md](CLI_COMMANDS_WORKING.md)
3. Set up [CONNECT_TO_SONGBIRD.md](CONNECT_TO_SONGBIRD.md)
4. Deploy to production

### **For Developers** (2+ hours)
1. Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. Study [docs/ZERO_COST_ARCHITECTURE_GUIDE.md](docs/ZERO_COST_ARCHITECTURE_GUIDE.md)
3. Browse [code/crates/](code/crates/)
4. Run `cargo test --all`

---

## 🔍 **Find By Topic**

### **Architecture**
- Core concepts: [NESTGATE_ROLE_CLARIFICATION.md](NESTGATE_ROLE_CLARIFICATION.md)
- System design: [ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md)
- Performance: [docs/ZERO_COST_ARCHITECTURE_GUIDE.md](docs/ZERO_COST_ARCHITECTURE_GUIDE.md)

### **Getting Started**
- Quick start: [START_HERE.md](START_HERE.md)
- Full setup: [LOCAL_INSTANCE_SETUP.md](LOCAL_INSTANCE_SETUP.md)
- First demo: [showcase/demos/07_connected_live/](showcase/demos/07_connected_live/)

### **Use Cases**
- Science: [showcase/demos/08_bioinformatics_live/](showcase/demos/08_bioinformatics_live/), [showcase/demos/10_scientific_computing/](showcase/demos/10_scientific_computing/)
- ML/AI: [showcase/demos/09_ml_model_serving/](showcase/demos/09_ml_model_serving/)
- DevOps: [showcase/demos/12_container_registry/](showcase/demos/12_container_registry/), [showcase/demos/13_git_lfs_alternative/](showcase/demos/13_git_lfs_alternative/)
- Creative: [showcase/demos/11_raw_photo_workflow/](showcase/demos/11_raw_photo_workflow/), [showcase/demos/14_media_server/](showcase/demos/14_media_server/)

### **Integration**
- Songbird: [CONNECT_TO_SONGBIRD.md](CONNECT_TO_SONGBIRD.md)
- APIs: [showcase/demos/08_bioinformatics_live/real_ncbi_fetch.py](showcase/demos/08_bioinformatics_live/real_ncbi_fetch.py)
- Ecosystem: [docs/UNIVERSAL_ADAPTER_ARCHITECTURE.md](docs/UNIVERSAL_ADAPTER_ARCHITECTURE.md)

### **Deployment**
- Local: [LOCAL_INSTANCE_SETUP.md](LOCAL_INSTANCE_SETUP.md)
- Production: [docs/DEPLOYMENT_GUIDE.md](docs/DEPLOYMENT_GUIDE.md)
- Multi-tower: [ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md)

---

## 📂 **Directory Structure**

```
nestgate/
├── README.md                           # Main entry point
├── START_HERE.md                       # Quick start guide
├── NESTGATE_ROLE_CLARIFICATION.md     # Architecture clarity ⭐
├── CURRENT_STATUS.md                   # Current state
├── DOCUMENTATION_MAP.md                # This file
├── 
├── showcase/                           # Demos & examples
│   ├── README.md                       # Showcase overview
│   ├── demos/                          # 8 production demos
│   └── *.md                            # Showcase documentation
├── 
├── docs/                               # Technical documentation
│   ├── guides/                         # How-to guides
│   ├── sessions/                       # Development sessions
│   └── *.md                            # Architecture docs
├── 
├── specs/                              # Specifications
├── code/crates/                        # Source code
└── tests/                              # Test suites
```

---

## 🎯 **Quick Links**

### **Most Important** ⭐
1. [README.md](README.md) - Start here!
2. [NESTGATE_ROLE_CLARIFICATION.md](NESTGATE_ROLE_CLARIFICATION.md) - Architecture
3. [showcase/demos/08_bioinformatics_live/](showcase/demos/08_bioinformatics_live/) - Best demo

### **Quick Actions**
- **Run a demo**: `cd showcase && ./demos/07_connected_live/demo.sh`
- **Build**: `cargo build --release`
- **Test**: `cargo test --all`
- **Start service**: `./target/release/nestgate service start`

### **Get Help**
- Documentation: [DOCUMENTATION_INDEX.md](DOCUMENTATION_INDEX.md)
- Contributing: [CONTRIBUTING.md](CONTRIBUTING.md)
- Issues: GitHub Issues
- Architecture: [NESTGATE_ROLE_CLARIFICATION.md](NESTGATE_ROLE_CLARIFICATION.md)

---

## 📊 **Documentation Status**

- ✅ **Core Docs**: Complete & current
- ✅ **Showcase**: 8 demos, 18+ files
- ✅ **Architecture**: Crystal clear
- ✅ **API Integration**: NCBI working
- ✅ **Deployment Guides**: Ready
- ✅ **Status**: Production-ready

**Last Updated**: November 11, 2025

---

## 🌟 **Highlights**

> "NestGate is a DATA HUB - it manages data for compute workloads"

> "8 production demos proving $92,077+ in savings"

> "800x faster Git clones, 120x faster model loading"

> "Works standalone OR in ecosystem with Songbird"

---

*NestGate Documentation Map*  
*Your guide to navigating all documentation*  
*Updated: November 11, 2025*

