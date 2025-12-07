use crate::api::motions::service::MotionService;
use crate::error::AppError;
use crate::models::Motion;
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
pub struct ListMotionsQuery {
    pub case_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateMotionRequest {
    pub case_id: Uuid,
    pub title: String,
    pub motion_type: String,
    pub status: String,
    pub filing_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMotionRequest {
    pub title: Option<String>,
    pub status: Option<String>,
    pub outcome: Option<String>,
    pub hearing_date: Option<DateTime<Utc>>,
}

/// List motions for a case
#[utoipa::path(
    get,
    path = "/api/motions",
    params(
        ("case_id" = Uuid, Query, description = "Case ID to filter motions")
    ),
    responses(
        (status = 200, description = "List of motions", body = Vec<Motion>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "motions",
    security(("bearer_auth" = []))
)]
pub async fn list_motions(
    State(service): State<Arc<MotionService>>,
    Query(query): Query<ListMotionsQuery>,
) -> Result<Json<Vec<Motion>>, AppError> {
    let motions = service.list_motions(query.case_id).await?;
    Ok(Json(motions))
}

/// Get a specific motion
#[utoipa::path(
    get,
    path = "/api/motions/{id}",
    params(
        ("id" = Uuid, Path, description = "Motion ID")
    ),
    responses(
        (status = 200, description = "Motion details", body = Motion),
        (status = 404, description = "Motion not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "motions",
    security(("bearer_auth" = []))
)]
pub async fn get_motion(
    State(service): State<Arc<MotionService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Motion>, AppError> {
    let motion = service.get_motion(id).await?;
    Ok(Json(motion))
}

/// Create a new motion
#[utoipa::path(
    post,
    path = "/api/motions",
    request_body = CreateMotionRequest,
    responses(
        (status = 201, description = "Motion created", body = Motion),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "motions",
    security(("bearer_auth" = []))
)]
pub async fn create_motion(
    State(service): State<Arc<MotionService>>,
    Json(req): Json<CreateMotionRequest>,
) -> Result<(StatusCode, Json<Motion>), AppError> {
    let motion = service
        .create_motion(
            req.case_id,
            req.title,
            req.motion_type,
            req.status,
            req.filing_date,
        )
        .await?;
    Ok((StatusCode::CREATED, Json(motion)))
}

/// Update a motion
#[utoipa::path(
    put,
    path = "/api/motions/{id}",
    params(
        ("id" = Uuid, Path, description = "Motion ID")
    ),
    request_body = UpdateMotionRequest,
    responses(
        (status = 200, description = "Motion updated", body = Motion),
        (status = 404, description = "Motion not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "motions",
    security(("bearer_auth" = []))
)]
pub async fn update_motion(
    State(service): State<Arc<MotionService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMotionRequest>,
) -> Result<Json<Motion>, AppError> {
    let motion = service
        .update_motion(id, req.title, req.status, req.outcome, req.hearing_date)
        .await?;
    Ok(Json(motion))
}

/// Delete a motion
#[utoipa::path(
    delete,
    path = "/api/motions/{id}",
    params(
        ("id" = Uuid, Path, description = "Motion ID")
    ),
    responses(
        (status = 204, description = "Motion deleted"),
        (status = 404, description = "Motion not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "motions",
    security(("bearer_auth" = []))
)]
pub async fn delete_motion(
    State(service): State<Arc<MotionService>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_motion(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
