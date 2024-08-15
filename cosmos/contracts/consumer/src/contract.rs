#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, WasmMsg};
// use price_feeds::ReceivePrice ;
use cosmwasm_std::Uint128;
use crate::state::{PRICE_FEED_CONTRACT ,PRICE_FEEDS ,PriceFeed } ; 


// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};


use price_feeds::{RequestPriceFeed , RequestPriceFeeds}; 
use price_feeds::msg::{PriceFeedResponse, PriceFeedsResponse , PriceFeedReq};
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:consumer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    PRICE_FEED_CONTRACT.save(deps.storage, &msg.price_feed_contract)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RequestSinglePrice { pair } => execute_request_single_price(deps, info, pair),
        ExecuteMsg::RequestMultiplePrices { pairs } => execute_request_multiple_prices(deps, info , pairs),
        ExecuteMsg::ReceivePrice { price_response } => execute_receive_price(deps, env, info, price_response),
        ExecuteMsg::ReceivePrices { prices_response } => execute_receive_prices(deps, env, info, prices_response),
    }
}

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

pub fn execute_request_multiple_prices(
    deps: DepsMut,
    _info: MessageInfo,
    pairs: Vec<String>,
) -> Result<Response, ContractError> {
    let price_feed_contract = PRICE_FEED_CONTRACT.load(deps.storage)?;
   let lenght =  pairs.clone().len();
    let request = PriceFeedReq { pairs };
    let request_msg = WasmMsg::Execute {
        contract_addr: deps.api.addr_validate(&price_feed_contract)?.to_string(),
        msg: to_json_binary(&RequestPriceFeeds { request })?,
        funds: vec![Coin{
            denom: "uom".to_string(),
            amount: Uint128::new(1000) * Uint128::new(lenght as u128),
        }],
    };

    Ok(Response::new()
        .add_message(request_msg)
        .add_attribute("action", "request_multiple_prices")
    )
}

pub fn execute_receive_price(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    price_response: PriceFeedResponse,
) -> Result<Response, ContractError> {
    

    let price_feed = PriceFeed {
        price: price_response.price
    };

    PRICE_FEEDS.save(deps.storage,price_response.symbol,  &price_feed)?;

    Ok(Response::new()
        .add_attribute("action", "receive_price")
        .add_attribute("price", price_response.price.to_string()))
}

pub fn execute_receive_prices(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    prices_response: PriceFeedsResponse,
) -> Result<Response, ContractError> {
   
    for price_feed in prices_response.price_feeds.iter() {
        let price_feed_data = PriceFeed {
            price: price_feed.price
        };
        PRICE_FEEDS.save(deps.storage ,price_feed.symbol.clone(), &price_feed_data )?;
    }

    Ok(Response::new()
        .add_attribute("action", "receive_prices")
        .add_attribute("count", prices_response.price_feeds.len().to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetValue { pair  } => to_json_binary(&query_price(deps , pair)?),
    }
}


pub fn query_price(deps: Deps , pair : String) -> StdResult<PriceFeed> {
    PRICE_FEEDS.load(deps.storage , pair)
}



