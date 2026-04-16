// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Native async service traits split by domain (networking, MCP, automation, service registry, security).

mod automation;
mod communication;
mod mcp;
mod security;
mod service;

pub use automation::{NativeAsyncAutomationService, NativeAsyncWorkflowService};
pub use communication::{NativeAsyncCommunicationProvider, NativeAsyncLoadBalancer};
pub use mcp::{NativeAsyncMCPProtocolHandler, NativeAsyncMcpService};
#[expect(
    deprecated,
    reason = "re-export for migration; see trait deprecation note"
)]
pub use security::NativeAsyncSecurityService;
pub use service::NativeAsyncUniversalServiceProvider;
