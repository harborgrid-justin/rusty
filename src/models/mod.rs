// Core entities
pub mod user;
pub mod case_management;
pub mod document;
pub mod workflow;
pub mod litigation;
pub mod discovery;
pub mod billing;
pub mod compliance;
pub mod organization;
pub mod communication;

// Re-export commonly used types
pub use user::*;
pub use case_management::*;
pub use document::*;
pub use workflow::*;
pub use litigation::*;
pub use discovery::*;
pub use billing::*;
pub use compliance::*;
pub use organization::*;
pub use communication::*;

// Common response types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Health check response
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub page: i32,
    pub per_page: i32,
    pub total: i64,
    pub total_pages: i32,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

/// Base entity trait for common fields
pub trait BaseEntity {
    fn id(&self) -> uuid::Uuid;
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
}
