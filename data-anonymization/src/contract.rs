#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point, Binary, StdResult, to_json_binary,
};
use cw2::set_contract_version;
use serde_json::json;
use crate::error::ContractError;
use crate::types::Data;
use crate::msg::{DataAnonymizationResponse, InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:data-anonymization-contract";
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    // Implement your logic to handle different execute messages here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom logic goes here
        ExecuteMsg::AnonymizeData { data } => execute::execute_data_anonymization(deps, info, data),
    }
}

pub mod execute {
    use super::*;

    pub fn execute_data_anonymization(deps: DepsMut, info: MessageInfo, data: Data) -> Result<Response, ContractError> {

        // Implement the smart contract with replacing your logic
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            Implement your algorithm to handle encreapt/decreapt here
            This is just a placeholder, replace it with your actual implementation

            let serialized = serde_json::to_string(&data).expect("Serialization failed");
            let payload = serialized.as_bytes();
        
            // Sample password used for encryption and decryption
            let password = b"your password";
        
            // Encrypt the payload using the password
            let encrypted = simplestcrypt::encrypt_and_serialize(&password[..], &payload).unwrap();
        
            // Decrypt the encrypted data using the password
            let decreapted = simplestcrypt::deserialize_and_decrypt(&password[..], &encrypted).unwrap();
        
            if payload != decreapted {
                return Err(ContractError::AnonymizedFailed {});
            }

            let decreapted_data: Data = serde_json::from_slice(&payload).unwrap();

            state.json_data = data;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "data_anonymization"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    // Implement your query logic here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom query logic goes here
        QueryMsg::GetAnonymizedData {} => to_json_binary(&query::get_anonymized_data(deps,)?),
    }
}

pub mod query {

    use super::*;

    pub fn get_anonymized_data(deps: Deps) -> StdResult<DataAnonymizationResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(DataAnonymizationResponse { qualified: validate_data(state.json_data).qualified })
    }

    fn validate_data(data: Data) -> DataAnonymizationResponse {

        let result: bool;

        // Replace it with your actual implementation
        if  json!(data).is_null(){
            result =  false
        } else {

            // Data produced successfully
            result = true
        }
    
        DataAnonymizationResponse {
            qualified: result,
        }
    }
 }



