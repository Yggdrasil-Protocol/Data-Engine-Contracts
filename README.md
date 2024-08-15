# Yggdrasil Data Engine Contracts

This repository contains the smart contracts for interacting with Yggdrasil's data feed services on both EVM-compatible chains and Cosmos chains. These contracts allow you to retrieve real-time price data directly into your decentralized applications (dApps).

## Overview

Yggdrasil provides data feeds that can be integrated into your smart contracts to access real-time pricing information. This repository includes example contracts and implementation guidelines for both EVM and Cosmos ecosystems.

## Table of Contents

- [Yggdrasil Data Engine Contracts](#yggdrasil-data-engine-contracts)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [EVM-Compatible Chains](#evm-compatible-chains)
      - [Interface Definition](#interface-definition)
      - [Implementation Example](#implementation-example)
      - [Callback Function](#callback-function)
    - [Cosmos Chains](#cosmos-chains)
      - [Getting the Data Feed Address](#getting-the-data-feed-address)
      - [Using Data Feed Crate](#using-data-feed-crate)
      - [Requesting Data Feeds](#requesting-data-feeds)
      - [Handling the Price Feed Response](#handling-the-price-feed-response)
  - [License](#license)

## Getting Started

### EVM-Compatible Chains

To integrate Yggdrasil's data feeds into your Solidity smart contracts, you need to implement the `IDataFeed` interface.

#### Interface Definition

```solidity
interface IDataFeed {
    function requestPrices(
        bytes32[] calldata _assets,
        function(uint8[] memory, uint256[] memory) external _callback
    ) external payable;

    function feePerAsset() external view returns (uint256);
}
```
requestPrices: Requests the price data for the specified assets and handles the response through the provided callback function.
feePerAsset: Returns the fee required per asset for the request.

## Implementation Example
Here’s an example implementation that demonstrates how to interact with the IDataFeed interface:

```solidity
contract DataFeedInteractor  {
    address public constant PRICE_FEED_PROXY = 0x30B3731d5fE29E768Ab282dBF2c79D9A70776Ad0; // Data Feed Address on Arbitrum Sepolia
    IDataFeed public priceFeed;

    uint8[] public lastDecimals;
    uint256[] public lastPrices;

    event PricesReceived(uint8[] decimals, uint256[] prices);

    constructor() {
        priceFeed = IDataFeed(PRICE_FEED_PROXY);
    }

    function requestPrices(bytes32[] calldata _assets) external payable {
        uint256 totalFee = priceFeed.feePerAsset() * _assets.length;
        require(msg.value >= totalFee, "Insufficient fee");

        priceFeed.requestPrices{value: totalFee}(_assets, this.dataCallback);

        if (msg.value > totalFee) {
            payable(msg.sender).transfer(msg.value - totalFee);
        }
    }

    function dataCallback(uint8[] memory _decimals, uint256[] memory _prices) external {
        require(msg.sender == PRICE_FEED_PROXY, "Unauthorized callback");

        lastDecimals = _decimals;
        lastPrices = _prices;

        emit PricesReceived(_decimals, _prices);
    }

    function getLastPrices() external view returns (uint8[] memory, uint256[] memory) {
        return (lastDecimals, lastPrices);
    }

    receive() external payable {}
}
```

## Callback Function
The dataCallback function processes the response from the requestPrices function. It stores the received prices and decimals in state variables and emits an event for further use.

## Cosmos Chains
For Cosmos chains, the integration is done using Rust and the price-feeds crate.

## Getting the Data Feed Address
Before you can request price data, you'll need the appropriate data feed contract address for the Cosmos chain you are working with.

## Using Data Feed Crate
Add the following dependency to your Cargo.toml file to use the Yggdrasil data feeds in your contracts:
```toml
price-feeds = "0.1.1"
```
```rust
use price_feeds::{RequestPriceFeed}; 
use price_feeds::msg::{PriceFeedResponse};

pub fn execute_request_single_price(
    deps: DepsMut,
    _info: MessageInfo,
    pair : String,
) -> Result<Response, ContractError> {
    let price_feed_contract = PRICE_FEED_CONTRACT.load(deps.storage)?;
    
    let request_msg = WasmMsg::Execute {
        contract_addr: deps.api.addr_validate(&price_feed_contract)?.to_string(),
        msg: to_json_binary(&RequestPriceFeed { symbol: pair})?,
        funds: vec![Coin{
            denom : "uom".to_string(), 
            amount: Uint128::new(1000)
        }],
    };

    Ok(Response::new()
        .add_message(request_msg)
        .add_attribute("action", "request_single_price")
    )
}
```
```rust
pub fn execute_receive_price(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    price_response: PriceFeedResponse,
) -> Result<Response, ContractError> {
    let price_feed = PriceFeed {
        price: price_response.price,
    };

    PRICE_FEEDS.save(deps.storage, price_response.symbol, &price_feed)?;

    Ok(Response::new()
        .add_attribute("action", "receive_price")
        .add_attribute("price", price_response.price.to_string()))
}
```
## License
This project is licensed under the MIT License - see the LICENSE file for details.

```
You can copy and paste this Markdown code directly into your `README.md` file in your GitHub repository.
```

