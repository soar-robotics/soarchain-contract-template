use cosmwasm_schema::{cw_serde, QueryResponses};
// use crate::types::Data;

#[cw_serde]
pub struct InstantiateMsg {
    // You can initialize any state or perform setup logic here
    pub data: String,
}

// Define your custom struct for messages
#[cw_serde]
pub struct ExecuteMsg {
        /* define parameters */    
    pub data: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(DataAnonymizationResponse)]
    GetAnonymizedData { 
        /* define parameters */
    },
    
}

// Define a custom struct for each query response
#[cw_serde]
pub struct DataAnonymizationResponse {
    /* define parameters */
    pub anonymized: bool,
}
