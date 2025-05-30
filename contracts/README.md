# MyToken Hardhat Project

This guide explains how to set up your environment, deploy and verify the MyToken ERC20 contract, and mint tokens on the Sepolia testnet.

Contract has been deployed at https://sepolia.etherscan.io/address/0xab809cb0ab6669d51f6189432f751f1a916a10cd

---

## Prerequisites
- [Node.js](https://nodejs.org/) (v18+ recommended)
- [npm](https://www.npmjs.com/)
- **Foundry**: Install Foundry by running the following command:
  ```sh
  curl -L https://foundry.paradigm.xyz | bash
  foundryup
  ```
- **Hardhat**: Install Hardhat globally (optional, as it's also installed as a dev dependency):
  ```sh
  npm install -g hardhat
  ```

---

## 1. Environment Setup

1. **Clone the repository** (if you haven't already):
   ```sh
   git clone <your-repo-url>
   cd contracts
   ```

2. **Install dependencies:**
   ```sh
   npm install
   ```

3. **Set up environment variables:**
   - Copy the `.env_template` to `.env` and fill in your values:
     ```sh
     cp .env_template .env
     ```
   - Edit `.env` and provide:
     - `SEPOLIA_RPC_URL`: Your Sepolia RPC endpoint (e.g., from Alchemy or Infura)
     - `SEPOLIA_PRIVATE_KEY`: Private key of your Sepolia testnet wallet (with test ETH)
     - `ETHERSCAN_API_KEY`: (Optional, for contract verification)
     - `TOKEN_ADDRESS`: (Leave blank for now; will be filled after deployment)

---

## 2. Deploy the Contract

Deploy the MyToken ERC20 contract to Sepolia using Hardhat Ignition:

```sh
npx hardhat ignition deploy ./ignition/modules/MyToken.ts --network sepolia
```

- After deployment, note the contract address from the output.
- Update your `.env` file with the deployed contract address as `TOKEN_ADDRESS`.

---

## 3. Verify the Contract (Optional)

To verify your contract on Etherscan (requires `ETHERSCAN_API_KEY` in your `.env`):

```sh
npx hardhat verify --network sepolia <DEPLOYED_CONTRACT_ADDRESS>
```

---

## 4. Mint Tokens

You can mint tokens to your wallet using the provided script:

```sh
npx ts-node scripts/mint.ts
```

- This will mint 100 tokens (with 18 decimals) to the wallet specified by your `SEPOLIA_PRIVATE_KEY`.
- Ensure `TOKEN_ADDRESS` in your `.env` is set to your deployed contract address.

---

## 5. Useful Commands

- Run Hardhat tasks:
  ```sh
  npx hardhat help
  ```
- Clean the cache and artifacts:
  ```sh
  npx hardhat clean
  ```

---

## 7. References
- [Hardhat Documentation](https://hardhat.org/docs)
- [Ethers.js Documentation](https://docs.ethers.org/)
- [OpenZeppelin Contracts](https://docs.openzeppelin.com/contracts)
