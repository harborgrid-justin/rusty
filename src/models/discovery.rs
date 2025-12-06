use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Discovery type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "discovery_type", rename_all = "PascalCase")]
pub enum DiscoveryType {
    Production,
    Interrogatory,
    Admission,
    Deposition,
}

/// Discovery status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "discovery_status", rename_all = "PascalCase")]
pub enum DiscoveryStatus {
    Draft,
    Served,
    Responded,
    Overdue,
    Closed,
    #[serde(rename = "Motion Filed")]
    #[sqlx(rename = "Motion Filed")]
    MotionFiled,
}

/// Discovery request model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DiscoveryRequest {
    pub id: Uuid,
    pub case_id: Uuid,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub discovery_type: DiscoveryType,
    pub propounding_party: String,
    pub responding_party: String,
    pub service_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub status: DiscoveryStatus,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Deposition model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Deposition {
    pub id: Uuid,
    pub case_id: Uuid,
    pub witness_name: String,
    pub date: DateTime<Utc>,
    pub location: String,
    pub status: String,
    pub court_reporter: Option<String>,
    pub prep_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// ESI source model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct ESISource {
    pub id: Uuid,
    pub case_id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub source_type: String,
    pub custodian: String,
    pub status: String,
    pub size: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
