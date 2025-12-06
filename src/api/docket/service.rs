use crate::error::AppError;
use crate::models::DocketEntry;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct DocketService {
    pool: PgPool,
}

impl DocketService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List docket entries for a case
    pub async fn list_entries(&self, case_id: Uuid) -> Result<Vec<DocketEntry>, AppError> {
        let entries = sqlx::query_as::<_, DocketEntry>(
            "SELECT * FROM docket_entries WHERE case_id = $1 ORDER BY date DESC, sequence_number DESC"
        )
        .bind(case_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(entries)
    }

    /// Get a specific docket entry
    pub async fn get_entry(&self, id: Uuid) -> Result<DocketEntry, AppError> {
        let entry = sqlx::query_as::<_, DocketEntry>(
            "SELECT * FROM docket_entries WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound("Docket entry not found".to_string()))?;

        Ok(entry)
    }

    /// Create a new docket entry
    pub async fn create_entry(
        &self,
        case_id: Uuid,
        sequence_number: i32,
        entry_type: String,
        title: String,
        description: Option<String>,
        date: Option<chrono::DateTime<Utc>>,
        filed_by: Option<String>,
    ) -> Result<DocketEntry, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let entry_date = date.unwrap_or(now);

        let entry = sqlx::query_as::<_, DocketEntry>(
            r#"
            INSERT INTO docket_entries (
                id, case_id, sequence_number, type, title, description, date, filed_by, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4::docket_entry_type, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(case_id)
        .bind(sequence_number)
        .bind(&entry_type)
        .bind(&title)
        .bind(&description)
        .bind(entry_date)
        .bind(&filed_by)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(entry)
    }

    /// Update a docket entry
    pub async fn update_entry(
        &self,
        id: Uuid,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<DocketEntry, AppError> {
        let now = Utc::now();
        let existing = self.get_entry(id).await?;

        let updated_title = title.unwrap_or(existing.title);
        let updated_description = description.or(existing.description);

        let entry = sqlx::query_as::<_, DocketEntry>(
            r#"
            UPDATE docket_entries
            SET title = $1, description = $2, updated_at = $3
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(&updated_title)
        .bind(&updated_description)
        .bind(now)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound("Docket entry not found".to_string()))?;

        Ok(entry)
    }

    /// Delete a docket entry
    pub async fn delete_entry(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM docket_entries WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Docket entry not found".to_string()));
        }

        Ok(())
    }
}
