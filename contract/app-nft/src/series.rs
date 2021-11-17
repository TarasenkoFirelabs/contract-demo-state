use crate::*;

use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::collections::UnorderedSet;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, env, near_bindgen, serde_json::json};
use std::collections::HashMap;


pub type NftSeriesId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftSeries {
    pub series_id: NftSeriesId,
    pub metadata: TokenMetadata,
    pub creator_id: AccountId,
    pub tokens: UnorderedSet<TokenId>,
    pub price: Option<Balance>,
    pub is_mintable: bool,
    pub closed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftSeriesJson {
    pub series_id: NftSeriesId,
    pub metadata: TokenMetadata,
    pub creator_id: AccountId,
}

