import { ethers } from "ethers";

async function main() {
  const provider = new ethers.JsonRpcProvider(process.env.SEPOLIA_RPC_URL!);
  const wallet = new ethers.Wallet(process.env.SEPOLIA_PRIVATE_KEY!, provider);

  const tokenAddress = process.env.TOKEN_ADDRESS!;
  const amount = ethers.parseUnits("100", 18); // 100 tokens, 18 decimals

  const erc20Abi = [
    "function mint(uint256 amount)",
  ];
  const tokenContract = new ethers.Contract(tokenAddress, erc20Abi, wallet);

  const tx = await tokenContract.mint(amount);
  console.log("Transaction sent:", tx.hash);
  await tx.wait();
  console.log("Transaction confirmed");
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
