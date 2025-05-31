# TechBank Backend

A Rust-based backend service for the TechBank application, built with Axum, SQLx, and Redis.

## Features

- RESTful API endpoints
- PostgreSQL database integration with SQLx
- Redis caching
- Docker support
- Comprehensive test suite

## Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- PostgreSQL 16
- Redis 7.2

## Quick Start

1. Clone the repository:
```bash
git clone <repository-url>
cd backend
```

2. Start the required services using Docker Compose:
```bash
docker-compose up -d
```

3. Set up the database:
```bash
# Install SQLx CLI if you haven't already
cargo install sqlx-cli

# Run database migrations
sqlx migrate run
```

4. Build and run the application:
```bash
cargo build
cargo run
```

## Development Setup

1. Install dependencies:
```bash
cargo build
```

2. Set up environment variables:
```bash
# Create a .env file with the following variables
DATABASE_URL=postgres://postgres:123abc@localhost:5432/postgres
REDIS_URL=redis://localhost:6379
```

3. Run tests:
```bash
cargo test
```

## SQLx Setup

The project uses SQLx for database operations. To set up SQLx:

1. Install the SQLx CLI:
```bash
cargo install sqlx-cli
```

## API Documentation

### Endpoints

#### Health Check
- `GET /health`
  - Checks the health of the system including database, Ethereum provider, and Redis cache
  - Returns 200 OK if all systems are healthy

#### Ping
- `GET /ping`
  - Simple endpoint to check if the service is running
  - Returns 200 OK if service is up

#### Ethereum Account Information
- `GET /v1/public/eth/accounts/{address}`
  - Get information about an Ethereum account
  - Parameters:
    - `address`: Ethereum address (must start with 0x and be 42 characters long)
  - Returns:
    ```json
    {
      "address": "string",
      "balance": "string"
    }
    ```

#### Blockchain Misc Information
- `GET /v1/public/eth/misc`
  - Get current blockchain information including block number and gas price
  - Returns:
    ```json
    {
      "current_block": "number",
      "gas_price": "number"
    }
    ```

#### ERC20 Token Balance
- `GET /v1/public/eth/accounts/{address}/erc20/{token_address}`
  - Get ERC20 token balance for an Ethereum account
  - Parameters:
    - `address`: Ethereum account address (must start with 0x and be 42 characters long)
    - `token_address`: ERC20 token contract address (must start with 0x and be 42 characters long)
  - Returns:
    ```json
    {
      "address": "string",
      "token_address": "string",
      "balance": "string"
    }
    ```

### Error Responses

The API uses standard HTTP status codes and returns errors in the following format:

```json
{
  "error_msg": "string"
}
```

Common error codes:
- `400 Bad Request`: Invalid input parameters
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server-side error

### Request/Response Examples

#### Health Check
```bash
# Request
curl -X GET http://localhost:8080/health

# Response (200 OK)
# Empty response body indicates all systems are healthy
```

#### Ping
```bash
# Request
curl -X GET http://localhost:8080/ping

# Response (200 OK)
# Empty response body indicates service is running
```

#### Ethereum Account Information
```bash
# Request
curl -X GET http://localhost:8080/v1/public/eth/accounts/0x742d35Cc6634C0532925a3b844Bc454e4438f44e

# Response (200 OK)
{
  "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "balance": "1000000000000000000"
}

# Error Response (400 Bad Request) - Invalid address format
{
  "error_msg": "Invalid Ethereum address format"
}
```

#### Blockchain Misc Information
```bash
# Request
curl -X GET http://localhost:8080/v1/public/eth/misc

# Response (200 OK)
{
  "current_block": 12345678,
  "gas_price": 25000000000
}

# Error Response (404 Not Found) - Block not found
{
  "error_msg": "Latest block not found"
}
```

#### ERC20 Token Balance
```bash
# Request
curl -X GET http://localhost:8080/v1/public/eth/accounts/0x742d35Cc6634C0532925a3b844Bc454e4438f44e/erc20/0xdAC17F958D2ee523a2206206994597C13D831ec7

# Response (200 OK)
{
  "address": "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
  "token_address": "0xdAC17F958D2ee523a2206206994597C13D831ec7",
  "balance": "500000000000000000"
}

# Error Response (400 Bad Request) - Invalid address format
{
  "error_msg": "Invalid Ethereum address format"
}

# Error Response (400 Bad Request) - Invalid token address format
{
  "error_msg": "Invalid token address format"
}
```

Note: 
- All Ethereum addresses must be 42 characters long and start with "0x"
- Balance values are returned as strings to preserve precision
- Gas price is returned in wei (1 ETH = 10^18 wei)
- Current block number represents the latest block in the Ethereum network

## Testing

The project includes both unit tests and integration tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test <test_name>

# Run integration test
make integration-test
```

## Docker Support

The project includes Docker configuration for development and production:

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down
```

## Project Structure

```
.
├── src/           # Source code
├── tests/         # Test files
├── migrations/    # Database migrations
├── config/        # Configuration files
├── abi/           # Contract ABIs
└── .sqlx/         # SQLx query metadata
```
