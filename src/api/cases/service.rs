use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Case, CaseResponse, CreateCaseRequest, Party, UpdateCaseRequest},
};

use super::handlers::ListCasesQuery;

pub struct CaseService {
    db: PgPool,
}

impl CaseService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn list_cases(&self, params: ListCasesQuery) -> Result<Vec<Case>, AppError> {
        let page = params.page.unwrap_or(1).max(1);
        let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * per_page;

        let mut query = String::from(
            "SELECT * FROM cases WHERE deleted_at IS NULL"
        );

        if let Some(ref status) = params.status {
            query.push_str(&format!(" AND status = '{}'", status));
        }

        if let Some(ref search) = params.search {
            query.push_str(&format!(
                " AND (title ILIKE '%{}%' OR client ILIKE '%{}%')",
                search, search
            ));
        }

        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

        let cases = sqlx::query_as::<_, Case>(&query)
            .fetch_all(&self.db)
            .await?;

        Ok(cases)
    }

    pub async fn get_case(&self, id: Uuid) -> Result<CaseResponse, AppError> {
        let case = sqlx::query_as::<_, Case>(
            "SELECT * FROM cases WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await?
        .ok_or(AppError::NotFound("Case not found".to_string()))?;

        let parties = self.get_case_parties(id).await?;

        Ok(CaseResponse { case, parties })
    }

    pub async fn create_case(&self, payload: CreateCaseRequest) -> Result<Case, AppError> {
        let case = sqlx::query_as::<_, Case>(
            r#"
            INSERT INTO cases (
                title, client, client_id, matter_type, matter_sub_type,
                status, filing_date, description, value, jurisdiction,
                court, judge, billing_model, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
            RETURNING *
            "#
        )
        .bind(&payload.title)
        .bind(&payload.client)
        .bind(payload.client_id)
        .bind(&payload.matter_type)
        .bind(&payload.matter_sub_type)
        .bind(payload.status.unwrap_or(crate::models::CaseStatus::PreFiling))
        .bind(payload.filing_date)
        .bind(&payload.description)
        .bind(payload.value)
        .bind(&payload.jurisdiction)
        .bind(&payload.court)
        .bind(&payload.judge)
        .bind(&payload.billing_model)
        .fetch_one(&self.db)
        .await?;

        Ok(case)
    }

    pub async fn update_case(&self, id: Uuid, payload: UpdateCaseRequest) -> Result<Case, AppError> {
        // First, verify the case exists
        let _existing = self.get_case(id).await?;

        let case = sqlx::query_as::<_, Case>(
            r#"
            UPDATE cases
            SET
                title = COALESCE($2, title),
                status = COALESCE($3, status),
                description = COALESCE($4, description),
                value = COALESCE($5, value),
                jurisdiction = COALESCE($6, jurisdiction),
                court = COALESCE($7, court),
                judge = COALESCE($8, judge),
                magistrate_judge = COALESCE($9, magistrate_judge),
                opposing_counsel = COALESCE($10, opposing_counsel),
                billing_model = COALESCE($11, billing_model),
                updated_at = NOW(),
                version = version + 1
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&payload.title)
        .bind(&payload.status)
        .bind(&payload.description)
        .bind(payload.value)
        .bind(&payload.jurisdiction)
        .bind(&payload.court)
        .bind(&payload.judge)
        .bind(&payload.magistrate_judge)
        .bind(&payload.opposing_counsel)
        .bind(&payload.billing_model)
        .fetch_one(&self.db)
        .await?;

        Ok(case)
    }

    pub async fn delete_case(&self, id: Uuid) -> Result<(), AppError> {
        // Soft delete
        let result = sqlx::query(
            "UPDATE cases SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .execute(&self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Case not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_case_parties(&self, case_id: Uuid) -> Result<Vec<Party>, AppError> {
        let parties = sqlx::query_as::<_, Party>(
            "SELECT * FROM parties WHERE case_id = $1 AND deleted_at IS NULL ORDER BY created_at"
        )
        .bind(case_id)
        .fetch_all(&self.db)
        .await?;

        Ok(parties)
    }
}
