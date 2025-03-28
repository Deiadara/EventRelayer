use std::{thread, time};
use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;


const SRC_RPC: &str = "http://localhost:8545";
const DST_RPC: &str = "http://localhost:8546";

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let src_provider = Provider::<Http>::try_from(SRC_RPC)?;
        let dst_provider = Provider::<Http>::try_from(DST_RPC)?;
        //let client = Arc::new(provider);

        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
    }
}



// todo : alloy docs (alloy contract), ethers is deprecated
// todo : build contracts with forge using cli
// todo : deployer imports ABI and Bytecode from a file
// todo : look up how to filter events from contracts (topics/events), check examples in discords Themis DMs
// todo : setup github for the project and add Themis