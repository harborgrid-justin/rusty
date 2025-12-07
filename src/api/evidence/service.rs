use crate::error::AppError;
use crate::models::EvidenceItem;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct EvidenceService {
    pool: PgPool,
}

impl EvidenceService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List evidence items for a case
    pub async fn list_evidence(&self, case_id: Uuid) -> Result<Vec<EvidenceItem>, AppError> {
        let items = sqlx::query_as::<_, EvidenceItem>(
            "SELECT * FROM evidence_items WHERE case_id = $1 ORDER BY created_at DESC",
        )
        .bind(case_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    /// Get a specific evidence item
    pub async fn get_evidence(&self, id: Uuid) -> Result<EvidenceItem, AppError> {
        let item = sqlx::query_as::<_, EvidenceItem>("SELECT * FROM evidence_items WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(AppError::NotFound("Evidence item not found".to_string()))?;

        Ok(item)
    }

    /// Create a new evidence item
    pub async fn create_evidence(
        &self,
        case_id: Uuid,
        title: String,
        evidence_type: String,
        description: String,
        collected_by: String,
        custodian: String,
        location: String,
        tags: Vec<String>,
    ) -> Result<EvidenceItem, AppError> {
        let id = Uuid::new_v4();
        let tracking_uuid = Uuid::new_v4();
        let now = Utc::now();

        let item = sqlx::query_as::<_, EvidenceItem>(
            r#"
            INSERT INTO evidence_items (
                id, case_id, title, type, description, collection_date,
                collected_by, custodian, location, admissibility, tags,
                tracking_uuid, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4::evidence_type, $5, $6, $7, $8, $9, $10::admissibility_status, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(case_id)
        .bind(&title)
        .bind(&evidence_type)
        .bind(&description)
        .bind(now)
        .bind(&collected_by)
        .bind(&custodian)
        .bind(&location)
        .bind("Pending")
        .bind(&tags)
        .bind(tracking_uuid)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    /// Update an evidence item
    pub async fn update_evidence(
        &self,
        id: Uuid,
        title: Option<String>,
        description: Option<String>,
        custodian: Option<String>,
        location: Option<String>,
        admissibility: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<EvidenceItem, AppError> {
        let now = Utc::now();
        let existing = self.get_evidence(id).await?;

        let updated_title = title.unwrap_or(existing.title);
        let updated_description = description.unwrap_or(existing.description);
        let updated_custodian = custodian.unwrap_or(existing.custodian);
        let updated_location = location.unwrap_or(existing.location);
        let updated_tags = tags.unwrap_or(existing.tags);

        let item = if let Some(adm) = admissibility {
            sqlx::query_as::<_, EvidenceItem>(
                r#"
                UPDATE evidence_items
                SET title = $1, description = $2, custodian = $3, location = $4,
                    admissibility = $5::admissibility_status, tags = $6, updated_at = $7
                WHERE id = $8
                RETURNING *
                "#,
            )
            .bind(&updated_title)
            .bind(&updated_description)
            .bind(&updated_custodian)
            .bind(&updated_location)
            .bind(&adm)
            .bind(&updated_tags)
            .bind(now)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, EvidenceItem>(
                r#"
                UPDATE evidence_items
                SET title = $1, description = $2, custodian = $3, location = $4,
                    tags = $5, updated_at = $6
                WHERE id = $7
                RETURNING *
                "#,
            )
            .bind(&updated_title)
            .bind(&updated_description)
            .bind(&updated_custodian)
            .bind(&updated_location)
            .bind(&updated_tags)
            .bind(now)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
        };

        item.ok_or(AppError::NotFound("Evidence item not found".to_string()))
    }

    /// Delete an evidence item
    pub async fn delete_evidence(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM evidence_items WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Evidence item not found".to_string()));
        }

        Ok(())
    }
}
