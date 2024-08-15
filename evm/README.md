## Foundry

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

### Build

```shell
$ forge build
```

### Test

```shell
$ forge test
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```


contracts/Evm/build

abigen --bin ../../../contracts/Evm/build/PriceFeed.bin --abi ../../../contracts/Evm/build/PriceFeed.abi --pkg PriceFeedContract --type PriceFeedContract --out ./PriceFeedContract/PriceFeedContract.go

 PriceFeed implementation deployed at: 0x5DF7b6f839B48c2D8fEa294DD9Eea00dcBe970BE
  PriceFeed proxy deployed at: 0x8e249327FCa2745324577e6203Ee4322c435EBF6