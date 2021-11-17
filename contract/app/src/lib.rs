//! Some js examples will be here.

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto,Into};
use std::str;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{self, json};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Promise, PromiseResult};
extern crate base64;
use base64::decode;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct AnalyticsData {
    account_id: AccountId,
    app_id: String,
    action_id: String,
    hash: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearApps {
    any_contracts: bool,
    any_tags: bool,
    owner_id: AccountId,
    approved_contracts: LookupSet<AccountId>,
    required_tags: UnorderedSet<String>,
    analytics_log: LookupSet<AnalyticsData>,
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn check_promise(tags: Vec<HashMap<String, String>>) -> bool;
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractArgs {
    function_name: String,
    params: String,
}

impl ContractArgs {
    ///Creates a new instance of ContractArgs from given data.
    pub fn new(function_name: String, params: String) -> Self {
        Self {
            function_name,
            params,
        }
    }
}

pub trait Ownable {
    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner(),
            "Ownable: predecessor is not the owner"
        );
    }
    fn owner(&self) -> AccountId;
    fn transfer_ownership(&mut self, owner: AccountId);
}

#[near_bindgen]
impl Ownable for NearApps {
    fn owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn transfer_ownership(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}

impl Default for NearApps {
    fn default() -> Self {
        let mut required_tags = UnorderedSet::new(b"t");
        required_tags.insert(&"person".to_string());
        required_tags.insert(&"company".to_string());
        required_tags.insert(&"purpose".to_string());
        Self {
            any_contracts: false,
            any_tags: false,
            owner_id: env::current_account_id(),
            approved_contracts: LookupSet::new(b"c"),
            required_tags,
            analytics_log: LookupSet::new(b"l"),
        }
    }
}

#[near_bindgen]
impl NearApps {
    ///Creates a new instance of NearApps from given data.
    ///any_contracts and any_tags are set to false.
    #[init]
    pub fn new(
        owner_id: AccountId,
        init_tags: Vec<String>,
        init_contracts: Vec<AccountId>,
    ) -> Self {
        let mut required_tags = UnorderedSet::new(b"t");
        required_tags.extend(init_tags);
        let mut approved_contracts = LookupSet::new(b"c");
        approved_contracts.extend(init_contracts);

        Self {
            any_contracts: false,
            any_tags: false,
            owner_id,
            approved_contracts,
            required_tags,
            analytics_log: LookupSet::new(b"c"),
        }
    }

    ///Payable function. Returns Promise.
    #[payable]
    pub fn call(
        &mut self,
        tags: Vec<HashMap<String, String>>,
        contract_name: AccountId,
        args: ContractArgs,
    ) -> Promise {
        self.verify_tags(&tags);
        self.verify_contract(&contract_name);
        Promise::new(contract_name)
            .function_call(
                args.function_name,
                args.params.into_bytes(),
                env::attached_deposit(),
                env::prepaid_gas() / 3,
            )
            .then(ext_self::check_promise(
                tags,
                env::current_account_id(),
                0,
                env::prepaid_gas() / 3,
            ))
    }

    fn verify_contract(&self, contract_name: &AccountId) {
        if !self.any_contracts && !self.approved_contracts.contains(contract_name) {
            env::panic_str("missing allowed contract");
        }
    }

    fn verify_tags(&self, tags: &Vec<HashMap<String, String>>) {
        if !self.any_tags {
            if tags.len() == 0 {
                env::panic_str("empty tags");
            }
            for str in self.required_tags.iter() {
                for tag in tags {
                    if !tag.contains_key(&str) {
                        env::panic_str("missing key");
                    }
                }
            }
        }
    }

    ///Logs all of the required tags.
    pub fn print_required_tags(self) {
        let s = format!("{:?}", self.required_tags.iter().collect::<Vec<String>>());
        env::log_str(&s[1..s.len()]);
    }

    ///Adds contract_name to the list of approved contracts.
    ///Can only be called by the owner.
    pub fn add_contract(&mut self, contract_name: AccountId) {
        self.assert_owner();
        self.approved_contracts.insert(&contract_name);
    }

    ///Removes contract from the list of approved contracts.
    ///Can only be called by the owner.
    pub fn remove_contract(&mut self, contract_name: AccountId) {
        self.assert_owner();
        self.approved_contracts.remove(&contract_name);
    }

    ///Sets the status of the field any_contracts.
    ///If true, all contracts are automatically verified.
    ///Can only be called by the owner.
    pub fn any_contracts_allowed(&mut self, any: bool) {
        self.assert_owner();
        self.any_contracts = any;
    }

    ///Adds new tag.
    ///Can only be called by the owner.
    pub fn add_tag(&mut self, tag_name: String) {
        self.assert_owner();
        self.required_tags.insert(&tag_name);
    }

    ///Removes the given tag.
    ///Can only be called by the owner.
    pub fn remove_tag(&mut self, tag_name: String) {
        self.assert_owner();
        self.required_tags.remove(&tag_name);
    }

    ///Sets the status of the field tags.
    ///If true, all tags are automatically verified.
    ///Can only be called by the owner.
    pub fn any_tags_allowed(&mut self, any: bool) {
        self.assert_owner();
        self.any_tags = any;
    }

    ///Decodes given string and logs resulting app_id, action_id and account_id.
    pub fn log_analytics(&mut self, encoded: String) {
        let call_encoded: Vec<&str> = encoded.split('_').collect();
        let mut call_decoded: Vec<String> = Vec::new();
        for i in 0..3 {
            let decoded = str::from_utf8(&decode(call_encoded[i]).unwrap())
                .unwrap()
                .to_string();
            call_decoded.push(decoded);
        }

        env::log_str(&format!(
            "app_id: {}, action_id: {}, user_name: {}",
            call_decoded[0], call_decoded[1], call_decoded[2]
        ));

        let analytics_data = AnalyticsData {
            app_id: call_decoded[0].clone(),
            action_id: call_decoded[1].clone(),
            account_id: AccountId::new_unchecked(call_decoded[2].clone()),
            hash: encoded,
        };
        self.analytics_log.insert(&analytics_data);
    }
 
    ///Can only be called by predecessor_account_id().
    #[private]
    pub fn check_promise(&mut self, tags: Vec<HashMap<String, String>>) -> bool {
        assert_eq!(env::promise_results_count(), 1, "ERR_TOO_MANY_RESULTS");
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                if tags.len() > 0 {
                    env::log_str(&serde_json::to_string(&tags).unwrap());
                }
                true
            }
            _ => env::panic_str("Promise with index 0 failed"),
        }
    }
}