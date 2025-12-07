# Backup and Disaster Recovery Plan

This document outlines backup strategies and disaster recovery procedures for Rusty SaaS.

## Table of Contents

- [Backup Strategy](#backup-strategy)
- [Backup Procedures](#backup-procedures)
- [Restore Procedures](#restore-procedures)
- [Disaster Recovery](#disaster-recovery)
- [Testing](#testing)
- [Compliance](#compliance)

## Backup Strategy

### Backup Types

1. **Full Backup**: Complete copy of all data
2. **Incremental Backup**: Only changes since last backup
3. **Differential Backup**: Changes since last full backup

### Backup Schedule

| Type | Frequency | Retention | Storage Location |
|------|-----------|-----------|------------------|
| Full Database | Daily at 2 AM UTC | 30 days | S3/Cloud Storage |
| Incremental DB | Every 6 hours | 7 days | S3/Cloud Storage |
| Configuration | On change + Daily | 90 days | Git + S3 |
| Application Logs | Continuous | 30 days | Log aggregation |
| Transaction Logs | Continuous | 7 days | S3/Cloud Storage |

### Backup Locations

**Primary**: AWS S3 / Google Cloud Storage / Azure Blob Storage
**Secondary**: Different cloud provider or region
**Tertiary**: On-premise or cold storage (for compliance)

## Backup Procedures

### Database Backups

#### Automated Daily Full Backup

Create `/opt/scripts/backup-database.sh`:

```bash
#!/bin/bash
set -euo pipefail

# Configuration
BACKUP_DIR="/var/backups/postgres"
S3_BUCKET="s3://your-backup-bucket/rusty-saas/database"
DB_NAME="rusty_saas"
DB_USER="postgres"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RETENTION_DAYS=30

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Perform backup
BACKUP_FILE="$BACKUP_DIR/${DB_NAME}_${TIMESTAMP}.sql.gz"
pg_dump -U "$DB_USER" -Fc "$DB_NAME" | gzip > "$BACKUP_FILE"

# Calculate backup size
BACKUP_SIZE=$(du -h "$BACKUP_FILE" | cut -f1)
echo "Backup created: $BACKUP_FILE ($BACKUP_SIZE)"

# Upload to S3
aws s3 cp "$BACKUP_FILE" "$S3_BUCKET/" \
  --storage-class STANDARD_IA \
  --server-side-encryption AES256

# Verify upload
if aws s3 ls "$S3_BUCKET/$(basename $BACKUP_FILE)" > /dev/null; then
    echo "Backup uploaded successfully to S3"
    
    # Optional: Remove local backup after successful upload
    # rm "$BACKUP_FILE"
else
    echo "ERROR: Backup upload to S3 failed"
    exit 1
fi

# Clean up old local backups
find "$BACKUP_DIR" -name "${DB_NAME}_*.sql.gz" -mtime +7 -delete

# Clean up old S3 backups
aws s3 ls "$S3_BUCKET/" | \
  grep "\.sql\.gz$" | \
  awk '{print $4}' | \
  sort | \
  head -n -$RETENTION_DAYS | \
  while read file; do
    aws s3 rm "$S3_BUCKET/$file"
    echo "Deleted old backup: $file"
  done

# Send notification
echo "Database backup completed: $BACKUP_FILE ($BACKUP_SIZE)" | \
  mail -s "Rusty SaaS Backup Success" admin@yourdomain.com

echo "Backup process completed successfully"
```

Make it executable and schedule:

```bash
chmod +x /opt/scripts/backup-database.sh

# Add to crontab
crontab -e
# 0 2 * * * /opt/scripts/backup-database.sh >> /var/log/backup.log 2>&1
```

#### Incremental Backup (WAL Archiving)

Configure PostgreSQL for WAL archiving in `postgresql.conf`:

```
# Enable WAL archiving
wal_level = replica
archive_mode = on
archive_command = 'test ! -f /var/lib/postgresql/wal_archive/%f && cp %p /var/lib/postgresql/wal_archive/%f'
archive_timeout = 3600  # Archive every hour
```

Create WAL backup script `/opt/scripts/backup-wal.sh`:

```bash
#!/bin/bash
set -euo pipefail

WAL_ARCHIVE="/var/lib/postgresql/wal_archive"
S3_BUCKET="s3://your-backup-bucket/rusty-saas/wal"

# Sync WAL files to S3
aws s3 sync "$WAL_ARCHIVE" "$S3_BUCKET/" \
  --storage-class STANDARD_IA \
  --exclude "*" --include "*.gz"

# Clean up old local WAL files (keep 24 hours)
find "$WAL_ARCHIVE" -name "*.gz" -mtime +1 -delete
```

### Configuration Backups

Backup configuration files:

```bash
#!/bin/bash
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
CONFIG_BACKUP="/var/backups/config/rusty-saas-config-$TIMESTAMP.tar.gz"
S3_BUCKET="s3://your-backup-bucket/rusty-saas/config"

# Backup configuration
tar -czf "$CONFIG_BACKUP" \
  /opt/rusty-saas/config/ \
  /opt/rusty-saas/.env \
  /etc/systemd/system/rusty-saas.service \
  /etc/nginx/sites-available/rusty-saas

# Upload to S3
aws s3 cp "$CONFIG_BACKUP" "$S3_BUCKET/"

# Upload to Git (without secrets)
cd /opt/rusty-saas
git add config/*.toml
git commit -m "Backup configuration $TIMESTAMP"
git push origin backup-branch
```

### Application State Backup

If your application stores files:

```bash
#!/bin/bash
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
DATA_DIR="/opt/rusty-saas/data"
BACKUP_FILE="/var/backups/app-data-$TIMESTAMP.tar.gz"
S3_BUCKET="s3://your-backup-bucket/rusty-saas/app-data"

tar -czf "$BACKUP_FILE" "$DATA_DIR"
aws s3 cp "$BACKUP_FILE" "$S3_BUCKET/"
```

## Restore Procedures

### Database Restore

#### Full Restore from Backup

```bash
#!/bin/bash
set -euo pipefail

BACKUP_FILE="$1"
DB_NAME="rusty_saas"
DB_USER="postgres"

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup-file>"
    exit 1
fi

# Download from S3 if it's an S3 path
if [[ "$BACKUP_FILE" == s3://* ]]; then
    LOCAL_BACKUP="/tmp/$(basename $BACKUP_FILE)"
    aws s3 cp "$BACKUP_FILE" "$LOCAL_BACKUP"
    BACKUP_FILE="$LOCAL_BACKUP"
fi

# Stop application
echo "Stopping application..."
systemctl stop rusty-saas

# Drop and recreate database
echo "Recreating database..."
sudo -u postgres psql -c "DROP DATABASE IF EXISTS ${DB_NAME};"
sudo -u postgres psql -c "CREATE DATABASE ${DB_NAME};"

# Restore backup
echo "Restoring from backup: $BACKUP_FILE"
gunzip -c "$BACKUP_FILE" | sudo -u postgres pg_restore -d "$DB_NAME" --no-owner --no-acl

# Verify restore
echo "Verifying restore..."
RECORD_COUNT=$(sudo -u postgres psql -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM users;")
echo "Restored database contains $RECORD_COUNT users"

# Start application
echo "Starting application..."
systemctl start rusty-saas

# Verify application
sleep 5
curl -f http://localhost:8080/health || echo "WARNING: Health check failed"

echo "Restore completed successfully"
```

#### Point-in-Time Recovery (PITR)

```bash
#!/bin/bash
set -euo pipefail

TARGET_TIME="$1"  # Format: YYYY-MM-DD HH:MM:SS
BASE_BACKUP="$2"   # S3 path to base backup
WAL_ARCHIVE="s3://your-backup-bucket/rusty-saas/wal"

# Stop application
systemctl stop rusty-saas

# Restore base backup
echo "Restoring base backup..."
aws s3 cp "$BASE_BACKUP" /tmp/base_backup.sql.gz
gunzip -c /tmp/base_backup.sql.gz | sudo -u postgres pg_restore -d rusty_saas_pitr

# Download WAL files
echo "Downloading WAL archive..."
mkdir -p /var/lib/postgresql/wal_restore
aws s3 sync "$WAL_ARCHIVE" /var/lib/postgresql/wal_restore/

# Configure recovery
sudo -u postgres cat > /var/lib/postgresql/data/recovery.conf << EOF
restore_command = 'cp /var/lib/postgresql/wal_restore/%f %p'
recovery_target_time = '$TARGET_TIME'
recovery_target_action = 'promote'
EOF

# Start PostgreSQL in recovery mode
systemctl start postgresql

# Wait for recovery to complete
while sudo -u postgres psql -c "SELECT pg_is_in_recovery();" | grep -q "t"; do
    echo "Recovery in progress..."
    sleep 5
done

echo "Point-in-time recovery completed to $TARGET_TIME"
```

### Configuration Restore

```bash
#!/bin/bash
BACKUP_FILE="$1"

# Download from S3
aws s3 cp "s3://your-backup-bucket/rusty-saas/config/$BACKUP_FILE" /tmp/

# Extract
tar -xzf "/tmp/$BACKUP_FILE" -C /

# Reload services
systemctl daemon-reload
systemctl reload nginx
```

## Disaster Recovery

### Disaster Scenarios

1. **Database Corruption**: Restore from latest backup
2. **Complete Server Failure**: Deploy to new server from backups
3. **Data Center Failure**: Failover to secondary region
4. **Ransomware Attack**: Restore from immutable backups
5. **Accidental Data Deletion**: Point-in-time recovery

### Recovery Time Objectives (RTO)

- **Critical (P1)**: 1 hour
- **High (P2)**: 4 hours
- **Medium (P3)**: 24 hours
- **Low (P4)**: 72 hours

### Recovery Point Objectives (RPO)

- **Database**: 6 hours (incremental backups every 6 hours)
- **Configuration**: 24 hours
- **Application Logs**: Real-time (continuous streaming)

### Disaster Recovery Steps

#### 1. Assessment Phase

```bash
# Determine extent of damage
# - What failed?
# - When did it fail?
# - What data is affected?
# - What is the most recent good backup?
```

#### 2. Communication Phase

```bash
# Notify stakeholders
# - Send incident notification
# - Provide status updates every 30 minutes
# - Escalate if RTO will be exceeded
```

#### 3. Recovery Phase

**Complete Server Rebuild:**

```bash
# 1. Provision new server
terraform apply -var="disaster_recovery=true"

# 2. Install dependencies
sudo apt-get update && sudo apt-get install -y postgresql-16 docker docker-compose

# 3. Restore configuration
aws s3 cp s3://your-backup-bucket/rusty-saas/config/latest.tar.gz /tmp/
tar -xzf /tmp/latest.tar.gz -C /

# 4. Restore database
./scripts/restore-database.sh s3://your-backup-bucket/rusty-saas/database/rusty_saas_YYYYMMDD.sql.gz

# 5. Deploy application
docker-compose up -d

# 6. Verify services
./scripts/health-check.sh
```

#### 4. Verification Phase

```bash
# Verify data integrity
# - Run data validation scripts
# - Check critical user accounts
# - Verify recent transactions
# - Test critical workflows
```

#### 5. Post-Recovery Phase

```bash
# Document incident
# - What happened
# - What was the impact
# - How was it resolved
# - What can be improved

# Update runbook
# - Add new procedures
# - Update contact information
# - Improve monitoring
```

### High Availability Setup

For critical deployments, implement HA:

```
Primary Region (us-east-1)
├── Load Balancer
├── App Server 1 (Active)
├── App Server 2 (Active)
└── PostgreSQL Primary

Secondary Region (us-west-2)
├── App Server 3 (Standby)
└── PostgreSQL Replica (Read-only)
```

## Testing

### Backup Verification

Test backups monthly:

```bash
#!/bin/bash
# Monthly backup test script

# 1. Get latest backup
LATEST_BACKUP=$(aws s3 ls s3://your-backup-bucket/rusty-saas/database/ | \
  grep "\.sql\.gz$" | \
  sort | \
  tail -n 1 | \
  awk '{print $4}')

# 2. Restore to test database
./scripts/restore-test-database.sh "$LATEST_BACKUP"

# 3. Run validation queries
psql -d rusty_saas_test -c "SELECT COUNT(*) FROM users;"
psql -d rusty_saas_test -c "SELECT COUNT(*) FROM cases;"

# 4. Report results
echo "Backup test completed for: $LATEST_BACKUP"
```

### Disaster Recovery Drills

Conduct quarterly DR drills:

**Drill Checklist:**
- [ ] Simulate failure scenario
- [ ] Follow DR procedures
- [ ] Time the recovery process
- [ ] Verify data integrity
- [ ] Document issues found
- [ ] Update procedures
- [ ] Train team members

## Compliance

### GDPR/Data Protection

- Encrypt backups at rest
- Encrypt backups in transit
- Implement access controls
- Log all backup/restore operations
- Implement data retention policies
- Provide mechanisms for data deletion

### Audit Requirements

Maintain audit logs for:
- Backup creation
- Backup deletion
- Restore operations
- Access to backups
- Configuration changes

### Retention Policies

| Data Type | Retention Period | Justification |
|-----------|------------------|---------------|
| Database Backups | 30 days | Business requirement |
| WAL Archives | 7 days | Technical requirement |
| Configuration | 90 days | Audit requirement |
| Application Logs | 30 days | Operational requirement |
| Security Logs | 1 year | Compliance requirement |

## Monitoring and Alerts

### Backup Monitoring

Set up alerts for:
- Backup failures
- Backup duration exceeds threshold
- Backup size anomalies
- Missing backups
- Storage quota warnings

### Example Alert Rules

```yaml
# Prometheus alerting rules
groups:
  - name: backup_alerts
    rules:
      - alert: BackupFailed
        expr: backup_last_success_timestamp{job="rusty-saas"} < time() - 86400
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Database backup has failed"
          description: "No successful backup in last 24 hours"
      
      - alert: BackupSizeAnomaly
        expr: abs(backup_size_bytes - backup_size_bytes offset 24h) / backup_size_bytes > 0.5
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Backup size changed significantly"
          description: "Backup size changed by more than 50%"
```

## Contact Information

### Emergency Contacts

**Primary On-Call**: [Name] - [Phone] - [Email]
**Backup On-Call**: [Name] - [Phone] - [Email]
**DBA**: [Name] - [Phone] - [Email]
**DevOps Lead**: [Name] - [Phone] - [Email]

### Escalation Path

1. On-call engineer (0-15 minutes)
2. Backup on-call (15-30 minutes)
3. Engineering manager (30-60 minutes)
4. CTO (60+ minutes)

---

**Remember**: The best disaster recovery plan is one that is tested regularly. Schedule and execute recovery drills to ensure procedures are up-to-date and team members are trained.
