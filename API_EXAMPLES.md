# API Examples

This document provides practical examples of using the Rusty SaaS API for LexiFlow Premium.

## Base URL

```
http://localhost:8080
```

## Authentication

All protected endpoints require a JWT token in the `Authorization` header:

```
Authorization: Bearer <your-jwt-token>
```

## Examples

### 1. Create a User

**Endpoint:** `POST /api/users`

**Request:**
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "username": "johndoe",
    "password": "SecurePass123!"
  }'
```

**Response:** (201 Created)
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "john.doe@example.com",
  "username": "johndoe",
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### 2. Login

**Endpoint:** `POST /api/auth/login`

**Request:**
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "password": "SecurePass123!"
  }'
```

**Response:**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "john.doe@example.com",
    "username": "johndoe",
    "is_active": true,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z"
  }
}
```

### 3. Create a Case

**Endpoint:** `POST /api/cases`

**Request:**
```bash
curl -X POST http://localhost:8080/api/cases \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{
    "title": "Smith v. Johnson Industries",
    "client": "John Smith",
    "matter_type": "Litigation",
    "filing_date": "2024-01-15T00:00:00Z",
    "description": "Product liability case involving defective machinery",
    "value": 500000.00,
    "jurisdiction": "Federal - Northern District of California",
    "court": "U.S. District Court",
    "judge": "Hon. Jane Anderson"
  }'
```

**Response:**
```json
{
  "id": "456e7890-e89b-12d3-a456-426614174000",
  "title": "Smith v. Johnson Industries",
  "client": "John Smith",
  "matter_type": "Litigation",
  "status": "Pre-Filing",
  "filing_date": "2024-01-15T00:00:00Z",
  "description": "Product liability case involving defective machinery",
  "value": 500000.00,
  "jurisdiction": "Federal - Northern District of California",
  "court": "U.S. District Court",
  "judge": "Hon. Jane Anderson",
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### 4. List Cases with Filtering

**Endpoint:** `GET /api/cases`

**Request:**
```bash
# List all active cases
curl -X GET "http://localhost:8080/api/cases?status=Discovery&page=1&per_page=20" \
  -H "Authorization: Bearer <token>"

# Search for cases
curl -X GET "http://localhost:8080/api/cases?search=Smith" \
  -H "Authorization: Bearer <token>"
```

### 5. Get Case with Parties

**Endpoint:** `GET /api/cases/{id}`

**Request:**
```bash
curl -X GET http://localhost:8080/api/cases/456e7890-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer <token>"
```

**Response:**
```json
{
  "case": {
    "id": "456e7890-e89b-12d3-a456-426614174000",
    "title": "Smith v. Johnson Industries",
    "status": "Discovery",
    ...
  },
  "parties": [
    {
      "id": "789e1234-e89b-12d3-a456-426614174000",
      "case_id": "456e7890-e89b-12d3-a456-426614174000",
      "name": "John Smith",
      "role": "Plaintiff",
      "type": "Individual",
      "counsel": "Anderson & Partners LLP"
    },
    {
      "id": "012e3456-e89b-12d3-a456-426614174000",
      "case_id": "456e7890-e89b-12d3-a456-426614174000",
      "name": "Johnson Industries Inc.",
      "role": "Defendant",
      "type": "Corporation",
      "counsel": "Smith Defense Group"
    }
  ]
}
```

### 6. Update Case Status

**Endpoint:** `PUT /api/cases/{id}`

**Request:**
```bash
curl -X PUT http://localhost:8080/api/cases/456e7890-e89b-12d3-a456-426614174000 \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{
    "status": "Trial",
    "description": "Moving to trial phase after discovery completion"
  }'
```

### 7. Get Dashboard Statistics

**Endpoint:** `GET /api/dashboard/stats`

**Request:**
```bash
curl -X GET http://localhost:8080/api/dashboard/stats \
  -H "Authorization: Bearer <token>"
```

**Response:**
```json
{
  "active_cases": 45,
  "pending_motions": 12,
  "billable_hours": 1240.5,
  "high_risks": 3,
  "total_revenue": 458000.00,
  "open_tasks": 89
}
```

### 8. Get Dashboard Chart Data

**Endpoint:** `GET /api/dashboard/chart-data`

**Request:**
```bash
curl -X GET http://localhost:8080/api/dashboard/chart-data \
  -H "Authorization: Bearer <token>"
```

**Response:**
```json
[
  { "name": "Discovery", "count": 18 },
  { "name": "Pre-Filing", "count": 12 },
  { "name": "Trial", "count": 8 },
  { "name": "Appeal", "count": 5 },
  { "name": "Settled", "count": 2 }
]
```

### 9. List Workflow Tasks

**Endpoint:** `GET /api/tasks`

**Request:**
```bash
# Get all tasks for a specific case
curl -X GET "http://localhost:8080/api/tasks?case_id=456e7890-e89b-12d3-a456-426614174000" \
  -H "Authorization: Bearer <token>"

# Get high priority pending tasks
curl -X GET "http://localhost:8080/api/tasks?status=Pending" \
  -H "Authorization: Bearer <token>"
```

### 10. Get Recent Alerts

**Endpoint:** `GET /api/dashboard/alerts`

**Request:**
```bash
curl -X GET http://localhost:8080/api/dashboard/alerts \
  -H "Authorization: Bearer <token>"
```

**Response:**
```json
[
  {
    "id": "alert-123",
    "message": "High Priority Task: File motion to compel discovery",
    "detail": "Opposition deadline approaching in 3 days",
    "time": "Today",
    "case_id": null
  }
]
```

## Error Responses

### 400 Bad Request
```json
{
  "error": "Validation error: Email is required"
}
```

### 401 Unauthorized
```json
{
  "error": "Invalid token"
}
```

### 404 Not Found
```json
{
  "error": "Case not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Database error occurred"
}
```

**Request:**
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "password": "SecurePass123!"
  }'
