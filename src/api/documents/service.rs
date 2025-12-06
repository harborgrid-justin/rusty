use crate::error::AppError;
use crate::models::{CreateDocumentRequest, Document};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct DocumentService {
    pool: PgPool,
}

impl DocumentService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// List documents with optional case filter
    pub async fn list_documents(&self, case_id: Option<Uuid>) -> Result<Vec<Document>, AppError> {
        let docs = if let Some(cid) = case_id {
            sqlx::query_as::<_, Document>(
                "SELECT * FROM documents WHERE case_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC"
            )
            .bind(cid)
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as::<_, Document>(
                "SELECT * FROM documents WHERE deleted_at IS NULL ORDER BY created_at DESC"
            )
            .fetch_all(&self.pool)
            .await?
        };

        Ok(docs)
    }

    /// Get document by ID
    pub async fn get_document(&self, id: Uuid) -> Result<Document, AppError> {
        let doc = sqlx::query_as::<_, Document>(
            "SELECT * FROM documents WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound("Document not found".to_string()))?;

        Ok(doc)
    }

    /// Create a new document
    pub async fn create_document(
        &self,
        req: CreateDocumentRequest,
        author_id: Uuid,
    ) -> Result<Document, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let tags = req.tags.unwrap_or_default();

        let doc = sqlx::query_as::<_, Document>(
            r#"
            INSERT INTO documents (
                id, case_id, title, type, content, upload_date, last_modified,
                tags, author_id, created_at, updated_at, version
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(req.case_id)
        .bind(&req.title)
        .bind(&req.doc_type)
        .bind(&req.content)
        .bind(now)
        .bind(now)
        .bind(&tags)
        .bind(author_id)
        .bind(now)
        .bind(now)
        .bind(1)
        .fetch_one(&self.pool)
        .await?;

        Ok(doc)
    }

    /// Update document
    pub async fn update_document(
        &self,
        id: Uuid,
        title: Option<String>,
        content: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<Document, AppError> {
        let now = Utc::now();

        // Get existing document
        let existing = self.get_document(id).await?;

        let updated_title = title.unwrap_or(existing.title);
        let updated_content = content.or(existing.content);
        let updated_tags = tags.unwrap_or(existing.tags);

        let doc = sqlx::query_as::<_, Document>(
            r#"
            UPDATE documents
            SET title = $1, content = $2, tags = $3, last_modified = $4, updated_at = $5
            WHERE id = $6 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(&updated_title)
        .bind(&updated_content)
        .bind(&updated_tags)
        .bind(now)
        .bind(now)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::NotFound("Document not found".to_string()))?;

        Ok(doc)
    }

    /// Soft delete document
    pub async fn delete_document(&self, id: Uuid) -> Result<(), AppError> {
        let now = Utc::now();

        let result = sqlx::query(
            "UPDATE documents SET deleted_at = $1 WHERE id = $2 AND deleted_at IS NULL"
        )
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Document not found".to_string()));
        }

        Ok(())
    }
}
