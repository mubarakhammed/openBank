# OpenBank API Collection - Postman Testing

This directory contains the co## OAuth2 Scopes

Available scopes for your projects:
- `identity` - Identity verification and KYC services
- `income` - Income verification services  
- `payments` - Payment processing and transfers
- `transactions` - Transaction history and management
- `user-data` - User profile and account data
- `virtual-accounts` - Virtual account creation and management

## 📋 Basic OAuth2 Test Scenarios

### 1. Health Check ✅
- **Purpose**: Verify server is running
- **Method**: `GET /health`
- **Expected**: Status 200, healthy response Postman collection for testing the OpenBank Banking-as-a-Service platform. Currently includes the **Auth Module** with comprehensive OAuth2 endpoints and security testing. Additional modules will be added as development progresses.

## 🚀 Quick Start

### Prerequisites
1. **OpenBank Server Running**: Ensure the OpenBank server is running on `http://127.0.0.1:8080`
2. **Database Setup**: PostgreSQL and MongoDB containers should be running
3. **Postman**: Install Postman desktop app or use Postman web

### Import Instructions

1. **Import Collection**:
   - Open Postman
   - Click "Import" → "Choose Files"
   - Select `OpenBank_OAuth2_AuthPlane.postman_collection.json`

2. **Import Environment**:
   - Click "Import" → "Choose Files"
   - Select `OpenBank_Development.postman_environment.json`
   - Set as active environment in top-right dropdown

## Files Overview

- `OpenBank_API.postman_collection.json` - Complete OpenBank API collection with modular structure
- `OpenBank_Development.postman_environment.json` - Development environment variables

## Current Modules

### 🔐 Auth Module (Complete)
- **Auth Module**: OAuth2 endpoints (register, create project, token generation, token refresh, validation)
- **Auth Tests**: Security validation tests (rate limiting, audit logging, security headers)
- **Security Tests**: Advanced auth security testing (RBAC, compliance, performance)

### 🚀 Future Modules (Planned)
- **Account Module**: Account management, KYC, user profiles
- **Transaction Module**: Payment processing, transfers, transaction history
- **Card Module**: Card issuance, management, and controls
- **Compliance Module**: AML, fraud detection, regulatory reporting

## �️ Enterprise Security Test Coverage

### Rate Limiting Tests
- **Burst Rate Limiting**: Tests immediate request bursts within limits
- **Sustained Rate Limiting**: Validates long-term request patterns
- **Rate Limit Exceeded**: Confirms proper 429 responses when limits are hit
- **Rate Limit Headers**: Verifies proper rate limit headers in responses
- **Per-User Rate Limiting**: Tests individual user rate limit enforcement

### Authentication & Authorization
- **OAuth2 Flow**: Complete authorization code flow testing
- **Token Validation**: Access token verification and refresh
- **RBAC Testing**: Role-based access control validation
- **Permission Hierarchy**: SuperAdmin > Admin > Developer > User testing
- **Invalid Token Handling**: Proper error responses for invalid/expired tokens

### Audit Logging
- **Audit Event Creation**: Validates audit log generation for sensitive operations
- **Compliance Fields**: Ensures SOC2/PCI DSS/GDPR required fields are captured
- **Audit Query**: Tests audit log retrieval with proper access controls
- **Event Classification**: Validates different audit event types (AUTH, ACCOUNT, ADMIN)

### Security Headers
- **CORS Headers**: Cross-Origin Resource Sharing validation
- **Security Headers**: Content Security Policy, X-Frame-Options, etc.
- **HTTPS Enforcement**: Secure transport validation
- **Content Type Validation**: Proper MIME type handling

### Account Security
- **Account Lockout**: Tests account lockout after failed attempts
- **Password Policy**: Validates password complexity requirements
- **Security Events**: Account security event logging
- **MFA Support**: Multi-factor authentication flow testing

## �📋 Basic OAuth2 Test Scenarios

### 1. Health Check ✅
- **Purpose**: Verify server is running
- **Method**: `GET /health`
- **Expected**: Status 200, healthy response

### 2. Register Developer ✅
- **Purpose**: Create a new developer account
- **Method**: `POST /oauth/oauth/developers`
- **Auto-generates**: Unique developer name and email
- **Saves**: `developer_id` for subsequent requests

### 3. Create Project ✅
- **Purpose**: Create OAuth2 project with client credentials
- **Method**: `POST /oauth/oauth/developers/{developer_id}/projects`
- **Extracts & Saves**: 
  - `client_key` (ck_...)
  - `client_secret` (cs_...)
  - `project_id`

### 4. Get OAuth2 Access Token ✅
- **Purpose**: Exchange client credentials for access token
- **Method**: `POST /oauth/oauth/token`
- **Grant Type**: `client_credentials`
- **Saves**: `access_token` for API calls

### 5. Refresh Access Token ✅
- **Purpose**: Refresh an existing access token using JWT ID
- **Method**: `POST /auth/token/refresh`
- **Requires**: Client credentials and JWT ID (jti) from current token

### 6. Validate Access Token ✅
- **Purpose**: Test token validation and get client info
- **Method**: `GET /auth/me`
- **Requires**: Valid Bearer token in Authorization header

## Environment Variables

