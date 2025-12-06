use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Communication model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Communication {
    pub id: Uuid,
    pub case_id: Uuid,
    pub user_id: Uuid,
    pub subject: String,
    pub date: DateTime<Utc>,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub comm_type: String,
    pub direction: String,
    pub sender: String,
    pub recipient: String,
    pub preview: Option<String>,
    pub has_attachment: Option<bool>,
    pub status: String,
    pub is_privileged: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Clause model (for knowledge base)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Clause {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub content: String,
    pub version: Option<i32>,
    pub usage_count: Option<i32>,
    pub last_updated: DateTime<Utc>,
    pub risk_rating: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Notification model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub text: String,
    pub time: DateTime<Utc>,
    pub read: Option<bool>,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub notif_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
