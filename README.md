# Rust REST API with Axum & MongoDB

A high-performance REST API built with Rust using the Axum web framework and MongoDB for user authentication. This project demonstrates best practices for building secure authentication systems with proper password hashing, JWT tokens, and MongoDB integration.

## Features

- **High Performance**: Built with Axum for excellent performance and async support
- **MongoDB Integration**: NoSQL database with the official MongoDB Rust driver
- **Secure Authentication**: JWT-based authentication with bcrypt password hashing
- **Input Validation**: Request validation using serde and validator
- **Error Handling**: Comprehensive error handling with custom error types
- **Structured Logging**: Application tracing with the tracing crate
- **CORS Support**: Cross-origin resource sharing configuration
- **Rate Limiting**: Basic rate limiting middleware
- **Health Checks**: Application health monitoring endpoints

## Tech Stack

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) - Ergonomic web framework for Rust
- **Database**: [MongoDB](https://www.mongodb.com/) with [mongodb](https://github.com/mongodb/mongo-rust-driver) driver
- **Async Runtime**: [Tokio](https://tokio.rs/) - Asynchronous runtime
- **Authentication**: [jsonwebtoken](https://github.com/Keats/jsonwebtoken) - JWT implementation
- **Password Hashing**: [bcrypt](https://github.com/Keats/bcrypt-rs) - Secure password hashing
- **Serialization**: [Serde](https://serde.rs/) - JSON serialization/deserialization
- **Validation**: [validator](https://github.com/Keats/validator) - Input validation
- **Logging**: [tracing](https://tracing.rs/) - Structured logging
- **Environment**: [dotenvy](https://github.com/allan2/dotenvy) - Environment variables

## Prerequisites

- Rust 1.70+ installed ([Install Rust](https://rustup.rs/))
- MongoDB 4.4+ running locally or remotely
- Git

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/rust-axum-mongodb-auth.git
cd rust-axum-mongodb-auth
```

### 2. Environment Setup

Create a `.env` file in the project root:

```env
# MongoDB Configuration
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=rust_auth_db

# JWT Configuration
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRES_IN=24h

# Server Configuration
SERVER_HOST=127.0.0.1
SERVER_PORT=3000

# Logging
RUST_LOG=debug

# Security
BCRYPT_COST=12
```

### 3. Install Dependencies

```bash
cargo build
```

### 4. Run the Application

```bash
# Development mode with hot reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Or standard run
cargo run
```

The API will be available at `http://localhost:3000`

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── lib.rs                  # Library root
├── config/
│   └── mod.rs             # Configuration management
├── database/
│   ├── mod.rs             # Database modules
│   └── connection.rs      # MongoDB connection setup
├── handlers/
│   ├── mod.rs             # Handler modules
│   ├── auth.rs            # Authentication handlers
│   ├── user.rs            # User management handlers
│   └── health.rs          # Health check handlers
├── middleware/
│   ├── mod.rs             # Middleware modules
│   ├── auth.rs            # JWT authentication middleware
│   ├── cors.rs            # CORS middleware
│   └── rate_limit.rs      # Rate limiting middleware
├── models/
│   ├── mod.rs             # Model modules
│   ├── user.rs            # User model and validation
│   └── auth.rs            # Authentication request/response models
├── services/
│   ├── mod.rs             # Service modules
│   ├── auth_service.rs    # Authentication business logic
│   └── user_service.rs    # User business logic
├── utils/
│   ├── mod.rs             # Utility modules
│   ├── jwt.rs             # JWT token utilities
│   ├── password.rs        # Password hashing utilities
│   └── validation.rs      # Custom validators
└── errors/
    └── mod.rs             # Custom error types

tests/
├── integration/           # Integration tests
│   ├── auth_tests.rs      # Authentication tests
│   └── user_tests.rs      # User management tests
└── common/
    └── mod.rs             # Test utilities
```

## Dependencies

Add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
# Web framework
axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "limit"] }

# Database
mongodb = "2.8"
bson = { version = "2.9", features = ["chrono-0_4"] }
futures = "0.3"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Authentication & Security
jsonwebtoken = "9.2"
bcrypt = "0.15"

# Validation
validator = { version = "0.18", features = ["derive"] }
regex = "1.10"

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# Logging & Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Environment & Configuration
dotenvy = "0.15"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Utilities
uuid = { version = "1.0", features = ["v4", "serde"] }

[dev-dependencies]
reqwest = { version = "0.11", features = ["json"] }
tokio-test = "0.4"
tower-test = "0.4"
```

## Database Schema

### User Collection

```javascript
// MongoDB User document structure
{
  "_id": ObjectId("..."),
  "id": "uuid-v4-string",
  "username": "john_doe",
  "email": "john@example.com",
  "password_hash": "$2b$12$...", // bcrypt hash
  "is_active": true,
  "is_verified": false,
  "created_at": ISODate("2024-01-01T00:00:00Z"),
  "updated_at": ISODate("2024-01-01T00:00:00Z"),
  "last_login": ISODate("2024-01-01T00:00:00Z")
}
```

### MongoDB Indexes

```javascript
// Create indexes for better performance
db.users.createIndex({ "email": 1 }, { unique: true })
db.users.createIndex({ "username": 1 }, { unique: true })
db.users.createIndex({ "id": 1 }, { unique: true })
```

## API Endpoints

### Authentication

#### Register User
- **POST** `/api/auth/register`
- **Description**: Register a new user account
- **Request Body**:
```json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "SecurePass123!"
}
```
- **Response** (201):
```json
{
  "success": true,
  "message": "User registered successfully",
  "data": {
    "user": {
      "id": "uuid-here",
      "username": "johndoe",
      "email": "john@example.com",
      "is_active": true,
      "created_at": "2024-01-01T00:00:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

#### Login User
- **POST** `/api/auth/login`
- **Description**: Authenticate user and return JWT token
- **Request Body**:
```json
{
  "email": "john@example.com",
  "password": "SecurePass123!"
}
```
- **Response** (200):
```json
{
  "success": true,
  "message": "Login successful",
  "data": {
    "user": {
      "id": "uuid-here",
      "username": "johndoe",
      "email": "john@example.com"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_at": "2024-01-02T00:00:00Z"
  }
}
```

#### Get Current User
- **GET** `/api/auth/me`
- **Description**: Get current authenticated user details
- **Headers**: `Authorization: Bearer <token>`
- **Response** (200):
```json
{
  "success": true,
  "data": {
    "id": "uuid-here",
    "username": "johndoe",
    "email": "john@example.com",
    "is_active": true,
    "created_at": "2024-01-01T00:00:00Z",
    "last_login": "2024-01-01T12:00:00Z"
  }
}
```

#### Logout User
- **POST** `/api/auth/logout`
- **Description**: Logout user (client should discard token)
- **Headers**: `Authorization: Bearer <token>`
- **Response** (200):
```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

### Health Check
- **GET** `/health`
- **Description**: Application health status
- **Response** (200):
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "database": "connected"
}
```

## Usage Examples

### Register a New User

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "johndoe",
    "email": "john@example.com",
    "password": "SecurePass123!"
  }'
```

### Login User

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "SecurePass123!"
  }'
```

### Access Protected Endpoint

```bash
curl -X GET http://localhost:3000/api/auth/me \
  -H "Authorization: Bearer YOUR_JWT_TOKEN_HERE"
```

## Password Requirements

- Minimum 8 characters
- At least 1 uppercase letter
- At least 1 lowercase letter
- At least 1 number
- At least 1 special character (!@#$%^&*(),.?":{}|<>)

## Security Features

- **Password Hashing**: Bcrypt with configurable cost factor
- **JWT Tokens**: Secure token-based authentication
- **Input Validation**: Comprehensive request validation
- **Rate Limiting**: Protection against brute force attacks
- **CORS Configuration**: Configurable cross-origin requests
- **Error Handling**: No sensitive information leakage
- **MongoDB Injection Prevention**: Using BSON and parameterized queries

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run integration tests only
cargo test --test integration

# Test specific module
cargo test auth

# Run tests with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Development

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting without applying
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with all targets and features
cargo clippy --all-targets --all-features -- -D warnings
```

### Database Operations

```bash
# Connect to MongoDB
mongosh mongodb://localhost:27017

# Use your database
use rust_auth_db

# View users collection
db.users.find().pretty()

# Create indexes
db.users.createIndex({ "email": 1 }, { unique: true })
db.users.createIndex({ "username": 1 }, { unique: true })
```

## Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `MONGODB_URI` | MongoDB connection string | `mongodb://localhost:27017` | Yes |
| `DATABASE_NAME` | Database name | `rust_auth_db` | Yes |
| `JWT_SECRET` | JWT signing secret | - | Yes |
| `JWT_EXPIRES_IN` | Token expiration time | `24h` | No |
| `SERVER_HOST` | Server bind address | `127.0.0.1` | No |
| `SERVER_PORT` | Server port | `3000` | No |
| `BCRYPT_COST` | Bcrypt hashing cost | `12` | No |
| `RUST_LOG` | Logging level | `info` | No |

## Deployment

### Docker

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-axum-mongodb-auth /usr/local/bin/app
EXPOSE 3000
CMD ["app"]
```

Build and run:

```bash
# Build image
docker build -t rust-axum-mongodb-auth .

# Run with docker-compose (recommended)
docker-compose up -d
```

### Production Considerations

- Use strong, randomly generated `JWT_SECRET`
- Set `RUST_LOG=info` in production
- Configure MongoDB with authentication
- Use connection pooling
- Set up SSL/TLS termination
- Implement proper rate limiting
- Add monitoring and alerting
- Use secrets management
- Set up backup strategies

## Error Handling

The API returns consistent error responses:

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid input data",
    "details": [
      {
        "field": "email",
        "message": "Invalid email format"
      }
    ]
  }
}
```

Common error codes:
- `VALIDATION_ERROR`: Input validation failed
- `UNAUTHORIZED`: Authentication required
- `FORBIDDEN`: Access denied
- `NOT_FOUND`: Resource not found
- `CONFLICT`: Resource already exists
- `INTERNAL_ERROR`: Server error

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Run `cargo fmt` and `cargo clippy`
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [MongoDB Rust Driver](https://docs.rs/mongodb/)
- [Tokio Documentation](https://docs.rs/tokio/)
- [JWT.io](https://jwt.io/) - JWT debugger
- [MongoDB Manual](https://docs.mongodb.com/)
- [Rust Security Guidelines](https://rustsec.org/)