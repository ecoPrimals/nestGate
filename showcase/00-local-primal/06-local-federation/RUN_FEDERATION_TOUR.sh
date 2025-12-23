#!/bin/bash
# 🌐 RUN FEDERATION TOUR - Automated Walkthrough
# Complete guided tour of NestGate federation capabilities

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

clear

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${CYAN}🌐 NestGate Federation Tour${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Welcome to the NestGate Federation automated tour!"
echo ""
echo "This tour will guide you through:"
echo "  • Two-node mesh networking (10 min)"
echo "  • ZFS replication (15 min)"
echo "  • Load balancing & failover (10 min)"
echo "  • Three-node production cluster (15 min)"
echo ""
echo "Total duration: ~50 minutes"
echo ""
echo -e "${YELLOW}Press Enter to begin the tour...${NC}"
read

# Demo 1: Two-Node Mesh
clear
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Demo 1/4: Two-Node Mesh (10 minutes)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 What you'll learn:"
echo "   • Automatic peer discovery via mDNS"
echo "   • Zero-configuration mesh formation"
echo "   • Bi-directional communication"
echo "   • Health monitoring"
echo ""
echo -e "${YELLOW}Press Enter to start Demo 1...${NC}"
read

cd 01-two-node-mesh
./demo-mesh.sh

echo ""
echo -e "${GREEN}✅ Demo 1 Complete!${NC}"
echo ""
echo -e "${YELLOW}Press Enter to continue to Demo 2...${NC}"
read
cd ..

# Demo 2: ZFS Replication
clear
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Demo 2/4: ZFS Replication (15 minutes)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 What you'll learn:"
echo "   • ZFS send/receive replication"
echo "   • Incremental updates (only changes)"
echo "   • Automatic compression (4x savings)"
echo "   • Block-level efficiency"
echo ""
echo -e "${YELLOW}Press Enter to start Demo 2...${NC}"
read

cd 02-replication
./demo-zfs-replication.sh

echo ""
echo -e "${GREEN}✅ Demo 2 Complete!${NC}"
echo ""
echo -e "${YELLOW}Press Enter to continue to Demo 3...${NC}"
read
cd ..

# Demo 3: Load Balancing
clear
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Demo 3/4: Load Balancing & Failover (10 minutes)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 What you'll learn:"
echo "   • Automatic load distribution"
echo "   • Failover detection (5-8 seconds)"
echo "   • Zero-downtime operation"
echo "   • Graceful recovery"
echo ""
echo -e "${YELLOW}Press Enter to start Demo 3...${NC}"
read

cd 03-load-balancing
./demo-failover.sh

echo ""
echo -e "${GREEN}✅ Demo 3 Complete!${NC}"
echo ""
echo -e "${YELLOW}Press Enter to continue to Demo 4 (final)...${NC}"
read
cd ..

# Demo 4: Three-Node Cluster
clear
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${BLUE}Demo 4/4: Three-Node Cluster (15 minutes)${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "📚 What you'll learn:"
echo "   • Full mesh topology (3 connections)"
echo "   • Distributed writes with replication"
echo "   • Strong consistency guarantees"
echo "   • Quorum-based consensus"
echo "   • Production-grade clustering"
echo ""
echo -e "${YELLOW}Press Enter to start Demo 4...${NC}"
read

cd 04-three-node-cluster
./demo-cluster.sh

echo ""
echo -e "${GREEN}✅ Demo 4 Complete!${NC}"
echo ""
cd ..

# Final Summary
clear
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "${GREEN}🎉 Federation Tour Complete!${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "Congratulations! You've completed the full federation showcase."
echo ""
echo "📊 Summary:"
echo "   ✅ Demo 1: Two-node mesh networking"
echo "   ✅ Demo 2: ZFS replication"
echo "   ✅ Demo 3: Load balancing & failover"
echo "   ✅ Demo 4: Three-node production cluster"
echo ""
echo "🎓 What you've learned:"
echo "   • Zero-configuration peer discovery"
echo "   • Full mesh topology benefits"
echo "   • ZFS replication efficiency"
echo "   • Automatic failover mechanisms"
echo "   • Strong consistency guarantees"
echo "   • Quorum-based consensus"
echo "   • Production-grade clustering"
echo ""
echo "💡 Key Takeaways:"
echo "   • NestGate federates automatically (zero config)"
echo "   • Full mesh = no single point of failure"
echo "   • Incremental replication = 91% bandwidth savings"
echo "   • Zero downtime during node failures"
echo "   • Strong consistency via quorum"
echo "   • Enterprise-grade on commodity hardware"
echo ""
echo "🏆 You now understand:"
echo "   • How to deploy multi-node NestGate"
echo "   • Why federation matters for sovereignty"
echo "   • How to achieve high availability"
echo "   • What makes NestGate production-ready"
echo ""
echo "📚 Additional Resources:"
echo "   • Federation README: ./README.md"
echo "   • Individual demo READMEs in each directory"
echo "   • Main showcase: ../README.md"
echo ""
echo "⏭️  What's Next?"
echo ""
echo "   Option A: Ecosystem Integration"
echo "      cd ../../02_ecosystem_integration"
echo "      See NestGate work with other primals"
echo ""
echo "   Option B: Re-run Individual Demos"
echo "      cd ./01-two-node-mesh && ./demo-mesh.sh"
echo "      cd ./02-replication && ./demo-zfs-replication.sh"
echo "      cd ./03-load-balancing && ./demo-failover.sh"
echo "      cd ./04-three-node-cluster && ./demo-cluster.sh"
echo ""
echo "   Option C: Complete Local Showcase"
echo "      cd ../ && ./RUN_ME_FIRST.sh"
echo "      Experience all local capabilities (60 min)"
echo ""
echo "🌐 Thank you for exploring NestGate Federation!"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

