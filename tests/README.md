# Integration Tests

This directory contains integration tests for the OpenBank Identity system.

## Test Structure

### Integration Tests (`integration/`)
- **`identity_tests.rs`** - API endpoint integration tests
- **`ml_service_tests.rs`** - ML pipeline integration tests  
- **`fraud_detection_tests.rs`** - Fraud detection workflow tests

### Fixtures (`fixtures/`)
- **`test_images/`** - Sample images for testing
- **`test_models/`** - Lightweight test models
- **`test_data.sql`** - Test database fixtures

## Running Tests

### Prerequisites
```bash
# Set up test database
export TEST_DATABASE_URL="postgres://localhost/openbank_test"
createdb openbank_test

# Set up test MongoDB
export TEST_MONGODB_URL="mongodb://localhost:27017/openbank_test"
```

### Run All Tests
```bash
cargo test
```

### Run Integration Tests Only
```bash
cargo test --test integration
```

### Run Specific Test Module
```bash
cargo test identity_integration_tests
```

### Run with Logging
```bash
RUST_LOG=debug cargo test -- --nocapture
```

## Test Categories

### 1. API Integration Tests
- ✅ Health check endpoints
- ✅ Biometric verification flow
- ✅ Face matching pipeline
- ✅ Liveness detection
- ✅ Error handling
- ✅ Request validation

### 2. ML Service Tests
- 🔄 Model loading and initialization
- 🔄 Image preprocessing
- 🔄 Face detection accuracy
- 🔄 Embedding extraction
- 🔄 Quality assessment

### 3. Database Integration Tests
- ✅ Vector similarity search
- ✅ CRUD operations
- ✅ Fraud alert creation
- ✅ Migration compatibility

### 4. Performance Tests
- 🔄 Response time benchmarks
- 🔄 Throughput testing
- 🔄 Memory usage profiling
- 🔄 Concurrent request handling

### 5. Security Tests
- 🔄 Input validation
- 🔄 Authentication checks
- 🔄 Rate limiting
- 🔄 Fraud detection accuracy

## Test Data

### Sample Images
```
fixtures/test_images/
├── valid_face.jpg          # Clear, high-quality face
├── low_quality.jpg         # Poor quality/blurry
├── no_face.jpg            # Image without faces
├── multiple_faces.jpg     # Multiple people
├── spoof_photo.jpg        # Photo of a photo
└── live_selfie.jpg        # Genuine selfie
```

### Database Fixtures
- Pre-populated user records
- Sample face embeddings
- Fraud alert scenarios
- Verification history

## Continuous Integration

### GitHub Actions
```yaml
name: Identity System Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:14
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: openbank_test
      mongodb:
        image: mongo:5
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: cargo test
```

## Mock Services

For testing without real ML models:

### Mock ML Service
- Returns deterministic results
- Simulates processing delays
- Tests error conditions

### Mock External APIs
- Simulated document verification
- Mock fraud detection services
- Test rate limiting scenarios

## Benchmarking

### Performance Targets
- **API Response Time**: < 200ms (95th percentile)
- **Face Embedding**: < 100ms per image
- **Database Queries**: < 50ms per query
- **Throughput**: > 100 verifications/minute

### Memory Usage
- **Total Service**: < 1GB RAM
- **Per Request**: < 50MB peak
- **Model Loading**: < 500MB total

## Debugging Tests

### Enable Detailed Logging
```bash
RUST_LOG=openbank::identity=trace cargo test -- --nocapture
```

### Debug Specific Test
```bash
cargo test test_biometric_verification_invalid_image -- --nocapture
```

### Profile Performance
```bash
cargo test --release -- --nocapture --test-threads=1
```

## Contributing

### Adding New Tests
1. Create test in appropriate module
2. Add test data to fixtures/ if needed
3. Update this README with test description
4. Ensure tests pass in CI

### Test Naming Convention
- `test_[feature]_[scenario]` - e.g., `test_face_match_with_valid_image`
- `bench_[operation]` - for benchmark tests
- `integration_[workflow]` - for end-to-end tests

### Test Coverage
Run with coverage reporting:
```bash
cargo tarpaulin --out Html --output-dir coverage/
```

Target: > 80% code coverage for identity module