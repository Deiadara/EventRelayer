use std::{thread, time};
use alloy::{
    primitives::{B256,Address,keccak256},
    providers::{Provider, ProviderBuilder},
    rpc::types::{Filter,eth::BlockNumberOrTag},
};
use eyre::Result;


const SRC_RPC: &str = "http://localhost:8545";
const DST_RPC: &str = "http://localhost:8546";

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let rpc_url = SRC_RPC.parse()?;
        let provider = ProviderBuilder::new().on_http(rpc_url);
        let latest_block = provider.get_block_number().await?;

        let event_sig = keccak256("Deposited(address,string)");

        let contract_address: Address = "0x5FbDB2315678afecb367f032d93F642f64180aa3".parse()?;

        let filter = Filter::new().address(contract_address).from_block(BlockNumberOrTag::Earliest)
        .to_block(BlockNumberOrTag::Latest);

        //.from_block(latest_block) to not check the entire blocckchain

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



// todo : alloy docs (alloy contract), ethers is deprecated
// todo : build contracts with forge using cli
// todo : deployer imports ABI and Bytecode from a file
// todo : look up how to filter events from contracts (topics/events), check examples in discords Themis DMs
// todo : setup github for the project and add Themis