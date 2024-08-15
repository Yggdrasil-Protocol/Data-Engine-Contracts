use crate::state::Price;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal256, Uint128};



#[cw_serde]
pub struct InstantiateMsg {
    pub denom: String,
}
#[cw_serde]
pub struct PriceFeedReq  {
    pub pairs  : Vec<String>
}

#[cw_serde]
pub enum ExecMsg {
    UpdatePrice { symbol: String, price: Price },
    RequestPriceFeed { symbol: String },
    RequestPriceFeeds { request: PriceFeedReq },
    ReceivePrices { prices_response : PriceFeedsResponse },
    ReceivePrice { price_response : PriceFeedResponse },
    SetCostPerRequest { cost_per_request: Uint128 },
    ChangeAdmin { address: String },
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<String>)]
    GetAllSymbols {},
}


#[cw_serde]
pub struct PriceFeedResponse {
    pub symbol: String,
    pub price: Decimal256,
}

/// Structure for multiple price feeds response
#[cw_serde]
pub struct PriceFeedsResponse {
    pub price_feeds: Vec<PriceFeedResponse>,
}

#[cw_serde]
pub enum MigrateMsg {}