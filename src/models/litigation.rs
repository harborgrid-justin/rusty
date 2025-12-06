use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Motion type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "motion_type", rename_all = "PascalCase")]
pub enum MotionType {
    Dismiss,
    #[serde(rename = "Summary Judgment")]
    #[sqlx(rename = "Summary Judgment")]
    SummaryJudgment,
    #[serde(rename = "Compel Discovery")]
    #[sqlx(rename = "Compel Discovery")]
    CompelDiscovery,
    #[serde(rename = "In Limine")]
    #[sqlx(rename = "In Limine")]
    InLimine,
    Continuance,
    Sanctions,
}

/// Motion status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "motion_status", rename_all = "PascalCase")]
pub enum MotionStatus {
    Draft,
    Filed,
    #[serde(rename = "Opposition Served")]
    #[sqlx(rename = "Opposition Served")]
    OppositionServed,
    #[serde(rename = "Reply Served")]
    #[sqlx(rename = "Reply Served")]
    ReplyServed,
    #[serde(rename = "Hearing Set")]
    #[sqlx(rename = "Hearing Set")]
    HearingSet,
    Submitted,
    Decided,
    Withdrawn,
}

/// Motion outcome enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "motion_outcome", rename_all = "PascalCase")]
pub enum MotionOutcome {
    Granted,
    Denied,
    Withdrawn,
    Moot,
}

/// Docket entry type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "docket_entry_type", rename_all = "PascalCase")]
pub enum DocketEntryType {
    Filing,
    Order,
    Notice,
    #[serde(rename = "Minute Entry")]
    #[sqlx(rename = "Minute Entry")]
    MinuteEntry,
    Exhibit,
    Hearing,
}

/// Evidence type enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "evidence_type", rename_all = "PascalCase")]
pub enum EvidenceType {
    Physical,
    Digital,
    Document,
    Testimony,
    Forensic,
}

/// Admissibility status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "admissibility_status", rename_all = "PascalCase")]
pub enum AdmissibilityStatus {
    Admissible,
    Challenged,
    Inadmissible,
    Pending,
}

/// Motion model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Motion {
    pub id: Uuid,
    pub case_id: Uuid,
    pub title: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub motion_type: MotionType,
    pub status: MotionStatus,
    pub outcome: Option<MotionOutcome>,
    pub filing_date: Option<DateTime<Utc>>,
    pub hearing_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Docket entry model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DocketEntry {
    pub id: Uuid,
    pub sequence_number: i32,
    pub pacer_sequence_number: Option<i32>,
    pub case_id: Uuid,
    pub date: DateTime<Utc>,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub entry_type: DocketEntryType,
    pub title: String,
    pub description: Option<String>,
    pub filed_by: Option<String>,
    pub is_sealed: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Evidence item model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct EvidenceItem {
    pub id: Uuid,
    pub case_id: Uuid,
    pub title: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub evidence_type: EvidenceType,
    pub description: String,
    pub collection_date: DateTime<Utc>,
    pub collected_by: String,
    pub custodian: String,
    pub location: String,
    pub admissibility: AdmissibilityStatus,
    pub tags: Vec<String>,
    pub tracking_uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Trial exhibit model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TrialExhibit {
    pub id: Uuid,
    pub case_id: Uuid,
    pub exhibit_number: String,
    pub title: String,
    pub date_marked: DateTime<Utc>,
    pub party: String,
    pub status: String,
    pub file_type: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
