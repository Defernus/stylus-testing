use ethers::types::{Address, U256};
use stylus_sdk::keccak_const::Keccak256;
use wasmer::{FunctionEnvMut, MemoryView};

use crate::contract::{ContractCall, ContractCallError, Env};

/// Returns if current call is reentrant
pub fn msg_reentrant(mut env: FunctionEnvMut<Env>) -> u32 {
    println!("call from wasm: msg_reentrant()");

    let (env, _) = env.data_and_store_mut();

    let counter = env.reentrant_counter();

    println!("\t└ result: {counter}");

    counter
}

pub fn read_args(mut env: FunctionEnvMut<Env>, dest_ptr: u32) {
    println!("call from wasm: read_args({dest_ptr:?})");

    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

    view.write(dest_ptr as u64, &env.entrypoint_data()).unwrap();
}

pub fn storage_store_bytes32(mut env: FunctionEnvMut<Env>, key_ptr: u32, value_ptr: u32) {
    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

    let key = read_u256(&view, key_ptr as u64);
    let value = read_u256(&view, value_ptr as u64);

    println!("call from wasm: storage_store_bytes32({key}, {value})");

    env.storage_bytes32_insert(key, value);
}

pub fn storage_load_bytes32(mut env: FunctionEnvMut<Env>, key_ptr: u32, dest_ptr: u32) {
    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

    let key = read_u256(&view, key_ptr as u64);

    let value = env.storage_bytes32_get(key);

    println!("call from wasm: storage_load_bytes32({key}, {dest_ptr})");

    println!("\t└ value: {value}");
    write_u256(&view, dest_ptr as u64, value);
}

pub fn write_result(mut env: FunctionEnvMut<Env>, data_ptr: u32, len: u32) {
    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

    let result = read_bytes(&view, data_ptr, len);

    println!("call from wasm: write_result({data_ptr}, {len})");
    println!("\t└ result: 0x{}", hex::encode(&result));

    env.set_result(result);
}

pub fn native_keccak256(mut env: FunctionEnvMut<Env>, bytes: u32, len: u32, output_ptr: u32) {
    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

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
    let view = env.view(&store);

    let value = env.value();

    let mut data = vec![0; 32];
    value.to_big_endian(&mut data);

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
    let view = env.view(&store);

    let sender = env.sender();
    println!("\t└ sender: {}", sender);

    let bytes: [u8; 20] = sender.into();

    view.write(sender_ptr as u64, &bytes).unwrap();
}

pub fn block_timestamp(mut env: FunctionEnvMut<Env>) -> u64 {
    println!("call from wasm: block_timestamp()");

    let (env, _) = env.data_and_store_mut();

    let block_timestamp = env.block_timestamp();

    println!("\t└ block_timestamp: {block_timestamp}");

    block_timestamp
}

pub fn call_contract(
    mut env: FunctionEnvMut<Env>,
    contract_ptr: u32,
    calldata_ptr: u32,
    calldata_len: u32,
    value_ptr: u32,
    _gas: u64,
    return_data_len_ptr: u32,
) -> u8 {
    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

    let contract_addr = read_addr(&view, contract_ptr as u64);

    let data = read_bytes(&view, calldata_ptr, calldata_len);

    let value = read_u256(&view, value_ptr as u64);

    let str_data = hex::encode(&data);
    println!("call from wasm: call_contract({contract_addr}, Ox{str_data}, {value})");

    let provider = env.provider();

    let contract_state = provider.contract(contract_addr);
    let mut contract = ContractCall::new(provider, contract_addr, contract_state)
        .with_value(value)
        .with_sender(env.address());

    let res = contract.entry_point_raw(&data);

    let (status, data) = match res {
        Ok(data) => (0, data),
        Err(err) => {
            println!("\t└ Error: {}", err);
            (
                1,
                match err {
                    ContractCallError::Message(data) => data.as_bytes().to_vec(),
                    ContractCallError::RuntimeError(data) => panic!("RuntimeError: {}", data),
                },
            )
        }
    };

    write_u64(&view, return_data_len_ptr as u64, data.len() as u64);

    env.set_return_data(data);

    status
}

pub fn delegate_call_contract(
    mut env: FunctionEnvMut<Env>,
    contract_ptr: u32,
    calldata_ptr: u32,
    calldata_len: u32,
    _gas: u64,
    return_data_len_ptr: u32,
) -> u8 {
    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

    let contract_addr = read_addr(&view, contract_ptr as u64);

    let data = read_bytes(&view, calldata_ptr, calldata_len);

    let provider = env.provider();

    let str_data = hex::encode(&data);
    println!("call from wasm: call_contract({contract_addr}, Ox{str_data})");

    let contract = provider.contract(contract_addr);
    let mut contract =
        ContractCall::new(provider, contract_addr, contract).with_sender(env.address());

    let res = contract.entry_point_raw(&data);

    let (status, data) = match res {
        Ok(data) => (0, data),
        Err(err) => {
            println!("\t└ Error: {}", err);
            (
                1,
                match err {
                    ContractCallError::Message(data) => data.as_bytes().to_vec(),
                    ContractCallError::RuntimeError(data) => panic!("RuntimeError: {}", data),
                },
            )
        }
    };

    write_u64(&view, return_data_len_ptr as u64, data.len() as u64);

    env.set_return_data(data);

    status
}

pub fn read_return_data(
    mut env: FunctionEnvMut<Env>,
    dest: u32,
    offset: usize,
    size: usize,
) -> usize {
    let (env, store) = env.data_and_store_mut();

    let view = env.view(&store);

    let data = env.return_data();

    write_bytes(&view, dest as u64, &data[offset..size]);

    0
}

pub fn log_txt(mut env: FunctionEnvMut<Env>, data_ptr: u32, len: u32) {
    let (env, store) = env.data_and_store_mut();
    let view = env.view(&store);

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

fn read_addr(view: &MemoryView, ptr: u64) -> Address {
    let mut data = vec![0; 20];
    view.read(ptr, &mut data).unwrap();

    Address::from_slice(&data)
}

#[allow(dead_code)]
fn read_u64(view: &MemoryView, ptr: u64) -> u64 {
    let mut data = vec![0; 8];
    view.read(ptr, &mut data).unwrap();

    u64::from_le_bytes(data.try_into().unwrap())
}

fn write_u64(view: &MemoryView, ptr: u64, value: u64) {
    let data = value.to_le_bytes();

    view.write(ptr, &data).unwrap();
}
