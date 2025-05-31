# FullStack Ethereum web3 boilerplate

BE Written in Rust. Contract using solidity, hardhat & foundry

## Overview

This project is a full-stack application that includes a frontend, backend, and smart contracts. It allows users to connect their Ethereum wallet, view their balance and transaction history, mint tokens, and interact with the blockchain.

## Prerequisites

- [Node.js](https://nodejs.org/) (v18+ recommended)
- [npm](https://www.npmjs.com/)
- [Foundry](https://book.getfoundry.sh/getting-started/installation)
- [Hardhat](https://hardhat.org/getting-started/)
- [Docker](https://docs.docker.com/get-docker/) (optional, for containerization)

## Setup Instructions

For detailed setup instructions, please refer to the README files in each directory:

- [Backend README](./backend/README.md)
- [Smart Contracts README](./contracts/README.md)

## Docker Compose Instructions

To run the application using Docker Compose:

1. Ensure Docker and Docker Compose are installed.
2. Run the following command from the root directory:

   ```sh
   cd backend && docker-compose up --build
   ```

## Design Decisions and Assumptions

### Backend Architecture

1. **Rust with Axum**: Chosen for its performance, type safety, and modern async runtime
2. **Caching Strategy**:
   - Redis for caching block numbers and gas prices
   - Cache TTL based on block mining duration, Block produce duration of ETH is 12 seconds (at the time of this repo created)
   - Fallback to provider when cache misses
3. **Database**:
   - PostgreSQL for persistent storage
   - SQLx for type-safe database queries
   - Auto Migrations when startup project

### Network Assumption

The current implementation assumes interaction with a single Ethereum network (e.g., Sepolia testnet). This assumption is reflected in:

1. **Database Design**:
   - No network identifier in account balance tables
   - Single network configuration in environment variables
   - No network-specific caching strategies

2. **API Design**:
   - No network parameter in API endpoints
   - Single provider configuration
   - Network-agnostic response structures

To support multiple networks in the future, the following changes would be required:

1. **Database Changes**:
   - Add network identifier column to relevant tables
   - Implement network-specific migrations
   - Consider network-specific indexes

2. **API Changes**:
   - Add network parameter to endpoints (e.g., `/v1/public/{network}/eth/accounts/{address}`)
   - Implement network-specific validation
   - Add network information in responses

3. **Infrastructure Changes**:
   - Multiple RPC provider configurations
   - Network-specific caching strategies
   - Network-specific rate limiting

4. **Configuration Changes**:
   - Network-specific environment variables
   - Network-specific constants and parameters
   - Network-specific ABI configurations

### API Design

1. **RESTful Endpoints**:
   - `/v1/public/eth/accounts/{address}` for account info
   - `/v1/public/eth/accounts/{address}/erc20/{token_address}` for token balances
   - `/v1/public/eth/misc` for blockchain information (block number and gas price)
   - `/health` for system health check
   - `/ping` for service availability check

2. **Error Handling**:
   - Consistent error response format with `error_msg` field
   - Validation for Ethereum addresses (must be 42 characters, start with 0x)
   - Proper HTTP status codes (200, 400, 404, 500)

3. **Response Formats**:
   - Account Info: `{ "address": "string", "balance": "string" }`
   - ERC20 Balance: `{ "address": "string", "token_address": "string", "balance": "string" }`
   - Blockchain Misc: `{ "current_block": "number", "gas_price": "number" }`

### Testing Strategy

1. **Integration Tests**:
   - Uses `axum-test` for HTTP testing
   - Tests both success and error cases
   - Includes concurrent request testing
2. **Test Environment**:
   - Docker Compose for service dependencies
   - Separate test database
   - Mock Ethereum provider responses

### Security Considerations

1. **Input Validation**:
   - Ethereum address format validation
   - Token address validation
2. **Rate Limiting**:
   - Implemented at the API level
   - Configurable limits per endpoint

### Performance Optimizations

1. **Caching**:
   - Block numbers cached with dynamic TTL
   - Gas prices cached with fixed TTL
2. **Concurrent Requests**:
   - Async handling of multiple requests
   - Connection pooling for database and Redis

## Known Issues and Limitations

- The application is currently configured for the Sepolia testnet and may require adjustments for other networks.
- Ensure your wallet has sufficient test ETH for gas fees during deployment and minting.
- Contract verification requires an Etherscan API key.

## Interacting with the Backend API

Once the backend server is running, you can interact with it using the following endpoints:

### Example API Endpoints

1. **Get Account Information**
   - **URL**: `http://localhost:8080/v1/public/eth/accounts/<ethereum-address>`
   - **Method**: `GET`
   - **Description**: Returns the account information, including the balance for the specified Ethereum address.

2. **Get Blockchain Misc Information**
   - **URL**: `http://localhost:8080/v1/public/eth/misc`
   - **Method**: `GET`
   - **Description**: Returns the current block number and gas price from the Ethereum network.

3. **Get ERC20 Token Balance**
   - **URL**: `http://localhost:8080/v1/public/eth/accounts/<ethereum-address>/erc20/<token-address>`
   - **Method**: `GET`
   - **Description**: Returns the ERC20 token balance for the specified Ethereum address and token address.

4. **Health Check**
   - **URL**: `http://localhost:8080/health`
   - **Method**: `GET`
   - **Description**: Checks the health of the backend services, including the database, Ethereum provider, and cache.

5. **Ping**
   - **URL**: `http://localhost:8080/ping`
   - **Method**: `GET`
   - **Description**: A simple endpoint to check if the server is running.

### Example Usage

You can use tools like `curl` or Postman to interact with these endpoints. Here are some examples:

- **Get Account Information**:

  ```sh
  curl http://localhost:8080/v1/public/eth/accounts/0xYourEthereumAddress
  ```

- **Get Blockchain Misc Information**:

  ```sh
  curl http://localhost:8080/v1/public/eth/misc
  ```

- **Get ERC20 Token Balance**:

  ```sh
  curl http://localhost:8080/v1/public/eth/accounts/0xYourEthereumAddress/erc20/0xYourTokenAddress
  ```

- **Health Check**:

  ```sh
  curl http://localhost:8080/health
  ```

- **Ping**:

  ```sh
  curl http://localhost:8080/ping
  ```

Replace `0xYourEthereumAddress` and `0xYourTokenAddress` with the actual Ethereum and token addresses you want to query.

---

## References

- [Hardhat Documentation](https://hardhat.org/docs)
- [Ethers.js Documentation](https://docs.ethers.org/)
- [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts)
