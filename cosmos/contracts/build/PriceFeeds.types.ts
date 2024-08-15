export interface InstantiateMsg {
  denom: string;
}
export type ExecuteMsg = {
  publish_price: {
    price: Price;
    symbol: string;
  };
} | {
  update_price: {
    price: Price;
    symbol: string;
  };
} | {
  request_price_feed: {
    symbol: string;
  };
} | {
  request_price_feeds: {
    request: PriceFeedReq;
  };
} | {
  receive_prices: {
    prices_response: PriceFeedsResponse;
  };
} | {
  receive_price: {
    price_response: PriceFeedResponse;
  };
} | {
  set_cost_per_request: {
    cost_per_request: Uint128;
  };
} | {
  change_admin: {
    address: string;
  };
};
export type Decimal256 = string;
export type Uint128 = string;
export interface Price {
  price: Decimal256;
}
export interface PriceFeedReq {
  pairs: string[];
}
export interface PriceFeedsResponse {
  price_feeds: PriceFeedResponse[];
}
export interface PriceFeedResponse {
  price: Decimal256;
  symbol: string;
}
export type QueryMsg = {
  get_all_symbols: {};
};
export type ArrayOfString = string[];