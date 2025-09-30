# openBank

A Rust-based, fast, secure, open-source modular monolith fintech backend. Supports core financial services including authorization, balance checks, transactions, identity verification, income verification, payments, and virtual accounts. Designed for scalability, maintainability, and high-performance.

## Authentication Module

OpenBank provides enterprise-grade OAuth2 authentication infrastructure designed for Banking-as-a-Service platforms. The authentication system implements industry-standard security protocols with comprehensive audit logging, rate limiting, and role-based access control.

### Core Capabilities

**OAuth2 Implementation**
- Client Credentials Grant flow for server-to-server authentication
- JWT-based access tokens with configurable expiration
- Token refresh mechanism with blacklist validation
- Scope-based authorization for granular permission control

**Enterprise Security Features**
- Advanced rate limiting with configurable thresholds
- Comprehensive audit logging for compliance requirements
- Account security monitoring and lockout protection
- Security headers for XSS, CSRF, and clickjacking protection

**Developer Management**
- Developer registration and profile management
- Project-based credential isolation
- Multi-environment support (development, staging, production)
- Client credential generation with secure storage

### API Endpoints

```
POST /auth/developers              # Register developer account
POST /auth/login                   # Developer login (dashboard access)
POST /auth/developers/{id}/projects # Create OAuth2 project
POST /auth/token                   # Generate access token
POST /auth/token/refresh           # Refresh existing token
GET  /auth/me                      # Validate token and get claims
```

### Usage Example

```bash
# 1. Register Developer
curl -X POST http://localhost:8080/auth/developers \
  -H "Content-Type: application/json" \
  -d '{"name":"API Developer","email":"dev@company.com","password":"secure123"}'

# 2. Login to Dashboard
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"dev@company.com","password":"secure123"}'

# 3. Create Project
curl -X POST http://localhost:8080/auth/developers/{developer_id}/projects \
  -H "Content-Type: application/json" \
  -d '{"name":"Production API","environment":"production","scopes":["identity","payments"]}'

# 4. Get Access Token
curl -X POST http://localhost:8080/auth/token \
  -H "Content-Type: application/json" \
  -d '{"grant_type":"client_credentials","client_id":"ck_xxx","client_secret":"cs_yyy","scope":"identity payments"}'

# 5. Use Token
curl -X GET http://localhost:8080/auth/me \
  -H "Authorization: Bearer {access_token}"
```

### Security Standards

The authentication module adheres to enterprise security standards including:
- OAuth2 RFC 6749 specification compliance
- JWT RFC 7519 implementation
- OWASP security guidelines
- SOC2 audit trail requirements
- GDPR privacy compliance