```

**Response:** (200 OK)
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "john.doe@example.com",
    "username": "johndoe",
    "is_active": true,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z"
  }
}
```

### 3. Get Current User

**Endpoint:** `GET /api/users/me`

**Request:**
```bash
curl -X GET http://localhost:8080/api/users/me \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

**Response:** (200 OK)
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "john.doe@example.com",
  "username": "johndoe",
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### 4. List All Users

**Endpoint:** `GET /api/users`

**Request:**
```bash
curl -X GET http://localhost:8080/api/users \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

**Response:** (200 OK)
```json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "email": "john.doe@example.com",
    "username": "johndoe",
    "is_active": true,
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z"
  },
  {
    "id": "234e5678-e89b-12d3-a456-426614174001",
    "email": "jane.smith@example.com",
    "username": "janesmith",
    "is_active": true,
    "created_at": "2024-01-02T12:00:00Z",
    "updated_at": "2024-01-02T12:00:00Z"
  }
]
```

### 5. Get User by ID

**Endpoint:** `GET /api/users/{id}`

**Request:**
```bash
curl -X GET http://localhost:8080/api/users/123e4567-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

**Response:** (200 OK)
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "john.doe@example.com",
  "username": "johndoe",
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:00:00Z"
}
```

### 6. Update User

**Endpoint:** `PUT /api/users/{id}`

**Request:**
```bash
curl -X PUT http://localhost:8080/api/users/123e4567-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.updated@example.com",
    "username": "johndoe_updated"
  }'
```

**Response:** (200 OK)
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "john.updated@example.com",
  "username": "johndoe_updated",
  "is_active": true,
  "created_at": "2024-01-01T12:00:00Z",
  "updated_at": "2024-01-01T12:30:00Z"
}
```

### 7. Delete User

**Endpoint:** `DELETE /api/users/{id}`

**Request:**
```bash
curl -X DELETE http://localhost:8080/api/users/123e4567-e89b-12d3-a456-426614174000 \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

**Response:** (204 No Content)

### 8. Health Check

**Endpoint:** `GET /health`

**Request:**
```bash
curl -X GET http://localhost:8080/health
```

**Response:** (200 OK)
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### 9. Readiness Check

**Endpoint:** `GET /ready`

**Request:**
```bash
curl -X GET http://localhost:8080/ready
```

**Response:** (200 OK)
```json
{
  "status": "ready",
  "version": "0.1.0",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### 10. Liveness Check

**Endpoint:** `GET /live`

**Request:**
```bash
curl -X GET http://localhost:8080/live
```

**Response:** (200 OK)
```json
{
  "status": "alive",
  "version": "0.1.0",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

## Error Responses

### 400 Bad Request

```json
{
  "error": "Validation error: Email must be a valid email address"
}
```

### 401 Unauthorized

```json
{
  "error": "Invalid credentials"
}
```

### 403 Forbidden

```json
{
  "error": "You can only update your own profile"
}
```

### 404 Not Found

```json
{
  "error": "User not found"
}
```

### 500 Internal Server Error

```json
{
  "error": "Database error occurred"
}
```

## Using JavaScript/TypeScript

### Fetch API Example

```javascript
// Create user
const createUser = async () => {
  const response = await fetch('http://localhost:8080/api/users', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      email: 'user@example.com',
      username: 'testuser',
      password: 'SecurePass123!'
    })
  });
  return await response.json();
};

// Login
const login = async (email, password) => {
  const response = await fetch('http://localhost:8080/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ email, password })
  });
  const data = await response.json();
  localStorage.setItem('token', data.token);
  return data;
};

// Get current user
const getCurrentUser = async () => {
  const token = localStorage.getItem('token');
  const response = await fetch('http://localhost:8080/api/users/me', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  return await response.json();
};
```

## Using Python

### Requests Library Example

```python
import requests

BASE_URL = "http://localhost:8080"

# Create user
def create_user(email, username, password):
    response = requests.post(
        f"{BASE_URL}/api/users",
        json={
            "email": email,
            "username": username,
            "password": password
        }
    )
    return response.json()

# Login
def login(email, password):
    response = requests.post(
        f"{BASE_URL}/api/auth/login",
        json={
            "email": email,
            "password": password
        }
    )
    return response.json()

# Get current user
def get_current_user(token):
    response = requests.get(
        f"{BASE_URL}/api/users/me",
        headers={
            "Authorization": f"Bearer {token}"
        }
    )
    return response.json()

# Example usage
if __name__ == "__main__":
    # Create a user
    user = create_user("test@example.com", "testuser", "SecurePass123!")
    print(f"Created user: {user}")
    
    # Login
    auth = login("test@example.com", "SecurePass123!")
    token = auth["token"]
    print(f"Logged in, token: {token[:20]}...")
    
    # Get current user
    current_user = get_current_user(token)
    print(f"Current user: {current_user}")
```

## Rate Limiting (Future Enhancement)

When rate limiting is implemented, responses will include:

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1640000000
```

## Pagination (Future Enhancement)

List endpoints will support pagination:

```
GET /api/users?page=1&per_page=20
```

Response will include pagination metadata:

```json
{
  "data": [...],
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
```
