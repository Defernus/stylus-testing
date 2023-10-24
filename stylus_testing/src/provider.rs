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
    types::{
        Address, Bytes, NameOrAddress, Transaction, TransactionReceipt, TransactionRequest, H256,
        U256, U64,
    },
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
    balances: Arc<Mutex<HashMap<Address, U256>>>,
    transactions: Arc<Mutex<HashMap<U256, Transaction>>>,
    block_number: Arc<Mutex<u64>>,
}

impl TestInnerProvider {
    pub fn new() -> Self {
        Self {
            contracts: Arc::default(),
            balances: Arc::default(),
            transactions: Arc::default(),
            block_number: Arc::default(),
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
        println!("method: {} -> {}", method, std::any::type_name::<R>());

        match method {
            "eth_call" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let params = serde_json::from_str::<(EthCallParams, String)>(&params).unwrap();

                let value = params.0.value;
                let data = hex::decode(&params.0.data[2..]).unwrap();
                let contract_address = params.0.to;
                let sender_address = params.0.from;

                let contract_state = self.contract(contract_address).expect("Contract not found");

                let mut contract =
                    ContractCall::new(self.clone(), contract_address, contract_state)
                        .with_sender(sender_address)
                        .with_value(value);

                let res: Bytes = contract.entry_point(&data)?.into();

                let res = serde_json::to_string(&res).unwrap();

                println!("res: {}", res);

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_chainId" => {
                let res = Bytes::from(CHAIN_UD.to_be_bytes());

                let res = serde_json::to_string(&res).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_getTransactionCount" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let transaction_count = U256::zero();

                let res = serde_json::to_string(&transaction_count).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_gasPrice" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let gas_price = U256::zero();

                let res = serde_json::to_string(&gas_price).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_estimateGas" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let gas = U256::zero();

                let res = serde_json::to_string(&gas).unwrap();

                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_sendRawTransaction" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let (tx_data,) = serde_json::from_str::<(Bytes,)>(&params).unwrap();

                let data = rlp::Rlp::new(tx_data.as_ref());

                let tx = TransactionRequest::decode_unsigned_rlp(&data).unwrap();
                println!("tx: {:?}", tx);

                if tx.data.is_some() {
                    unimplemented!("Data is not supported yet {tx:?}");
                }

                let to = tx.to.clone().map(|v| match v {
                    NameOrAddress::Address(address) => address,
                    _ => unimplemented!("Name not implemented yet {:?}", tx),
                });

                if let Some(value) = tx.value {
                    match (tx.from, to) {
                        (Some(from), Some(to)) => {
                            self.send_eth(from, to, value);
                        }
                        (None, Some(to)) => {
                            self.mint_eth(to, value);
                        }
                        _ => unimplemented!("Unknown {:?}", tx),
                    };
                }

                let tx_hash = rand::random::<[u8; 32]>();
                let tx_hash = U256::from_big_endian(&tx_hash);

                {
                    let mut transactions = self.transactions.lock().unwrap();

                    let mut hash_data = vec![0u8; 32];
                    tx_hash.to_big_endian(&mut hash_data);

                    let mut result_tx = Transaction::default();
                    result_tx.hash = H256::from_slice(&hash_data);
                    result_tx.from = tx.from.unwrap_or_default();
                    result_tx.value = tx.value.unwrap_or_default();
                    result_tx.to = to;
                    result_tx.block_number = Some(self.block_number());

                    transactions.insert(tx_hash, result_tx);
                }

                let res = serde_json::to_string(&tx_hash).unwrap();
                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_getTransactionByHash" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let (tx_hash,) = serde_json::from_str::<(U256,)>(&params).unwrap();

                let tx = self.transactions.lock().unwrap().get(&tx_hash).cloned();

                let res = serde_json::to_string(&tx).unwrap();
                return Ok(serde_json::from_str(&res).unwrap());
            }
            "eth_getTransactionReceipt" => {
                let params = serde_json::to_string(&params).unwrap();
                println!("raw_params: {}", params);

                let (tx_hash,) = serde_json::from_str::<(U256,)>(&params).unwrap();

                let mut hash_data = vec![0u8; 32];
                tx_hash.to_big_endian(&mut hash_data);

                let mut res = TransactionReceipt::default();

                res.transaction_hash = H256::from_slice(&hash_data);
                res.block_number = Some(self.block_number());

                let res = serde_json::to_string(&res).unwrap();
                return Ok(serde_json::from_str(&res).unwrap());
            }
            method => unimplemented!("Method \"{method}\""),
        };
    }
}

