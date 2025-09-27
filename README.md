# openBank

A Rust-based, fast, secure, open-source modular monolith fintech backend. Supports core financial services including authorization, balance checks, transactions, identity verification, income verification, payments, and virtual accounts. Designed for scalability, maintainability, and high-performance.

## Features

- **Authentication & Authorization**: Secure user registration, login, and JWT-based authentication
- **Balance Management**: Real-time balance tracking with transaction history
- **Transaction Processing**: Support for deposits, withdrawals, transfers, and payments
- **Identity Verification**: KYC/AML compliance with document verification
- **Income Verification**: Employment and income validation services
- **Payment Processing**: Multiple payment methods and external gateway integration
- **Virtual Accounts**: Create and manage virtual accounts for different purposes
- **Audit Trail**: Comprehensive logging and analytics with MongoDB
- **Real-time Processing**: Async/await architecture with Tokio
- **Database Support**: PostgreSQL for transactional data, MongoDB for logs/analytics

## Technology Stack

- **Language**: Rust 2021 Edition
- **Web Framework**: Axum (high-performance async web framework)
- **Database**: PostgreSQL (primary), MongoDB (logs/analytics)
- **Authentication**: JWT tokens with bcrypt password hashing
- **ORM**: SQLx (compile-time checked queries)
- **Async Runtime**: Tokio
- **Serialization**: Serde (JSON)
- **Validation**: Validator crate
- **Logging**: Tracing

## Project Structure

```
openBank/
├── Cargo.toml                 # Project dependencies and metadata
├── .env.example              # Environment variables template
├── README.md                 # Project documentation
├── src/
│   ├── main.rs              # Application entry point
│   ├── core/                # Core infrastructure
│   │   ├── mod.rs           # Module exports
│   │   ├── config.rs        # Configuration management
│   │   ├── database.rs      # Database connections
│   │   └── error.rs         # Error handling
│   ├── shared/              # Shared utilities
│   │   ├── mod.rs           # Module exports
│   │   ├── types.rs         # Common types and aliases
│   │   ├── constants.rs     # Application constants
│   │   └── traits.rs        # Common traits
│   ├── auth/                # Authentication module
│   │   ├── mod.rs           # Module exports and routes
│   │   ├── controller.rs    # HTTP handlers
│   │   ├── service.rs       # Business logic
│   │   ├── repository.rs    # Data access layer
│   │   └── model.rs         # Data models
│   ├── balance/             # Balance management module
│   │   ├── mod.rs
│   │   ├── controller.rs
│   │   ├── service.rs
│   │   ├── repository.rs
│   │   └── model.rs
│   ├── transactions/        # Transaction processing module
│   │   ├── mod.rs
│   │   ├── controller.rs
│   │   ├── service.rs
│   │   ├── repository.rs
│   │   └── model.rs
│   ├── identity/            # Identity verification module
│   │   ├── mod.rs
│   │   ├── controller.rs
│   │   ├── service.rs
│   │   ├── repository.rs
│   │   └── model.rs
│   ├── income/              # Income verification module
│   │   ├── mod.rs
│   │   ├── controller.rs
│   │   ├── service.rs
│   │   ├── repository.rs
│   │   └── model.rs
│   ├── payments/            # Payment processing module
│   │   ├── mod.rs
│   │   ├── controller.rs
│   │   ├── service.rs
│   │   ├── repository.rs
│   │   └── model.rs
│   └── virtual_accounts/    # Virtual accounts module
│       ├── mod.rs
│       ├── controller.rs
│       ├── service.rs
│       ├── repository.rs
│       └── model.rs
└── migrations/              # Database migrations
    ├── 20240101000001_create_users_and_accounts.sql
    ├── 20240101000002_create_transactions.sql
    ├── 20240101000003_create_verifications.sql
    └── 20240101000004_create_payments_and_virtual_accounts.sql
```

## Getting Started

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- PostgreSQL 14+
- MongoDB 6.0+
- Docker (optional, for containerized databases)

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/mubarakhammed/openBank.git
   cd openBank
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your database configurations
   ```

3. **Install dependencies**
   ```bash
   cargo build
   ```

4. **Set up databases**

   **PostgreSQL:**
   ```bash
   # Create database
   createdb openbank
   
   # Run migrations (when implemented)
   # cargo run --bin migrate
   ```

   **MongoDB:**
   ```bash
   # Start MongoDB (if using Docker)
   docker run -d --name mongodb -p 27017:27017 mongo:6.0
   ```

5. **Run the application**
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:8080`

## API Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/refresh` - Refresh JWT token
- `POST /api/v1/auth/logout` - User logout

### Balance Management
- `GET /api/v1/balance` - Get account balance
- `GET /api/v1/balance/history` - Get balance history

