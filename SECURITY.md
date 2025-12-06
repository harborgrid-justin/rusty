# Security Considerations

This document outlines security best practices and considerations for the Rusty SaaS platform.

## Authentication & Authorization

### JWT Tokens

- **Secret Management**: Never commit JWT secrets to git
  - Use environment variables: `APP_JWT__SECRET`
  - Rotate secrets regularly in production
  - Use strong, random values (min 32 characters)

- **Token Expiration**: Default is 24 hours
  - Adjust via `APP_JWT__EXPIRATION_HOURS`
  - Consider refresh tokens for long-lived sessions
  - Implement token revocation for critical operations

### Password Security

- **Hashing**: Uses Argon2 (winner of Password Hashing Competition)
  - Resistant to GPU/ASIC attacks
  - Memory-hard algorithm
  - Automatic salt generation

- **Password Requirements**:
  - Minimum 8 characters (enforced in validation)
  - Consider adding complexity requirements
  - Implement password strength meter on frontend
  - Consider password breach checking

## Database Security

### SQL Injection Prevention

- **Parameterized Queries**: All queries use SQLx parameterization
  ```rust
  // ✅ Safe - parameterized
  sqlx::query("SELECT * FROM users WHERE email = $1")
      .bind(email)
  
  // ❌ Unsafe - concatenation
  // format!("SELECT * FROM users WHERE email = '{}'", email)
  ```

### Connection Security

- Use SSL/TLS for database connections in production
- Implement connection pooling limits
- Use read replicas for read-heavy operations
- Implement database credentials rotation

### Data Protection

- Never log sensitive data (passwords, tokens, PII)
- Implement data encryption at rest
- Use database-level encryption for sensitive fields
- Regular database backups

## Input Validation

### Request Validation

- All inputs validated using `validator` crate
- Email format validation
- Length constraints on strings
- Numeric range validation

### Example:
```rust
#[derive(Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}
```

## CORS Configuration

### Development vs Production

```toml
# Development - permissive
allowed_origins = ["http://localhost:3000"]

# Production - strict
allowed_origins = ["https://yourdomain.com"]
```

### Best Practices

- Never use `*` in production
- Whitelist specific origins
- Configure allowed methods
- Set appropriate headers

## Error Handling

### Information Disclosure

- Generic error messages to clients
- Detailed logging server-side
- No stack traces in production responses

```rust
// ✅ Good - generic message
AppError::Database(_) => (
    StatusCode::INTERNAL_SERVER_ERROR,
    "Database error occurred"
)

// ❌ Bad - exposes details
// e.to_string() // Don't send to client
```

## Environment Variables

### Sensitive Data

Never commit:
- `APP_JWT__SECRET`
- `APP_DATABASE__URL` with credentials
- API keys
- Third-party secrets

Use:
- Environment variables
- Secrets management systems (AWS Secrets Manager, HashiCorp Vault)
- `.env` files (in `.gitignore`)

## Network Security

### HTTPS/TLS

Production requirements:
- Use reverse proxy (nginx, Caddy)
- Enable HTTPS/TLS
- Use valid SSL certificates (Let's Encrypt)
- Enforce HTTPS redirects

### Example nginx config:
```nginx
server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;
    
    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;
    
    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## Rate Limiting

### Future Implementation

Consider adding:
- Per-IP rate limiting
- Per-user rate limiting
- Different limits for different endpoints
- Token bucket or sliding window algorithms

### Libraries to consider:
- `tower-governor`
- `tower-http` rate limiting
- Redis-based distributed rate limiting

## Security Headers

Add to production setup:

```rust
// In middleware
response.headers_mut().insert(
    "X-Content-Type-Options",
    "nosniff".parse().unwrap(),
);
response.headers_mut().insert(
    "X-Frame-Options",
    "DENY".parse().unwrap(),
);
response.headers_mut().insert(
    "X-XSS-Protection",
    "1; mode=block".parse().unwrap(),
);
```

## Dependency Security

### Regular Updates

```bash
# Check for security advisories
cargo audit

# Update dependencies
cargo update

# Check for outdated deps
cargo outdated
```

### CI/CD Integration

Included in `.github/workflows/ci.yml`:
- Automated security audits
- Dependency vulnerability scanning
- Regular dependency updates

## Logging & Monitoring

### What to Log

- Authentication attempts (success/failure)
- Authorization failures
- Unusual access patterns
- Error rates
- Performance metrics

### What NOT to Log

- Passwords (even hashed)
- JWT tokens
- Personal identification information
- Credit card numbers
- Any sensitive user data

### Log Levels

```bash
# Development
RUST_LOG=debug

# Production
RUST_LOG=info,rusty_saas=info
```

## API Security

### Protected Routes

Routes requiring authentication:
- `/api/users/me`
- `/api/users` (GET - list)
- `/api/users/:id` (GET, PUT, DELETE)

### Public Routes

- `/health`, `/ready`, `/live`
- `/api/users` (POST - registration)
- `/api/auth/login`

### Authorization Rules

- Users can only update/delete their own data
- Implement role-based access control (RBAC) for admin features
- Validate user ownership before operations

## Container Security

### Docker Best Practices

Our Dockerfile implements:
- Multi-stage builds (smaller attack surface)
- Non-root user execution
- Minimal base image (Debian slim)
- No unnecessary packages
- Security updates in base image

### Additional Recommendations

- Scan images for vulnerabilities: `docker scan`
- Use specific image tags, not `latest`
- Implement image signing
- Regular base image updates

## Compliance

### GDPR Considerations

- User data export functionality
- Data deletion (right to be forgotten)
- Consent management
- Data processing agreements

### Data Retention

- Define retention policies
- Implement automated cleanup
- Log retention policies
- Backup retention policies

## Incident Response

### Preparation

1. Define security incident procedures
2. Set up alerting for suspicious activity
3. Maintain audit logs
4. Regular backup testing
5. Disaster recovery plan

### Response Plan

1. Detect and analyze
2. Contain the incident
3. Eradicate the threat
4. Recover systems
5. Post-incident review
6. Update security measures

## Security Checklist

### Development

- [ ] Use parameterized queries
- [ ] Validate all inputs
- [ ] Hash passwords with Argon2
- [ ] Use HTTPS in production
- [ ] Set security headers
- [ ] Implement rate limiting
- [ ] Log security events
- [ ] Regular dependency updates
- [ ] Code reviews for security
- [ ] Security testing

### Deployment

- [ ] Change default secrets
- [ ] Use environment variables
- [ ] Enable database SSL
- [ ] Configure firewall rules
- [ ] Set up monitoring
- [ ] Implement backups
- [ ] Use non-root containers
- [ ] Scan for vulnerabilities
- [ ] Configure CORS properly
- [ ] Set resource limits

### Operations

- [ ] Monitor logs
- [ ] Review audit trails
- [ ] Rotate secrets regularly
- [ ] Update dependencies
- [ ] Patch vulnerabilities
- [ ] Test disaster recovery
- [ ] Security training
- [ ] Incident response drills

## Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)

## Reporting Security Issues

If you discover a security vulnerability:

1. **Do NOT** open a public issue
2. Email security concerns privately
3. Include detailed reproduction steps
4. Allow time for patching before disclosure

---

**Security is a continuous process, not a one-time event. Stay vigilant!**
