use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg};
use crate::msg::{PriceFeedReq, PriceFeedResponse, PriceFeedsResponse};
use crate::state::{Price, ADMIN, DENOM, PRICES, REQUEST_FEES};
use cosmwasm_std::{
    coins, entry_point, to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply,
    ReplyOn, Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};

use crate::error::ContractError::{
    InsufficientFees, InvalidExecuteMsg, PriceDoesNotExist, PriceFeedExists, Unauthorized,
};

use cw2::set_contract_version;
use cw2::get_contract_version;

use crate::msg::MigrateMsg ; 
use semver::Version;

const CONTRACT_NAME: &str = "crates.io:Yggdrasil-Data-Feeds";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


//helper functions

fn check_admin(deps: &DepsMut, info: MessageInfo) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Ok(false);
    }
    Ok(true)
}

fn get_request_fees(deps: &DepsMut) -> Result<Uint128, ContractError> {
    let fees = REQUEST_FEES.load(deps.storage)?;
    Ok(fees)
}

// Entry point for contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Save the contract admin (sender of the instantiate message)
    let admin = info.sender.to_string();
    ADMIN.save(deps.storage, &info.sender)?;

    // Save the denomination for price feeds
    DENOM.save(deps.storage, &msg.denom)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", admin))
}

// Entry point for executing contract functions
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecMsg::UpdatePrice { symbol, price } => try_update_price(deps, info, symbol, price),
        ExecMsg::RequestPriceFeed { symbol } => try_request_price(deps, info, symbol),
        ExecMsg::RequestPriceFeeds { request } => try_request_price_feeds(deps, info, request),
        ExecMsg::ChangeAdmin { address } => try_change_admin(deps, info, address),
        ExecMsg::SetCostPerRequest { cost_per_request } => {
            execute_set_cost(deps, info, cost_per_request)
        }
        _ => Err(InvalidExecuteMsg {}),
    }
}

fn execute_set_cost(
    deps: DepsMut,
    info: MessageInfo,
    cost_per_request: Uint128,
) -> Result<Response, ContractError> {
    if check_admin(&deps, info).unwrap() {
        REQUEST_FEES.save(deps.storage, &cost_per_request)?;
        Ok(Response::new().add_attribute("action", "set_cost_per_request"))
    } else {
        return Err(Unauthorized {
            function: "set_cost_per_request".to_string(),
        });
    }
}

fn try_change_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    // Check if the sender is the admin
    if check_admin(&deps, info).unwrap() {
        let admin = deps.api.addr_validate(&address)?;
        ADMIN.save(deps.storage, &admin)?;
        Ok(Response::new()
            .add_attribute("action", "change_admin")
            .add_attribute("admin", address))
    } else {
        return Err(Unauthorized {
            function: "change_admin".to_string(),
        });
    }
}


// Function to update an existing price feed
fn try_update_price(
    deps: DepsMut,
    info: MessageInfo,
    symbol: String,
    price: Price,
) -> Result<Response, ContractError> {
    // Check if the sender is the admin
    if check_admin(&deps, info).unwrap() {
        PRICES.update(
            deps.storage,
            &symbol,
            |old| -> Result<Price, ContractError> {
                match old {
                    Some(_) => Ok(price.clone()),
                    None => Ok(price.clone())
                }
            },
        )?;

        Ok(Response::new()
            .add_attribute("action", "update_price")
            .add_attribute("symbol", symbol))
    } else {
        return Err(Unauthorized {
            function: "update_price".to_string(),
        });
    }

    // Update the price feed if it exists
}

