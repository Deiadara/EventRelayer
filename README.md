Steps to run:

1) Run the first local anvil instance on default port 8545 by running : anvil
2) Run the second local anvil instance by running : anvil --port 8546
3) Deploy the contracts Deposit.sol and Token.sol by navigating to project_eth and running : node ./script/deploy.js
4) Start the relayer instance by navigating to the relayer directory, running cargo build and then cargo run
5) Trigger the event emission by navigating to project_eth and running : node ./script/trigger.js
