# DEPRECATED: NestGate UI Component

**Status**: DEPRECATED  
**Date**: 2025-01-06  
**Reason**: Moving to pure Rust API-only architecture

## What This Component Was

The `nestgate-ui` component was a **native Rust GUI application** built using:
- **egui** - Immediate mode GUI framework
- **eframe** - Cross-platform GUI runtime
- **Native Rust** - No web dependencies

**Note**: Despite the misleading README mentioning npm/React, this was actually a pure Rust native application.

## Migration Path

This UI component is being deprecated in favor of:
- **Pure Rust API interfaces** - All functionality exposed via well-structured APIs
- **AI-driven interfaces** - Integration with AI systems that can interact with the APIs
- **Headless operation** - No GUI dependencies for server/enterprise deployments

## Timeline

- **Current**: Component marked as deprecated (2025-01-26)
- **Future**: Component will be removed in next major version (v3.0)
- **Migration**: All functionality available via comprehensive REST APIs
- **Replacement**: biomeOS UI integration via API endpoints

## For Developers

If you need UI functionality:
1. Use the REST API endpoints in `nestgate-api`
2. Build custom interfaces using the provided APIs
3. Integrate with AI systems via the MCP (Model Context Protocol) interfaces

## Removal

This component will be removed in a future major version. The core functionality remains available through the API layer. 