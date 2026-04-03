// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Axum router builder for zero-cost ZFS API routes.

use axum::Router;
use axum::extract::Path;
use axum::response::Json;
use axum::routing::{delete, get, post};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::zfs::types::PoolConfig;

use super::dataset_handler::ZeroCostDatasetHandler;
use super::pool_handler::ZeroCostPoolHandler;

/// **ZERO-COST ROUTER BUILDER**
/// High-performance router construction with compile-time optimization
pub struct ZeroCostRouterBuilder<const MAX_ROUTES: usize = 100, const MAX_MIDDLEWARE: usize = 10> {
    routes: Vec<(&'static str, &'static str)>, // (method, path)
    middleware_count: usize,
    _phantom: PhantomData<()>,
}
impl<const MAX_ROUTES: usize, const MAX_MIDDLEWARE: usize> Default
    for ZeroCostRouterBuilder<MAX_ROUTES, MAX_MIDDLEWARE>
{
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_ROUTES: usize, const MAX_MIDDLEWARE: usize>
    ZeroCostRouterBuilder<MAX_ROUTES, MAX_MIDDLEWARE>
{
    /// Create new router builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            routes: Vec::with_capacity(MAX_ROUTES),
            middleware_count: 0,
            _phantom: PhantomData,
        }
    }

    /// Check if we can add more routes
    #[must_use]
    pub const fn can_add_route(&self) -> bool {
        self.routes.len() < MAX_ROUTES
    }

    /// Check if we can add more middleware
    #[must_use]
    pub const fn can_add_middleware(&self) -> bool {
        self.middleware_count < MAX_MIDDLEWARE
    }

    /// Get max routes at compile-time
    #[must_use]
    pub const fn max_routes() -> usize {
        MAX_ROUTES
    }

    /// Get max middleware at compile-time
    #[must_use]
    pub const fn max_middleware() -> usize {
        MAX_MIDDLEWARE
    }

    /// Build ZFS API router with zero-cost patterns
    pub fn build_zfs_api_router(
        pool_handler: Arc<ZeroCostPoolHandler<1000, 30000>>,
        _dataset_handler: Arc<ZeroCostDatasetHandler<serde_json::Value, 1000, 30000>>,
    ) -> Router {
        Router::new()
            // Pool routes
            .route(
                "/api/v1/pools",
                get({
                    let handler = pool_handler.clone();
                    move || async move { handler.handle_list_pools() }
                }),
            )
            .route(
                "/api/v1/pools/:name",
                get({
                    let handler = pool_handler.clone();
                    move |Path(name): Path<String>| async move { handler.handle_get_pool(name) }
                }),
            )
            .route(
                "/api/v1/pools",
                post({
                    let handler = pool_handler.clone();
                    move |Json(config): Json<PoolConfig>| async move {
                        handler.handle_create_pool(config)
                    }
                }),
            )
            .route(
                "/api/v1/pools/:name",
                delete({
                    let handler = pool_handler;
                    move |Path(name): Path<String>| async move { handler.handle_delete_pool(name) }
                }),
            )
            // Dataset routes would be similar...
            .route("/api/v1/datasets", get(|| async { "Datasets endpoint" }))
            .route("/api/v1/health", get(|| async { "OK" }))
    }
}
