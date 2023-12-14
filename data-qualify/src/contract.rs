#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point, Binary, StdResult, to_json_binary,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::types::Data;
use crate::msg::{QualifiedDataResponse, DataQualificationResultResponse, InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:data-quality-contract";
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
        ExecuteMsg::QualifyData { data } => execute::execute_data_qualification(deps, info, data),
    }
}

pub mod execute {
    use super::*;

    pub fn execute_data_qualification(deps: DepsMut, info: MessageInfo, data: Data) -> Result<Response, ContractError> {

        // Implement the smart contract with replacing your logic
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            if data.data_info.data_details.vehicle_info.temp == 0 || data.pubkey == "" || data.sign == "" {
                return Err(ContractError::NotQualified {});
            } 

            state.json_data = data;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "data_qualification"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    // Implement your query logic here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom query logic goes here
        QueryMsg::GetDataQualificationResult {} => to_json_binary(&query::get_qualification_result(deps,)?),
        QueryMsg::GetQualifiedData {} => to_json_binary(&query::get_qualified_data(deps,)?),
    }
}

pub mod query {

    use super::*;

    pub fn get_qualification_result(deps: Deps) -> StdResult<DataQualificationResultResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(DataQualificationResultResponse { qualified: qualification_result(state.json_data).qualified })
    }

    pub fn get_qualified_data(deps: Deps) -> StdResult<QualifiedDataResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(QualifiedDataResponse { data: state.json_data })
    }

    fn qualification_result(data: Data) -> DataQualificationResultResponse {

        let result: bool;

        // Check if the "Data" is qualified and not invalid
        // Replace it with your actual implementation
        if data.data_info.data_details.vehicle_info.temp == 0 || data.pubkey == "" || data.sign == "" {
            result =  false
        } else {

            // Data produced successfully
            result = true
        }
    
        DataQualificationResultResponse {
            qualified: result,
        }
        
    }

 }



