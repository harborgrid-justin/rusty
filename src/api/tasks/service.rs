use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, models::WorkflowTask};

use super::handlers::ListTasksQuery;

pub struct TaskService {
    db: PgPool,
}

impl TaskService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn list_tasks(&self, params: ListTasksQuery) -> Result<Vec<WorkflowTask>, AppError> {
        let mut query_str = String::from(
            "SELECT * FROM workflow_tasks WHERE deleted_at IS NULL"
        );
        let mut bind_count = 0;

        if params.case_id.is_some() {
            bind_count += 1;
            query_str.push_str(&format!(" AND case_id = ${}", bind_count));
        }

        if params.status.is_some() {
            bind_count += 1;
            query_str.push_str(&format!(" AND status::text = ${}", bind_count));
        }

        if params.assignee_id.is_some() {
            bind_count += 1;
            query_str.push_str(&format!(" AND assignee_id = ${}", bind_count));
        }

        query_str.push_str(" ORDER BY due_date ASC");

        let mut query = sqlx::query_as::<_, WorkflowTask>(&query_str);

        if let Some(case_id) = params.case_id {
            query = query.bind(case_id);
        }

        if let Some(ref status) = params.status {
            query = query.bind(status);
        }

        if let Some(assignee_id) = params.assignee_id {
            query = query.bind(assignee_id);
        }

        let tasks = query.fetch_all(&self.db).await?;

        Ok(tasks)
    }

    pub async fn get_task(&self, id: Uuid) -> Result<WorkflowTask, AppError> {
        let task = sqlx::query_as::<_, WorkflowTask>(
            "SELECT * FROM workflow_tasks WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(AppError::NotFound("Task not found".to_string()))?;

        Ok(task)
    }
}
