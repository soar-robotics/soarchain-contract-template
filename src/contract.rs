#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Response, entry_point,
};

use cw2::set_contract_version;
use crate::error::ContractError;
use crate::types::Data;
use crate::msg::{DataQualificationResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use cosmwasm_std::{
    to_json_binary, Binary, StdResult
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:data-qualify-assurance-demo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {

    match msg { QueryMsg::DataQualificationAssurance { data, } => to_json_binary(&data_qualification(deps, data)), }
}

fn data_qualification (_deps: Deps, data: Data) -> DataQualificationResponse {

    let result: bool;

    if data.data_info.data_details.vehicle_info.temp == 0 || data.pubkey == "" || data.sign == "" {
        result =  false
    } else {
        result = true
    }

    DataQualificationResponse {
        qualified: result,
    }
}