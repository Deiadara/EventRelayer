import { ethers } from "ethers";

// ABI for Event.sol
const abi = [{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"sender","type":"address"},{"indexed":false,"internalType":"string","name":"amount","type":"string"}],"name":"Deposited","type":"event"},{"inputs":[{"internalType":"string","name":"amount","type":"string"}],"name":"deposit","outputs":[],"stateMutability":"nonpayable","type":"function"}]

// Deployed address of Event contract (Anvil #1)
const address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

// Anvil #1 RPC provider
const provider = new ethers.JsonRpcProvider("http://localhost:8545");

// Anvil default private key
const privateKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

// Wallet from Anvil account
const wallet = new ethers.Wallet(privateKey, provider);

const main = async () => {
  const contract = new ethers.Contract(address, abi, wallet);
  const tx = await contract.deposit("5");
  const receipt = await tx.wait();
  console.log(receipt);
  console.log("✅ Event emitted from Deposit.sol on Anvil #1!");
};

main().catch(console.error);


