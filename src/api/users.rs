use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::auth::AuthService;
use crate::db::Database;
use crate::error::{AppError, Result};
use crate::models::{
    Claims, CreateUserRequest, LoginRequest, LoginResponse, UpdateUserRequest, User, UserResponse,
};

/// User service for business logic
pub struct UserService {
    pub db: Arc<Database>,
    auth_service: Arc<AuthService>,
}

impl UserService {
    pub fn new(db: Arc<Database>, auth_service: Arc<AuthService>) -> Self {
        Self { db, auth_service }
    }

    /// Create a new user
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserResponse> {
        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Check if user already exists
        let existing =
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 OR username = $2")
                .bind(&request.email)
                .bind(&request.username)
                .fetch_optional(self.db.pool())
                .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("User already exists".to_string()));
        }

        // Hash password
        let password_hash = self.auth_service.hash_password(&request.password)?;

        // Insert user
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, username, password_hash, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(&request.email)
        .bind(&request.username)
        .bind(&password_hash)
        .bind(true)
        .bind(Utc::now())
        .bind(Utc::now())
        .fetch_one(self.db.pool())
        .await?;

        Ok(user.into())
    }

    /// Login user and generate token
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse> {
        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Find user by email
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(&request.email)
            .fetch_optional(self.db.pool())
            .await?
            .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

        // Verify password
        if !self
            .auth_service
            .verify_password(&request.password, &user.password_hash)?
        {
            return Err(AppError::Authentication("Invalid credentials".to_string()));
        }

        // Check if user is active
        if !user.is_active {
            return Err(AppError::Authorization(
                "User account is inactive".to_string(),
            ));
        }

        // Generate token
        let token = self
            .auth_service
            .generate_token(&user.id.to_string(), &user.email)?;

        Ok(LoginResponse {
            token,
            user: user.into(),
        })
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: Uuid) -> Result<UserResponse> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(self.db.pool())
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(user.into())
    }

    /// Update user
    pub async fn update_user(
        &self,
        user_id: Uuid,
        request: UpdateUserRequest,
    ) -> Result<UserResponse> {
        request
            .validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // Check if user exists
        let mut user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(self.db.pool())
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Update fields if provided
        if let Some(email) = request.email {
            user.email = email;
        }
        if let Some(username) = request.username {
            user.username = username;
        }
        user.updated_at = Utc::now();

        // Save changes
        let updated_user = sqlx::query_as::<_, User>(
            "UPDATE users SET email = $1, username = $2, updated_at = $3 WHERE id = $4 RETURNING *",
        )
        .bind(&user.email)
        .bind(&user.username)
        .bind(user.updated_at)
        .bind(user_id)
        .fetch_one(self.db.pool())
        .await?;

        Ok(updated_user.into())
    }

    /// Delete user
    pub async fn delete_user(&self, user_id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(user_id)
            .execute(self.db.pool())
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    /// List all users (with pagination in production)
    /// NOTE: In production, this should implement proper pagination
    /// and access control (e.g., admin-only access)
    pub async fn list_users(&self) -> Result<Vec<UserResponse>> {
        // TODO: Implement pagination with page/per_page parameters
        // TODO: Add role-based access control
        let users =
            sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC LIMIT 100")
                .fetch_all(self.db.pool())
                .await?;

        Ok(users.into_iter().map(|u| u.into()).collect())
    }
}

/// API handlers
pub mod handlers {
    use super::*;

