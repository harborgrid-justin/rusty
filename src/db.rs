use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use std::time::Duration;

use crate::config::DatabaseConfig;
use crate::error::{AppError, Result};

/// Database connection pool wrapper
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(config: &DatabaseConfig) -> Result<Arc<Self>> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(30))
            .connect(&config.url)
            .await
            .map_err(|e| {
                tracing::error!("Failed to connect to database: {:?}", e);
                AppError::Database(e)
            })?;

        tracing::info!("Database connection pool established");

        Ok(Arc::new(Self { pool }))
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run database migrations
    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to run migrations: {:?}", e);
                AppError::InternalServerError(format!("Migration failed: {}", e))
            })?;

        tracing::info!("Database migrations completed successfully");
        Ok(())
    }

    /// Health check for the database
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }
}
