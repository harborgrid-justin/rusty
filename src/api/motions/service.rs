use crate::error::AppError;
use crate::models::Motion;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct MotionService {
    pool: PgPool,
}

impl MotionService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List motions for a case
    pub async fn list_motions(&self, case_id: Uuid) -> Result<Vec<Motion>, AppError> {
        let motions = sqlx::query_as::<_, Motion>(
            "SELECT * FROM motions WHERE case_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC"
        )
        .bind(case_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(motions)
    }

    /// Get a specific motion
    pub async fn get_motion(&self, id: Uuid) -> Result<Motion, AppError> {
        let motion = sqlx::query_as::<_, Motion>(
            "SELECT * FROM motions WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound("Motion not found".to_string()))?;

        Ok(motion)
    }

    /// Create a new motion
    pub async fn create_motion(
        &self,
        case_id: Uuid,
        title: String,
        motion_type: String,
        status: String,
        filing_date: Option<chrono::DateTime<Utc>>,
    ) -> Result<Motion, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let motion = sqlx::query_as::<_, Motion>(
            r#"
            INSERT INTO motions (
                id, case_id, title, type, status, filing_date, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4::motion_type, $5::motion_status, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(case_id)
        .bind(&title)
        .bind(&motion_type)
        .bind(&status)
        .bind(filing_date)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(motion)
    }

    /// Update a motion
    pub async fn update_motion(
        &self,
        id: Uuid,
        title: Option<String>,
        status: Option<String>,
        outcome: Option<String>,
        hearing_date: Option<chrono::DateTime<Utc>>,
    ) -> Result<Motion, AppError> {
        let now = Utc::now();
        let existing = self.get_motion(id).await?;

        let updated_title = title.unwrap_or(existing.title);

        let motion = if let Some(stat) = status {
            if let Some(out) = outcome {
                if let Some(hd) = hearing_date {
                    sqlx::query_as::<_, Motion>(
                        r#"
                        UPDATE motions
                        SET title = $1, status = $2::motion_status, outcome = $3::motion_outcome, 
                            hearing_date = $4, updated_at = $5
                        WHERE id = $6 AND deleted_at IS NULL
                        RETURNING *
                        "#,
                    )
                    .bind(&updated_title)
                    .bind(&stat)
                    .bind(&out)
                    .bind(hd)
                    .bind(now)
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?
                } else {
                    sqlx::query_as::<_, Motion>(
                        r#"
                        UPDATE motions
                        SET title = $1, status = $2::motion_status, outcome = $3::motion_outcome, updated_at = $4
                        WHERE id = $5 AND deleted_at IS NULL
                        RETURNING *
                        "#,
                    )
                    .bind(&updated_title)
                    .bind(&stat)
                    .bind(&out)
                    .bind(now)
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?
                }
            } else if let Some(hd) = hearing_date {
                sqlx::query_as::<_, Motion>(
                    r#"
                    UPDATE motions
                    SET title = $1, status = $2::motion_status, hearing_date = $3, updated_at = $4
                    WHERE id = $5 AND deleted_at IS NULL
                    RETURNING *
                    "#,
                )
                .bind(&updated_title)
                .bind(&stat)
                .bind(hd)
                .bind(now)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?
            } else {
                sqlx::query_as::<_, Motion>(
                    r#"
                    UPDATE motions
                    SET title = $1, status = $2::motion_status, updated_at = $3
                    WHERE id = $4 AND deleted_at IS NULL
                    RETURNING *
                    "#,
                )
                .bind(&updated_title)
                .bind(&stat)
                .bind(now)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?
            }
        } else {
            sqlx::query_as::<_, Motion>(
                r#"
                UPDATE motions
                SET title = $1, updated_at = $2
                WHERE id = $3 AND deleted_at IS NULL
                RETURNING *
                "#,
            )
            .bind(&updated_title)
            .bind(now)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
        };

        motion.ok_or(AppError::NotFound("Motion not found".to_string()))
    }

    /// Soft delete a motion
    pub async fn delete_motion(&self, id: Uuid) -> Result<(), AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            "UPDATE motions SET deleted_at = $1 WHERE id = $2 AND deleted_at IS NULL"
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Motion not found".to_string()));
        }

        Ok(())
    }
}
