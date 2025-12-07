use crate::api::documents::service::DocumentService;
use crate::error::AppError;
use crate::models::{Claims, CreateDocumentRequest, Document};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ListDocumentsQuery {
    pub case_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDocumentRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// List documents with optional filtering
#[utoipa::path(
    get,
    path = "/api/documents",
    params(
        ("case_id" = Option<Uuid>, Query, description = "Filter by case ID")
    ),
    responses(
        (status = 200, description = "List of documents", body = Vec<Document>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "documents",
    security(("bearer_auth" = []))
)]
pub async fn list_documents(
    State(service): State<Arc<DocumentService>>,
    Query(query): Query<ListDocumentsQuery>,
) -> Result<Json<Vec<Document>>, AppError> {
    let docs = service.list_documents(query.case_id).await?;
    Ok(Json(docs))
}

/// Get document by ID
#[utoipa::path(
    get,
    path = "/api/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    responses(
        (status = 200, description = "Document details", body = Document),
        (status = 404, description = "Document not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "documents",
    security(("bearer_auth" = []))
)]
pub async fn get_document(
    State(service): State<Arc<DocumentService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Document>, AppError> {
    let doc = service.get_document(id).await?;
    Ok(Json(doc))
}

/// Create a new document
#[utoipa::path(
    post,
    path = "/api/documents",
    request_body = CreateDocumentRequest,
    responses(
        (status = 201, description = "Document created", body = Document),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "documents",
    security(("bearer_auth" = []))
)]
pub async fn create_document(
    State(service): State<Arc<DocumentService>>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateDocumentRequest>,
) -> Result<(StatusCode, Json<Document>), AppError> {
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::BadRequest("Invalid user ID in token".to_string()))?;

    let doc = service.create_document(req, user_id).await?;
    Ok((StatusCode::CREATED, Json(doc)))
}

/// Update document
#[utoipa::path(
    put,
    path = "/api/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    request_body = UpdateDocumentRequest,
    responses(
        (status = 200, description = "Document updated", body = Document),
        (status = 404, description = "Document not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "documents",
    security(("bearer_auth" = []))
)]
pub async fn update_document(
    State(service): State<Arc<DocumentService>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateDocumentRequest>,
) -> Result<Json<Document>, AppError> {
    let doc = service
        .update_document(id, req.title, req.content, req.tags)
        .await?;
    Ok(Json(doc))
}

/// Delete document (soft delete)
#[utoipa::path(
    delete,
    path = "/api/documents/{id}",
    params(
        ("id" = Uuid, Path, description = "Document ID")
    ),
    responses(
        (status = 204, description = "Document deleted"),
        (status = 404, description = "Document not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "documents",
    security(("bearer_auth" = []))
)]
pub async fn delete_document(
    State(service): State<Arc<DocumentService>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_document(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