### Transactions
- `POST /api/v1/transactions` - Create transaction
- `GET /api/v1/transactions` - List transactions
- `GET /api/v1/transactions/:id` - Get transaction by ID
- `POST /api/v1/transactions/transfer` - Transfer funds

### Identity Verification
- `POST /api/v1/identity/verify` - Initiate verification
- `GET /api/v1/identity/verify/status/:id` - Check verification status
- `POST /api/v1/identity/verify/complete` - Complete verification

### Income Verification
- `POST /api/v1/income/verify` - Initiate income verification
- `GET /api/v1/income/verify/status/:id` - Check verification status
- `GET /api/v1/income/report` - Get income report

### Payments
- `POST /api/v1/payments` - Create payment
- `GET /api/v1/payments` - List payments
- `GET /api/v1/payments/:id` - Get payment by ID
- `POST /api/v1/payments/:id/cancel` - Cancel payment

### Virtual Accounts
- `POST /api/v1/virtual-accounts` - Create virtual account
- `GET /api/v1/virtual-accounts` - List virtual accounts
- `GET /api/v1/virtual-accounts/:id` - Get virtual account by ID
- `POST /api/v1/virtual-accounts/:id/deactivate` - Deactivate account

### Health Check
- `GET /health` - Service health status

## Architecture

### Modular Monolith Design

The application follows a modular monolith architecture with clear separation of concerns:

- **Core Module**: Shared infrastructure (database, config, errors)
- **Shared Module**: Common types, traits, and constants
- **Business Modules**: Each domain has its own module with MVC pattern
  - Controller: HTTP request handling
  - Service: Business logic implementation
  - Repository: Data access layer
  - Model: Data structures and validation

### Database Strategy

- **PostgreSQL**: ACID-compliant transactions, user data, financial records
- **MongoDB**: Event logs, analytics, audit trails, flexible schema data

### Security Features

- JWT-based authentication
- Bcrypt password hashing
- Request validation
- SQL injection prevention (SQLx compile-time checks)
- Rate limiting (configurable)
- CORS protection

## Configuration

All configuration is handled through environment variables. See `.env.example` for available options:

- Database connection strings
- JWT secret and expiration
- Server host and port
- Security settings
- External service URLs

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Development Mode
```bash
# With auto-reload (install cargo-watch)
cargo install cargo-watch
cargo watch -x run
```

## Database Migrations

Database schema is managed through SQL migration files in the `migrations/` directory. The application automatically runs pending migrations on startup.

To create a new migration:
1. Create a new SQL file with timestamp prefix
2. Include both forward and rollback operations
3. Test thoroughly before deploying

## Monitoring and Observability

- **Logging**: Structured logging with tracing crate
- **Metrics**: Application metrics (to be implemented)
- **Health Checks**: Built-in health endpoint
- **Database Monitoring**: Connection pool metrics

## Deployment

The application is designed for cloud-native deployment:

- **Docker**: Containerization ready
- **Kubernetes**: Scalable orchestration
- **Environment-based Configuration**: 12-factor app compliant
- **Graceful Shutdowns**: Proper cleanup on termination

## Contributing

We welcome contributions! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Follow Rust conventions**:
   - Use `cargo fmt` for formatting
   - Ensure `cargo clippy` passes
   - Add tests for new functionality
   - Update documentation as needed
4. **Commit changes** (`git commit -m 'Add amazing feature'`)
5. **Push to branch** (`git push origin feature/amazing-feature`)
6. **Open a Pull Request**

### Code Standards

- Follow Rust idioms and best practices
- Write comprehensive tests
- Document public APIs
- Use meaningful commit messages
- Keep PRs focused and atomic

### Development Workflow

1. Check existing issues or create new ones
2. Discuss major changes before implementation
3. Write tests first (TDD approach)
4. Ensure all tests pass
5. Update documentation
6. Submit PR with clear description

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Rust community for excellent ecosystem
- Axum team for the high-performance web framework
- SQLx team for compile-time SQL verification
- All contributors and supporters

## Roadmap

- [ ] Complete API implementations
- [ ] Add comprehensive test suite
- [ ] Implement real-time notifications
- [ ] Add API rate limiting
- [ ] Performance optimization
- [ ] Add Docker containerization
- [ ] Kubernetes deployment manifests
- [ ] API documentation with OpenAPI/Swagger
- [ ] Integration with external payment gateways
- [ ] Advanced analytics and reporting
- [ ] Multi-currency support enhancement
- [ ] Webhook system for external integrations

## Support

For support and questions:

- Create an issue on GitHub
- Check existing documentation
- Review the codebase for examples

---

**Note**: This is an active development project. APIs and features are subject to change. Please refer to the latest documentation and release notes for updates.


