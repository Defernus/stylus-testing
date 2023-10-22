use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use ethers::{providers::Provider, types::Address};
use ethers_providers::{JsonRpcClient, JsonRpcError, ProviderError, RpcError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error as ThisError;

use crate::contract::Contract;

pub type TestProvider = Provider<TestInnerProvider>;

#[derive(Debug, Clone)]
pub struct TestInnerProvider {
    contract: Arc<Mutex<Contract>>,
}

impl TestInnerProvider {
    pub fn new(contract: Arc<Mutex<Contract>>) -> Self {
        Self { contract }
    }
}

#[derive(ThisError, Debug)]
/// Error thrown when sending an HTTP request
pub enum ClientError {
    #[error("error: {0}")]
    ClientError(String),

    #[error(transparent)]
    /// Thrown if the response could not be parsed
    JsonRpcError(#[from] JsonRpcError),

    #[error("Deserialization Error: {err}. Response: {text}")]
    /// Serde JSON Error
    SerdeJson {
        /// Underlying error
        err: serde_json::Error,
        /// The contents of the HTTP response that could not be deserialized
        text: String,
    },
}

impl From<ClientError> for ProviderError {
    fn from(src: ClientError) -> Self {
        match src {
            ClientError::ClientError(err) => ProviderError::CustomError(err),
            _ => ProviderError::JsonRpcClientError(Box::new(src)),
        }
    }
}

impl RpcError for ClientError {
    fn as_error_response(&self) -> Option<&JsonRpcError> {
        if let ClientError::JsonRpcError(err) = self {
            Some(err)
        } else {
            None
        }
    }

    fn as_serde_error(&self) -> Option<&serde_json::Error> {
        match self {
            ClientError::SerdeJson { err, .. } => Some(err),
            _ => None,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl JsonRpcClient for TestInnerProvider {
    type Error = ClientError;

    async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        method: &str,
        params: T,
    ) -> Result<R, ClientError> {
        println!("method: {}", method);

        match method {
            "eth_call" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let params = serde_json::from_str::<(RequestParams, String)>(&params).unwrap();

                let data = hex::decode(&params.0.data[2..]).unwrap();

                let mut contract = self.contract.lock().unwrap();

                let res = contract.entry_point(&data).unwrap();

                println!("res: {:?}", res);

                let v = serde_json::from_str("\"\"").unwrap();

                return Ok(v);
            }
            method => unimplemented!("Method \"{method}\""),
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestParams {
    // [{"accessList":[],"data":"...","from":"0x3cc222811060826c2699ea8ff9b35fdb0963d6ad","to":"0x00000000000000000000000000000000000004d2","type":"0x02"},"latest"]
    #[serde(rename = "accessList")]
    access_list: Vec<()>,
    data: String,
    from: Address,
    to: Address,
    #[serde(rename = "type")]
    tx_type: String,
}
