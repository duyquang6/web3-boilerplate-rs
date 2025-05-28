# Full Stack Developer Tech Exam

## Overview:
This tech exam is designed to assess your skills across various domains of full stack development, including frontend, backend, smart contract development, and integration. It is divided into sections, with each section representing a progressively more challenging tier. You are not required to complete all sections to pass, as the assessment is designed to evaluate your current skill level.

Complete as much as you can, and we will assess your capabilities accordingly.

---

## **Tier 1: Frontend Development**

### Task:
Create a basic web interface that allows users to connect their Ethereum wallet (using MetaMask or WalletConnect) and view their balance and transaction history.

### Requirements:
1. Build a UI that allows users to connect their Ethereum wallet.
2. Fetch and display the user's Ethereum balance (in ETH).
3. Fetch and display the user's last 10 transactions.
4. Include basic error handling for failed connections or API calls.
5. Use web3.js or ethers.js for blockchain interactions.

### Bonus:
- Use TypeScript for the frontend code.

### Difficulty Level: Easy
This tier is designed to assess your ability to build simple UI interactions with blockchain data.

---

## **Tier 2: Backend Development**

### Task:
Create an API endpoint that returns the gas price, current Ethereum block number, and basic account details for a given Ethereum address.

### Requirements:
1. Create a REST API with an endpoint to accept an Ethereum address as input.
2. Call the Ethereum network to get:
   - Current gas price.
   - Current block number.
   - The balance of the given address.
3. Return the above data in a JSON format.
4. Properly structure the code to allow future extensibility.

### Resources:
- Etherscan - https://docs.etherscan.io/
- Alchemy - https://docs.alchemy.com/reference/api-overview
- Covalent - https://www.covalenthq.com/docs/api/#/0/0/USD/1

### Bonus:
- Implement simple caching (e.g., using Redis) for the gas price and block number to reduce API calls.
- Set up a PostgreSQL or MongoDB database to store account balances.

### Difficulty Level: Intermediate
This tier introduces backend development and API integration with external services.

---

## **Tier 3: Smart Contract Development**

### Task:
Write a simple Ethereum smart contract (ERC-721 or ERC-20) that allows minting tokens and transferring them between users.

### Requirements:
1. Write a smart contract in Solidity that:
   - Allows a user to mint a token (ERC-721 or ERC-20).
   - Allows users to transfer tokens to another address.
2. Use OpenZeppelin libraries for base contracts.

### Bonus:
- Deploy the contract to a testnet (e.g., Rinkeby or Goerli).

### Difficulty Level: Intermediate to Advanced
This tier assesses your understanding of blockchain development through smart contract creation.

---

## **Tier 4: Integration**

### Task:
Integrate the frontend with the backend and smart contract to enable users to mint a token from the frontend and view their minted tokens.

### Requirements:
1. From the frontend, allow users to mint tokens by interacting with the smart contract.
2. Once a token is minted, show the token details (e.g., token ID, owner) on the frontend.
3. Fetch token data using the backend API for the given address.
4. Handle errors (e.g., insufficient funds for gas fees, contract deployment issues).

### Bonus:
- **Docker**: Create a Dockerfile to containerize the frontend and backend services, and use Docker Compose to orchestrate the services.

### Difficulty Level: Advanced
This tier challenges you to integrate all components into a full-stack application, demonstrating both frontend and backend skills, as well as blockchain knowledge.

---

## **Scoring Criteria**:
You will be assessed based on the sections you complete, with the difficulty level factored into your final evaluation. You can earn bonus points in each section by completing the optional tasks.

### **Frontend (20 points)**:
- Functionality and usability (10 points)
- Code quality and structure (10 points)
- Bonus: TypeScript usage (5 points)

### **Backend (20 points)**:
- API functionality (10 points)
- Code structure and error handling (10 points)
- Bonus: Caching and database (5 points)

### **Smart Contract (20 points)**:
- Contract functionality (15 points)
- Code organization and readability (5 points)
- Bonus: Testnet deployment (5 points)

### **Integration (20 points)**:
- Seamless integration of frontend, backend, and smart contract (15 points)
- Error handling (5 points)
- Bonus: Docker (5 points)

---

## **Submission Guidelines**:
1. **GitHub Repository**:
   - Push all your code (frontend, backend, smart contracts, Docker files, etc.) to a new GitHub repository.
   - Organize the repository with clear directories for each section (e.g., /frontend, /backend, /contracts, /docker).
   - Ensure all sensitive information (like API keys or private keys) is excluded from the repository.

2. **README**:
   - Include a README.md file at the root of your repository with:
     - An overview of your project.
     - Instructions on how to set up and run the application locally.
     - Prerequisites or dependencies.
     - Docker Compose instructions (if applicable).
     - Assumptions or decisions you made.
     - Known issues or limitations.

3. **Collaborator**:
   - Add [apquinit](https://github.com/apquinit) as a collaborator to your repository.

4. **Schedule a Review**:
   - Book an appointment with us to review your submission by scheduling a meeting with Alex on [Calendly](https://calendly.com/apquinit). Include your GitHub repository link and any additional notes or comments.

---

Thank you for participating! We look forward to seeing your skills in action!
