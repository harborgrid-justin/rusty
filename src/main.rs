use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use rusty_saas::{
    api::{
        health::{health_check, liveness_check, readiness_check},
        users::{handlers, UserService},
    },
    auth::AuthService,
    config::Config,
    db::Database,
    middleware::{auth_middleware, metrics_middleware, request_id_middleware},
    models::{
        CreateUserRequest, HealthResponse, LoginRequest, LoginResponse, UpdateUserRequest,
        UserResponse,
    },
};

/// OpenAPI documentation structure
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::create_user,
        handlers::login,
        handlers::get_current_user,
        handlers::get_user,
        handlers::update_user,
        handlers::delete_user,
        handlers::list_users,
    ),
    components(
        schemas(
            HealthResponse,
            UserResponse,
            CreateUserRequest,
            UpdateUserRequest,
            LoginRequest,
            LoginResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
    )
)]
struct ApiDoc;

/// Security scheme for OpenAPI
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            )
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusty_saas=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rusty SaaS application...");

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config: {}. Using defaults.", e);
        Arc::new(Config::default())
    });

    tracing::info!(
        "Configuration loaded: {}:{}",
        config.server.host,
        config.server.port
    );

    // Initialize database connection pool
    let db = Database::new(&config.database).await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    db.migrate().await?;

    // Initialize services
    let auth_service = Arc::new(AuthService::new(Arc::new(config.jwt.clone())));
    let user_service = Arc::new(UserService::new(db.clone(), auth_service.clone()));

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build public routes (no auth required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .route("/live", get(liveness_check))
        .route("/api/users", post(handlers::create_user))
        .route("/api/auth/login", post(handlers::login))
        .with_state(user_service.clone());

    // Build protected routes (auth required)
    let protected_routes = Router::new()
        .route("/api/users/me", get(handlers::get_current_user))
        .route("/api/users", get(handlers::list_users))
        .route("/api/users/:id", get(handlers::get_user))
        .route("/api/users/:id", put(handlers::update_user))
        .route("/api/users/:id", delete(handlers::delete_user))
        .route_layer(middleware::from_fn_with_state(
            auth_service.clone(),
            auth_middleware,
        ))
        .with_state(user_service);

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(CompressionLayer::new())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(metrics_middleware))
        .layer(middleware::from_fn(request_id_middleware));

    // Start server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("🚀 Server started on http://{}", addr);
    tracing::info!(
        "📚 API Documentation available at http://{}/swagger-ui",
        addr
    );
    tracing::info!("🏥 Health check available at http://{}/health", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