impl TestInnerProvider {
    pub fn contract(&self, address: Address) -> Option<Arc<Mutex<ContractState>>> {
        let contracts = self.contracts.lock().unwrap();

        contracts.get(&address).cloned()
    }

    pub fn block_number(&self) -> U64 {
        let block_number = self.block_number.lock().unwrap();

        U64::from(*block_number)
    }

    pub fn next_block(&self) {
        let mut block_number = self.block_number.lock().unwrap();

        *block_number += 1;
    }
}

pub trait TestProvider {
    fn contract(&self, address: Address) -> Option<Arc<Mutex<ContractState>>>;

    fn deploy_contract(&self, bytes: &[u8]) -> Address;

    fn mint_eth(&self, to: Address, amount: U256);

    fn send_eth(&self, from: Address, to: Address, amount: U256);

    fn balance(&self, address: Address) -> U256;
}

impl TestProvider for TestInnerProvider {
    fn contract(&self, address: Address) -> Option<Arc<Mutex<ContractState>>> {
        let contracts = self.contracts.lock().unwrap();

        contracts.get(&address).cloned()
    }

    fn deploy_contract(&self, bytes: &[u8]) -> Address {
        let address = Address::random();

        let state = Arc::new(Mutex::new(ContractState::new(bytes)));

        let mut contracts = self.contracts.lock().unwrap();
        contracts.insert(address, state.clone());

        address
    }

    fn mint_eth(&self, to: Address, amount: U256) {
        let mut balances = self.balances.lock().unwrap();

        let balance = balances.entry(to).or_insert(U256::zero());
        *balance += amount;

        println!("mint_eth: {} {}", to, amount);
        println!("\t└ balance: {}", balance);
    }

    // TODO add error handling
    fn send_eth(&self, from: Address, to: Address, amount: U256) {
        let mut balances = self.balances.lock().unwrap();

        let from_balance = balances.entry(from).or_insert(U256::zero());

        println!("send_eth: {} -> {} {}", from, to, amount);
        println!("\t└ sender_balance: {}", from_balance);

        if *from_balance < amount {
            panic!("Insufficient funds");
        }

        *from_balance -= amount;

        let to_balance = balances.entry(to).or_insert(U256::zero());
        *to_balance += amount;
    }

    fn balance(&self, address: Address) -> U256 {
        let balances = self.balances.lock().unwrap();

        balances.get(&address).cloned().unwrap_or_default()
    }
}

impl TestProvider for Arc<TestClient> {
    fn contract(&self, address: Address) -> Option<Arc<Mutex<ContractState>>> {
        let p: TestInnerProvider = self.provider().as_ref().clone();
        p.contract(address)
    }

    fn deploy_contract(&self, bytes: &[u8]) -> Address {
        let p: TestInnerProvider = self.provider().as_ref().clone();
        p.deploy_contract(bytes)
    }

    fn mint_eth(&self, to: Address, amount: U256) {
        let p: TestInnerProvider = self.provider().as_ref().clone();
        p.mint_eth(to, amount)
    }

    fn send_eth(&self, from: Address, to: Address, amount: U256) {
        let p: TestInnerProvider = self.provider().as_ref().clone();
        p.send_eth(from, to, amount)
    }

    fn balance(&self, address: Address) -> U256 {
        let p: TestInnerProvider = self.provider().as_ref().clone();
        p.balance(address)
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
    #[serde(default)]
    value: U256,
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
        // String::from_utf8(result.to_vec()).unwrap_or_else(|_| format!("0x{}", hex::encode(result)))
        format!("{}", hex::encode(result))
    }
}

impl FromContractResult for U256 {
    fn from_contract_result(result: &[u8]) -> Self {
        U256::from_big_endian(result)
    }
}
