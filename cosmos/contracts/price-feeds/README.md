# Cosmwasm Data Feed Integration Guide

## Introduction

This guide explains how to read data from Yggdrasil data feeds on Cosmos chains. It covers obtaining the data feed address, requesting data feeds, and handling responses using callback messages. Suitable for both beginners and experienced developers.

## Getting the Data Feed Address

Find the addresses and their respective contract addresses in the relevant section of the documentation.

## Using the Data Feed Crate in Your Smart Contracts

Add the following line of code to the Cargo.toml file of your contract:

```toml
price-feeds = "0.1.0"
```

## Requesting Data Feeds

### Requesting a Single Data Feed

Example code to request a single price feed:

```rust
pub fn execute_request_single_price(
    deps: DepsMut,
    _info: MessageInfo,
    pair: String,
) -> Result<Response, ContractError> {
    let price_feed_contract = PRICE_FEED_CONTRACT.load(deps.storage)?;
    
    let request_msg = WasmMsg::Execute {
        contract_addr: deps.api.addr_validate(&price_feed_contract)?.to_string(),
        msg: to_json_binary(&RequestPriceFeed { symbol: pair })?,
        funds: vec![Coin {
            denom: "uom".to_string(),
            amount: Uint128::new(1000),
        }],
    };

    Ok(Response::new()
        .add_message(request_msg)
        .add_attribute("action", "request_single_price"))
}
```

## Handling Price Feed Responses

### Handling a Single Price Feed Response

Example code to handle a single price feed response:

```rust 
ExecuteMsg::ReceivePrice { price_response } => execute_receive_price(deps, env, info, price_response),

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

## Requesting Data Feeds

### Requesting a Single Data Feed

Example code to request a single price feed:

```rust 
pub fn execute_request_single_price(
    deps: DepsMut,
    _info: MessageInfo,
    pair: String,
) -> Result<Response, ContractError> {
    let price_feed_contract = PRICE_FEED_CONTRACT.load(deps.storage)?;
    
    let request_msg = WasmMsg::Execute {
        contract_addr: deps.api.addr_validate(&price_feed_contract)?.to_string(),
        msg: to_json_binary(&RequestPriceFeed { symbol: pair })?,
        funds: vec![Coin {
            denom: "uom".to_string(),
            amount: Uint128::new(1000),
        }],
    };

    Ok(Response::new()
        .add_message(request_msg)
        .add_attribute("action", "request_single_price"))
}
```

## Handling Price Feed Responses

### Handling a Single Price Feed Response

Example code to handle a single price feed response:

```rust 
ExecuteMsg::ReceivePrice { price_response } => execute_receive_price(deps, env, info, price_response),

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

# Conclusion
```
By following the steps outlined above, you can efficiently request and handle data feeds in your smart contracts across all Cosmos chains. Use the provided data feed crate to avoid request failures and customize the callback logic to fit your specific needs. This will enable seamless integration of accurate and reliable price feeds into your decentralized applications, ensuring you have up-to-date data for your operations.
```
