pub mod contract;
pub mod msg;
pub mod state;
pub mod error;


pub use crate::msg::ExecMsg::{RequestPriceFeed , RequestPriceFeeds}  ;