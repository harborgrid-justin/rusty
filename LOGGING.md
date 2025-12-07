# Logging Configuration Guide

This document describes the logging configuration for Rusty SaaS and best practices for production logging.

## Log Levels

The application uses the following log levels (in order of severity):

1. **ERROR**: Critical errors that require immediate attention
2. **WARN**: Warning conditions that should be reviewed
3. **INFO**: General informational messages (default for production)
4. **DEBUG**: Detailed information for debugging (use sparingly in production)
5. **TRACE**: Very detailed information (avoid in production)

## Environment Configuration

### Development

```bash
# Verbose logging for development
RUST_LOG=rusty_saas=debug,tower_http=debug,sqlx=debug
```

### Staging

```bash
# Balanced logging for staging
RUST_LOG=rusty_saas=debug,tower_http=info,sqlx=info
```

### Production

```bash
# Minimal logging for production performance
RUST_LOG=rusty_saas=info,tower_http=info,sqlx=warn
```

### Performance-Critical Production

```bash
# Minimal logging for high-throughput scenarios
RUST_LOG=rusty_saas=warn,tower_http=warn,sqlx=error
```

## Structured Logging

The application uses structured logging with the `tracing` crate. All logs are output in a structured format that can be easily parsed by log aggregation systems.

### JSON Format

To enable JSON output (recommended for production):

```bash
# Set in environment
RUST_LOG_FORMAT=json
```

Example JSON log entry:

```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "target": "rusty_saas",
  "fields": {
    "message": "Request processed",
    "method": "GET",
    "uri": "/api/users",
    "status": 200,
    "duration_ms": 45,
    "request_id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

## Log Fields

### Standard Fields

Every log entry includes:
- `timestamp`: ISO 8601 timestamp
- `level`: Log level (ERROR, WARN, INFO, DEBUG, TRACE)
- `target`: Module that generated the log
- `message`: Human-readable message

### Request Logs

HTTP request logs include:
- `request_id`: Unique identifier for request tracing
- `method`: HTTP method (GET, POST, etc.)
- `uri`: Request URI
- `status`: HTTP status code
- `duration_ms`: Request processing time in milliseconds
- `remote_ip`: Client IP address (if available)
- `user_agent`: Client user agent

### Database Logs

Database operation logs include:
- `query`: SQL query (sanitized in production)
- `duration_ms`: Query execution time
- `rows_affected`: Number of rows affected

### Error Logs

Error logs include:
- `error`: Error message
- `error_type`: Type of error
- `backtrace`: Stack trace (only in debug builds)

## Log Aggregation

### ELK Stack Integration

Forward logs to Elasticsearch:

```bash
# Using filebeat
filebeat.inputs:
- type: log
  enabled: true
  paths:
    - /var/log/rusty-saas/*.log
  json.keys_under_root: true
  json.add_error_key: true

output.elasticsearch:
  hosts: ["elasticsearch:9200"]
  index: "rusty-saas-%{+yyyy.MM.dd}"
```

### Grafana Loki Integration

Forward logs to Loki:

```bash
# Using promtail
scrape_configs:
  - job_name: rusty-saas
    static_configs:
      - targets:
          - localhost
        labels:
          job: rusty-saas
          __path__: /var/log/rusty-saas/*.log
```

### Cloud Logging

#### AWS CloudWatch

```bash
# Using awslogs Docker driver
docker run --log-driver=awslogs \
  --log-opt awslogs-region=us-east-1 \
  --log-opt awslogs-group=rusty-saas \
  --log-opt awslogs-stream=app \
  rusty-saas:latest
```

#### Google Cloud Logging

```bash
# Using gcplogs Docker driver
docker run --log-driver=gcplogs \
  --log-opt gcp-project=your-project \
  --log-opt labels=app=rusty-saas \
  rusty-saas:latest
```

## Log Rotation

### Using logrotate

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
        /bin/systemctl reload rusty-saas > /dev/null 2>&1 || true
    endscript
}
```

### Docker Logging

Configure in `docker-compose.yml`:

```yaml
services:
  app:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "5"
```

## Sensitive Data

### Never Log

- Passwords or password hashes
- JWT tokens or API keys
- Credit card numbers
- Social Security Numbers
- Personal identification information (unless required by law)
- Database credentials

### Redaction Example

```rust
// Bad - logs sensitive data
tracing::info!("User login: {} with password {}", username, password);

// Good - no sensitive data
tracing::info!("User login attempt: {}", username);

// Better - structured logging
tracing::info!(
    username = %username,
    "User login attempt"
);
```

## Performance Considerations

### Log Level Impact

Approximate performance impact:
- **ERROR/WARN**: ~5-10 µs per log call
- **INFO**: ~10-20 µs per log call
- **DEBUG**: ~20-50 µs per log call
- **TRACE**: ~50-100 µs per log call

### Best Practices

1. Use appropriate log levels
2. Avoid logging in tight loops
3. Use conditional logging for expensive operations
4. Batch logs when possible
5. Use async logging for high-throughput scenarios

### Conditional Logging

```rust
// Only compute expensive debug information if debug logging is enabled
if tracing::level_enabled!(tracing::Level::DEBUG) {
    let expensive_debug_info = compute_expensive_debug_info();
    tracing::debug!("Debug info: {:?}", expensive_debug_info);
}
```

## Monitoring and Alerting

### Log-Based Alerts

Set up alerts for:
- High error rate (> 1% of requests)
- Repeated errors (same error > 10 times/minute)
- Slow queries (duration > 1000ms)
- Authentication failures (> 5 failures/minute from same IP)

### Example Alert Rules

**High Error Rate:**
```promql
rate(log_messages_total{level="error"}[5m]) > 0.01
```

**Repeated Errors:**
```promql
increase(log_messages_total{level="error"}[1m]) > 10
```

## Troubleshooting

### Enable Debug Logging

Temporarily enable debug logging without restart:

```bash
# Send SIGUSR1 to toggle debug logging (if implemented)
kill -USR1 $(pidof rusty_saas)
```

### View Recent Errors

```bash
# Last 100 error logs
journalctl -u rusty-saas -p err -n 100

# Errors in last hour
journalctl -u rusty-saas -p err --since "1 hour ago"

# Follow error logs
journalctl -u rusty-saas -p err -f
```

### Search Logs

```bash
# Search for specific error
journalctl -u rusty-saas | grep "database connection"

# Search with context
journalctl -u rusty-saas | grep -A 5 -B 5 "error"
```

## Compliance

### GDPR Considerations

- Log only necessary personal data
- Implement log retention policies
- Provide mechanisms for data deletion
- Encrypt logs at rest
- Restrict log access

### Retention Policies

Recommended retention periods:
- **Security logs**: 1 year
- **Access logs**: 90 days
- **Application logs**: 30 days
- **Debug logs**: 7 days

## Log Analysis

### Common Queries

**Top Error Messages:**
```bash
journalctl -u rusty-saas -p err --since today | \
  grep -o "error.*" | sort | uniq -c | sort -rn | head -10
```

**Slowest Requests:**
```bash
journalctl -u rusty-saas --since today | \
  grep "duration_ms" | \
  awk '{print $NF}' | \
  sort -rn | \
  head -10
```

**Request Count by Endpoint:**
```bash
journalctl -u rusty-saas --since today | \
  grep "Request processed" | \
  grep -o '"uri":"[^"]*"' | \
  sort | uniq -c | sort -rn
```

---

**Remember**: Good logging is a balance between visibility and performance. Log what you need to troubleshoot issues, but not so much that it impacts application performance or storage costs.
