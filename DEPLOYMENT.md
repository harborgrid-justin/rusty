# Production Deployment Guide

This guide provides detailed instructions for deploying Rusty SaaS to production environments.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Environment Setup](#environment-setup)
- [Database Configuration](#database-configuration)
- [Application Deployment](#application-deployment)
- [Monitoring and Observability](#monitoring-and-observability)
- [Security Hardening](#security-hardening)
- [Backup and Disaster Recovery](#backup-and-disaster-recovery)
- [Scaling Considerations](#scaling-considerations)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### Infrastructure Requirements

- **Server**: Minimum 2 CPU cores, 4GB RAM (recommended: 4 cores, 8GB RAM)
- **OS**: Ubuntu 22.04 LTS, Debian 12, or similar Linux distribution
- **Storage**: Minimum 50GB SSD (depends on data volume)
- **Network**: Static IP address, domain name, valid SSL certificate

### Software Requirements

- Docker 24.0+ and Docker Compose 2.20+
- PostgreSQL 16+ (or use Docker container)
- Rust 1.91+ (for building from source)
- nginx or Caddy for reverse proxy
- Let's Encrypt for SSL certificates

## Environment Setup

### 1. Clone the Repository

```bash
git clone https://github.com/harborgrid-justin/rusty.git
cd rusty
```

### 2. Configure Environment Variables

Copy the production environment template:

```bash
cp .env.production .env
```

Edit `.env` with production values:

```bash
# Generate a strong JWT secret
openssl rand -hex 64

# Update .env with the generated secret
nano .env
```

**Critical Environment Variables:**

- `APP_JWT__SECRET`: Use the generated 64-character random hex string
- `APP_DATABASE__URL`: Production database connection string
- `APP_CORS__ALLOWED_ORIGINS`: Your production domain(s)
- `RUST_LOG`: Set to `info` or `warn` for production

### 3. SSL/TLS Configuration

#### Using Let's Encrypt with Certbot:

```bash
sudo apt-get install certbot python3-certbot-nginx
sudo certbot --nginx -d api.yourdomain.com
```

#### Using Caddy (automatic HTTPS):

Create `Caddyfile`:

```
api.yourdomain.com {
    reverse_proxy localhost:8080
    
    # Security headers
    header {
        Strict-Transport-Security "max-age=31536000; includeSubDomains"
        X-Content-Type-Options "nosniff"
        X-Frame-Options "DENY"
        Referrer-Policy "strict-origin-when-cross-origin"
    }
    
    # Rate limiting
    rate_limit {
        zone dynamic {
            key {remote_host}
            events 100
            window 1m
        }
    }
}
```

## Database Configuration

### 1. Production PostgreSQL Setup

#### Using Managed Database (Recommended):

Use managed PostgreSQL from cloud providers:
- AWS RDS
- Google Cloud SQL
- Azure Database for PostgreSQL
- DigitalOcean Managed Databases

#### Self-Hosted PostgreSQL:

```bash
# Install PostgreSQL
sudo apt-get install postgresql-16 postgresql-contrib

# Configure PostgreSQL for production
sudo nano /etc/postgresql/16/main/postgresql.conf
```

**Recommended PostgreSQL Settings:**

```
max_connections = 200
shared_buffers = 2GB
effective_cache_size = 6GB
maintenance_work_mem = 512MB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
work_mem = 10485kB
min_wal_size = 1GB
max_wal_size = 4GB
```

### 2. Database Security

```sql
-- Create dedicated database user
CREATE USER rusty_app WITH PASSWORD 'strong_random_password';

-- Create database
CREATE DATABASE rusty_saas OWNER rusty_app;

-- Grant minimal required permissions
GRANT CONNECT ON DATABASE rusty_saas TO rusty_app;
GRANT USAGE ON SCHEMA public TO rusty_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO rusty_app;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO rusty_app;
```

### 3. Run Migrations

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Run migrations
sqlx migrate run
```

## Application Deployment

### Option 1: Docker Deployment (Recommended)

#### 1. Build the Docker Image

```bash
docker build -t rusty-saas:latest .
```

#### 2. Run with Docker Compose

```bash
# Start all services
docker-compose up -d

# Check logs
docker-compose logs -f app

# Stop services
docker-compose down
```

#### 3. With Monitoring Stack

```bash
# Start with monitoring
docker-compose --profile monitoring up -d
```

### Option 2: Binary Deployment

#### 1. Build Release Binary

```bash
cargo build --release
```

#### 2. Create Systemd Service

Create `/etc/systemd/system/rusty-saas.service`:

```ini
[Unit]
Description=Rusty SaaS Application
After=network.target postgresql.service

[Service]
Type=simple
User=rusty
WorkingDirectory=/opt/rusty-saas
EnvironmentFile=/opt/rusty-saas/.env
ExecStart=/opt/rusty-saas/rusty_saas
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/rusty-saas/data

[Install]
WantedBy=multi-user.target
```

#### 3. Start the Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable rusty-saas
sudo systemctl start rusty-saas
sudo systemctl status rusty-saas
```

## Monitoring and Observability

### 1. Application Metrics

Access Prometheus metrics at: `http://your-server:8080/metrics`

### 2. Health Checks

Configure health check endpoints:

- **Liveness**: `GET /live` - Returns 200 if app is running
- **Readiness**: `GET /ready` - Returns 200 if app is ready to serve traffic
- **Health**: `GET /health` - Returns detailed health status with database check

### 3. Log Management

#### Structured JSON Logging:

Set environment variable:
```bash
RUST_LOG=rusty_saas=info,tower_http=info
```

#### Log Aggregation:

Forward logs to centralized logging:
- **ELK Stack**: Elasticsearch, Logstash, Kibana
- **Grafana Loki**: Lightweight log aggregation
- **Cloud Services**: AWS CloudWatch, Google Cloud Logging, Azure Monitor

Example with systemd journal:

```bash
# View logs
journalctl -u rusty-saas -f

# Export to file
journalctl -u rusty-saas > /var/log/rusty-saas.log
```

### 4. Grafana Dashboards

Access Grafana at: `http://your-server:3000`

Default credentials (change immediately):
- Username: `admin`
- Password: Set via `GRAFANA_PASSWORD` env var

## Security Hardening

### 1. Firewall Configuration

```bash
# Allow only necessary ports
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable
```

### 2. Database Connection Security

Always use SSL/TLS for database connections:

```bash
APP_DATABASE__URL="postgres://user:pass@host:5432/db?sslmode=require"
```

### 3. Secrets Management

Use a secrets management service:
- **AWS Secrets Manager**
- **HashiCorp Vault**
- **Azure Key Vault**
- **Google Secret Manager**

### 4. Regular Security Updates

```bash
# Update system packages
sudo apt-get update && sudo apt-get upgrade -y

# Update Rust dependencies
cargo update
cargo audit
```

### 5. Rate Limiting

Configure rate limiting in reverse proxy (nginx example):

```nginx
limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;

server {
    location /api/ {
        limit_req zone=api_limit burst=20 nodelay;
        proxy_pass http://localhost:8080;
    }
}
```

## Backup and Disaster Recovery

### 1. Database Backups

#### Automated Backup Script:

Create `/opt/scripts/backup-db.sh`:

```bash
#!/bin/bash
BACKUP_DIR="/var/backups/postgres"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
DB_NAME="rusty_saas"

# Create backup directory
mkdir -p $BACKUP_DIR

# Backup database
pg_dump -U postgres $DB_NAME | gzip > $BACKUP_DIR/rusty_saas_$TIMESTAMP.sql.gz

# Keep only last 30 days of backups
find $BACKUP_DIR -name "rusty_saas_*.sql.gz" -mtime +30 -delete

echo "Backup completed: rusty_saas_$TIMESTAMP.sql.gz"
```

#### Schedule with Cron:

```bash
# Edit crontab
crontab -e

# Add daily backup at 2 AM
0 2 * * * /opt/scripts/backup-db.sh
```

### 2. Application State Backup

```bash
# Backup configuration
tar -czf /var/backups/rusty-config-$(date +%Y%m%d).tar.gz /opt/rusty-saas/config

# Backup uploaded files (if any)
tar -czf /var/backups/rusty-data-$(date +%Y%m%d).tar.gz /opt/rusty-saas/data
```

### 3. Disaster Recovery Plan

1. **Document Recovery Procedures**: Maintain runbooks for common scenarios
2. **Test Restores**: Regularly test backup restoration
3. **Multi-Region Deployment**: Deploy to multiple regions for high availability
4. **Database Replication**: Set up read replicas and failover

### 4. Restore from Backup

```bash
# Restore database
gunzip -c rusty_saas_20240101_020000.sql.gz | psql -U postgres rusty_saas
```

## Scaling Considerations

### Horizontal Scaling

Run multiple application instances behind a load balancer:

```
┌──────────────┐
│ Load Balancer│
└──────┬───────┘
       │
   ────┼────────
   │   │   │   │
   v   v   v   v
[App1][App2][App3][App4]
   │   │   │   │
   └───┴───┴───┴───> [Database]
```

### Database Scaling

1. **Read Replicas**: Offload read operations to replicas
2. **Connection Pooling**: Use PgBouncer for connection management
3. **Partitioning**: Partition large tables by date or tenant
4. **Caching**: Use Redis for frequently accessed data

### Performance Tuning

```toml
# config/production.toml
[database]
max_connections = 100  # Adjust based on load
min_connections = 10
acquire_timeout = 30
idle_timeout = 600
max_lifetime = 1800
```

## Troubleshooting

### Common Issues

#### 1. Application Won't Start

```bash
# Check logs
docker-compose logs app
# or
journalctl -u rusty-saas -n 100

# Verify environment variables
docker-compose config

# Test database connection
psql -h localhost -U postgres -d rusty_saas
```

#### 2. High Memory Usage

```bash
# Check resource usage
docker stats

# Adjust memory limits in docker-compose.yml
# Monitor with Grafana dashboards
```

#### 3. Slow Database Queries

```bash
# Enable query logging in PostgreSQL
ALTER SYSTEM SET log_min_duration_statement = 1000;  # Log queries > 1s
SELECT pg_reload_conf();

# Check slow queries
SELECT * FROM pg_stat_statements ORDER BY total_exec_time DESC LIMIT 10;
```

#### 4. Connection Pool Exhaustion

```toml
# Increase connection pool size
[database]
max_connections = 150
```

### Getting Help

- **Documentation**: Check the docs in the repository
- **GitHub Issues**: Report bugs and feature requests
- **Logs**: Always include relevant logs when seeking help
- **Metrics**: Check Grafana dashboards for insights

## Production Checklist

Before going live:

- [ ] Strong JWT secret configured
- [ ] Database credentials secured
- [ ] SSL/TLS certificates installed
- [ ] CORS configured for production domains
- [ ] Firewall rules configured
- [ ] Backups automated and tested
- [ ] Monitoring and alerting set up
- [ ] Log aggregation configured
- [ ] Rate limiting enabled
- [ ] Security headers configured
- [ ] Health checks working
- [ ] Load testing completed
- [ ] Disaster recovery plan documented
- [ ] Team trained on operations

## Maintenance

### Regular Tasks

**Daily:**
- Monitor error rates
- Check disk usage
- Review security alerts

**Weekly:**
- Review slow query logs
- Check backup success
- Update dependencies (after testing)

**Monthly:**
- Review access logs
- Rotate secrets (if policy requires)
- Performance review
- Security audit

---

**Need Help?** Consult the [README.md](README.md) for general information or open an issue on GitHub.
