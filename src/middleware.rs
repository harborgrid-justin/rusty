use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::auth::AuthService;
use crate::error::AppError;

/// Authentication middleware to protect routes
pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    let token = AuthService::extract_token_from_header(auth_header)?;
    let claims = auth_service.validate_token(token)?;

    // Add claims to request extensions for use in handlers
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Request ID middleware for tracing
pub async fn request_id_middleware(mut req: Request, next: Next) -> impl IntoResponse {
    let request_id = uuid::Uuid::new_v4().to_string();

    // Add request ID to tracing span
    tracing::info!("Request started: {}", request_id);

    req.extensions_mut().insert(request_id.clone());

    let response = next.run(req).await;

    tracing::info!("Request completed: {}", request_id);

    response
}

/// Metrics middleware to track request counts and duration
pub async fn metrics_middleware(req: Request, next: Next) -> impl IntoResponse {
    let method = req.method().clone();
    let uri = req.uri().clone();

    let start = std::time::Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed();

    tracing::info!(
        method = %method,
        uri = %uri,
        status = %response.status(),
        duration_ms = %duration.as_millis(),
        "Request processed"
    );

    response
}
