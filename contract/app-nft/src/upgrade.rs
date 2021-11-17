use crate::*;

pub trait UpgradableNFT {
    fn upgrade(&mut self,token_id:TokenId, account_id: ValidAccountId, upgrage_to_id: NftSeriesId) -> TokenId;
}

impl UpgradableNFT for NftContract {
    fn upgrade(&mut self, token_id:TokenId,account_id: ValidAccountId, upgrage_to_id: NftSeriesId) -> TokenId {
        let _ = self
            .token_series
            .get(&upgrage_to_id)
            .expect("Upgrade series does not exist");

        assert_eq!(
            self.token_series.len() < 3,
            true,
            "Upgrade series does not exist"
        );

        //Find old series index
        let old_index=(self.token_series.len() - 2).try_into().unwrap();
        //Find 
        let (_,mut old_series) = self
            .token_series
            .iter()
            .nth(old_index)
            .expect("This NFT is not upgradable");

        //Remove old one from token series
        self.internal_remove_token(&token_id, account_id.as_ref());
        old_series.tokens.remove(&token_id);

        //Mint new token in series
        self.nft_mint_series(upgrage_to_id, account_id)
    }
}
/*
--30 NFT Series

Series1 1000 copies
Series2 1000 copies
Series2 1000 copies

User1
Series1/copy1

User2
Series1/copy2

--Upgrade for user 1, but only user has a copy from series1

--After Update:
User1
Series2/copy1

*/
