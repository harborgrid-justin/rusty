use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

use crate::error::AppError;

use super::{models::*, DashboardService};

/// Get dashboard statistics
#[utoipa::path(
    get,
    path = "/api/dashboard/stats",
    tag = "dashboard",
    responses(
        (status = 200, description = "Statistics retrieved successfully", body = DashboardStats),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_stats(
    State(service): State<Arc<DashboardService>>,
) -> Result<impl IntoResponse, AppError> {
    let stats = service.get_stats().await?;
    Ok(Json(stats))
}

/// Get chart data for case status distribution
#[utoipa::path(
    get,
    path = "/api/dashboard/chart-data",
    tag = "dashboard",
    responses(
        (status = 200, description = "Chart data retrieved successfully", body = Vec<ChartData>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_chart_data(
    State(service): State<Arc<DashboardService>>,
) -> Result<impl IntoResponse, AppError> {
    let data = service.get_chart_data().await?;
    Ok(Json(data))
}

/// Get recent alerts
#[utoipa::path(
    get,
    path = "/api/dashboard/alerts",
    tag = "dashboard",
    responses(
        (status = 200, description = "Alerts retrieved successfully", body = Vec<Alert>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_alerts(
    State(service): State<Arc<DashboardService>>,
) -> Result<impl IntoResponse, AppError> {
    let alerts = service.get_alerts().await?;
    Ok(Json(alerts))
}
