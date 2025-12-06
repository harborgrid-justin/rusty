use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Dashboard statistics
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DashboardStats {
    pub active_cases: i64,
    pub pending_motions: i64,
    pub billable_hours: f64,
    pub high_risks: i64,
    pub total_revenue: f64,
    pub open_tasks: i64,
}

/// Chart data point
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ChartData {
    pub name: String,
    pub count: i64,
}

/// Alert item
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Alert {
    pub id: String,
    pub message: String,
    pub detail: String,
    pub time: String,
    pub case_id: Option<String>,
}