    /// Create user handler
    #[utoipa::path(
        post,
        path = "/api/users",
        request_body = CreateUserRequest,
        responses(
            (status = 201, description = "User created successfully", body = UserResponse),
            (status = 400, description = "Bad request"),
        ),
        tag = "users"
    )]
    pub async fn create_user(
        State(service): State<Arc<UserService>>,
        Json(request): Json<CreateUserRequest>,
    ) -> Result<(StatusCode, Json<UserResponse>)> {
        let user = service.create_user(request).await?;
        Ok((StatusCode::CREATED, Json(user)))
    }

    /// Login handler
    #[utoipa::path(
        post,
        path = "/api/auth/login",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "Login successful", body = LoginResponse),
            (status = 401, description = "Invalid credentials"),
        ),
        tag = "auth"
    )]
    pub async fn login(
        State(service): State<Arc<UserService>>,
        Json(request): Json<LoginRequest>,
    ) -> Result<Json<LoginResponse>> {
        let response = service.login(request).await?;
        Ok(Json(response))
    }

    /// Get current user handler (protected)
    #[utoipa::path(
        get,
        path = "/api/users/me",
        responses(
            (status = 200, description = "User retrieved successfully", body = UserResponse),
            (status = 401, description = "Unauthorized"),
        ),
        security(
            ("bearer_auth" = [])
        ),
        tag = "users"
    )]
    pub async fn get_current_user(
        State(service): State<Arc<UserService>>,
        Extension(claims): Extension<Claims>,
    ) -> Result<Json<UserResponse>> {
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;
        let user = service.get_user(user_id).await?;
        Ok(Json(user))
    }

    /// Get user by ID handler (protected)
    #[utoipa::path(
        get,
        path = "/api/users/{id}",
        responses(
            (status = 200, description = "User retrieved successfully", body = UserResponse),
            (status = 404, description = "User not found"),
        ),
        params(
            ("id" = Uuid, Path, description = "User ID")
        ),
        security(
            ("bearer_auth" = [])
        ),
        tag = "users"
    )]
    pub async fn get_user(
        State(service): State<Arc<UserService>>,
        Path(id): Path<Uuid>,
    ) -> Result<Json<UserResponse>> {
        let user = service.get_user(id).await?;
        Ok(Json(user))
    }

    /// Update user handler (protected)
    #[utoipa::path(
        put,
        path = "/api/users/{id}",
        request_body = UpdateUserRequest,
        responses(
            (status = 200, description = "User updated successfully", body = UserResponse),
            (status = 404, description = "User not found"),
        ),
        params(
            ("id" = Uuid, Path, description = "User ID")
        ),
        security(
            ("bearer_auth" = [])
        ),
        tag = "users"
    )]
    pub async fn update_user(
        State(service): State<Arc<UserService>>,
        Path(id): Path<Uuid>,
        Extension(claims): Extension<Claims>,
        Json(request): Json<UpdateUserRequest>,
    ) -> Result<Json<UserResponse>> {
        // Only allow users to update their own profile
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;

        if user_id != id {
            return Err(AppError::Authorization(
                "You can only update your own profile".to_string(),
            ));
        }

        let user = service.update_user(id, request).await?;
        Ok(Json(user))
    }

    /// Delete user handler (protected)
    #[utoipa::path(
        delete,
        path = "/api/users/{id}",
        responses(
            (status = 204, description = "User deleted successfully"),
            (status = 404, description = "User not found"),
        ),
        params(
            ("id" = Uuid, Path, description = "User ID")
        ),
        security(
            ("bearer_auth" = [])
        ),
        tag = "users"
    )]
    pub async fn delete_user(
        State(service): State<Arc<UserService>>,
        Path(id): Path<Uuid>,
        Extension(claims): Extension<Claims>,
    ) -> Result<StatusCode> {
        // Only allow users to delete their own account
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::InternalServerError("Invalid user ID".to_string()))?;

        if user_id != id {
            return Err(AppError::Authorization(
                "You can only delete your own account".to_string(),
            ));
        }

        service.delete_user(id).await?;
        Ok(StatusCode::NO_CONTENT)
    }

    /// List users handler (protected)
    #[utoipa::path(
        get,
        path = "/api/users",
        responses(
            (status = 200, description = "Users retrieved successfully", body = Vec<UserResponse>),
        ),
        security(
            ("bearer_auth" = [])
        ),
        tag = "users"
    )]
    pub async fn list_users(
        State(service): State<Arc<UserService>>,
    ) -> Result<Json<Vec<UserResponse>>> {
        let users = service.list_users().await?;
        Ok(Json(users))
    }
}
