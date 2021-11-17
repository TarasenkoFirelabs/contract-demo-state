mod internal;
mod mint;
mod series;
mod query;
mod upgrade;
pub mod airdrop;

//mod airdrop;
use crate::internal::*;
use crate::series::*;
use near_sdk::collections::LookupMap;
use near_sdk::collections::UnorderedMap;

use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{
    metadata::{
        NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
    },
    Token, TokenId,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::Balance;
use near_sdk::Gas;
use near_sdk::{

    collections::LazyOption, env, ext_contract, near_bindgen, serde_json::json, AccountId,
    BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, PromiseResult,
};
use std::convert::*;

//sdk init
near_sdk::setup_alloc!();

//Constants
pub const TOKEN_DELIMETER: char = ':';
pub const TITLE_DELIMETER: &str = " #";
pub const EDITION_DELIMETER: &str = "/";
pub const GAS_FOR_ROYALTIES: Gas = 0;
pub const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NftContract {
    token: NonFungibleToken,
    token_series: UnorderedMap<NftSeriesId, NftSeries>,
    owner_id: AccountId,
    metadata: LazyOption<NFTContractMetadata>,
    pending_nft_rewards: LookupMap<AccountId, TokenId>,
    total_supply :u128,
}
#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    TokenMetadata,
    TokenSeriesById,
    Enumeration,
    Approval,
    Metadata,
    PendingRewards,
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
impl NonFungibleTokenMetadataProvider for NftContract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

#[near_bindgen]
impl NftContract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Neap Apps".to_string(),
                symbol: "NAPP".to_string(),
                icon: None,
                base_uri: Some("https://ipfs.fleek.co/ipfs".to_string()),
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert_initialized();
        metadata.assert_valid();
        let owner = ValidAccountId::try_from(owner_id.clone()).expect("Invalid AccountId");

        let nft = NonFungibleToken::new(
            StorageKey::NonFungibleToken,
            owner,
            Some(StorageKey::TokenMetadata),
            Some(StorageKey::Enumeration),
            Some(StorageKey::Approval),
        );
        Self {
            owner_id,
            token: nft,
            token_series: UnorderedMap::new(StorageKey::TokenSeriesById),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            pending_nft_rewards: LookupMap::new(StorageKey::PendingRewards),
            total_supply: 0,
        }
    }
    pub(crate) fn nft_transfer_unsafe(
        &mut self,
        token_id: &TokenId,
        owner_id: &AccountId,
        receiver_id: &AccountId,
    ) {
        self.token
            .internal_transfer_unguarded(token_id, owner_id, receiver_id);
        env::log(
            json!({
                "type": "nft_transfer",
                "params": {
                    "token_id": token_id,
                    "sender_id": owner_id,
                    "receiver_id": receiver_id
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

     pub(crate) fn internal_remove_token(&mut self,token_id:&TokenId, owner_id:&AccountId){
        
        if let Some(tokens_per_owner) = &mut self.token.tokens_per_owner {
            let mut token_ids = tokens_per_owner.get(&owner_id).unwrap();
            token_ids.remove(&token_id);
            tokens_per_owner.insert(&owner_id, &token_ids);
        }
        
        self.token.owner_by_id.remove(&token_id);

        if let Some(token_metadata_by_id) = &mut self.token.token_metadata_by_id {
            token_metadata_by_id.remove(&token_id);
        }
    }
}

near_contract_standards::impl_non_fungible_token_core!(NftContract, token);
near_contract_standards::impl_non_fungible_token_approval!(NftContract, token);
near_contract_standards::impl_non_fungible_token_enumeration!(NftContract, token);
