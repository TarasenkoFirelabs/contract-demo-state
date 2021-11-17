use std::str::FromStr;

use near_sdk::{AccountId, PublicKey, json_types::U128};
use near_sdk_sim::{ContractAccount, DEFAULT_GAS, ExecutionResult, STORAGE_AMOUNT, UserAccount, call, deploy, init_simulator, lazy_static_include::syn::token::Use, runtime::GenesisConfig, to_yocto, transaction::ExecutionStatus, view};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    CONTRACT_BYTES => "res/app_wallet_creation.wasm",
    LINKDROP_BYTES => "res/linkdrop.wasm"
}

use app_wallet_creation::{MakeWalletsContract, NewAccount};

const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, ContractAccount<MakeWalletsContract>, UserAccount) {
    let mut config: GenesisConfig = GenesisConfig::default();
    //config.init_root_signer("near");
    config.gas_price = 0;
    config.gas_limit = u64::MAX;
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(Some(config));

    root.deploy(&LINKDROP_BYTES, "near".parse().unwrap(), to_yocto("100"));

    let contract = deploy!{
        contract: MakeWalletsContract,
        contract_id: CONTRACT_ID,
        bytes: &CONTRACT_BYTES,
        signer_account: root
    };

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}

#[test]
pub fn successful_wallet_creation(){
    let (_root, contract, user) = init();
    
    let account_id = "adsick".parse().unwrap();
    let public_key = PublicKey::from_str("ed25519:8MtAwUtEuU18u9xrehUEBWgcziTHxFhXLNE9F5xq7ExU").unwrap();
    let initial_amount = to_yocto("1");
    let new_account = NewAccount::new(account_id, public_key);

    // uses default gas amount
    let result = call!(user, contract.make_wallets(new_account), deposit = initial_amount);
    //println!("{:#?}", result.promise_results());
    println!("{:?}", result.outcome().logs);
    println!("{:?}", result.outcome().status);
    assert!(result.is_ok());
    let status = format!("{:?}", result.outcome().status);
    assert_eq!(status, "SuccessValue(`true`)");
}


#[test]
fn wallet_already_exists(){
    let (_root, contract, user) = init();

    let account_id: AccountId = "adsick".parse().unwrap();
    let public_key = PublicKey::from_str("ed25519:8MtAwUtEuU18u9xrehUEBWgcziTHxFhXLNE9F5xq7ExU").unwrap();
    let initial_amount = to_yocto("1");
    let new_account = NewAccount::new(account_id.clone(), public_key.clone());

    let result = call!(user, contract.make_wallets(new_account), deposit = initial_amount);
    println!("{:#?}", result.promise_results());    
    assert!(result.is_ok());

    let new_account = NewAccount::new(account_id, public_key);
    let result = call!(user, contract.make_wallets(new_account), deposit = initial_amount);

    println!("status: {:?}", result.outcome().status);
    assert!(result.is_ok());

    let status = format!("{:?}", result.outcome().status);
    assert_eq!(status, "SuccessValue(`false`)");
}