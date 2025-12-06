use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Document model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Document {
    pub id: Uuid,
    pub case_id: Uuid,
    pub title: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub doc_type: String,
    pub content: Option<String>,
    pub upload_date: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub tags: Vec<String>,
    pub file_size: Option<String>,
    pub source_module: Option<String>,
    pub status: Option<String>,
    pub author_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: Option<i32>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Document version model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DocumentVersion {
    pub id: Uuid,
    pub document_id: Uuid,
    pub version_number: i32,
    pub uploaded_by: String,
    pub upload_date: DateTime<Utc>,
    pub content_snapshot: Option<String>,
    pub storage_key: Option<String>,
    pub author: Option<String>,
    pub author_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Create document request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDocumentRequest {
    pub case_id: Uuid,
    pub title: String,
    pub doc_type: String,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}
