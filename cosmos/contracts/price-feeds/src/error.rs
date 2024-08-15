use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {
        function : String
    },

    #[error("PriceDoesNotExist")]
    PriceDoesNotExist{
        symbol : String
    }, 

    #[error("InvalidExecuteMsg")]
    InvalidExecuteMsg{},

    #[error("PriceFeedExists")]
    PriceFeedExists{
        symbol : String
    },

    #[error("Insufficient Fees")]
    InsufficientFees{
        fees : u128
        
    },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
