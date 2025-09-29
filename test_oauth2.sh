#!/bin/bash

# OAuth2 Auth Plane Test Script
echo "🚀 Testing OAuth2 Auth Plane Implementation"
echo "============================================="

# Server endpoint
BASE_URL="http://localhost:8080"

echo "1. Testing server health..."
curl -s "$BASE_URL/health" || echo "Health check failed - server might not be running"

echo -e "\n2. Testing developer registration..."
REGISTER_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/developers" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john.doe@example.com", 
    "password": "securepassword123"
  }')

echo "Register Response: $REGISTER_RESPONSE"

echo -e "\n3. Testing project creation..."
PROJECT_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/projects" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Test API",
    "description": "Test API project for OAuth2",
    "environment": "development",
    "redirect_uris": ["http://localhost:3000/callback"]
  }')

echo "Project Response: $PROJECT_RESPONSE"

# Extract client_id and client_secret from project response if available
CLIENT_ID=$(echo "$PROJECT_RESPONSE" | grep -o '"client_id":"[^"]*"' | cut -d'"' -f4)
CLIENT_SECRET=$(echo "$PROJECT_RESPONSE" | grep -o '"client_secret":"[^"]*"' | cut -d'"' -f4)

if [ -n "$CLIENT_ID" ] && [ -n "$CLIENT_SECRET" ]; then
    echo -e "\n4. Testing OAuth2 client credentials flow..."
    TOKEN_RESPONSE=$(curl -s -X POST "$BASE_URL/auth/token" \
      -H "Content-Type: application/json" \
      -d "{
        \"grant_type\": \"client_credentials\",
        \"client_id\": \"$CLIENT_ID\",
        \"client_secret\": \"$CLIENT_SECRET\",
        \"scopes\": [\"read\", \"write\"]
      }")
    
    echo "Token Response: $TOKEN_RESPONSE"
    
    # Extract access token
    ACCESS_TOKEN=$(echo "$TOKEN_RESPONSE" | grep -o '"access_token":"[^"]*"' | cut -d'"' -f4)
    
    if [ -n "$ACCESS_TOKEN" ]; then
        echo -e "\n5. Testing token validation..."
        ME_RESPONSE=$(curl -s -X GET "$BASE_URL/auth/me" \
          -H "Authorization: Bearer $ACCESS_TOKEN")
        
        echo "Me Response: $ME_RESPONSE"
    else
        echo "❌ No access token received"
    fi
else
    echo "❌ No client credentials received from project creation"
fi

echo -e "\n✅ OAuth2 Auth Plane Test Complete!"
echo "============================================="