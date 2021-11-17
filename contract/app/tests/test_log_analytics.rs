use near_sdk_sim::{call, deploy, init_simulator, ContractAccount, UserAccount, to_yocto};
use std::str;

extern crate app;
use app::NearAppsContract;

extern crate base64;
use base64::encode;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    ANALYTICS_BYTES => "res/app.wasm",
}

const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, ContractAccount<NearAppsContract>, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let contract = deploy!(
        contract: NearAppsContract,
        contract_id: CONTRACT_ID,
        bytes: &ANALYTICS_BYTES,
        signer_account: root
    );

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}

#[test]
fn simulate_log_analytics_1() {
    let (root, contract, _) = init();

    let app_id = "some_id".to_string();
    let action_id = "some_action_id".to_string();
    let user_name = "some_user_name".to_string();
    let initial = format!(
        "app_id: {}, action_id: {}, user_name: {}",
        app_id, action_id, user_name);
    let encoded: String = format!("{}_{}_{}", encode(app_id), encode(action_id), encode(user_name));

    let result = call!(
        root,
           contract.log_analytics(encoded)
    );
    result.assert_success();

    let decoded: String = (*result.logs()[0]).to_string(); 

    assert_eq!(initial, decoded);
}

#[test]
fn simulate_log_analytics_2() {
    let (root, contract, _) = init();

    let app_id = "another_id".to_string();
    let action_id = "another_action_id".to_string();
    let user_name = "another_user_name".to_string();
    let initial = format!(
        "app_id: {}, action_id: {}, user_name: {}",
        app_id, action_id, user_name);
    let encoded: String = format!("{}_{}_{}", encode(app_id), encode(action_id), encode(user_name));

    let result = call!(
        root,
        contract.log_analytics(encoded)
    );
    result.assert_success();

    let decoded: String = (*result.logs()[0]).to_string(); 

    assert_eq!(initial, decoded);
}

#[should_panic]
#[test]
fn simulate_log_analytics_panic() {
    let (root, contract, _) = init();

    let app_id = "another_id".to_string();
    let action_id = "another_action_id".to_string();
    let user_name = "another_user_name".to_string();
    let initial = format!(
        "app_id: {}, action_id: {}, user_name: {}",
        app_id, action_id, user_name);
    let encoded: String = format!("{}_{}, {}", encode(app_id), encode(action_id), encode(user_name));

    let result = call!(
        root,
        contract.log_analytics(encoded)
    );
    result.assert_success();

    let decoded: String = (*result.logs()[0]).to_string(); 

    assert_eq!(initial, decoded);
}