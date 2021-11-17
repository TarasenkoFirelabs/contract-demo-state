use crate::*;
use near_sdk::collections::UnorderedSet;
use near_sdk::json_types::U64;
#[near_bindgen]
impl NftContract {
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: ValidAccountId,
        metadata: TokenMetadata,
    ) -> TokenId {
        self.assert_owner();
        let token = self.token.mint(token_id, receiver_id, Some(metadata));
        
        token.token_id
    }
    #[payable]
    pub fn nft_create_series(
        &mut self,
        series_id: NftSeriesId,
        token_metadata: TokenMetadata,
        price: Option<Balance>,
    ) -> NftSeriesJson {
        let initial_storage_usage = env::storage_usage();
        let creator_id = env::predecessor_account_id();

        assert!(
            self.token_series.get(&series_id).is_none(),
            "Near Apps: duplicate series_id"
        );

        let title = token_metadata.title.clone();
        assert!(
            title.is_some(),
            "Near Apps: token_metadata.title is required"
        );

        self.token_series.insert(
            &series_id,
            &NftSeries {
                series_id: (*&series_id).clone(),
                metadata: token_metadata.clone(),
                creator_id: creator_id.to_string(),
                tokens: UnorderedSet::new(
                    StorageKey::TokensBySeriesInner {
                        identifier: series_id.clone(),
                    }
                    .try_to_vec()
                    .unwrap(),
                ),
                closed: false,
                price: price,
                is_mintable: true,
            },
        );

        env::log(
            json!({
                "type": "nft_create_series",
                "params": {
                    "series_id": series_id,
                    "token_metadata": token_metadata,
                    "creator_id": creator_id,
                    "price": price,
                }
            })
            .to_string()
            .as_bytes(),
        );

        refund_deposit(env::storage_usage() - initial_storage_usage, 0);

        NftSeriesJson {
            series_id,
            metadata: token_metadata,
            creator_id: creator_id.into(),
        }
    }

    fn nft_mint_series_internal(
        &mut self,
        token_series_id: NftSeriesId,
        receiver_id: ValidAccountId,
    ) -> TokenId {
        let mut token_series = self
            .token_series
            .get(&token_series_id)
            .expect("Near Apps: Token series not exist");
        assert!(
            token_series.is_mintable,
            "Near Apps: Token series is not mintable"
        );

        let num_tokens = token_series.tokens.len();
        let max_copies = token_series.metadata.copies.unwrap_or(u64::MAX);
        assert!(num_tokens < max_copies, "Series supply maxed");

        if (num_tokens + 1) >= max_copies {
            token_series.is_mintable = false;
        }

        let token_id = format!("{}{}{}", &token_series_id, TOKEN_DELIMETER, num_tokens + 1);
        token_series.tokens.insert(&token_id);
        self.token_series.insert(&token_series_id, &token_series);

        // you can add custom metadata to each token here
        let metadata = Some(TokenMetadata {
            title: None,       // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
            description: None, // free-form description
            media: None, // URL to associated media, preferably to decentralized, content-addressed storage
            media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
            copies: None, // number of copies of this set of metadata in existence when token was minted.
            issued_at: Some(env::block_timestamp().to_string()), // ISO 8601 datetime when token was issued or minted
            expires_at: None,     // ISO 8601 datetime when token expires
            starts_at: None,      // ISO 8601 datetime when token starts being valid
            updated_at: None,     // ISO 8601 datetime when token was last updated
            extra: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
            reference: None, // URL to an off-chain JSON file with more info.
            reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
        });

        //let token =
        self.token.mint(token_id.clone(), receiver_id, metadata);
        token_id
    }

    #[payable]
    pub fn nft_set_series_non_mintable(&mut self, token_series_id: NftSeriesId) {
        assert_one_yocto();

        let mut token_series = self
            .token_series
            .get(&token_series_id)
            .expect("Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Near Apps: Creator only"
        );

        assert_eq!(
            token_series.is_mintable, true,
            "Near Apps: already non-mintable"
        );

        assert_eq!(
            token_series.metadata.copies, None,
            "Near Apps: decrease supply if copies not null"
        );

        token_series.is_mintable = false;
        self.token_series.insert(&token_series_id, &token_series);
        env::log(
            json!({
                "type": "nft_set_series_non_mintable",
                "params": {
                    "token_series_id": token_series_id,
                }
            })
            .to_string()
            .as_bytes(),
        );
    }

    #[payable]
    pub fn nft_decrease_series_copies(
        &mut self,
        token_series_id: NftSeriesId,
        decrease_copies: U64,
    ) -> U64 {
        assert_one_yocto();

        let mut token_series = self
            .token_series
            .get(&token_series_id)
            .expect("Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Near Apps: Creator only"
        );

        let minted_copies = token_series.tokens.len();
        let copies = token_series.metadata.copies.unwrap();

        assert!(
            (copies - decrease_copies.0) >= minted_copies,
            "Near Apps: cannot decrease supply, already minted : {}",
            minted_copies
        );

        let is_non_mintable = if (copies - decrease_copies.0) == minted_copies {
            token_series.is_mintable = false;
            true
        } else {
            false
        };

        token_series.metadata.copies = Some(copies - decrease_copies.0);

        self.token_series.insert(&token_series_id, &token_series);
        env::log(
            json!({
                "type": "nft_decrease_series_copies",
                "params": {
                    "token_series_id": token_series_id,
                    "copies": U64::from(token_series.metadata.copies.unwrap()),
                    "is_non_mintable": is_non_mintable,
                }
            })
            .to_string()
            .as_bytes(),
        );
        U64::from(token_series.metadata.copies.unwrap())
    }

    #[payable]
    pub fn nft_mint_series(
        &mut self,
        series_id: NftSeriesId,
        receiver_id: ValidAccountId,
    ) -> TokenId {
        let initial_storage_usage = env::storage_usage();

        let token_series = self
            .token_series
            .get(&series_id)
            .expect("Near Apps: Token series not exist");
        assert_eq!(
            env::predecessor_account_id(),
            token_series.creator_id,
            "Near Apps: not creator"
        );
        let token_id: TokenId = self.nft_mint_series_internal(series_id, receiver_id.clone());
        
        refund_deposit(env::storage_usage() - initial_storage_usage, 0);
        env::log(
            json!({
                "type": "nft_transfer",
                "params": {
                    "token_id": token_id.clone(),
                    "sender_id": "",
                    "receiver_id": receiver_id.to_string(),
                }
            })
            .to_string()
            .as_bytes(),
        );

        token_id
    }
}
