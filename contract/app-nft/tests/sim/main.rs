use near_sdk_sim::{call, view, deploy, init_simulator, ContractAccount, UserAccount, to_yocto, DEFAULT_GAS};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::TokenId;
use std::str;
//use near_sdk::env;

use std::convert::{TryFrom, TryInto};
use near_sdk::json_types::ValidAccountId;

extern crate app_nft;
use app_nft::NftContractContract;
use app_nft::airdrop::{AirdropReward, AirdropRewards};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    AIRDROP_BYTES => "res/app_nft.wasm",
}

const CONTRACT_ID: &str = "contract";

pub fn init() -> (UserAccount, ContractAccount<NftContractContract>, UserAccount) {
    let mut genesis = near_sdk_sim::runtime::GenesisConfig::default();
    genesis.gas_limit = u64::MAX;
    genesis.gas_price = 0;
    let root = init_simulator(Some(genesis));
    let contract = deploy! {
        contract: NftContractContract,
        contract_id: "contract",
        bytes: &AIRDROP_BYTES,
        signer_account: root
    };

    let alice = root.create_user(
        "alice".parse().unwrap(),
        to_yocto("100") // initial balance
    );

    (root, contract, alice)
}

#[test]
//#[should_panic(expected = "Ownable: predecessor is not the owner")]
fn simulate_airdrop_default_meta() {
    let (root, contract, alice) = init();

    let valid_account: ValidAccountId = root.account_id().clone().try_into().unwrap();
    let res = call!(
        root, contract.new_default_meta(valid_account.to_string()));

    let token_meta = TokenMetadata{
        title: Some("TestMetadata".to_string()),
        description: None,
        media: None,
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    };

    let valid_account: ValidAccountId = root.account_id().clone().try_into().unwrap();
    
    let token_id: TokenId = call!(
        root,
        contract.nft_mint("New_test_token".to_string(), valid_account, token_meta),
        deposit = to_yocto("0.59")
    ).unwrap_json();
    
    let reward = AirdropReward {
        account_id: alice.account_id().clone(),
        token_id: token_id.clone(),
    };
    let rewards = AirdropRewards(vec![reward]);
    
    let res = call!(
        root, 
        contract.add_pending_rewards(vec![(alice.account_id().clone(), token_id.clone())])
    );
    res.assert_success();
    let res = call!(
        root,
        contract.airdrop(rewards)
    );
    res.assert_success();
    let res: TokenId = view!(
        contract.pending_rewards_by_key(&alice.account_id())
    ).unwrap_json();
    assert!(&res.eq(&token_id));
}


#[test]
#[should_panic(expected = "Ownable: predecessor is not the owner")]
fn simulate_airdrop_default_meta_panic() {
    let (root, contract, alice) = init();

    let valid_account: ValidAccountId = root.account_id().clone().try_into().unwrap();
    let res = call!(
        root, contract.new_default_meta(valid_account.to_string()));

    let token_meta = TokenMetadata{
        title: Some("TestMetadata".to_string()),
        description: None,
        media: None,
        media_hash: None,
        copies: None,
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    };

    let valid_account: ValidAccountId = root.account_id().clone().try_into().unwrap();
    
    let token_id: TokenId = call!(
        root,
        contract.nft_mint("New_test_token".to_string(), valid_account, token_meta),
        deposit = to_yocto("0.59")
    ).unwrap_json();
    
    let res = call!(
        alice, 
        contract.add_pending_rewards(vec![(alice.account_id().clone(), token_id.clone())])
    );
    res.assert_success();
}
