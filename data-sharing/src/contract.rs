#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point, Binary, StdResult, to_json_binary,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::types::SharedData;
use crate::msg::{DataSharingResultResponse, SharedDataResponse, InstantiateMsg, ExecuteMsg, QueryMsg};
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
        sharing: msg.sharing
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
        ExecuteMsg::ShareData { sharing } => Ok(execute::share_data(deps, env, info, sharing)?),
    }
}

pub mod execute {
    use super::*;

    // Implement the proper helper functions match with your needs
    // This is just a placeholder, replace it with your actual implementation

    pub fn share_data(deps: DepsMut, env: Env, info: MessageInfo, sharing: SharedData) -> Result<Response, ContractError> {

        // Implement the smart contract with replacing your logic
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            if meets_condition(&env, &info, &sharing) {

                state.sharing = sharing;
                Ok(state)
                
            } else {
                Err(ContractError::Unauthorized {})
            }
        })?;

        Ok(Response::new().add_attribute("action", "data_sharing"))
    }

    pub fn meets_condition(env: &Env, _info: &MessageInfo, sharing: &SharedData) -> bool {

        // Implement your logic for checking conditions
        // This can involve checking sender, time, or any other parameters
        // For simplicity, let's assume the condition is always met

        let mut data_size: u64 = 0;
       
        if (sharing.condition.min_data_size as u64) > 0 {
            // Convert the string to a string using the Display trait
            let string_representation = format!("{:?}", sharing.data);
            // Convert usize to u64
            data_size = string_representation.as_bytes().len() as u64;
        }


        // Example condition check replace with your desired conditions
        if data_size > sharing.condition.min_data_size || env.block.chain_id == sharing.condition.chain_id || env.block.height > sharing.condition.min_height || is_after_timestamp(sharing.condition.min_time, env) || is_pubkey_valid(&sharing) {
            return true;
        } else {
            return false;
        }
    }

    // Example function to check if the current block time is after a specific timestamp
    pub fn is_after_timestamp(min_timestamp: u64, env: &Env) -> bool {

        // Example timestamp (replace with your desired timestamp)
        //target_timestamp;

        // Check if the current block time is after the target timestamp
        if env.block.time.seconds() > min_timestamp {
            return true
        } 
        return false;
    }

    // Example function to check if the pubkey is vali
    pub fn is_pubkey_valid(sharing: &SharedData) -> bool {

        // Parse the public key bytes into a bytes
        // Cosmos SDK public key bytes should be 144
        let pubkey_size = sharing.data.pubkey.as_bytes().len();

        if pubkey_size == 144 {
            return true
        } 
        return false;

        // Optionally, you can further validate the public key if needed
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    // Implement your query logic here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom query logic goes here
        QueryMsg::GetDataSharingResult {} => to_json_binary(&query::get_share_data_result(deps,)?),
        QueryMsg::GetSharedData {} => to_json_binary(&query::get_shared_data(deps,)?),
    }
}

pub mod query {

    use super::*;

    pub fn get_share_data_result(deps: Deps) -> StdResult<DataSharingResultResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(DataSharingResultResponse { shared: shared_data_result(state.sharing).shared })
    }

    fn shared_data_result(sharing: SharedData) -> DataSharingResultResponse {

        let result: bool;

        // Check if the "Data" is shared and not invalid
        // Replace it with your actual implementation
        if sharing.data.data_info.data_details.vehicle_info.temp == 0 || sharing.data.pubkey == "" || sharing.data.sign == "" {
            result =  false
        } else {

            // Data produced successfully
            result = true
        }
    
        DataSharingResultResponse {
            shared: result,
        }
    }

    pub fn get_shared_data(deps: Deps) -> StdResult<SharedDataResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(SharedDataResponse { data: state.sharing })
    }

 }



