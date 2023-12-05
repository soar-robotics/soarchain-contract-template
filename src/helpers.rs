#[cfg(test)]
pub mod test {
    use std::marker::PhantomData;
    use crate::execute;
    use crate::error::ContractError;
    use std::fmt::Debug;
    use cosmwasm_std::testing::{
        MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::OwnedDeps;
    use schemars::JsonSchema;
    use serde::Deserialize;
    use serde::Serialize;


    pub fn our_mock_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: MockQuerier::default(),
            custom_query_type: PhantomData,
        }
    }


    pub fn setup() -> Result<OwnedDeps<MockStorage, MockApi, MockQuerier>, ContractError> {

        let deps = our_mock_dependencies();
        let operations = vec![
            execute::ConnectionInput {
                operation: execute::FullOperation::Set,
                source_chain: "soarchain".to_string(),
            },

        ];
        execute::connection_operations(operations)?;

        Ok(deps)
    }

    #[derive(Serialize, Deserialize, Clone, Eq, PartialEq, JsonSchema, Debug)]
    pub struct Client {
        pub address: String,
        pub index: String,
        pub score: String,
    }

    impl Client {
        pub fn new(address: String, index: String, score: String) -> Self {
            Client {
                address: address.to_string(),
                index: index.to_string(),
                score: score.to_string(),
            }
        }
    }

}
