use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Case status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "case_status", rename_all = "PascalCase")]
pub enum CaseStatus {
    #[serde(rename = "Pre-Filing")]
    #[sqlx(rename = "Pre-Filing")]
    PreFiling,
    Discovery,
    Trial,
    Settled,
    Closed,
    Appeal,
    Transferred,
}

/// Matter type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "matter_type", rename_all = "PascalCase")]
pub enum MatterType {
    Litigation,
    #[serde(rename = "M&A")]
    #[sqlx(rename = "M&A")]
    MA,
    IP,
    #[serde(rename = "Real Estate")]
    #[sqlx(rename = "Real Estate")]
    RealEstate,
    General,
    Appeal,
}

/// Billing model enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "billing_model", rename_all = "PascalCase")]
pub enum BillingModel {
    Hourly,
    Fixed,
    Contingency,
    Hybrid,
}

/// Case model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Case {
    pub id: Uuid,
    pub title: String,
    pub client: String,
    pub client_id: Option<Uuid>,
    pub matter_type: MatterType,
    pub matter_sub_type: Option<String>,
    pub status: CaseStatus,
    pub filing_date: DateTime<Utc>,
    pub description: Option<String>,
    pub value: Option<f64>,
    pub jurisdiction: Option<String>,
    pub court: Option<String>,
    pub judge: Option<String>,
    pub magistrate_judge: Option<String>,
    pub opposing_counsel: Option<String>,
    pub orig_case_number: Option<String>,
    pub orig_court: Option<String>,
    pub orig_judgment_date: Option<DateTime<Utc>>,
    pub notice_of_appeal_date: Option<DateTime<Utc>>,
    pub owner_id: Option<Uuid>,
    pub owner_org_id: Option<Uuid>,
    pub lead_case_id: Option<Uuid>,
    pub is_consolidated: Option<bool>,
    pub date_terminated: Option<DateTime<Utc>>,
    pub nature_of_suit: Option<String>,
    pub billing_model: Option<BillingModel>,
    pub pacer_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub version: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Party model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Party {
    pub id: Uuid,
    pub case_id: Uuid,
    pub name: String,
    pub role: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub party_type: String,
    pub contact: Option<String>,
    pub counsel: Option<String>,
    pub party_group: Option<String>,
    pub linked_org_id: Option<Uuid>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub representation_type: Option<String>,
    pub attorneys: Option<serde_json::Value>,
    pub pacer_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub version: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Create case request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCaseRequest {
    #[validate(length(min = 1, max = 500))]
    pub title: String,
    
    #[validate(length(min = 1, max = 255))]
    pub client: String,
    
    pub client_id: Option<Uuid>,
    pub matter_type: MatterType,
    pub matter_sub_type: Option<String>,
    pub status: Option<CaseStatus>,
    pub filing_date: DateTime<Utc>,
    pub description: Option<String>,
    pub value: Option<f64>,
    pub jurisdiction: Option<String>,
    pub court: Option<String>,
    pub judge: Option<String>,
    pub billing_model: Option<BillingModel>,
}

/// Update case request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCaseRequest {
    pub title: Option<String>,
    pub status: Option<CaseStatus>,
    pub description: Option<String>,
    pub value: Option<f64>,
    pub jurisdiction: Option<String>,
    pub court: Option<String>,
    pub judge: Option<String>,
    pub magistrate_judge: Option<String>,
    pub opposing_counsel: Option<String>,
    pub billing_model: Option<BillingModel>,
}

/// Case response with related data
#[derive(Debug, Serialize, ToSchema)]
pub struct CaseResponse {
    #[serde(flatten)]
    pub case: Case,
    pub parties: Vec<Party>,
}

/// Create party request
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePartyRequest {
    pub case_id: Uuid,
    
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    
    #[validate(length(min = 1, max = 100))]
    pub role: String,
    
    pub party_type: String,
    pub contact: Option<String>,
    pub counsel: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}
