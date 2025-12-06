use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

/// Task status enum
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "task_status", rename_all = "PascalCase")]
pub enum TaskStatus {
    Pending,
    #[serde(rename = "In Progress")]
    #[sqlx(rename = "In Progress")]
    InProgress,
    Review,
    Done,
    Completed,
}

/// Workflow task model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct WorkflowTask {
    pub id: Uuid,
    pub title: String,
    pub status: TaskStatus,
    pub assignee: String,
    pub assignee_id: Option<Uuid>,
    pub start_date: Option<DateTime<Utc>>,
    pub due_date: DateTime<Utc>,
    pub priority: String,
    pub description: Option<String>,
    pub case_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub completion: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Project model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Project {
    pub id: Uuid,
    pub case_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub lead: String,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Create task request
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTaskRequest {
    pub title: String,
    pub assignee: String,
    pub assignee_id: Option<Uuid>,
    pub due_date: DateTime<Utc>,
    pub priority: String,
    pub description: Option<String>,
    pub case_id: Option<Uuid>,
}
