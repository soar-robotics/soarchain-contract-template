use cosmwasm_schema::write_api;
pub use sharing::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
