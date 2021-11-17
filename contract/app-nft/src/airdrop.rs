use near_contract_standards::non_fungible_token::TokenId;
use crate::*;
use near_sdk::{AccountId};
use near_sdk::serde::{Serialize, Deserialize};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropRewards(pub Vec<AirdropReward>);

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AirdropReward {
    pub account_id: AccountId,
    pub token_id: TokenId,
}

pub trait SupportsAirdrop {
    fn airdrop(&mut self, rewards: AirdropRewards);
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, TokenId)>);
    fn pending_rewards_by_key(&self, account: &AccountId) -> TokenId;
}

#[near_bindgen]
impl SupportsAirdrop for NftContract {
    fn add_pending_rewards(&mut self, rewards: Vec<(AccountId, TokenId)>) {
        self.assert_owner();
        for reward in rewards {
            let account_id = reward.0.to_string();
            let token_id = reward.1;
            self.pending_nft_rewards.insert(&account_id, &token_id);
        }
    }

    fn airdrop(&mut self, rewards: AirdropRewards) {
        self.assert_owner();
        for reward in rewards.0 {
            let account = reward.account_id.to_string();
            self.nft_transfer_unsafe(&reward.token_id, &self.owner(), &account);
        }
    }

    fn pending_rewards_by_key(&self, account: &AccountId) -> TokenId {
        self.pending_nft_rewards.get(&account).unwrap()
    }
}
