# Full Stack Developer Tech Exam

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
   - **Description**: Returns the account information, including the current block number, gas price, and balance for the specified Ethereum address.

2. **Get ERC20 Token Balance**
   - **URL**: `http://localhost:8080/v1/public/eth/accounts/<ethereum-address>/erc20/<token-address>`
   - **Method**: `GET`
   - **Description**: Returns the ERC20 token balance for the specified Ethereum address and token address.

3. **Health Check**
   - **URL**: `http://localhost:8080/health`
   - **Method**: `GET`
   - **Description**: Checks the health of the backend services, including the database, Ethereum provider, and cache.

4. **Ping**
   - **URL**: `http://localhost:8080/ping`
   - **Method**: `GET`
   - **Description**: A simple endpoint to check if the server is running.

### Example Usage

You can use tools like `curl` or Postman to interact with these endpoints. Here are some examples:

- **Get Account Information**:
  ```sh
  curl http://localhost:8080/v1/public/eth/accounts/0xYourEthereumAddress
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