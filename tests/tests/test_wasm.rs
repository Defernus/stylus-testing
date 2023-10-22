use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    signers::{LocalWallet, Signer},
    types::Address,
};

use stylus_testing::{
    contract::Contract,
    private_key::key_from_file,
    provider::{TestInnerProvider, TestProvider},
};

static CONTRACT_BYTES: &'static [u8] = include_bytes!("../../contracts/erc20.wasm");

abigen!(
    Erc20,
    r#"[
        function init(address gov, string calldata name, string calldata symbol, uint8 decimals) external
        function setMinter(address minter, bool is_active) external
        function setGov(address gov) external
        function mint(address account, uint256 amount) external
        function burn(uint256 amount) external
        function name() external view returns (string memory)
        function symbol() external view returns (string memory)
        function decimals() external view returns (uint8)
        function totalSupply() external view returns (uint256)
        function balanceOf(address account) external view returns (uint256)
        function transfer(address recipient, uint256 amount) external returns (bool)
        function allowance(address owner, address spender) external view returns (uint256)
        function approve(address spender, uint256 amount) external returns (bool)
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool)
        function increaseAllowance(address spender, uint256 added_value) external returns (bool)
        function decreaseAllowance(address spender, uint256 subtracted_value) external returns (bool)
    ]"#
);

// 0004d2","type":"0x02"},"latest"]
// call from wasm: msg_reentrant()
//         └ result: 0
// call from wasm: read_args(130812)
// call from wasm: msg_value(25600) -> 0
// call from wasm: storage_load_bytes32(0, 24784)
//         └ value: 0
// call from wasm: storage_load_bytes32(1, 24656)
//         └ value: 0
// call from wasm: storage_load_bytes32(2, 24656)
//         └ value: 0
// call from wasm: storage_load_bytes32(3, 24912)
//         └ value: 0
// call from wasm: native_keccak256(25448, 64, 25128)
// call from wasm: storage_store_bytes32(1, 30038970231641295373582720784850654067238680715539897351645650438096506847246)
// call from wasm: storage_store_bytes32(0, 88798411917788436148661468515950041286013063965953)
// call from wasm: storage_store_bytes32(3, 11)
// call from wasm: storage_store_bytes32(2, 30001525576942200884098149691257528959455990793249145324804069187071678349318)
// call from wasm: write_result(1, 0)

#[tokio::test]
async fn test_aaa() {
    let contract = Contract::new(CONTRACT_BYTES);
    let contract = Arc::new(Mutex::new(contract));

    let private_key = key_from_file("../key");
    let wallet = LocalWallet::from_str(&private_key)
        .unwrap()
        .with_chain_id(4_u64);

    let test_provider = TestProvider::new(TestInnerProvider::new(contract));

    let client = Arc::new(SignerMiddleware::new(test_provider, wallet.clone()));

    let token = Erc20::new(Address::from_low_u64_be(1234), client.clone());

    let result = token
        .init(wallet.address(), "Bitcoin".into(), "BTC".into(), 8)
        .await
        .unwrap();

    println!("result: {:?}", result);

    let result = token
        .init(wallet.address(), "Bitcoin".into(), "BTC".into(), 8)
        .await
        .unwrap();

    println!("result: {:?}", result);

    // contract.set_value(U256::from(12345));

    // let data = hex::decode("3bf73798").unwrap();
    // // selector.to_be_bytes();

    // let res = contract.entry_point(&data);

    // println!("res: {:?}", res);
    // let res = res.unwrap();

    // let data = contract.read_mem(res[0].i32().unwrap() as u64, 32);

    // println!("data: {:?}", data);
}
