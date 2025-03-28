import { ethers } from "ethers";
import fs from "fs";
import path from "path";

// ABI for Event.sol
const abiPath = path.join(process.cwd(), "data", "DepositABI.txt");
const abi = JSON.parse(fs.readFileSync(abiPath, "utf8"));

// Deployed address of Event contract
const address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";

const provider = new ethers.JsonRpcProvider("http://localhost:8545");

const privateKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

const wallet = new ethers.Wallet(privateKey, provider);

const main = async () => {
  const contract = new ethers.Contract(address, abi, wallet);
  const tx = await contract.deposit("5");
  const receipt = await tx.wait();
  console.log(receipt);
  console.log("Event emitted from Deposit.sol on Anvil #1!");
};

main().catch(console.error);


