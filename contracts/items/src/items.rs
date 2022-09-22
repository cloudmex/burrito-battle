use near_sdk::{
    env, serde_json::json
};

use crate::*;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
//const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {
    
    pub fn nft_mint(&mut self,token_owner_id: AccountId, token_metadata: TokenMetadata) -> String{
        self.assert_owner();
        let mut new_item = token_metadata;
        let item_id: TokenId = (self.token_metadata_by_id.len()+1).to_string();

        let newextradata = str::replace(&new_item.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraAccessory = serde_json::from_str(&newextradata).unwrap();


        /*Obtener imagen
        if rand_image > 0 &&  rand_image <= 64 {
            burrito_image = BURRITO1.to_string();
        }
        if rand_image > 64 &&  rand_image <= 128 {
            burrito_image = BURRITO2.to_string();
        }
        if rand_image > 128 &&  rand_image <= 192 {
            burrito_image = BURRITO3.to_string();
        }
        if rand_image > 192 &&  rand_image < 255 {
            burrito_image = BURRITO4.to_string();
        }*/

        //new_item.media = Some(burrito_image);


        let royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: token_owner_id.clone(),
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&item_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&item_id, &new_item);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &item_id);

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
                token_ids: vec![item_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());


        let accessory = Accessory {
            owner_id : token_owner_id.clone().to_string(),
            name : new_item.title.as_ref().unwrap().to_string(),
            description : new_item.description.as_ref().unwrap().to_string(),
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed
            };


        env::log(
            json!(accessory)
            .to_string()
            .as_bytes(),
        );

        serde_json::to_string(&accessory).unwrap()
    }

    pub fn get_number_accessories(&self) -> u64 {
        self.token_metadata_by_id.len()
    }
}

   

