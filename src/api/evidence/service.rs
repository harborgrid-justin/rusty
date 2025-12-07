use crate::error::AppError;
use crate::models::EvidenceItem;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct CreateEvidenceParams {
    pub case_id: Uuid,
    pub title: String,
    pub evidence_type: String,
    pub description: String,
    pub collected_by: String,
    pub custodian: String,
    pub location: String,
    pub tags: Vec<String>,
}

pub struct UpdateEvidenceParams {
    pub title: Option<String>,
    pub description: Option<String>,
    pub custodian: Option<String>,
    pub location: Option<String>,
    pub admissibility: Option<String>,
    pub tags: Option<Vec<String>>,
}

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
    pub async fn create_evidence(&self, params: CreateEvidenceParams) -> Result<EvidenceItem, AppError> {
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
        .bind(params.case_id)
        .bind(&params.title)
        .bind(&params.evidence_type)
        .bind(&params.description)
        .bind(now)
        .bind(&params.collected_by)
        .bind(&params.custodian)
        .bind(&params.location)
        .bind("Pending")
        .bind(&params.tags)
        .bind(tracking_uuid)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(item)
    }

    /// Update an evidence item
    pub async fn update_evidence(&self, id: Uuid, params: UpdateEvidenceParams) -> Result<EvidenceItem, AppError> {
        let now = Utc::now();
        let existing = self.get_evidence(id).await?;

        let updated_title = params.title.unwrap_or(existing.title);
        let updated_description = params.description.unwrap_or(existing.description);
        let updated_custodian = params.custodian.unwrap_or(existing.custodian);
        let updated_location = params.location.unwrap_or(existing.location);
        let updated_tags = params.tags.unwrap_or(existing.tags);

        let item = if let Some(adm) = params.admissibility {
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
