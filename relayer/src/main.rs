use std::{convert::TryInto, fs, str, thread, time};
use alloy::{
    primitives::{keccak256, Address, Bytes, B256,hex},
    providers::{Provider, ProviderBuilder},
    rpc::types::{eth::BlockNumberOrTag, Filter}
};
use eyre::Result;
use serde_json::Value;
use alloy_dyn_abi::{DynSolType, DynSolValue};

const SRC_RPC: &str = "http://localhost:8545";
const DST_RPC: &str = "http://localhost:8546";


fn decode_string_from_hex(data: &Bytes) -> Result<String> {
    let raw = data.as_ref();

    if raw.len() < 64 {
        return Err(eyre::eyre!("Log data too short to be valid ABI-encoded string"));
    }

    // The string length is at bytes 32..64 (second word)
    let length_bytes = &raw[32..64];
    let str_len = u32::from_be_bytes(length_bytes[28..32].try_into().unwrap()) as usize;

    // The string data starts at byte 64
    let string_data = &raw[64..64 + str_len];

    // Convert to UTF-8 string
    let decoded = str::from_utf8(string_data)?.to_string();
    Ok(decoded)
}


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
            // extract address and amount
            let topics = log.topics();
            let raw_topic = topics.get(1).expect("Expected at least 2 topics");

            // topic is 32 bytes, we need the last 20 bytes
            let raw_bytes: &[u8] = raw_topic.as_ref();
            let address_bytes: [u8; 20] = raw_bytes[12..].try_into().expect("Expected 20-byte slice");

            let sender = Address::from(address_bytes);

            println!("Sender address: {:?}", sender);
            
            let result = usize::from_str_radix("2A2F", 16);


            let raw_data = log.data().data.clone();

            println!("{:?}",raw_data);

            let string_from_hex = decode_string_from_hex(&raw_data)?;
            println!("{:?}",string_from_hex);
            let my_int = string_from_hex.parse::<i32>().unwrap();
            println!("{:?}",my_int);

            // success
        }

        let two_sec = time::Duration::from_millis(2000);
        thread::sleep(two_sec);


    }
}


// todo : build contracts with forge using cli (needed?)
