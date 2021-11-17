use std::str;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen};

extern crate base64;
use base64::decode;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Call {}
#[near_bindgen]
impl Call {
    pub fn log_analytics(encoded: String) {
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
    }
}
