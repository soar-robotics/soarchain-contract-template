use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::types::RoadUsageCharge;

#[cw_serde]
pub struct InstantiateMsg {
    // You can initialize any state or perform setup logic here
    pub charging: RoadUsageCharge,
}

// Define your custom struct for messages
#[cw_serde]
pub enum ExecuteMsg {
    RoadUsageCharge { 
        /* define parameters */  
        charging: RoadUsageCharge 
    }, 
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(RoadUsageResultResponse)]
    GetRoadUsageResultData { 
        /* define parameters */
    },

    #[returns(RoadUsageDataResponse)]
    GetRoadUsageData { 
        /* define parameters */
    },

    #[returns(RoadUsageChargeResponse)]
    GetRoadUsageCharge { 
        /* define parameters */
    },
}

// Define a custom struct for each query response
#[cw_serde]
pub struct RoadUsageResultResponse {
    /* define parameters */
    pub calculated: bool,
}

#[cw_serde]
pub struct RoadUsageDataResponse {

    /* define parameters */

    pub data: RoadUsageCharge,
}

#[cw_serde]
pub struct RoadUsageChargeResponse {

    /* define parameters */

    pub charge: u64,
}
