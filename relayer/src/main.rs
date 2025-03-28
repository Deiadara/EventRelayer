use std::{fs, thread, time};
use alloy::{
    primitives::{B256,Address,keccak256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{Filter,eth::BlockNumberOrTag},
};
use eyre::Result;
use serde_json::Value;

const SRC_RPC: &str = "http://localhost:8545";
const DST_RPC: &str = "http://localhost:8546";

#[tokio::main]
async fn main() -> Result<()> {

    let abi_path = "../project_eth/data/TokenABI.txt";
    let abi_str = fs::read_to_string(abi_path)?;
    let dst_abi: Value = serde_json::from_str(&abi_str)?;
    println!("Loaded dst_abi: {:?}", dst_abi);

    let bytecode_path = "../project_eth/data/TokenBytecode.txt";
    let bytecode_str = fs::read_to_string(bytecode_path)?;
    let dst_bytecode: Value = serde_json::from_str(&bytecode_str)?;
    println!("Loaded dst_bytecode: {:?}", dst_bytecode);

    loop {
        let rpc_url = SRC_RPC.parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);
        let latest_block = provider.get_block_number().await?;

        let event_sig = keccak256("Deposited(address,string)");

        let contract_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse()?;

        // if we want to check entire blockchain
        // let filter = Filter::new().address(contract_address).from_block(BlockNumberOrTag::Earliest)
        // .to_block(BlockNumberOrTag::Latest);   

        // if we want to check just the final block
        let filter = Filter::new().address(contract_address).from_block(latest_block);

        println!("Scanning from earliest to latest...");
        println!("Filter topic0: {:?}", B256::from(event_sig));

        let logs = provider.get_logs(&filter).await?;

        println!("Latest block: {:?}", latest_block);   
        println!("Got {} logs", logs.len());

        for log in logs {
            println!("Transfer event: {log:?}");
        }

        let one_sec = time::Duration::from_millis(2000);
        thread::sleep(one_sec);


    }
}


// todo : build contracts with forge using cli
// todo : deployer imports ABI and Bytecode from a file