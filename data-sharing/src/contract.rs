#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point, Binary, StdResult, to_json_binary,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::types::Data;
use crate::msg::{DataSharingResponse, InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:data-sharing-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // You can initialize any state or perform setup logic here

    let state = State {
        owner: info.sender.to_string(),
        json_data: msg.data
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    // Implement your logic to handle different execute messages here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom logic goes here
        ExecuteMsg::ShareData { data } => Ok(execute::share_data(deps, env, info, data)?),
    }
}

pub mod execute {
    use super::*;

    // Implement the proper helper functions match with your needs
    // This is just a placeholder, replace it with your actual implementation

    pub fn share_data(deps: DepsMut, env: Env, info: MessageInfo, data: Data) -> Result<Response, ContractError> {

        // Implement the smart contract with replacing your logic
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            if meets_condition(&env, &info, &data) {

                state.json_data = data;
                Ok(state)
                
            } else {
                Err(ContractError::Unauthorized {})
            }
        })?;

        Ok(Response::new().add_attribute("action", "data_sharing"))
    }

    pub fn meets_condition(env: &Env, _info: &MessageInfo, data: &Data) -> bool {

        // Implement your logic for checking conditions
        // This can involve checking sender, time, or any other parameters
        // For simplicity, let's assume the condition is always met

        // Convert the string to a string using the Display trait
        let string_representation = format!("{:?}", data);
        let data_size = string_representation.as_bytes().len();

        // Example condition check replace with your desired conditions
        if data_size > 5 || env.block.chain_id == "soarchaindevnet" || env.block.height > 12 || is_after_timestamp(env) || is_pubkey_valid(data) {
            return true;
        } else {
            return false;
        }
    }

    // Example function to check if the current block time is after a specific timestamp
    pub fn is_after_timestamp(env: &Env) -> bool {

        // Example timestamp (replace with your desired timestamp)
        let target_timestamp = 60;

        // Check if the current block time is after the target timestamp
        if env.block.time.seconds() > target_timestamp {
            return true
        } 
        return false;
    }

    // Example function to check if the current block time is after a specific timestamp
    pub fn is_pubkey_valid(data: &Data) -> bool {

        // Example, replace with your desired timestamp
        let pubkey_size = data.pubkey.as_bytes().len();

        // Check if the current block time is after the target timestamp
        if pubkey_size == 144 {
            return true
        } 
        return false;
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    // Implement your query logic here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom query logic goes here
        QueryMsg::GetSharedData {} => to_json_binary(&query::get_share_data(deps,)?),
    }
}

pub mod query {

    use super::*;

    pub fn get_share_data(deps: Deps) -> StdResult<DataSharingResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(DataSharingResponse { shared: validate_data(state.json_data).shared })
    }

    fn validate_data(data: Data) -> DataSharingResponse {

        let result: bool;

        // Check if the "Data" is shared and not invalid
        // Replace it with your actual implementation
        if data.data_info.data_details.vehicle_info.temp == 0 || data.pubkey == "" || data.sign == "" {
            result =  false
        } else {

            // Data produced successfully
            result = true
        }
    
        DataSharingResponse {
            shared: result,
        }
    }
 }



