# OpenBank OAuth2 Auth Plane - Postman Collection

This Postman collection provides comprehensive testing for the OpenBank OAuth2 Client Credentials flow implementation.

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

## 📋 Test Scenarios

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

### 5. Validate Access Token ✅
- **Purpose**: Test token validation and get client info
- **Method**: `GET /oauth/oauth/me`
- **Requires**: Valid Bearer token in Authorization header

### 6. Error Scenarios 🔧
- **Invalid Token**: Test with malformed token
- **Missing Token**: Test without Authorization header
- **Invalid Credentials**: Test with wrong client credentials

## 🔄 Complete OAuth2 Flow

### Sequential Execution (Recommended)
Run requests in this order for complete end-to-end testing:

1. **Health Check** → Verify server
2. **Register Developer** → Create developer account
3. **Create Project** → Generate client credentials
4. **Get OAuth2 Access Token** → Obtain access token
5. **Validate Access Token** → Test token usage

### Collection Runner
- Use Postman's "Collection Runner" feature
- All requests will execute automatically in sequence
- Variables are passed between requests automatically

## 🔧 Configuration

### Environment Variables
- `base_url`: Server URL (default: `http://127.0.0.1:8080`)
- `environment`: API environment (default: `development`)

### Collection Variables (Auto-managed)
- `developer_id`: UUID of registered developer
- `project_id`: UUID of created project  
- `client_key`: OAuth2 client key (ck_...)
- `client_secret`: OAuth2 client secret (cs_...)
- `access_token`: JWT access token

## 📊 Test Validation

Each request includes comprehensive test scripts that validate:

### ✅ HTTP Status Codes
- Success responses (200, 201)
- Error responses (401, 422, 500)

### ✅ Response Structure
- Required fields presence
- Data type validation
- Business logic validation

### ✅ Data Flow
- Variables extracted from responses
- Cross-request data consistency
- OAuth2 flow integrity

## 🐛 Troubleshooting

### Common Issues

**1. Server Not Running**
```
Error: getaddrinfo ENOTFOUND 127.0.0.1
```
- **Solution**: Start OpenBank server with `cargo run`

**2. Database Connection Failed**
```
Response: {"error":"database_error","message":"..."}
```
- **Solution**: Ensure PostgreSQL container is running on port 5433

**3. Invalid Client Credentials**
```
Status: 401 Unauthorized
```
- **Solution**: Run "Create Project" first to generate valid credentials

**4. Token Expired**
```
{"error":"invalid_token","message":"Token expired"}
```
- **Solution**: Run "Get OAuth2 Access Token" to get fresh token

### Debug Tips

1. **Check Console Logs**: View Postman console for detailed request/response logs
2. **Variable Values**: Check collection variables tab for current values
3. **Server Logs**: Monitor terminal where `cargo run` is executed
4. **Database State**: Use database client to inspect data

## 🔐 Security Notes

### Client Credentials Format
- **Client Key**: `ck_` prefix (used for project identification)
- **Client Secret**: `cs_` prefix (used for authentication)
- **Full Client ID**: Returned as `ck_xxx:cs_yyy` format

### Token Security
- **JWT Tokens**: Include expiration and scope information
- **Bearer Authentication**: Standard OAuth2 token usage
- **Secure Storage**: Tokens stored temporarily in collection variables

## 📈 Performance Testing

### Response Time Expectations
- **Health Check**: < 50ms
- **Developer Registration**: < 200ms  
- **Project Creation**: < 300ms
- **Token Generation**: < 100ms
- **Token Validation**: < 50ms

### Load Testing
For load testing, use Postman's performance testing features or export to Newman for automated testing.

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