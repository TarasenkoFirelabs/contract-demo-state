use std::collections::HashMap;
use std::str::FromStr;

use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::{env, PendingContractTx};
use near_sdk::{AccountId, PublicKey};
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::{
    call, deploy, init_simulator, lazy_static_include::syn::token::Use, to_yocto, view,
    ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};

extern crate app;
extern crate app_wallet_creation;
use app_wallet_creation::NewAccount;
use app::{ContractArgs, NearAppsContract};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONTRACT_BYTES => "res/app.wasm",
    TEST_FILE_BYTES => "res/status_message.wasm",
    MAKE_WALLET_BYTES => "../app-wallet-creation/res/app_wallet_creation.wasm",
    LINKDROP_BYTES => "../app-wallet-creation/res/linkdrop.wasm"
}

fn init() -> (UserAccount, ContractAccount<NearAppsContract>) {
    let mut genesis = near_sdk_sim::runtime::GenesisConfig::default();
    genesis.gas_limit = u64::MAX;
    genesis.gas_price = 0;
    let master_account = init_simulator(Some(genesis));
    master_account.deploy(&LINKDROP_BYTES, "near".parse().unwrap(), to_yocto("100"));
    let contract_account = deploy! {
        contract: NearAppsContract,
        contract_id: "contract",
        bytes: &CONTRACT_BYTES,
        signer_account: master_account
    };

    (master_account, contract_account)
}

#[test]
fn simulate_successful_call() {
    let (master_account, near_apps) = init();
    let status = master_account.deploy(&TEST_FILE_BYTES, "status".parse().unwrap(), to_yocto("35"));
    let status_id: near_sdk::AccountId = status.account_id;
    let status_amt = to_yocto("35");
    let message = json!({"message": "hello world"});
    let res = call!(
        near_apps.user_account,
        near_apps.add_contract(status_id.clone()),
        gas = DEFAULT_GAS
    );
    assert!(res.is_ok());

    let res = view!(near_apps.print_required_tags());
    println!("Required tags: {:#?}", res.logs());

    let mut map = HashMap::new();
    map.insert("person".to_string(), "Mike".to_string());
    map.insert("company".to_string(), "Near.org".to_string());
    map.insert("purpose".to_string(), "testing123".to_string());
    let arr = vec![map];
    let res = call!(
        master_account,
        near_apps.call(
            arr,
            status_id.clone(),
            ContractArgs::new("set_status".to_string(), message.to_string())
        ),
        gas = DEFAULT_GAS * 3
    );
    println!("status_message call: {:#?}", res.promise_results());
    assert!(res.is_ok());
}

#[test]
fn simulate_fail_call() {
    let (master_account, near_apps) = init();
    let status = master_account.deploy(&TEST_FILE_BYTES, "status".parse().unwrap(), to_yocto("35"));
    let status_id: near_sdk::AccountId = status.account_id;
    let message = json!({"message": "hello world"});
    call!(
        near_apps.user_account,
        near_apps.add_contract(status_id.clone()),
        gas = DEFAULT_GAS
    );
    let res = call!(
        master_account,
        near_apps.call(
            Vec::new(),
            status_id.clone(),
            ContractArgs::new("set_status".to_string(), message.to_string())
        ),
        gas = DEFAULT_GAS * 3
    );
    assert!(!res.is_ok());

    let mut map = HashMap::new();
    map.insert("person".to_string(), "Mike".to_string());
    map.insert("company".to_string(), "Near.org".to_string());
    //map.insert("purpose".to_string(), "testing123".to_string()); // missing key
    let arr = vec![map];
    let res = call!(
        master_account,
        near_apps.call(
            arr,
            status_id.clone(),
            ContractArgs::new("set_status".to_string(), message.to_string())
        ),
        gas = DEFAULT_GAS * 3
    );
    assert!(!res.is_ok());
}

#[test]
fn simulate_wallet_call() {
    let (master_account, near_apps) = init();
    let make_wallet = master_account.deploy(
        &MAKE_WALLET_BYTES,
        "status".parse().unwrap(),
        to_yocto("35"),
    );
    let make_wallet_id = make_wallet.account_id;
    let account_id = "make_wallet_1".parse().unwrap();
    let public_key =
        PublicKey::from_str("ed25519:Bnsj1BXvhRaJuA316v5GWk6M5G3Wyq8ZEVJHtorXt1DP").unwrap();
    let new_account = NewAccount::new(
        account_id,
        public_key,
    );
    let make_wallet_json = json!({ "new_account": new_account });
    let res = call!(
        near_apps.user_account,
        near_apps.add_contract(make_wallet_id.clone()),
        gas = DEFAULT_GAS
    );
    assert!(res.is_ok());
    let res = call!(
        near_apps.user_account,
        near_apps.any_tags_allowed(true),
        gas = DEFAULT_GAS
    );
    assert!(res.is_ok());
    let res = call!(
        near_apps.user_account,
        near_apps.call(
            Vec::new(),
            make_wallet_id,
            ContractArgs::new("make_wallets".to_string(), make_wallet_json.to_string())
        ),
        to_yocto("1"),
        DEFAULT_GAS * 3
    );
    println!("make_wallet called: {:#?}", res.promise_results());
    assert!(res.is_ok());
    assert!(
        matches!(&res.outcome().status, ExecutionStatus::SuccessValue(x) if x == &b"true".to_vec())
    );
}
