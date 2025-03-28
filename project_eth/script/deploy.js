import fs from "fs";
import path from "path";
import solc from "solc";
import { ethers } from "ethers";
const __dirname = path.dirname(new URL(import.meta.url).pathname);

async function build_and_deploy(wallet,contract_name) {
    const file_name = contract_name + '.sol';
    const source = fs.readFileSync(`src/${file_name}`, "utf8");
    const input = {
        language: "Solidity",
        sources: {
            [file_name]: {
                content: source,
            },
        },
        settings: {
            outputSelection: {
                "*": {
                    "*": ["abi", "evm.bytecode"],
                },
            },
        },
    };

    const output = JSON.parse(solc.compile(JSON.stringify(input)));
    const contractData = output.contracts[file_name][contract_name];
    const bytecode = contractData.evm.bytecode.object;
    const abi = contractData.abi;

    const factory = new ethers.ContractFactory(abi, bytecode, wallet);
    const contract = await factory.deploy();
    await contract.waitForDeployment();

    console.log(`${contract_name} deployed to:`, await contract.getAddress());
    console.log(JSON.stringify(abi))

    const outputDir = path.join(__dirname, '..', "data");
    const filePathABI = path.join(outputDir, `${contract_name}ABI.txt`);
    const filePathBytecode = path.join(outputDir, `${contract_name}Bytecode.txt`);

    if (!fs.existsSync(outputDir)) {
        fs.mkdirSync(outputDir);
    }

    fs.writeFileSync(filePathABI, JSON.stringify(abi, null, 2));
    console.log(`ABI saved to ${filePathABI}`);

    fs.writeFileSync(filePathBytecode, JSON.stringify(bytecode, null, 2));
    console.log(`Bytecode saved to ${filePathBytecode}`);
}


async function main() {

    const src_provider = new ethers.JsonRpcProvider("http://localhost:8545");
    const src_wallet = new ethers.Wallet("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", src_provider);

    const dst_provider = new ethers.JsonRpcProvider("http://localhost:8546");
    const dst_wallet = new ethers.Wallet("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", dst_provider);

    await build_and_deploy(src_wallet,"Deposit")
    await build_and_deploy(dst_wallet,"Token")
}

main().catch((error) => {
    console.error(error);
    process.exit(1);
});
