# Rusty SaaS - Enterprise-Grade Rust Web Platform

An enterprise-grade, modular Rust-based scaffolding for SaaS web delivery platforms. Built with modern frameworks and best practices for scalability, security, and maintainability.

## 🚀 Features

- **Modern Web Framework**: Built with [Axum](https://github.com/tokio-rs/axum) for high-performance, type-safe web services
- **Async Runtime**: Powered by [Tokio](https://tokio.rs/) for efficient async I/O
- **Database Support**: PostgreSQL integration with [SQLx](https://github.com/launchbadge/sqlx) and compile-time query verification
- **Authentication**: JWT-based authentication with Argon2 password hashing
- **API Documentation**: Auto-generated OpenAPI/Swagger documentation with [utoipa](https://github.com/juhaku/utoipa)
- **Structured Logging**: Comprehensive logging with [tracing](https://github.com/tokio-rs/tracing)
- **Configuration Management**: Flexible config system supporting files and environment variables
- **Input Validation**: Request validation with [validator](https://github.com/Keats/validator)
- **Error Handling**: Type-safe error handling with custom error types
- **Middleware**: CORS, compression, request tracing, and metrics
- **Docker Support**: Production-ready Docker and Docker Compose configurations
- **Database Migrations**: Automated database migrations with SQLx
- **Testing Infrastructure**: Comprehensive testing setup with mocking support
- **Modular Architecture**: Clean separation of concerns for easy extension

## 📋 Prerequisites

- Rust 1.91+ (install from [rustup.rs](https://rustup.rs/))
- PostgreSQL 14+ (or use Docker Compose)
- Docker and Docker Compose (optional, for containerized deployment)

## 🏗️ Project Structure

```
rusty/
├── src/
│   ├── api/              # API endpoint handlers
│   │   ├── health.rs     # Health check endpoints
│   │   ├── users.rs      # User management endpoints
│   │   └── mod.rs
│   ├── auth.rs           # Authentication service
│   ├── config.rs         # Configuration management
│   ├── db.rs             # Database connection and migrations
│   ├── error.rs          # Error types and handling
│   ├── middleware.rs     # Custom middleware
│   ├── models.rs         # Data models and schemas
│   ├── lib.rs            # Library exports
│   └── main.rs           # Application entry point
├── migrations/           # Database migrations
├── config/               # Configuration files
│   ├── default.toml      # Default configuration
│   └── production.toml   # Production overrides
├── tests/                # Integration tests
├── Cargo.toml            # Rust dependencies
├── Dockerfile            # Multi-stage Docker build
├── docker-compose.yml    # Docker Compose setup
└── .env.example          # Environment variables template

```

## 🚦 Quick Start

### Local Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/harborgrid-justin/rusty.git
   cd rusty
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start PostgreSQL** (if not using Docker)
   ```bash
   # Using Docker
   docker run -d \
     --name postgres \
     -e POSTGRES_PASSWORD=postgres \
     -e POSTGRES_DB=rusty_saas \
     -p 5432:5432 \
     postgres:16-alpine
   ```

4. **Build and run the application**
   ```bash
   cargo build --release
   cargo run
   ```

5. **Access the application**
   - API: http://localhost:8080
   - Swagger UI: http://localhost:8080/swagger-ui
   - Health Check: http://localhost:8080/health

### Using Docker Compose

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f app

# Stop services
docker-compose down
```

## 📚 API Documentation

Once the application is running, visit http://localhost:8080/swagger-ui for interactive API documentation.

### Key Endpoints

#### Health Checks
- `GET /health` - Full health check (includes database)
- `GET /ready` - Readiness probe
- `GET /live` - Liveness probe

#### Authentication
- `POST /api/auth/login` - User login (returns JWT token)

#### User Management
- `POST /api/users` - Create new user (public)
- `GET /api/users/me` - Get current user (protected)
- `GET /api/users` - List all users (protected)
- `GET /api/users/{id}` - Get user by ID (protected)
- `PUT /api/users/{id}` - Update user (protected)
- `DELETE /api/users/{id}` - Delete user (protected)

## 🔐 Authentication

The API uses JWT (JSON Web Tokens) for authentication.

1. **Create a user**
   ```bash
   curl -X POST http://localhost:8080/api/users \
     -H "Content-Type: application/json" \
     -d '{
       "email": "user@example.com",
       "username": "testuser",
       "password": "securepassword123"
     }'
   ```

2. **Login to get a token**
   ```bash
   curl -X POST http://localhost:8080/api/auth/login \
     -H "Content-Type: application/json" \
     -d '{
       "email": "user@example.com",
       "password": "securepassword123"
     }'
   ```

3. **Use the token in subsequent requests**
   ```bash
   curl -X GET http://localhost:8080/api/users/me \
     -H "Authorization: Bearer YOUR_JWT_TOKEN"
   ```

## ⚙️ Configuration

Configuration is managed through a hierarchical system:

1. `config/default.toml` - Base configuration
2. `config/{environment}.toml` - Environment-specific overrides
3. Environment variables - Highest priority (prefix: `APP_`)

### Environment Variables

```bash
# Server
APP_SERVER__HOST=0.0.0.0
APP_SERVER__PORT=8080

# Database
APP_DATABASE__URL=postgres://user:pass@localhost:5432/dbname

# JWT
APP_JWT__SECRET=your-secret-key
APP_JWT__EXPIRATION_HOURS=24

# CORS
APP_CORS__ALLOWED_ORIGINS=http://localhost:3000
```

## 🗄️ Database Migrations

Migrations are automatically run on application startup. To manage migrations manually:

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Create a new migration
sqlx migrate add create_new_table

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## 🔨 Development

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### Linting

```bash
# Format code
cargo fmt

# Check code
cargo clippy -- -D warnings
```

## 📦 Deployment

### Docker

```bash
# Build image
docker build -t rusty-saas:latest .

# Run container
docker run -d \
  -p 8080:8080 \
  -e APP_DATABASE__URL=postgres://... \
  -e APP_JWT__SECRET=your-secret \
  rusty-saas:latest
```

### Production Checklist

- [ ] Change JWT secret (use strong random value)
- [ ] Update CORS allowed origins
- [ ] Configure production database
- [ ] Set up SSL/TLS certificates
- [ ] Configure logging aggregation
- [ ] Set up monitoring and alerting
- [ ] Enable rate limiting
- [ ] Review and harden security settings
- [ ] Set up backup strategy
- [ ] Configure CI/CD pipeline

## 🏗️ Architecture

### Modular Design

The application follows a modular architecture:

- **API Layer**: HTTP handlers and routing
- **Service Layer**: Business logic
- **Data Layer**: Database access and models
- **Auth Layer**: Authentication and authorization
- **Config Layer**: Configuration management
- **Error Layer**: Centralized error handling

### Extensibility

To add new features:

1. Create new module in `src/api/`
2. Define models in `src/models.rs`
3. Add database migration if needed
4. Implement service logic
5. Register routes in `main.rs`
6. Update OpenAPI documentation

## 🔒 Security Features

- **Password Hashing**: Argon2 algorithm (winner of Password Hashing Competition)
- **JWT Authentication**: Secure token-based auth
- **Input Validation**: Request validation at API layer
- **SQL Injection Protection**: Parameterized queries with SQLx
- **CORS**: Configurable cross-origin resource sharing
- **Security Headers**: HTTP security headers via Tower middleware
- **Error Handling**: No sensitive data in error responses

## 📊 Monitoring

- Structured logging with tracing
- Health check endpoints for orchestrators
- Request/response logging middleware
- Metrics collection ready

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📄 License

This project is licensed under the MIT OR Apache-2.0 License.

## 🙏 Acknowledgments

Built with these amazing projects:
- [Axum](https://github.com/tokio-rs/axum)
- [Tokio](https://tokio.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- [Serde](https://serde.rs/)
- [utoipa](https://github.com/juhaku/utoipa)
- And many more in the Rust ecosystem

## 📞 Support

For issues and questions, please use the GitHub issue tracker.

---

**Built with ❤️ using Rust**