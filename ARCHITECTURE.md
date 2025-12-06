# Architecture Overview

## System Architecture

Rusty SaaS follows a layered architecture pattern with clear separation of concerns:

```
┌─────────────────────────────────────────────────┐
│              HTTP Layer (Axum)                  │
│  - Routing                                      │
│  - Middleware (CORS, Auth, Logging)             │
│  - Request/Response handling                    │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│            API Layer (Handlers)                 │
│  - Input validation                             │
│  - Request transformation                       │
│  - Response formatting                          │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│          Service Layer (Business Logic)         │
│  - Authentication & Authorization               │
│  - Business rules                               │
│  - Data orchestration                           │
└─────────────────────────────────────────────────┘
                     ↓
┌─────────────────────────────────────────────────┐
│          Data Layer (Database)                  │
│  - SQLx queries                                 │
│  - Migrations                                   │
│  - Connection pooling                           │
└─────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules

#### `main.rs`
- Application entry point
- Server initialization
- Route configuration
- Middleware setup
- OpenAPI documentation

#### `lib.rs`
- Public API exports
- Module organization

#### `config.rs`
- Configuration management
- Environment-based settings
- Secrets handling

#### `error.rs`
- Custom error types
- Error handling
- HTTP error responses

#### `db.rs`
- Database connection pooling
- Migration management
- Health checks

#### `auth.rs`
- JWT token generation/validation
- Password hashing (Argon2)
- Token extraction

#### `middleware.rs`
- Authentication middleware
- Request ID generation
- Metrics collection
- Logging

#### `models.rs`
- Data models
- Request/Response DTOs
- Database schema mapping

### API Modules

#### `api/health.rs`
- Health check endpoints
- Readiness probes
- Liveness probes

#### `api/users.rs`
- User CRUD operations
- User authentication
- User management

## Request Flow

1. **HTTP Request** arrives at Axum router
2. **Middleware Pipeline** processes request:
   - Request ID generation
   - Logging
   - CORS handling
   - Authentication (if protected route)
3. **Route Handler** receives request:
   - Validates input
   - Extracts parameters
4. **Service Layer** executes business logic:
   - Applies business rules
   - Interacts with database
5. **Response** is formatted and sent back through middleware

## Database Schema

### Users Table

```sql
users
- id (UUID, PRIMARY KEY)
- email (VARCHAR, UNIQUE)
- username (VARCHAR, UNIQUE)
- password_hash (TEXT)
- is_active (BOOLEAN)
- created_at (TIMESTAMPTZ)
- updated_at (TIMESTAMPTZ)
```

## Security Architecture

### Authentication Flow

1. User registers with email/password
2. Password is hashed using Argon2
3. User credentials stored in database
4. User logs in with email/password
5. Password verified against hash
6. JWT token generated with user claims
7. Token sent to client
8. Client includes token in subsequent requests
9. Middleware validates token
10. Claims extracted and added to request

### Security Layers

- **Transport**: HTTPS (in production)
- **Authentication**: JWT tokens
- **Password Storage**: Argon2 hashing
- **Input Validation**: Request validation
- **SQL Injection**: Parameterized queries
- **Error Handling**: No sensitive data leaks

## Scalability

### Horizontal Scaling

- Stateless application design
- JWT tokens (no session storage)
- Database connection pooling
- Can deploy multiple instances behind load balancer

### Performance

- Async/await throughout
- Connection pooling
- Compile-time optimizations
- Zero-copy deserialization where possible

## Extensibility

### Adding New Features

1. Create new model in `models.rs`
2. Create database migration if needed
3. Implement service logic
4. Create API handlers
5. Register routes in `main.rs`
6. Update OpenAPI documentation

### Adding New Middleware

1. Create middleware function in `middleware.rs`
2. Register in route/app setup in `main.rs`

## Configuration Management

Configuration is loaded in this priority order (highest to lowest):

1. Environment variables (prefix: `APP_`)
2. `config/local.toml` (not in git)
3. `config/{environment}.toml`
4. `config/default.toml`

## Monitoring and Observability

- Structured logging with tracing
- Request/response logging
- Health check endpoints
- Metrics collection ready
- Error tracking

## Deployment Architecture

### Development
- Local PostgreSQL
- Cargo run
- File-based configuration

### Docker
- Multi-stage build
- Minimal runtime image
- Health checks
- Non-root user

### Production
- Container orchestration (Kubernetes/ECS)
- External PostgreSQL (RDS/Cloud SQL)
- Environment-based configuration
- Auto-scaling
- Load balancing

## Future Enhancements

- Rate limiting
- API versioning
- Caching layer (Redis)
- Message queue integration
- Multi-tenancy support
- Advanced monitoring (Prometheus/Grafana)
- Distributed tracing
- Admin panel
- Email service integration
- File upload/storage
- Real-time features (WebSocket)
