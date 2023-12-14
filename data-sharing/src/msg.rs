use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::types::SharedData;

#[cw_serde]
pub struct InstantiateMsg {
    // You can initialize any state or perform setup logic here
    pub sharing: SharedData,
}

// Define your custom struct for messages
#[cw_serde]
pub enum ExecuteMsg {
    ShareData { 
        /* define parameters */  
        sharing: SharedData 
    }, 
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(DataSharingResultResponse)]
    GetDataSharingResult { 
        /* define parameters */
    },

    #[returns(SharedDataResponse)]
    GetSharedData { 
        /* define parameters */
    },
}

// Define a custom struct for each query response
#[cw_serde]
pub struct DataSharingResultResponse {
    /* define parameters */
    pub shared: bool,
}

#[cw_serde]
pub struct SharedDataResponse {

    /* define parameters */

    pub data: SharedData,
}
