use crate::api::evidence::service::EvidenceService;
use crate::error::AppError;
use crate::models::EvidenceItem;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListEvidenceQuery {
    pub case_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEvidenceRequest {
    pub case_id: Uuid,
    pub title: String,
    pub evidence_type: String,
    pub description: String,
    pub collected_by: String,
    pub custodian: String,
    pub location: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEvidenceRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub custodian: Option<String>,
    pub location: Option<String>,
    pub admissibility: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// List evidence items for a case
#[utoipa::path(
    get,
    path = "/api/evidence",
    params(
        ("case_id" = Uuid, Query, description = "Case ID to filter evidence items")
    ),
    responses(
        (status = 200, description = "List of evidence items", body = Vec<EvidenceItem>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "evidence",
    security(("bearer_auth" = []))
)]
pub async fn list_evidence(
    State(service): State<Arc<EvidenceService>>,
    Query(query): Query<ListEvidenceQuery>,
) -> Result<Json<Vec<EvidenceItem>>, AppError> {
    let items = service.list_evidence(query.case_id).await?;
    Ok(Json(items))
}

/// Get a specific evidence item
#[utoipa::path(
    get,
    path = "/api/evidence/{id}",
    params(
        ("id" = Uuid, Path, description = "Evidence item ID")
    ),
    responses(
        (status = 200, description = "Evidence item details", body = EvidenceItem),
        (status = 404, description = "Evidence item not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "evidence",
    security(("bearer_auth" = []))
)]
pub async fn get_evidence(
    State(service): State<Arc<EvidenceService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<EvidenceItem>, AppError> {
    let item = service.get_evidence(id).await?;
    Ok(Json(item))
}

/// Create a new evidence item
#[utoipa::path(
    post,
    path = "/api/evidence",
    request_body = CreateEvidenceRequest,
    responses(
        (status = 201, description = "Evidence item created", body = EvidenceItem),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "evidence",
    security(("bearer_auth" = []))
)]
pub async fn create_evidence(
    State(service): State<Arc<EvidenceService>>,
    Json(req): Json<CreateEvidenceRequest>,
) -> Result<(StatusCode, Json<EvidenceItem>), AppError> {
    let tags = req.tags.unwrap_or_default();
    let item = service
        .create_evidence(crate::api::evidence::service::CreateEvidenceParams {
            case_id: req.case_id,
            title: req.title,
            evidence_type: req.evidence_type,
            description: req.description,
            collected_by: req.collected_by,
            custodian: req.custodian,
            location: req.location,
            tags,
        })
        .await?;
    Ok((StatusCode::CREATED, Json(item)))
}

/// Update an evidence item
#[utoipa::path(
    put,
    path = "/api/evidence/{id}",
    params(
        ("id" = Uuid, Path, description = "Evidence item ID")
    ),
    request_body = UpdateEvidenceRequest,
    responses(
        (status = 200, description = "Evidence item updated", body = EvidenceItem),
        (status = 404, description = "Evidence item not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "evidence",
    security(("bearer_auth" = []))
)]
pub async fn update_evidence(
    State(service): State<Arc<EvidenceService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEvidenceRequest>,
) -> Result<Json<EvidenceItem>, AppError> {
    let item = service
        .update_evidence(
            id,
            crate::api::evidence::service::UpdateEvidenceParams {
                title: req.title,
                description: req.description,
                custodian: req.custodian,
                location: req.location,
                admissibility: req.admissibility,
                tags: req.tags,
            },
        )
        .await?;
    Ok(Json(item))
}

/// Delete an evidence item
#[utoipa::path(
    delete,
    path = "/api/evidence/{id}",
    params(
        ("id" = Uuid, Path, description = "Evidence item ID")
    ),
    responses(
        (status = 204, description = "Evidence item deleted"),
        (status = 404, description = "Evidence item not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "evidence",
    security(("bearer_auth" = []))
)]
pub async fn delete_evidence(
    State(service): State<Arc<EvidenceService>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_evidence(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
