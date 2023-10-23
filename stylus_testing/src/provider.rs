use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use ethers::{
    middleware::SignerMiddleware,
    providers::Provider,
    signers::LocalWallet,
    types::{Address, Bytes, U256, U64},
};
use ethers_providers::{JsonRpcClient, JsonRpcError, Middleware, ProviderError, RpcError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error as ThisError;

use crate::{
    constants::CHAIN_UD,
    contract::{ContractCall, ContractCallError, ContractState},
};

pub type TestOuterProvider = Provider<TestInnerProvider>;
pub type TestClient = SignerMiddleware<TestOuterProvider, LocalWallet>;

#[derive(Debug, Clone)]
pub struct TestInnerProvider {
    contracts: Arc<Mutex<HashMap<Address, Arc<Mutex<ContractState>>>>>,
}

impl TestInnerProvider {
    pub fn new() -> Self {
        Self {
            contracts: Arc::new(Mutex::new(HashMap::new())),
        }
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

    #[error("Contract call error: {0}")]
    ContractCallError(#[from] ContractCallError),
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

                let params = serde_json::from_str::<(EthCallParams, String)>(&params).unwrap();

                let data = hex::decode(&params.0.data[2..]).unwrap();
                let contract_address = params.0.to;
                let sender_address = params.0.from;

                let contract_state = self.contract(contract_address);
                let mut contract =
                    ContractCall::new(self.clone(), contract_address, contract_state)
                        .with_sender(sender_address);

                let res: String = contract.entry_point(&data)?;
                let res = hex::encode(res);

                let res = serde_json::to_string::<String>(&res).unwrap();

                println!("R type: {}", std::any::type_name::<R>());
                println!("res: {}", res);

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_chainId" => {
                let res = Bytes::from(CHAIN_UD.to_be_bytes());
                println!("R type: {}", std::any::type_name::<R>());

                let res = serde_json::to_string(&res).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_getTransactionCount" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                println!("R type: {}", std::any::type_name::<R>());

                let transaction_count = U256::zero();

                let res = serde_json::to_string(&transaction_count).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_gasPrice" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                println!("R type: {}", std::any::type_name::<R>());

                let gas_price = U256::zero();

                let res = serde_json::to_string(&gas_price).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_estimateGas" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                println!("R type: {}", std::any::type_name::<R>());

                let gas = U256::zero();

                let res = serde_json::to_string(&gas).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_sendRawTransaction" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                println!("R type: {}", std::any::type_name::<R>());

                let gas = U256::zero();

                let res = serde_json::to_string(&gas).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            method => unimplemented!("Method \"{method}\""),
        };
    }
}

impl TestInnerProvider {
    pub fn contract(&self, address: Address) -> Arc<Mutex<ContractState>> {
        let contracts = self.contracts.lock().unwrap();

        contracts
            .get(&address)
            .cloned()
            .expect(format!("Contract not found: {}", address).as_str())
    }
}

pub trait TestProvider {
    fn contract(&self, address: Address) -> Arc<Mutex<ContractState>>;

    fn deploy_contract(&self, bytes: &[u8]) -> Address;
}

impl TestProvider for TestInnerProvider {
    fn contract(&self, address: Address) -> Arc<Mutex<ContractState>> {
        let contracts = self.contracts.lock().unwrap();

        contracts
            .get(&address)
            .cloned()
            .expect(format!("Contract not found: {}", address).as_str())
    }

    fn deploy_contract(&self, bytes: &[u8]) -> Address {
        let address = Address::random();

        let state = Arc::new(Mutex::new(ContractState::new(bytes)));

        let mut contracts = self.contracts.lock().unwrap();
        contracts.insert(address, state.clone());

        address
    }
}

impl TestProvider for Arc<TestClient> {
    fn contract(&self, address: Address) -> Arc<Mutex<ContractState>> {
        let p: TestInnerProvider = self.provider().as_ref().clone();
        p.contract(address)
    }

    fn deploy_contract(&self, bytes: &[u8]) -> Address {
        let p: TestInnerProvider = self.provider().as_ref().clone();
        p.deploy_contract(bytes)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthCallParams {
    // [{"accessList":[],"data":"...","from":"0x3cc222811060826c2699ea8ff9b35fdb0963d6ad","to":"0x00000000000000000000000000000000000004d2","type":"0x02"},"latest"]
    #[serde(rename = "accessList")]
    access_list: Vec<()>,
    data: String,
    from: Address,
    to: Address,
    #[serde(rename = "type")]
    tx_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimateRequestParams {
    // [{"accessList":[],"data":"...","from":"0x3cc222811060826c2699ea8ff9b35fdb0963d6ad","to":"0x00000000000000000000000000000000000004d2","type":"0x02"},"latest"]
    #[serde(rename = "accessList")]
    access_list: Vec<()>,
    data: String,
    from: Address,
    to: Address,
    #[serde(rename = "type")]
    tx_type: String,
    #[serde(rename = "maxFeePerGas")]
    max_fee_per_gas: U64,
    #[serde(rename = "maxPriorityFeePerGas")]
    max_priority_fee_per_gas: U64,
    nonce: U256,
}

pub trait FromContractResult {
    fn from_contract_result(result: &[u8]) -> Self;
}

impl FromContractResult for String {
    fn from_contract_result(result: &[u8]) -> Self {
        String::from_utf8(result.to_vec()).unwrap()
    }
}

impl FromContractResult for U256 {
    fn from_contract_result(result: &[u8]) -> Self {
        U256::from_big_endian(result)
    }
}
