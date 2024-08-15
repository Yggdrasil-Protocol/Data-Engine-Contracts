use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal256, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Price {
    pub price: Decimal256,
}

pub const PRICES: Map<&str, Price> = Map::new("prices");
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const DENOM : Item<String> = Item::new("denom");
pub const REQUEST_FEES : Item<Uint128> = Item::new("request_fees");
