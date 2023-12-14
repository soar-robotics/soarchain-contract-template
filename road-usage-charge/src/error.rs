use cosmwasm_std::StdError;
use thiserror::Error;

#[derive( Error, Debug, PartialEq)]
pub enum ContractError {
    // Add more errors to check your logic

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Data not founs")]
    NoFunds {},

    #[error("Parameters not valid. Price should > 0.0 & avrage_speed > 0, distance_traveled > 0 is based on (km) .")]
    NoValidParameters {},
}
