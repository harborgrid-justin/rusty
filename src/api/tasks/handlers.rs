use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{error::AppError, models::WorkflowTask};

use super::TaskService;

#[derive(Debug, Deserialize)]
pub struct ListTasksQuery {
    pub case_id: Option<Uuid>,
    pub status: Option<String>,
    pub assignee_id: Option<Uuid>,
}

/// List workflow tasks
#[utoipa::path(
    get,
    path = "/api/tasks",
    tag = "tasks",
    responses(
        (status = 200, description = "Tasks retrieved successfully", body = Vec<WorkflowTask>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_tasks(
    State(service): State<Arc<TaskService>>,
    Query(params): Query<ListTasksQuery>,
) -> Result<impl IntoResponse, AppError> {
    let tasks = service.list_tasks(params).await?;
    Ok(Json(tasks))
}

/// Get a specific task
#[utoipa::path(
    get,
    path = "/api/tasks/{id}",
    tag = "tasks",
    params(
        ("id" = Uuid, Path, description = "Task ID")
    ),
    responses(
        (status = 200, description = "Task retrieved successfully", body = WorkflowTask),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_task(
    State(service): State<Arc<TaskService>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let task = service.get_task(id).await?;
    Ok(Json(task))
}
