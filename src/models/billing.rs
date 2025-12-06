use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Client model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub industry: String,
    pub status: String,
    pub total_billed: f64,
    pub matters: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Time entry model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TimeEntry {
    pub id: Uuid,
    pub case_id: Uuid,
    pub user_id: Uuid,
    pub date: DateTime<Utc>,
    pub duration: f64,
    pub description: String,
    pub rate: f64,
    pub total: f64,
    pub status: String,
    pub invoice_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Invoice model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Invoice {
    pub id: Uuid,
    pub client: String,
    pub matter: String,
    pub case_id: Uuid,
    pub date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub amount: f64,
    pub status: String,
    pub items: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Create time entry request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTimeEntryRequest {
    pub case_id: Uuid,
    pub date: DateTime<Utc>,
    pub duration: f64,
    pub description: String,
    pub rate: f64,
}
