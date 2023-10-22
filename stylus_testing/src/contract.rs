use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ethers::types::{Address, U256};
use stylus_sdk::keccak_const::Keccak256;
use thiserror::Error as ThisError;
use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, MemoryView, Module,
    RuntimeError, Store, Value,
};

use crate::provider::FromContractResult;

#[derive(Debug, ThisError, Clone)]
pub enum ContractCallError {
    #[error("Contract call error: {0}")]
    Message(String),
    #[error("Runtime error: {0}")]
    RuntimeError(#[from] RuntimeError),
}

#[derive(Clone, Debug)]
pub struct Env {
    reentrant_counter: Arc<Mutex<i32>>,
    memory: Option<Memory>,
    entrypoint_data: Arc<Mutex<Vec<u8>>>,
    value: Arc<Mutex<U256>>,
    storage_bytes32: Arc<Mutex<HashMap<U256, U256>>>,
    result: Arc<Mutex<Vec<u8>>>,
    sender: Arc<Mutex<Address>>,
    block_number: Arc<Mutex<u64>>,
    transactions_count: Arc<Mutex<u64>>,
}

pub type ContractCallResult<T> = Result<T, ContractCallError>;

#[derive(Debug)]
pub struct ContractState {
    env: FunctionEnv<Env>,
    store: Store,
    instance: Instance,
}

impl ContractState {
    pub fn new(bytes: &[u8]) -> Self {
        let mut store = Store::default();

        let module = Module::new(&store, bytes).unwrap();

        let shared_counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
        let value: Arc<Mutex<U256>> = Arc::new(Mutex::new(U256::zero()));

        let env = FunctionEnv::new(
            &mut store,
            Env {
                value,
                block_number: Arc::new(Mutex::new(0)),
                reentrant_counter: shared_counter.clone(),
                entrypoint_data: Arc::new(Mutex::new(Vec::new())),
                storage_bytes32: Arc::new(Mutex::new(HashMap::new())),
                result: Arc::new(Mutex::new(Vec::new())),
                sender: Arc::new(Mutex::new(Address::zero())),
                memory: None,
                transactions_count: Arc::new(Mutex::new(0)),
            },
        );

        let import_object = imports! {
            "vm_hooks" => {
                "msg_reentrant" => Function::new_typed_with_env(&mut store, &env, msg_reentrant),
                "read_args" => Function::new_typed_with_env(&mut store, &env, read_args),
                "storage_store_bytes32" => Function::new_typed_with_env(&mut store, &env, storage_store_bytes32),
                "write_result" => Function::new_typed_with_env(&mut store, &env, write_result),
                "native_keccak256" => Function::new_typed_with_env(&mut store, &env, native_keccak256),
                "storage_load_bytes32" => Function::new_typed_with_env(&mut store, &env, storage_load_bytes32),
                "msg_value" => Function::new_typed_with_env(&mut store, &env, msg_value),
                "emit_log" => Function::new_typed_with_env(&mut store, &env, emit_log),
                "memory_grow" => Function::new_typed_with_env(&mut store, &env, memory_grow),
                "msg_sender" => Function::new_typed_with_env(&mut store, &env, msg_sender),
            },
            "console" => {
                "log_txt" => Function::new_typed_with_env(&mut store, &env, log_txt),
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

    pub fn set_value(&mut self, new_value: U256) {
        let mut value = self.env.as_mut(&mut self.store).value.lock().unwrap();

        *value = new_value;
    }

    pub fn set_sender(&mut self, new_sender: Address) {
        let mut sender = self.env.as_mut(&mut self.store).sender.lock().unwrap();

        *sender = new_sender;
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

    pub fn transactions_count(&self) -> u64 {
        let transactions_count = self
            .env
            .as_ref(&self.store)
            .transactions_count
            .lock()
            .unwrap()
            .clone();

        transactions_count
    }
}

/// Returns if current call is reentrant
pub fn msg_reentrant(mut env: FunctionEnvMut<Env>) -> i32 {
    println!("call from wasm: msg_reentrant()");

    let (env, _) = env.data_and_store_mut();

    let mut counter = env.reentrant_counter.lock().unwrap();

    let result = *counter;

    *counter += 1;

    println!("\t└ result: {result}");

    result
}

pub fn read_args(mut env: FunctionEnvMut<Env>, dest_ptr: u32) {
    println!("call from wasm: read_args({dest_ptr:?})");

    let (env, store) = env.data_and_store_mut();

    let memory = env.memory.as_mut().unwrap();

    let data = env.entrypoint_data.lock().unwrap();

    let view = memory.view(&store);
    view.write(dest_ptr as u64, &data).unwrap();
}

pub fn storage_store_bytes32(mut env: FunctionEnvMut<Env>, key_ptr: u32, value_ptr: u32) {
    let (env, store) = env.data_and_store_mut();

    let memory = env.memory.as_mut().unwrap();
    let view = memory.view(&store);

    let key = read_u256(&view, key_ptr as u64);
    let value = read_u256(&view, value_ptr as u64);

    println!("call from wasm: storage_store_bytes32({key}, {value})");

    let mut storage = env.storage_bytes32.lock().unwrap();

    storage.insert(key, value);
}

pub fn storage_load_bytes32(mut env: FunctionEnvMut<Env>, key_ptr: u32, dest_ptr: u32) {
    let (env, store) = env.data_and_store_mut();

    let memory = env.memory.as_mut().unwrap();
    let view = memory.view(&store);

    let key = read_u256(&view, key_ptr as u64);

    println!("call from wasm: storage_load_bytes32({key}, {dest_ptr})");

    let storage = env.storage_bytes32.lock().unwrap();

    let value = storage.get(&key).cloned().unwrap_or_default();

    println!("\t└ value: {value}");
    write_u256(&view, dest_ptr as u64, value);
}

pub fn write_result(mut env: FunctionEnvMut<Env>, data_ptr: u32, len: u32) {
    let (env, store) = env.data_and_store_mut();

    let memory = env.memory.as_mut().unwrap();
    let view = memory.view(&store);

    let result = read_bytes(&view, data_ptr, len);

    println!("call from wasm: write_result({data_ptr}, {len})");
    println!("\t└ result: 0x{}", hex::encode(&result));

    let mut env_result = env.result.lock().unwrap();
    *env_result = result;
}

pub fn native_keccak256(mut env: FunctionEnvMut<Env>, bytes: u32, len: u32, output_ptr: u32) {
    let (env, store) = env.data_and_store_mut();

    let memory = env.memory.as_mut().unwrap();
    let view = memory.view(&store);

    let data = read_bytes(&view, bytes, len);
    println!("call from wasm: native_keccak256({data:?}, {output_ptr})");

    let output = Keccak256::new().update(&data).finalize();
    println!(
        "\t└ output: 0x{} ({})",
        hex::encode(&output),
        U256::from_big_endian(&output)
    );

    write_bytes(&view, output_ptr as u64, &output);
}

pub fn msg_value(mut env: FunctionEnvMut<Env>, value_ptr: u32) {
    let (env, store) = env.data_and_store_mut();
    let value = env.value.lock().unwrap().clone();

    let memory = env.memory.as_mut().unwrap();

    let mut data = vec![0; 32];
    value.to_big_endian(&mut data);

    let view = memory.view(&store);
    view.write(value_ptr as u64, &data).unwrap();
    println!("call from wasm: msg_value({value_ptr}) -> {value}");
}

pub fn emit_log(mut _env: FunctionEnvMut<Env>, data_ptr: u32, len: u32, topics: u32) {
    println!("call from wasm: emit_log({data_ptr}, {len}, {topics})");
}

pub fn memory_grow(mut _env: FunctionEnvMut<Env>, pages: u32) {
    println!("call from wasm: memory_grow({pages})");
}

pub fn msg_sender(mut env: FunctionEnvMut<Env>, sender_ptr: u32) {
    println!("call from wasm: msg_sender({sender_ptr})");

    let (env, store) = env.data_and_store_mut();

    let memory = env.memory.as_mut().unwrap();

    let view = memory.view(&store);

    let sender = env.sender.lock().unwrap().clone();
    println!("\t└ sender: {}", sender);

    let bytes: [u8; 20] = sender.into();

    view.write(sender_ptr as u64, &bytes).unwrap();
}

pub fn log_txt(mut env: FunctionEnvMut<Env>, data_ptr: u32, len: u32) {
    let (env, store) = env.data_and_store_mut();

    let memory = env.memory.as_ref().unwrap();

    let view = memory.view(&store);
    let msg = read_str(&view, data_ptr, len);

    println!("call from wasm: log_txt({msg})");
}

fn read_str(view: &MemoryView, data_ptr: u32, len: u32) -> String {
    let mut buf = vec![0; len as usize];
    view.read(data_ptr as u64, &mut buf).unwrap();

    String::from_utf8(buf).unwrap()
}

fn read_bytes(view: &MemoryView, data_ptr: u32, len: u32) -> Vec<u8> {
    let len = len as usize;

    let mut buf = vec![0; len];
    view.read(data_ptr as u64, &mut buf).unwrap();

    buf
}

fn read_u256(view: &MemoryView, ptr: u64) -> U256 {
    let mut data = vec![0; 32];
    view.read(ptr, &mut data).unwrap();

    U256::from_big_endian(&data)
}

fn write_u256(view: &MemoryView, ptr: u64, value: U256) {
    let mut data = vec![0; 32];
    value.to_big_endian(&mut data);

    write_bytes(view, ptr, &data);
}

fn write_bytes(view: &MemoryView, ptr: u64, data: &[u8]) {
    view.write(ptr, data).unwrap();
}
