use cosmwasm_schema::{cw_serde, QueryResponses};
use crate::types::Data;

#[cw_serde]
pub struct InstantiateMsg {
    pub data: Data,
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(DataQualificationResponse)]
    DataQualificationAssurance { data: Data },
    
}

// We define a custom struct for each query response
#[cw_serde]
pub struct DataQualificationResponse {
    pub qualified: bool,
}
