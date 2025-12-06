use axum::{extract::State, Json};
use chrono::Utc;
use std::sync::Arc;

use crate::api::UserService;
use crate::error::Result;
use crate::models::HealthResponse;

/// Health check handler
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
        (status = 503, description = "Service is unhealthy"),
    ),
    tag = "health"
)]
pub async fn health_check(State(service): State<Arc<UserService>>) -> Result<Json<HealthResponse>> {
    // Check database connectivity through service
    service.db.health_check().await?;

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    }))
}

/// Readiness check handler (for Kubernetes)
#[utoipa::path(
    get,
    path = "/ready",
    responses(
        (status = 200, description = "Service is ready"),
    ),
    tag = "health"
)]
pub async fn readiness_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ready".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    })
}

/// Liveness check handler (for Kubernetes)
#[utoipa::path(
    get,
    path = "/live",
    responses(
        (status = 200, description = "Service is alive"),
    ),
    tag = "health"
)]
pub async fn liveness_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "alive".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    })
}
