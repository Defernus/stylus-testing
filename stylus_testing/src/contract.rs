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

use crate::{provider::FromContractResult, vm_hooks};

#[derive(Debug, ThisError, Clone)]
pub enum ContractCallError {
    #[error("Contract call error: {0}")]
    Message(String),
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Clone, Debug)]
pub struct Env {
    reentrant_counter: Arc<Mutex<u32>>,
    memory: Option<Memory>,
    entrypoint_data: Arc<Mutex<Vec<u8>>>,
    value: Arc<Mutex<U256>>,
    storage_bytes32: Arc<Mutex<HashMap<U256, U256>>>,
    result: Arc<Mutex<Vec<u8>>>,
    block_number: Arc<Mutex<u64>>,
    sender: Address,
    address: Address,
}

pub type ContractCallResult<T> = Result<T, ContractCallError>;

#[derive(Debug)]
pub struct ContractState {
    env: FunctionEnv<Env>,
    store: Store,
    instance: Instance,
}

impl ContractState {
    pub fn new(bytes: &[u8], address: Address) -> Self {
        let mut store = Store::default();

        let module = Module::new(&store, bytes).unwrap();

        let env = FunctionEnv::new(
            &mut store,
            Env {
                value: Arc::new(Mutex::new(U256::zero())),
                block_number: Arc::new(Mutex::new(0)),
                reentrant_counter: Arc::new(Mutex::new(0)),
                entrypoint_data: Arc::new(Mutex::new(Vec::new())),
                storage_bytes32: Arc::new(Mutex::new(HashMap::new())),
                result: Arc::new(Mutex::new(Vec::new())),
                sender: Address::zero(),
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

    pub fn set_value(&mut self, new_value: U256) {
        let mut value = self.env.as_mut(&mut self.store).value.lock().unwrap();

        *value = new_value;
    }

    pub fn set_sender(&mut self, sender: Address) {
        self.env.as_mut(&mut self.store).sender = sender;
    }

    pub fn entry_point<T: FromContractResult>(&mut self, data_ptr: &[u8]) -> ContractCallResult<T> {
        let data_len = data_ptr.len() as i32;

        {
            let env = self.env.as_mut(&mut self.store);

            env.entrypoint_data = Arc::new(Mutex::new(data_ptr.to_vec()));
            env.result = Arc::new(Mutex::new(Vec::new()));
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
            .result
            .lock()
            .unwrap()
            .clone();

        if result != 0 {
            return Err(ContractCallError::Message(
                String::from_utf8(result_data).unwrap(),
            ));
        }

        Ok(FromContractResult::from_contract_result(&result_data))
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
        let block_number = self
            .env
            .as_ref(&self.store)
            .block_number
            .lock()
            .unwrap()
            .clone();

        block_number
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

    pub fn value(&self) -> U256 {
        let value = self.value.lock().unwrap().clone();

        value
    }

    pub fn set_value(&mut self, new_value: U256) {
        let mut value = self.value.lock().unwrap();

        *value = new_value;
    }

    pub fn set_result(&mut self, data: Vec<u8>) {
        let mut result = self.result.lock().unwrap();

        *result = data;
    }

    pub fn storage_bytes32_get(&self, key: U256) -> U256 {
        let storage = self.storage_bytes32.lock().unwrap();

        storage.get(&key).cloned().unwrap_or_default()
    }

    pub fn storage_bytes32_insert(&mut self, key: U256, value: U256) {
        let mut storage = self.storage_bytes32.lock().unwrap();

        storage.insert(key, value);
    }

    pub fn entrypoint_data(&self) -> Vec<u8> {
        let entrypoint_data = self.entrypoint_data.lock().unwrap();

        entrypoint_data.clone()
    }

    pub fn set_entrypoint_data(&mut self, data: Vec<u8>) {
        let mut entrypoint_data = self.entrypoint_data.lock().unwrap();

        *entrypoint_data = data;
    }

    pub fn reentrant_counter(&self) -> u32 {
        self.reentrant_counter.lock().unwrap().clone()
    }

    pub fn inc_reentrant_counter(&mut self) {
        let mut reentrant_counter = self.reentrant_counter.lock().unwrap();

        *reentrant_counter += 1;
    }

    pub fn reset_reentrant_counter(&mut self) {
        let mut reentrant_counter = self.reentrant_counter.lock().unwrap();

        *reentrant_counter = 0;
    }

    pub fn view(&self, store: &impl AsStoreRef) -> MemoryView {
        let memory = self.memory();
        memory.view(store)
    }
}
