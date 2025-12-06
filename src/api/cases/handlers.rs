use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    error::AppError,
    models::{Case, CaseResponse, CreateCaseRequest, Party, UpdateCaseRequest},
};

use super::CaseService;

/// Query parameters for listing cases
#[derive(Debug, Deserialize)]
pub struct ListCasesQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub status: Option<String>,
    pub search: Option<String>,
}

/// List all cases
#[utoipa::path(
    get,
    path = "/api/cases",
    tag = "cases",
    params(
        ("page" = Option<i32>, Query, description = "Page number"),
        ("per_page" = Option<i32>, Query, description = "Items per page"),
        ("status" = Option<String>, Query, description = "Filter by status"),
        ("search" = Option<String>, Query, description = "Search term"),
    ),
    responses(
        (status = 200, description = "Cases retrieved successfully", body = Vec<Case>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_cases(
    State(service): State<Arc<CaseService>>,
    Query(params): Query<ListCasesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let cases = service.list_cases(params).await?;
    Ok(Json(cases))
}

/// Get a specific case by ID
#[utoipa::path(
    get,
    path = "/api/cases/{id}",
    tag = "cases",
    params(
        ("id" = Uuid, Path, description = "Case ID")
    ),
    responses(
        (status = 200, description = "Case retrieved successfully", body = CaseResponse),
        (status = 404, description = "Case not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_case(
    State(service): State<Arc<CaseService>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let case = service.get_case(id).await?;
    Ok(Json(case))
}

/// Create a new case
#[utoipa::path(
    post,
    path = "/api/cases",
    tag = "cases",
    request_body = CreateCaseRequest,
    responses(
        (status = 201, description = "Case created successfully", body = Case),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_case(
    State(service): State<Arc<CaseService>>,
    Json(payload): Json<CreateCaseRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let case = service.create_case(payload).await?;
    Ok((StatusCode::CREATED, Json(case)))
}

/// Update a case
#[utoipa::path(
    put,
    path = "/api/cases/{id}",
    tag = "cases",
    params(
        ("id" = Uuid, Path, description = "Case ID")
    ),
    request_body = UpdateCaseRequest,
    responses(
        (status = 200, description = "Case updated successfully", body = Case),
        (status = 404, description = "Case not found"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_case(
    State(service): State<Arc<CaseService>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCaseRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let case = service.update_case(id, payload).await?;
    Ok(Json(case))
}

/// Delete a case
#[utoipa::path(
    delete,
    path = "/api/cases/{id}",
    tag = "cases",
    params(
        ("id" = Uuid, Path, description = "Case ID")
    ),
    responses(
        (status = 204, description = "Case deleted successfully"),
        (status = 404, description = "Case not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_case(
    State(service): State<Arc<CaseService>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service.delete_case(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Get parties for a case
#[utoipa::path(
    get,
    path = "/api/cases/{id}/parties",
    tag = "cases",
    params(
        ("id" = Uuid, Path, description = "Case ID")
    ),
    responses(
        (status = 200, description = "Parties retrieved successfully", body = Vec<Party>),
        (status = 404, description = "Case not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_case_parties(
    State(service): State<Arc<CaseService>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let parties = service.get_case_parties(id).await?;
    Ok(Json(parties))
}
