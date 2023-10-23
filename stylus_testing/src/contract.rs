use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ethers::types::{Address, U256};
use thiserror::Error as ThisError;
use wasmer::{
    imports, AsStoreRef, Function, FunctionEnv, Instance, Memory, MemoryView, Module, RuntimeError,
    Store, Value,
};

use crate::{
    provider::{FromContractResult, TestInnerProvider},
    vm_hooks,
};

#[derive(Debug, ThisError, Clone)]
pub enum ContractCallError {
    #[error("Contract call error: {0}")]
    Message(String),
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Debug, Clone)]
pub struct ContractState {
    reentrant_counter: u32,
    binary: Vec<u8>,
    storage_bytes32: HashMap<U256, U256>,
    result: Vec<u8>,
    return_data: Vec<u8>,
}

impl ContractState {
    pub fn new(binary: &[u8]) -> Self {
        Self {
            binary: binary.to_vec(),
            reentrant_counter: 0,
            storage_bytes32: HashMap::new(),
            result: Vec::new(),
            return_data: Vec::new(),
        }
    }

    pub fn reset_result(&mut self) {
        self.result = Vec::new();
    }
}

#[derive(Clone, Debug)]
pub struct Env {
    state: Arc<Mutex<ContractState>>,
    provider: TestInnerProvider,
    value: U256,
    entrypoint_data: Vec<u8>,
    memory: Option<Memory>,
    sender: Address,
    address: Address,
    block_number: u64,
    block_timestamp: u64,
}

pub type ContractCallResult<T> = Result<T, ContractCallError>;

#[derive(Debug)]
pub struct ContractCall {
    env: FunctionEnv<Env>,
    store: Store,
    instance: Instance,
}

impl ContractCall {
    pub fn new(
        provider: TestInnerProvider,
        address: Address,
        state: Arc<Mutex<ContractState>>,
    ) -> Self {
        let mut store = Store::default();

        let bytes = { state.lock().unwrap().binary.clone() };

        let module = Module::new(&store, bytes).unwrap();

        let env = FunctionEnv::new(
            &mut store,
            Env {
                state,
                sender: Address::zero(),
                value: U256::zero(),
                // TODO set real block info
                block_number: 0,
                block_timestamp: 0,
                entrypoint_data: Vec::new(),
                provider,
                address,
                memory: None,
            },
        );

        let import_object = imports! {
            "vm_hooks" => {
                "msg_reentrant" => Function::new_typed_with_env(&mut store, &env, vm_hooks::msg_reentrant),
                "read_args" => Function::new_typed_with_env(&mut store, &env, vm_hooks::read_args),
                "storage_store_bytes32" => Function::new_typed_with_env(&mut store, &env, vm_hooks::storage_store_bytes32),
                "write_result" => Function::new_typed_with_env(&mut store, &env, vm_hooks::write_result),
                "native_keccak256" => Function::new_typed_with_env(&mut store, &env, vm_hooks::native_keccak256),
                "storage_load_bytes32" => Function::new_typed_with_env(&mut store, &env, vm_hooks::storage_load_bytes32),
                "msg_value" => Function::new_typed_with_env(&mut store, &env, vm_hooks::msg_value),
                "emit_log" => Function::new_typed_with_env(&mut store, &env, vm_hooks::emit_log),
                "memory_grow" => Function::new_typed_with_env(&mut store, &env, vm_hooks::memory_grow),
                "msg_sender" => Function::new_typed_with_env(&mut store, &env, vm_hooks::msg_sender),
                "block_timestamp" => Function::new_typed_with_env(&mut store, &env, vm_hooks::block_timestamp),
                "call_contract" => Function::new_typed_with_env(&mut store, &env, vm_hooks::call_contract),
                "delegate_call_contract" => Function::new_typed_with_env(&mut store, &env, vm_hooks::delegate_call_contract),
                "static_call_contract" => Function::new_typed_with_env(&mut store, &env, vm_hooks::static_call_contract),
                "read_return_data" => Function::new_typed_with_env(&mut store, &env, vm_hooks::read_return_data),
                "contract_address" => Function::new_typed_with_env(&mut store, &env, vm_hooks::contract_address),
            },
            "console" => {
                "log_txt" => Function::new_typed_with_env(&mut store, &env, vm_hooks::log_txt),
            }
        };

        // Compile our webassembly into an `Instance`.
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();

        let memory = instance.exports.get_memory("memory").unwrap().clone();

        env.as_mut(&mut store).memory = Some(memory);

        Self {
            instance,
            env,
            store,
        }
    }

