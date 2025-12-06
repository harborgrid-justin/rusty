use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Organization type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "organization_type", rename_all = "PascalCase")]
pub enum OrganizationType {
    LawFirm,
    Corporate,
    Government,
    Court,
    Vendor,
}

/// Organization model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub org_type: OrganizationType,
    pub domain: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Group model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Group {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Citation model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Citation {
    pub id: Uuid,
    pub citation: String,
    pub title: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub citation_type: String,
    pub description: Option<String>,
    pub relevance: String,
    pub shepards_signal: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Legal rule type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "legal_rule_type", rename_all = "UPPERCASE")]
pub enum LegalRuleType {
    FRE,
    FRCP,
    FRAP,
    Local,
    State,
}

/// Legal rule model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LegalRule {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub rule_type: LegalRuleType,
    pub level: Option<String>,
    pub summary: Option<String>,
    pub text: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Entity type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "entity_type", rename_all = "PascalCase")]
pub enum EntityType {
    Individual,
    Corporation,
    Court,
    Government,
    Vendor,
    #[serde(rename = "Law Firm")]
    #[sqlx(rename = "Law Firm")]
    LawFirm,
}

/// Legal entity model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct LegalEntity {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub entity_type: EntityType,
    pub roles: Vec<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub status: String,
    pub risk_score: Option<i32>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub linked_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
