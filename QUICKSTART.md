# Quick Start Guide

Get your Rusty SaaS application running in minutes!

## Prerequisites

- **Rust 1.91+**: Install from [rustup.rs](https://rustup.rs/)
- **PostgreSQL 14+**: Or use Docker
- **Docker** (optional): For containerized deployment

## Local Development Setup

### 1. Clone and Setup

```bash
git clone https://github.com/harborgrid-justin/rusty.git
cd rusty
cp .env.example .env
```

### 2. Start PostgreSQL

**Option A: Using Docker**
```bash
docker run -d \
  --name rusty-postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=rusty_saas \
  -p 5432:5432 \
  postgres:16-alpine
```

**Option B: Using Docker Compose (includes app)**
```bash
docker-compose up -d
# Skip to step 5 if using this option
```

### 3. Configure Environment

Edit `.env` if needed:
```env
APP_DATABASE__URL=postgres://postgres:postgres@localhost:5432/rusty_saas
APP_JWT__SECRET=your-secret-key-change-this
```

### 4. Build and Run

```bash
# Build
cargo build --release

# Run (migrations run automatically)
cargo run --release
```

### 5. Verify It's Working

```bash
# Health check
curl http://localhost:8080/health

# Expected response:
# {"status":"healthy","version":"0.1.0","timestamp":"..."}
```

## First API Calls

### Create a User

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "username": "admin",
    "password": "SecurePass123!"
  }'
```

### Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@example.com",
    "password": "SecurePass123!"
  }'
```

**Save the token from the response!**

### Get Current User (Protected Route)

```bash
curl http://localhost:8080/api/users/me \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

## Explore the API

Visit **http://localhost:8080/swagger-ui** for interactive API documentation!

## Development Commands

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check

# Build for production
cargo build --release

# Run with auto-reload (install cargo-watch first)
cargo install cargo-watch
cargo watch -x run
```

## Using Make

```bash
# View all available commands
make help

# Quick commands
make build    # Build the project
make run      # Run the application
make test     # Run tests
make lint     # Run linter
make fmt      # Format code
```

## Docker Deployment

### Build Image

```bash
docker build -t rusty-saas:latest .
```

### Run Container

```bash
docker run -d \
  -p 8080:8080 \
  -e APP_DATABASE__URL=postgres://user:pass@host/db \
  -e APP_JWT__SECRET=your-secret \
  --name rusty-saas \
  rusty-saas:latest
```

### Using Docker Compose

```bash
# Start all services
docker-compose up -d

# View logs
docker-compose logs -f app

# Stop services
docker-compose down
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `APP_SERVER__HOST` | Server host | `0.0.0.0` |
| `APP_SERVER__PORT` | Server port | `8080` |
| `APP_DATABASE__URL` | PostgreSQL connection string | See .env.example |
| `APP_JWT__SECRET` | JWT signing secret | **MUST CHANGE** |
| `APP_JWT__EXPIRATION_HOURS` | Token expiration | `24` |
| `RUST_LOG` | Logging level | `info` |

## Troubleshooting

### Database Connection Failed

- Check PostgreSQL is running: `pg_isready -h localhost`
- Verify connection string in `.env`
- Ensure database exists: `createdb rusty_saas`

### Compilation Errors

- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`

### Port Already in Use

- Change port in `.env`: `APP_SERVER__PORT=3000`
- Or kill process: `lsof -ti:8080 | xargs kill`

### JWT Token Invalid

- Check your secret matches between server config and client
- Ensure token hasn't expired (24h default)

## Next Steps

1. **Read the docs**: Check `README.md` for detailed information
2. **Explore API**: Visit Swagger UI at `/swagger-ui`
3. **Extend**: See `EXTENDING.md` for adding features
4. **Architecture**: Read `ARCHITECTURE.md` to understand the design
5. **Examples**: Check `API_EXAMPLES.md` for more usage examples

## Production Checklist

Before deploying to production:

- [ ] Change JWT secret to a strong random value
- [ ] Use a production PostgreSQL database
- [ ] Configure CORS allowed origins
- [ ] Set up HTTPS/TLS
- [ ] Configure logging aggregation
- [ ] Set up monitoring and alerting
- [ ] Implement rate limiting
- [ ] Review security settings
- [ ] Set up database backups
- [ ] Configure CI/CD pipeline
- [ ] Set proper resource limits
- [ ] Enable health checks in orchestrator
- [ ] Review and update environment variables

## Support

- **Issues**: Use GitHub Issues
- **Documentation**: See docs in repository
- **Examples**: Check `API_EXAMPLES.md`

---

**Happy Coding! 🚀**
