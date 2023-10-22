use std::sync::{Arc, Mutex};

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    signers::{LocalWallet, Signer},
    types::Address,
};

use stylus_testing::{
    contract::ContractState,
    private_key::key_from_index,
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

#[tokio::main]
async fn main() {
    let contract = ContractState::new(CONTRACT_BYTES);
    let contract = Arc::new(Mutex::new(contract));

    let private_key = key_from_index(0);
    let wallet = LocalWallet::from_bytes(&private_key)
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
