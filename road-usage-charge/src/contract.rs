#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point, Binary, StdResult, to_json_binary,
};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::types::RoadUsageCharge;
use crate::msg::{RoadUsageResponse, InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:road-usage-contract";
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
        charging: msg.charging
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
        ExecuteMsg::RoadUsageCharge { charging } => Ok(execute::road_usage(deps, env, info, charging)?),
    }
}

pub mod execute {
    use super::*;

    // Implement the proper helper functions match with your needs
    // This is just a placeholder, replace it with your actual implementation

    pub fn road_usage(deps: DepsMut, env: Env, info: MessageInfo, charging: RoadUsageCharge) -> Result<Response, ContractError> {

        // Implement the smart contract with replacing your logic
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            if meets_condition(&env, &info, &charging) {

                state.charging = charging;
                Ok(state)
                
            } else {
                Err(ContractError::NoValidParameters {})
            }
        })?;

        Ok(Response::new().add_attribute("action", "road_usage"))
    }

    pub fn meets_condition(env: &Env, _info: &MessageInfo, charging: &RoadUsageCharge) -> bool {

        // Implement your logic for checking conditions
        // This can involve checking sender, time, or any other parameters
        // For simplicity, let's assume the condition is always met

        if charging.parameters.price == 0.0 && charging.parameters.price < 0.0 {
            return false
        }

        if charging.parameters.avrage_speed == 0 && charging.parameters.avrage_speed < 0 {
            return false
        }

        if charging.parameters.distance_traveled == 0 && charging.parameters.distance_traveled < 0 {
            return false
        }

        return true;
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    // Implement your query logic here
    // This is just a placeholder, replace it with your actual implementation

    match msg {

        // Your custom query logic goes here
        QueryMsg::GetRoadUsageData {} => to_json_binary(&query::get_road_usage_calculation(deps,)?),
    }
}

pub mod query {

    use super::*;

    pub fn get_road_usage_calculation(deps: Deps) -> StdResult<RoadUsageResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(RoadUsageResponse { calculated: validate_data(state.charging).calculated })
    }

    fn validate_data(charging: RoadUsageCharge) -> RoadUsageResponse {

        let result: bool;

        // Check if the "Data" is shared and not invalid
        // Replace it with your actual implementation
        if charging.data.data_info.data_details.vehicle_info.temp == 0 || charging.data.pubkey == "" || charging.data.sign == "" {
            result =  false
        } else {

            // Data produced successfully
            result = true
        }
    
        RoadUsageResponse {
            calculated: result,
        }
    }
 }



