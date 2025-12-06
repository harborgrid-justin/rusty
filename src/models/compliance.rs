use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Risk category enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "risk_category", rename_all = "PascalCase")]
pub enum RiskCategory {
    Legal,
    Financial,
    Reputational,
    Operational,
    Strategic,
}

/// Risk level enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "risk_level", rename_all = "PascalCase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Risk status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "risk_status", rename_all = "PascalCase")]
pub enum RiskStatus {
    Identified,
    Mitigated,
    Accepted,
    Closed,
}

/// Risk model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Risk {
    pub id: Uuid,
    pub case_id: Uuid,
    pub title: String,
    pub description: String,
    pub category: RiskCategory,
    pub probability: RiskLevel,
    pub impact: RiskLevel,
    pub status: RiskStatus,
    pub date_identified: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub mitigation_plan: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Conflict check model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ConflictCheck {
    pub id: Uuid,
    pub entity_name: String,
    pub date: DateTime<Utc>,
    pub status: String,
    pub found_in: Vec<String>,
    pub checked_by_id: Option<Uuid>,
    pub checked_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Audit log model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct AuditLog {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub user_name: String,
    pub action: String,
    pub resource: String,
    pub ip: Option<String>,
    pub hash: Option<String>,
    pub prev_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}