// Function to request a single price feed
pub fn try_request_price(
    deps: DepsMut,
    info: MessageInfo,
    symbol: String,
) -> Result<Response, ContractError> {
    let price = PRICES.load(deps.storage, &symbol)?;
    let denom = DENOM.load(deps.storage)?;

    let cost = get_request_fees(&deps).unwrap();
    // Check if sufficient fees are sent
    let sent_funds = info.funds.iter().find(|c| c.denom == denom);

    match sent_funds {
        Some(coin) if coin.amount >= cost => {
            // Create a bank message to send fees to admin
            let bank_msg = BankMsg::Send {
                to_address: ADMIN.load(deps.storage)?.into_string(),
                amount: coins(coin.amount.u128(), &coin.denom),
            };

            // Prepare price response
            let price_response = PriceFeedResponse {
                symbol,
                price: price.price,
            };

            // Create a callback message to send price to requester
            let callback_msg = SubMsg {
                msg: WasmMsg::Execute {
                    contract_addr: info.sender.clone().to_string(),
                    msg: to_json_binary(&ExecMsg::ReceivePrice { price_response })?,
                    funds: vec![],
                }
                .into(),
                gas_limit: None,
                id: 1,
                reply_on: ReplyOn::Success,
                payload: Binary::default(),
            };

            Ok(Response::new()
                .add_message(bank_msg)
                .add_submessage(callback_msg))
        }
        _ => Err(InsufficientFees {
            fees: info.funds[0].amount.u128(),
        }),
    }
}


// Function to request multiple price feeds
pub fn try_request_price_feeds(
    deps: DepsMut,
    info: MessageInfo,
    req: PriceFeedReq,
) -> Result<Response, ContractError> {
    let pairs = req.pairs;
    let denom = DENOM.load(deps.storage)?;
    let cost = get_request_fees(&deps).unwrap() * Uint128::new(pairs.len() as u128);
    // Collect prices for all requested symbols
    let prices: StdResult<Vec<PriceFeedResponse>> = pairs
        .iter()
        .map(|pair| {
            if !PRICES.has(deps.storage, &pair) {
                Ok(PriceFeedResponse {
                    symbol: pair.clone(),
                    price: Default::default(), // if the price feed does not exist , it will be returned as zero
                })
            } else {
                let price = PRICES.load(deps.storage, pair)?;
                Ok(PriceFeedResponse {
                    symbol: pair.clone(),
                    price: price.price,
                })
            }
        })
        .collect();

    let prices = prices?;

    // Check if sufficient fees are sent
    let sent_funds = info.funds.iter().find(|c| c.denom == denom);

    match sent_funds {
        Some(coin) if coin.amount >= cost => {
            // Create a bank message to send fees to admin
            let bank_msg = BankMsg::Send {
                to_address: ADMIN.load(deps.storage)?.into_string(),
                amount: coins(coin.amount.u128(), &coin.denom),
            };

            // Prepare prices response
            let prices_response = PriceFeedsResponse {
                price_feeds: prices,
            };

            // Create a callback message to send prices to requester
            let callback_msg = SubMsg {
                msg: WasmMsg::Execute {
                    contract_addr: info.sender.clone().to_string(),
                    msg: to_json_binary(&ExecMsg::ReceivePrices { prices_response })?,
                    funds: vec![],
                }
                .into(),
                gas_limit: None,
                id: 2,
                reply_on: ReplyOn::Success,
                payload: Binary::default(),
            };

            Ok(Response::new()
                .add_message(bank_msg)
                .add_submessage(callback_msg))
        }
        _ => Err(InsufficientFees {
            fees: info.funds[0].amount.u128(),
        }),
    }
}

// Entry point for querying contract state
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAllSymbols {} => to_json_binary(&query_all_symbols(deps)?),
    }
}

// Function to query all available symbols
fn query_all_symbols(deps: Deps) -> StdResult<Vec<String>> {
    PRICES
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect()
}


// Entry point for handling replies from submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        1 => Ok(Response::new().add_attribute("action", "price_feed_reply")),
        2 => Ok(Response::new().add_attribute("action", "price_feeds_reply")),
        _ => Err(StdError::generic_err("Unknown reply ID")),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let version: Version = CONTRACT_VERSION.parse().unwrap();
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse().unwrap();
    if storage_version < version {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        // If state structure changed in any contract version in the way migration is needed, it
        // should occur here
    }
    Ok(Response::default())
}