    pub fn address(&self) -> Address {
        self.env.as_ref(&self.store).address
    }

    pub fn with_value(mut self, value: U256) -> Self {
        self.env.as_mut(&mut self.store).value = value;

        self
    }

    pub fn with_sender(mut self, sender: Address) -> Self {
        self.env.as_mut(&mut self.store).sender = sender;

        self
    }

    /// Call contract entry point and return result
    pub fn entry_point<T: FromContractResult>(&mut self, data_ptr: &[u8]) -> ContractCallResult<T> {
        let result_data = self.entry_point_raw(data_ptr)?;

        Ok(FromContractResult::from_contract_result(&result_data))
    }

    /// Call contract entry point and return raw result
    pub fn entry_point_raw(&mut self, data_ptr: &[u8]) -> ContractCallResult<Vec<u8>> {
        let data_len = data_ptr.len() as i32;

        {
            let env = self.env.as_mut(&mut self.store);

            env.entrypoint_data = data_ptr.to_vec();
            env.state.lock().unwrap().reset_result();
        }

        let entrypoint = self
            .instance
            .exports
            .get_function("user_entrypoint")
            .unwrap();

        let result = entrypoint.call(&mut self.store, &[Value::I32(data_len)])?;

        let results = result.to_vec();
        let result = results[0].i32().unwrap();

        let result_data = self
            .env
            .as_mut(&mut self.store)
            .state
            .lock()
            .unwrap()
            .result
            .clone();

        println!("{} -> result: {}", self.address(), result);

        if result != 0 {
            return Err(ContractCallError::Message(
                String::from_utf8(result_data).unwrap(),
            ));
        }

        Ok(result_data)
    }

    pub fn read_mem(&self, ptr: u64, len: usize) -> Vec<u8> {
        let memory = self
            .env
            .as_ref(&self.store)
            .memory
            .as_ref()
            .expect("memory should be initialized");

        let view = memory.view(&self.store);

        let mut data = vec![0; len];

        view.read(ptr, &mut data).unwrap();

        data
    }

    pub fn env(&self) -> &Env {
        self.env.as_ref(&self.store)
    }

    pub fn block_number(&self) -> u64 {
        self.env.as_ref(&self.store).block_number
    }
}

impl Env {
    pub fn memory_mut(&mut self) -> &mut Memory {
        self.memory.as_mut().expect("memory should be initialized")
    }

    pub fn memory(&self) -> &Memory {
        self.memory.as_ref().expect("memory should be initialized")
    }

    pub fn sender(&self) -> Address {
        self.sender
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn provider(&self) -> TestInnerProvider {
        self.provider.clone()
    }

    pub fn value(&self) -> U256 {
        self.value
    }

    pub fn set_value(&mut self, value: U256) {
        self.value = value;
    }

    pub fn set_result(&mut self, result: Vec<u8>) {
        let mut state = self.state.lock().unwrap();

        state.result = result;
    }

    pub fn storage_bytes32_get(&self, key: U256) -> U256 {
        let storage = &self.state.lock().unwrap().storage_bytes32;

        storage.get(&key).cloned().unwrap_or_default()
    }

    pub fn storage_bytes32_insert(&mut self, key: U256, value: U256) {
        let mut storage = self.state.lock().unwrap();

        storage.storage_bytes32.insert(key, value);
    }

    pub fn return_data(&self) -> Vec<u8> {
        self.state.lock().unwrap().return_data.clone()
    }

    pub fn set_return_data(&mut self, return_data: Vec<u8>) {
        self.state.lock().unwrap().return_data = return_data;
    }

    pub fn entrypoint_data(&self) -> Vec<u8> {
        self.entrypoint_data.clone()
    }

    pub fn block_timestamp(&self) -> u64 {
        self.block_timestamp
    }

    pub fn set_entrypoint_data(&mut self, entrypoint_data: Vec<u8>) {
        self.entrypoint_data = entrypoint_data;
    }

    pub fn reentrant_counter(&self) -> u32 {
        self.state.lock().unwrap().reentrant_counter
    }

    pub fn inc_reentrant_counter(&mut self) {
        let mut state = self.state.lock().unwrap();

        state.reentrant_counter += 1;
    }

    pub fn reset_reentrant_counter(&mut self) {
        let mut state = self.state.lock().unwrap();

        state.reentrant_counter = 0;
    }

    pub fn view(&self, store: &impl AsStoreRef) -> MemoryView {
        let memory = self.memory();
        memory.view(store)
    }
}
