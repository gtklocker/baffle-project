# Baffle
Test your Solidity code faster than you can say truffle oil.

## How
* You should run your own node: `docker run --rm -p 8545:8545 trufflesuite/ganache-cli`
* You should have `baffle` installed: `cargo install baffle`
* Your smart contracts should all live in `contracts/*.sol`.
* Build them with `baffle compile`.
* Your tests should live in `src/lib.rs`.
* Run them with `cargo test`.

Enjoy!
