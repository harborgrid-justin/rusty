# Operations Runbook

This document provides procedures for common operational tasks and incident response for Rusty SaaS.

## Table of Contents

- [System Health Checks](#system-health-checks)
- [Incident Response](#incident-response)
- [Common Operations](#common-operations)
- [Performance Tuning](#performance-tuning)
- [Scaling Operations](#scaling-operations)

## System Health Checks

### Quick Health Check

```bash
# Check application health
curl -f http://localhost:8080/health || echo "Health check failed"

# Check liveness
curl -f http://localhost:8080/live || echo "Liveness check failed"

# Check readiness
curl -f http://localhost:8080/ready || echo "Readiness check failed"
```

### Detailed System Status

```bash
# Application status
systemctl status rusty-saas
# or for Docker
docker-compose ps

# Database status
sudo -u postgres psql -c "SELECT version();"
sudo -u postgres psql -c "SELECT count(*) FROM pg_stat_activity;"

# Disk usage
df -h
du -sh /var/lib/postgresql/data
du -sh /var/log

# Memory usage
free -h
ps aux | grep rusty_saas | awk '{sum+=$4} END {print "Memory: " sum "%"}'

# Network connections
netstat -an | grep 8080 | wc -l
```

## Incident Response

### Severity Levels

- **P1 (Critical)**: Complete service outage, data loss
- **P2 (High)**: Partial outage, major feature broken
- **P3 (Medium)**: Minor feature broken, workaround available
- **P4 (Low)**: Cosmetic issue, no impact on functionality

### P1: Service Down

**Symptoms:**
- Health check endpoint not responding
- Users cannot access the application
- 5xx errors on all endpoints

**Investigation:**

```bash
# 1. Check if service is running
systemctl status rusty-saas
docker-compose ps

# 2. Check recent logs
journalctl -u rusty-saas -n 100 --no-pager
docker-compose logs --tail=100 app

# 3. Check system resources
top
df -h
free -h

# 4. Check database connectivity
psql -h localhost -U postgres -d rusty_saas -c "SELECT 1;"
```

**Resolution Steps:**

```bash
# 1. Restart the service
sudo systemctl restart rusty-saas
# or
docker-compose restart app

# 2. If restart fails, check configuration
docker-compose config

# 3. Check environment variables
cat .env

# 4. If database is down, restart it
sudo systemctl restart postgresql
# or
docker-compose restart postgres

# 5. Verify recovery
curl http://localhost:8080/health
```

### P1: Database Connection Issues

**Symptoms:**
- Application running but all DB queries failing
- "Connection refused" or "Too many connections" errors

**Investigation:**

```bash
# Check active connections
sudo -u postgres psql -c "SELECT count(*) FROM pg_stat_activity;"

# Check connection limits
sudo -u postgres psql -c "SHOW max_connections;"

# Check for locks
sudo -u postgres psql -d rusty_saas -c "
  SELECT pid, usename, application_name, state, query
  FROM pg_stat_activity
  WHERE state != 'idle';
"
```

**Resolution:**

```bash
# If connection pool exhausted, restart app
sudo systemctl restart rusty-saas

# If PostgreSQL max_connections reached, kill idle connections
sudo -u postgres psql -c "
  SELECT pg_terminate_backend(pid)
  FROM pg_stat_activity
  WHERE state = 'idle'
  AND state_change < NOW() - INTERVAL '10 minutes';
"

# Increase max_connections (requires restart)
sudo nano /etc/postgresql/16/main/postgresql.conf
# Set: max_connections = 200
sudo systemctl restart postgresql
```

### P2: High Response Times

**Symptoms:**
- Application responding slowly
- Request timeouts
- High latency in metrics

**Investigation:**

```bash
# Check CPU and memory
top
htop

# Check slow queries
sudo -u postgres psql -d rusty_saas -c "
  SELECT pid, now() - pg_stat_activity.query_start AS duration, query
  FROM pg_stat_activity
  WHERE state = 'active'
  ORDER BY duration DESC;
"

# Check connection pool
# Look for "waiting for connection" in logs
journalctl -u rusty-saas -n 100 | grep -i "waiting"
```

**Resolution:**

```bash
# 1. Identify slow queries and add indexes
# 2. Scale up database connection pool
# 3. Add application instances
# 4. Enable caching for frequently accessed data
```

### P2: High Error Rate

**Symptoms:**
- Increased 5xx errors
- Errors in application logs

**Investigation:**

```bash
# Check error logs
journalctl -u rusty-saas -p err -n 50

# Check error rate in Prometheus
# Query: rate(http_requests_total{status=~"5.."}[5m])

# Identify error patterns
journalctl -u rusty-saas -n 1000 | grep ERROR | sort | uniq -c | sort -rn
```

## Common Operations

### Deploying Updates

**Zero-Downtime Deployment:**

```bash
# 1. Build new version
cargo build --release

# 2. Test health check on new build
./target/release/rusty_saas &
NEW_PID=$!
sleep 5
curl -f http://localhost:8081/health || kill $NEW_PID
kill $NEW_PID

# 3. Deploy with systemd
sudo cp target/release/rusty_saas /opt/rusty-saas/rusty_saas.new
sudo systemctl stop rusty-saas
sudo mv /opt/rusty-saas/rusty_saas.new /opt/rusty-saas/rusty_saas
sudo systemctl start rusty-saas

# 4. Verify deployment
curl http://localhost:8080/health
journalctl -u rusty-saas -n 20 --no-pager
```

**Docker Deployment:**

```bash
# 1. Build new image
docker build -t rusty-saas:$(git rev-parse --short HEAD) .

# 2. Tag as latest
docker tag rusty-saas:$(git rev-parse --short HEAD) rusty-saas:latest

# 3. Update and restart
docker-compose up -d --no-deps --build app

# 4. Verify
docker-compose ps
docker-compose logs --tail=50 app
curl http://localhost:8080/health
```

### Database Maintenance

**Vacuum Database:**

```bash
# Analyze database
sudo -u postgres psql -d rusty_saas -c "VACUUM ANALYZE;"

# Full vacuum (requires exclusive lock, plan downtime)
sudo -u postgres psql -d rusty_saas -c "VACUUM FULL;"
```

**Rebuild Indexes:**

```bash
sudo -u postgres psql -d rusty_saas -c "REINDEX DATABASE rusty_saas;"
```

**Update Statistics:**

```bash
sudo -u postgres psql -d rusty_saas -c "ANALYZE;"
```

### Log Rotation

**Configure logrotate:**

Create `/etc/logrotate.d/rusty-saas`:

```
/var/log/rusty-saas/*.log {
    daily
    rotate 14
    compress
    delaycompress
    notifempty
    create 0640 rusty rusty
    sharedscripts
    postrotate
        systemctl reload rusty-saas > /dev/null 2>&1 || true
    endscript
}
```

### SSL Certificate Renewal

**Let's Encrypt with Certbot:**

```bash
# Check expiration
sudo certbot certificates

# Renew (automatic with systemd timer)
sudo certbot renew

# Test renewal
sudo certbot renew --dry-run
```

## Performance Tuning

### Database Optimization

**Identify Missing Indexes:**

```sql
-- Find tables without indexes
SELECT schemaname, tablename, attname, n_distinct, correlation
FROM pg_stats
WHERE schemaname = 'public'
AND n_distinct > 100
ORDER BY schemaname, tablename;

-- Find unused indexes
SELECT schemaname, tablename, indexname, idx_scan
FROM pg_stat_user_indexes
WHERE idx_scan = 0
ORDER BY schemaname, tablename;
```

**Add Indexes:**

```sql
-- Example: Add index on frequently queried column
CREATE INDEX CONCURRENTLY idx_cases_status ON cases(status);
```

### Connection Pool Tuning

Adjust based on load:

```toml
# config/production.toml
[database]
max_connections = 100    # Total connections
min_connections = 10     # Always maintain
acquire_timeout = 30     # Wait time for connection
idle_timeout = 600       # Close idle connections after 10 min
max_lifetime = 1800      # Recycle connections after 30 min
```

### Application Tuning

**Tokio Runtime:**

Set environment variables:

```bash
# Number of worker threads (default: CPU cores)
export TOKIO_WORKER_THREADS=8

# Enable work-stealing
export TOKIO_ENABLE_WORK_STEALING=1
```

## Scaling Operations

### Vertical Scaling (Scale Up)

**Increase Resources:**

```bash
# Update server resources (CPU, RAM)
# Adjust database connection pool
# Update docker-compose resource limits

# docker-compose.yml
deploy:
  resources:
    limits:
      cpus: '4'
      memory: 4G
```

### Horizontal Scaling (Scale Out)

**Add Application Instances:**

```bash
# Docker Compose
docker-compose up -d --scale app=3

# Verify
docker-compose ps
```

**Configure Load Balancer:**

nginx configuration:

```nginx
upstream rusty_backend {
    least_conn;
    server 10.0.1.10:8080 max_fails=3 fail_timeout=30s;
    server 10.0.1.11:8080 max_fails=3 fail_timeout=30s;
    server 10.0.1.12:8080 max_fails=3 fail_timeout=30s;
}

server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://rusty_backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Database Scaling

**Setup Read Replica:**

```bash
# On primary server
sudo -u postgres psql -c "CREATE ROLE replicator WITH REPLICATION LOGIN PASSWORD 'rep_password';"

# Configure pg_hba.conf to allow replication
# host replication replicator replica_ip/32 md5
```

**Configure Application:**

```toml
# config/production.toml
[database]
# Write operations
primary_url = "postgres://user:pass@primary:5432/db"

# Read operations
replica_urls = [
    "postgres://user:pass@replica1:5432/db",
    "postgres://user:pass@replica2:5432/db"
]
```

## Monitoring Alerts

### Critical Alerts

**Application Down:**
- Trigger: Health check fails for 3 consecutive attempts
- Action: Page on-call engineer immediately

**Database Down:**
- Trigger: Cannot connect to database
- Action: Page on-call engineer immediately

**Disk Space Critical:**
- Trigger: Disk usage > 90%
- Action: Immediate cleanup or scale storage

### Warning Alerts

**High Response Time:**
- Trigger: p95 latency > 1000ms for 5 minutes
- Action: Investigate and optimize

**High Error Rate:**
- Trigger: Error rate > 1% for 5 minutes
- Action: Review logs and investigate

**Connection Pool Saturation:**
- Trigger: Connection pool usage > 80%
- Action: Scale connection pool or add instances

## Emergency Contacts

Maintain an on-call rotation with:
- Primary on-call engineer
- Backup on-call engineer
- Database administrator
- DevOps lead

## Post-Incident Review

After any P1/P2 incident:

1. Document timeline of events
2. Identify root cause
3. List action items to prevent recurrence
4. Update runbook with lessons learned
5. Schedule postmortem meeting

---

**Keep this runbook updated!** Every incident is an opportunity to improve procedures.
