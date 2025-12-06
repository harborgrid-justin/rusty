use sqlx::PgPool;

use crate::error::AppError;

use super::models::*;

pub struct DashboardService {
    db: PgPool,
}

impl DashboardService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_stats(&self) -> Result<DashboardStats, AppError> {
        // Get active cases count
        let active_cases: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cases WHERE status != 'Closed' AND deleted_at IS NULL"
        )
        .fetch_one(&self.db)
        .await?;

        // Get pending motions count
        let pending_motions: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM motions WHERE status IN ('Draft', 'Filed') AND deleted_at IS NULL"
        )
        .fetch_one(&self.db)
        .await?;

        // Get total billable hours (placeholder calculation)
        let billable_hours: f64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(duration), 0.0) FROM time_entries WHERE deleted_at IS NULL"
        )
        .fetch_one(&self.db)
        .await
        .unwrap_or(0.0);

        // Get high risks count
        let high_risks: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM risks WHERE impact = 'High' AND status = 'Identified' AND deleted_at IS NULL"
        )
        .fetch_one(&self.db)
        .await?;

        // Get total revenue (placeholder calculation)
        let total_revenue: f64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount), 0.0) FROM invoices WHERE status = 'Paid' AND deleted_at IS NULL"
        )
        .fetch_one(&self.db)
        .await
        .unwrap_or(0.0);

        // Get open tasks count
        let open_tasks: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM workflow_tasks WHERE status NOT IN ('Done', 'Completed') AND deleted_at IS NULL"
        )
        .fetch_one(&self.db)
        .await?;

        Ok(DashboardStats {
            active_cases,
            pending_motions,
            billable_hours,
            high_risks,
            total_revenue,
            open_tasks,
        })
    }

    pub async fn get_chart_data(&self) -> Result<Vec<ChartData>, AppError> {
        let data = sqlx::query_as::<_, (String, i64)>(
            "SELECT status::text as name, COUNT(*) as count FROM cases WHERE deleted_at IS NULL GROUP BY status ORDER BY count DESC"
        )
        .fetch_all(&self.db)
        .await?;

        Ok(data
            .into_iter()
            .map(|(name, count)| ChartData { name, count })
            .collect())
    }

    pub async fn get_alerts(&self) -> Result<Vec<Alert>, AppError> {
        // Get high priority tasks that are due soon
        let alerts = sqlx::query_as::<_, (String, String, Option<String>, String)>(
            r#"
            SELECT 
                id::text,
                title,
                description,
                CASE 
                    WHEN due_date::date = CURRENT_DATE THEN 'Today'
                    WHEN due_date::date = CURRENT_DATE + 1 THEN 'Tomorrow'
                    ELSE due_date::text
                END as time_text
            FROM workflow_tasks
            WHERE priority = 'High' 
                AND status NOT IN ('Done', 'Completed')
                AND deleted_at IS NULL
            ORDER BY due_date
            LIMIT 5
            "#
        )
        .fetch_all(&self.db)
        .await?;

        Ok(alerts
            .into_iter()
            .map(|(id, title, description, time)| Alert {
                id: id.clone(),
                message: format!("High Priority Task: {}", title),
                detail: description.unwrap_or_default(),
                time,
                case_id: None,
            })
            .collect())
    }
}
