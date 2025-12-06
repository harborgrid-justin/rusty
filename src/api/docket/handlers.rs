use crate::api::docket::service::DocketService;
use crate::error::AppError;
use crate::models::DocketEntry;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListDocketEntriesQuery {
    pub case_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDocketEntryRequest {
    pub case_id: Uuid,
    pub sequence_number: i32,
    pub entry_type: String,
    pub title: String,
    pub description: Option<String>,
    pub date: Option<DateTime<Utc>>,
    pub filed_by: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDocketEntryRequest {
    pub title: Option<String>,
    pub description: Option<String>,
}

/// List docket entries for a case
#[utoipa::path(
    get,
    path = "/api/docket",
    params(
        ("case_id" = Uuid, Query, description = "Case ID to filter docket entries")
    ),
    responses(
        (status = 200, description = "List of docket entries", body = Vec<DocketEntry>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "docket",
    security(("bearer_auth" = []))
)]
pub async fn list_docket_entries(
    State(service): State<Arc<DocketService>>,
    Query(query): Query<ListDocketEntriesQuery>,
) -> Result<Json<Vec<DocketEntry>>, AppError> {
    let entries = service.list_entries(query.case_id).await?;
    Ok(Json(entries))
}

/// Get a specific docket entry
#[utoipa::path(
    get,
    path = "/api/docket/{id}",
    params(
        ("id" = Uuid, Path, description = "Docket entry ID")
    ),
    responses(
        (status = 200, description = "Docket entry details", body = DocketEntry),
        (status = 404, description = "Docket entry not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "docket",
    security(("bearer_auth" = []))
)]
pub async fn get_docket_entry(
    State(service): State<Arc<DocketService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<DocketEntry>, AppError> {
    let entry = service.get_entry(id).await?;
    Ok(Json(entry))
}

/// Create a new docket entry
#[utoipa::path(
    post,
    path = "/api/docket",
    request_body = CreateDocketEntryRequest,
    responses(
        (status = 201, description = "Docket entry created", body = DocketEntry),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "docket",
    security(("bearer_auth" = []))
)]
pub async fn create_docket_entry(
    State(service): State<Arc<DocketService>>,
    Json(req): Json<CreateDocketEntryRequest>,
) -> Result<(StatusCode, Json<DocketEntry>), AppError> {
    let entry = service
        .create_entry(
            req.case_id,
            req.sequence_number,
            req.entry_type,
            req.title,
            req.description,
            req.date,
            req.filed_by,
        )
        .await?;
    Ok((StatusCode::CREATED, Json(entry)))
}

/// Update a docket entry
#[utoipa::path(
    put,
    path = "/api/docket/{id}",
    params(
        ("id" = Uuid, Path, description = "Docket entry ID")
    ),
    request_body = UpdateDocketEntryRequest,
    responses(
        (status = 200, description = "Docket entry updated", body = DocketEntry),
        (status = 404, description = "Docket entry not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "docket",
    security(("bearer_auth" = []))
)]
pub async fn update_docket_entry(
    State(service): State<Arc<DocketService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDocketEntryRequest>,
) -> Result<Json<DocketEntry>, AppError> {
    let entry = service.update_entry(id, req.title, req.description).await?;
    Ok(Json(entry))
}

/// Delete a docket entry
#[utoipa::path(
    delete,
    path = "/api/docket/{id}",
    params(
        ("id" = Uuid, Path, description = "Docket entry ID")
    ),
    responses(
        (status = 204, description = "Docket entry deleted"),
        (status = 404, description = "Docket entry not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "docket",
    security(("bearer_auth" = []))
)]
pub async fn delete_docket_entry(
    State(service): State<Arc<DocketService>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_entry(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
