# Extending Rusty SaaS

This guide explains how to extend the Rusty SaaS platform with new features.

## Adding a New API Endpoint

### 1. Define the Model

Add your data structure in `src/models.rs`:

```rust
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
}
```

### 2. Create Database Migration

Create a new migration file in `migrations/`:

```sql
-- migrations/20240102000000_create_products_table.up.sql
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 3. Create Service Module

Create `src/api/products.rs`:

```rust
use crate::db::Database;
use crate::error::{AppError, Result};
use crate::models::{CreateProductRequest, Product};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct ProductService {
    pub db: Arc<Database>,
}

impl ProductService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_product(&self, request: CreateProductRequest) -> Result<Product> {
        request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        let product = sqlx::query_as::<_, Product>(
            "INSERT INTO products (id, name, description, price, created_at) 
             VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(Uuid::new_v4())
        .bind(&request.name)
        .bind(&request.description)
        .bind(request.price)
        .bind(Utc::now())
        .fetch_one(self.db.pool())
        .await?;

        Ok(product)
    }
}
```

### 4. Add Handlers

Add handlers in the service module:

```rust
pub mod handlers {
    use super::*;
    use axum::{extract::State, http::StatusCode, Json};

    #[utoipa::path(
        post,
        path = "/api/products",
        request_body = CreateProductRequest,
        responses(
            (status = 201, description = "Product created", body = Product),
        ),
        tag = "products"
    )]
    pub async fn create_product(
        State(service): State<Arc<ProductService>>,
        Json(request): Json<CreateProductRequest>,
    ) -> Result<(StatusCode, Json<Product>)> {
        let product = service.create_product(request).await?;
        Ok((StatusCode::CREATED, Json(product)))
    }
}
```

### 5. Register Routes

Update `src/main.rs`:

```rust
use rusty_saas::api::products::{handlers as product_handlers, ProductService};

// In main function:
let product_service = Arc::new(ProductService::new(db.clone()));

let app = Router::new()
    .route("/api/products", post(product_handlers::create_product))
    .with_state(product_service)
    // ... other routes
```

### 6. Update OpenAPI Documentation

Add to the `#[openapi]` macro in `src/main.rs`:

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        product_handlers::create_product,
        // ... other paths
    ),
    components(
        schemas(Product, CreateProductRequest)
    )
)]
```

## Adding Custom Middleware

Create a new middleware in `src/middleware.rs`:

```rust
use axum::{extract::Request, middleware::Next, response::Response};

pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Implement rate limiting logic
    // Check request count from IP/user
    // Return error if limit exceeded
    
    Ok(next.run(req).await)
}
```

Register in `main.rs`:

```rust
.layer(middleware::from_fn(rate_limit_middleware))
```

## Adding Background Jobs

### 1. Create a Job Module

Create `src/jobs/mod.rs`:

```rust
use std::sync::Arc;
use tokio::time::{interval, Duration};

pub struct JobScheduler {
    // Your dependencies
}

impl JobScheduler {
    pub async fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                if let Err(e) = self.run_job().await {
                    tracing::error!("Job failed: {:?}", e);
                }
            }
        });
    }

    async fn run_job(&self) -> Result<()> {
        // Your job logic
        Ok(())
    }
}
```

### 2. Start Jobs in Main

```rust
let job_scheduler = Arc::new(JobScheduler::new(/* deps */));
tokio::spawn(async move {
    job_scheduler.start().await;
});
```

## Adding WebSocket Support

### 1. Add Dependencies

Update `Cargo.toml`:

```toml
axum = { version = "0.7", features = ["ws"] }
```

### 2. Create WebSocket Handler

```rust
use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, State},
    response::Response,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    // Handle WebSocket messages
}
```

### 3. Add Route

```rust
.route("/ws", get(ws_handler))
```

## Adding Redis Caching

### 1. Add Dependencies

```toml
redis = { version = "0.25", features = ["tokio-comp", "connection-manager"] }
```

### 2. Create Cache Module

```rust
use redis::{Client, aio::ConnectionManager};

pub struct Cache {
    client: ConnectionManager,
}

impl Cache {
    pub async fn new(url: &str) -> Result<Self> {
        let client = Client::open(url)?;
        let conn = ConnectionManager::new(client).await?;
        Ok(Self { client: conn })
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<String>> {
        // Implementation
    }

    pub async fn set(&mut self, key: &str, value: &str, expiry: usize) -> Result<()> {
        // Implementation
    }
}
```

## Adding Email Service

### 1. Add Dependencies

```toml
lettre = { version = "0.11", features = ["tokio1-rustls-tls"] }
```

### 2. Create Email Service

```rust
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message,
    transport::smtp::authentication::Credentials,
};

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailService {
    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        let email = Message::builder()
            .from("noreply@example.com".parse()?)
            .to(to.parse()?)
            .subject(subject)
            .body(body.to_string())?;

        self.mailer.send(email).await?;
        Ok(())
    }
}
```

## Best Practices

1. **Always validate input** using the `validator` crate
2. **Use database transactions** for multi-step operations
3. **Add proper error handling** with meaningful error messages
4. **Write tests** for new features
5. **Document APIs** using utoipa annotations
6. **Log important events** using tracing
7. **Keep services stateless** for horizontal scaling
8. **Use connection pooling** for external services
9. **Implement health checks** for new dependencies
10. **Follow the existing code structure** for consistency

## Testing New Features

Create integration tests in `tests/`:

```rust
#[tokio::test]
async fn test_create_product() {
    let db = setup_test_db().await;
    let service = ProductService::new(db);
    
    let request = CreateProductRequest {
        name: "Test Product".to_string(),
        description: "Test".to_string(),
        price: 99.99,
    };
    
    let product = service.create_product(request).await.unwrap();
    assert_eq!(product.name, "Test Product");
}
```

## Deployment Considerations

- Use environment variables for configuration
- Enable HTTPS in production
- Set up proper logging aggregation
- Implement rate limiting
- Add monitoring and alerting
- Use database migrations for schema changes
- Configure backup strategies
- Set resource limits in containerization

## Further Resources

- [Axum Documentation](https://docs.rs/axum)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
