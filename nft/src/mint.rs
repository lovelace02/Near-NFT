use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(
        &mut self,
    ) {
        if (env::block_timestamp() / 1000000) < 1651651200000 {
            env::panic(b"Not time for WL mint");
        }
        assert!(self.token_metadata_by_id.len() < 777, "Minting ended");

        let account_id = env::predecessor_account_id();

        let amount = env::attached_deposit();

        if amount == 0 {
            let end = self.contributor_0.len();
            let mut exist = false;
            for i in 0..end {
                if account_id == self.contributor_0.get(i).unwrap(){
                    self.contributor_0.swap_remove(i);
                    exist = true;
                    break;
                }
            }
            if exist == false {
                env::panic(b"Require correct amount of Near attached");
            }
        } else if amount == 4000000000000000000000000 {
            let end = self.contributor_4.len();
            let mut exist = false;
            for i in 0..end {
                if account_id == self.contributor_4.get(i).unwrap(){
                    self.contributor_4.swap_remove(i);
                    exist = true;
                    break;
                }
            }
            if exist == false {
                env::panic(b"Require correct amount of Near attached");
            }
        } else if amount == 7000000000000000000000000 {
            let end = self.contributor_7.len();
            let mut exist = false;
            for i in 0..end {
                if account_id == self.contributor_7.get(i).unwrap(){
                    self.contributor_7.swap_remove(i);
                    exist = true;
                    break;
                }
            }
            if exist == false {
                env::panic(b"Require correct amount of Near attached");
            }
        } else if amount == 11000000000000000000000000 {
            if (env::block_timestamp() / 1000000) < 1651665600000 {
                env::panic(b"Not time for public mint");
            }
        } else {
            env::panic(b"Require correct amount of Near attached");
        }

        let mut index = 0;
        if self.remain_ids.len() > 1 {
            index =(env::block_timestamp() / 1000000) % self.remain_ids.len();
        }
        let token_id = self.remain_ids.get(index).unwrap();

        if self.remain_ids.len() > 1 {
            self.remain_ids.swap_remove(index);
        }

        // let token_id = 777.to_string();

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        // create a royalty map to store in the token
        let mut royalty = HashMap::new();
        royalty.insert("xuguangxia725.near".to_string().try_into().unwrap(), 100);
        royalty.insert("zerotime.near".to_string().try_into().unwrap(), 100);
        royalty.insert("luciddream.near".to_string().try_into().unwrap(), 100);
        royalty.insert("terraspaces-treasury.near".to_string().try_into().unwrap(), 300);
        
        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: account_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &TokenMetadata{
            title: Some("Dungeon Goblin #".to_owned() + token_id.as_ref()),
            description: Some("Genesis collection of 777 abstract NFTs. The First Generative Landmarks Collection on NEAR featuring Gated-via-Staking Access to Analaytical Dashboard. Tap into revenue generation via Staking-as-a-Service [SaaS] business model.".to_owned()),
            media: Some(token_id.clone() + ".png"),
            media_hash: None,
            copies: None,
            issued_at: Some(env::block_timestamp() / 1000000),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: Some(token_id.clone() + ".json"),
            reference_hash: None
        });

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        // //calculate the required storage which was the used - initial
        // let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        // refund_deposit(required_storage_in_bytes);
    }
}