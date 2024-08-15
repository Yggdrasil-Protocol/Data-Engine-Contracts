use cw_storage_plus::{Item , Map};
use cosmwasm_std::Decimal256;
use cosmwasm_schema::cw_serde;


#[cw_serde]
pub struct PriceFeed {
    pub price: Decimal256
}

pub const PRICE_FEED_CONTRACT: Item<String> = Item::new("price_feed_contract");
pub const PRICE_FEEDS: Map<String , PriceFeed> = Map::new("price_feeds");