### Required for Testing
- `base_url`: API base URL (default: http://127.0.0.1:8080)
- `access_token`: OAuth2 access token (populated during auth flow)
- `admin_token`: Admin-level access token for privileged operations
- `user_id`: Current authenticated user ID

### Rate Limiting Configuration
- `rate_limit_window`: Time window in seconds (default: 60)
- `max_requests_per_minute`: Maximum requests per window (default: 100)

### Test Data
- `test_client_id`: OAuth2 client ID for testing
- `test_user_email`: Test user email address
- `audit_log_id`: Audit log entry ID for query testing

## Usage Instructions

### 1. Import Collections
1. Open Postman
2. Import `OpenBank_API.postman_collection.json`
3. Import `OpenBank_Development.postman_environment.json`
4. Select the "OpenBank Development" environment

### 2. Running Enterprise Security Tests
1. **Start with Authentication**: Run the "OAuth2 Authorization" folder first
2. **Rate Limiting Tests**: Execute rate limiting tests to validate DDoS protection
3. **RBAC Tests**: Test role-based access with different user roles
4. **Audit Tests**: Validate compliance logging functionality
5. **Security Headers**: Verify all security headers are properly set

### 3. Test Sequence Recommendations

#### Basic OAuth2 Flow Validation
```
1. Auth Module → Register Developer
2. Auth Module → Create Project
3. Auth Module → Get OAuth2 Access Token
4. Auth Module → Refresh Access Token
5. Auth Module → Validate Access Token (Get Me)
```

#### Rate Limiting Validation
```
1. Rate Limiting → Test Burst Rate Limit (should pass)
2. Rate Limiting → Test Sustained Rate Limit (should pass)
3. Rate Limiting → Trigger Rate Limit (should return 429)
4. Rate Limiting → Verify Rate Limit Headers
```

#### RBAC Validation
```
1. RBAC Tests → Test User Role Access
2. RBAC Tests → Test Admin Role Access
3. RBAC Tests → Test Developer Role Access
4. RBAC Tests → Test Permission Denied
```

#### Compliance Testing
```
1. Audit Logging → Create Audit Event
2. Audit Logging → Query Audit Logs
3. Audit Logging → Verify Compliance Fields
4. Account Security → Test Account Lockout
```

## Performance Testing

### Load Testing with Newman
```bash
# Install Newman CLI
npm install -g newman

# Run basic performance test
newman run OpenBank_API.postman_collection.json \
  -e OpenBank_Development.postman_environment.json \
  --iteration-count 100 \
  --delay-request 100

# Run rate limiting stress test
newman run OpenBank_API.postman_collection.json \
  -e OpenBank_Development.postman_environment.json \
  --folder "Rate Limiting Tests" \
  --iteration-count 200 \
  --delay-request 50
```

### Scaling Test Scenarios
- **10K Users**: 100 iterations with 100ms delay
- **100K Users**: 1000 iterations with 50ms delay
- **1M Users**: 10000 iterations with 10ms delay

## Monitoring & Validation

### Key Metrics to Monitor
- **Response Times**: < 200ms for auth, < 500ms for complex operations
- **Rate Limit Accuracy**: Proper 429 responses when limits exceeded
- **Audit Log Coverage**: 100% coverage for sensitive operations
- **Security Header Compliance**: All required headers present
- **Error Handling**: Proper error codes and messages

### Success Criteria
- ✅ All authentication flows complete successfully
- ✅ Rate limiting prevents abuse while allowing legitimate traffic
- ✅ RBAC properly restricts access based on user roles
- ✅ Audit logs capture all required compliance information
- ✅ Security headers protect against common vulnerabilities

## Troubleshooting

### Common Issues
1. **401 Unauthorized**: Check if access token is valid and not expired
2. **429 Rate Limited**: Wait for rate limit window to reset
3. **403 Forbidden**: Verify user has required role/permissions
4. **Missing Audit Logs**: Check MongoDB connection and audit service status

### Debug Tips
- Enable Postman Console to see detailed request/response logs
- Check environment variables are properly set
- Verify server is running and accessible at `base_url`
- Monitor server logs for detailed error information

## Enterprise Compliance

This test suite validates compliance with:
- **SOC 2 Type II**: Access controls, monitoring, and audit logging
- **PCI DSS**: Secure data handling and access controls
- **GDPR**: Data processing audit trails and access controls
- **NIST Cybersecurity Framework**: Identity and access management

## Scaling Considerations

The test suite is designed to validate the distributed architecture changes for 10M+ users:
- Rate limiting tests verify Redis-based distributed rate limiting
- RBAC tests validate database-backed role management with Redis caching
- Audit tests confirm MongoDB horizontal scaling capabilities
- Performance tests identify bottlenecks before production deployment

## 🔄 CI/CD Integration

### Newman (Command Line)
```bash
# Install Newman
npm install -g newman

# Run collection
newman run OpenBank_OAuth2_AuthPlane.postman_collection.json \\
  -e OpenBank_Development.postman_environment.json \\
  --reporters cli,json \\
  --reporter-json-export results.json
```

### GitHub Actions
Create `.github/workflows/api-test.yml` for automated testing on code changes.

## 📝 Example Responses

### Successful Developer Registration
```json
{
    "id": "9de839a1-4a06-4211-9ab2-e86d4def5e3c",
    "name": "Test Developer 1727632051234",
    "email": "developer1727632051234@example.com",
    "created_at": "2025-09-29T16:07:31.836304Z"
}
```

### Successful Project Creation
```json
{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Test API Project",
    "description": "A test project for OAuth2 API access",
    "environment": "development",
    "client_id": "ck_1234567890abcdef:cs_fedcba0987654321",
    "redirect_uris": ["http://localhost:3000/callback"],
    "scopes": ["read", "write", "admin"],
    "is_active": true,
    "created_at": "2025-09-29T16:07:45.123456Z"
}
```

### Successful Token Response
```json
{
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "scope": "read write"
}
```

## 🎯 Next Steps

After successful testing:

1. **API Integration**: Use generated tokens to access protected endpoints
2. **Production Setup**: Update environment for production server
3. **Monitoring**: Set up API monitoring and alerting
4. **Documentation**: Generate API documentation from collection

---

**Happy Testing! 🚀**

For issues or questions, check the server logs and database connections first.