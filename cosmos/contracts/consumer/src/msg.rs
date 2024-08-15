use cosmwasm_schema::{cw_serde, QueryResponses} ; 
use price_feeds::msg::{PriceFeedResponse, PriceFeedsResponse};


#[cw_serde]
pub struct InstantiateMsg {
    pub price_feed_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    RequestSinglePrice { pair : String },
    RequestMultiplePrices { pairs : Vec<String> },
    ReceivePrice { price_response: PriceFeedResponse },
    ReceivePrices { prices_response: PriceFeedsResponse },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceFeedResponse)]
    GetValue { pair : String },
}

#[cw_serde]
pub struct RequestPriceFeed {
    pub symbol: String,
}

pub struct PriceFeedReq  {
    pub pairs  : Vec<String>
}