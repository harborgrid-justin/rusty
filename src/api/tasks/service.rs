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
        let mut query = String::from(
            "SELECT * FROM workflow_tasks WHERE deleted_at IS NULL"
        );
        let mut conditions = Vec::new();

        if let Some(case_id) = params.case_id {
            conditions.push(format!("case_id = '{}'", case_id));
        }

        if let Some(ref status) = params.status {
            conditions.push(format!("status::text = '{}'", status));
        }

        if let Some(assignee_id) = params.assignee_id {
            conditions.push(format!("assignee_id = '{}'", assignee_id));
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" ORDER BY due_date ASC");

        let tasks = sqlx::query_as::<_, WorkflowTask>(&query)
            .fetch_all(&self.db)
            .await?;

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
