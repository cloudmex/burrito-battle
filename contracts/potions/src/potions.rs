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
    // Minar un nuevo token 600,000 $STRW tokens + 5 $NEAR tokens
    #[payable]
    pub fn nft_mint(&mut self,token_owner_id: AccountId, token_metadata: TokenMetadata, type_potion: String) -> Potion {
        let account_id = env::signer_account_id();        
        let deposit = env::attached_deposit();

        let initial_storage_usage = env::storage_usage();
        let deposit = env::attached_deposit();   

        let mut new_potion = token_metadata;
        let potion_id: TokenId = (self.token_metadata_by_id.len()).to_string();
        let mut potion_data = ExtraPotion {
            potion_type: "".to_string(),
            points_increment : "".to_string()
        };

        let mut extra_data_string = serde_json::to_string(&potion_data).unwrap();
        extra_data_string = str::replace(&extra_data_string, "\"", "'");
        new_potion.extra = Some(extra_data_string);
        let mut name_potion = "".to_string();
        let mut desription_potion = "".to_string();

        if type_potion != "Health" && type_potion != "Attack" && type_potion != "Shield" {
            env::panic_str("El tipo de posión a minar no existe");
        }

        if type_potion == "Health" {
            potion_data.potion_type = "Health".to_string();
            potion_data.points_increment = "10".to_string();
            new_potion.media = Some(POTIONHEALTH.to_string());
            name_potion = "Posion de Salud".to_string()+&" #".to_string()+&self.token_metadata_by_id.len().to_string();
            desription_potion = "Esta es una posión de aumento de salud".to_string();
        }
        if type_potion == "Attack" {
            potion_data.potion_type = "Attack".to_string();
            potion_data.points_increment = "3".to_string();
            new_potion.media = Some(POTIONATTACKS.to_string());
            name_potion = "Posion de Ataques".to_string()+&" #".to_string()+&self.token_metadata_by_id.len().to_string();
            desription_potion = "Esta es una posión de aumento de ataques pesados".to_string();
        }
        if type_potion == "Shield" {
            potion_data.potion_type = "Shield".to_string();
            potion_data.points_increment = "3".to_string();
            new_potion.media = Some(POTIONSHIELDS.to_string());
            name_potion = "Posion de Escudos".to_string()+&" #".to_string()+&self.token_metadata_by_id.len().to_string();
            desription_potion = "Esta es una posión de aumento de escudos".to_string();
        }

        new_potion.title = Some(name_potion);
        new_potion.description = Some(desription_potion);

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
            self.tokens_by_id.insert(&potion_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&potion_id, &new_potion);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &potion_id);

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
                token_ids: vec![potion_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);

        let potion = Potion {
            owner_id : token_owner_id.clone().to_string(),
            name : new_potion.title.as_ref().unwrap().to_string(),
            description : new_potion.description.as_ref().unwrap().to_string(),
            potion_type : potion_data.potion_type,
            points_increment : potion_data.points_increment,
            media : new_potion.media.as_ref().unwrap().to_string()
        };

        potion
    }

    pub fn get_number_potions(&self) -> u64 {
        self.token_metadata_by_id.len()
    }

    pub fn get_potion(&self, potion_id: TokenId) -> Potion {
        if potion_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe la posion con el id ingresado");
        }
    
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&potion_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        if account_id.clone() != owner_id.clone().parse::<AccountId>().unwrap() {
            env::panic_str("La posion no te pertenece");
        }

        let metadata = self.token_metadata_by_id.get(&potion_id).unwrap();
        let token = self.tokens_by_id.get(&potion_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraPotion = serde_json::from_str(&newextradata).unwrap();

        let potion = Potion {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            potion_type : extradatajson.potion_type,
            points_increment : extradatajson.points_increment,
            media : metadata.media.as_ref().unwrap().to_string()
        };

        potion
    }

}