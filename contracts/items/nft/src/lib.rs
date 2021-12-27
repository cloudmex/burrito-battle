//Implementación de los standards NFT de near
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{ValidAccountId,Base64VecU8};
use std::sync::{Mutex};
use lazy_static::lazy_static;
use near_sdk::{
    env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
    Promise, PromiseOrValue,};
near_sdk::setup_alloc!();
use std::convert::TryInto;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: NonFungibleToken,
    accessories: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u64,
    n_accessories: u64,
    accessories_hash_map:HashMap<TokenId, Vec<String>>
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Accessory {
    // token_id : String,
    owner_id : String,
    name : String,
    description : String,
    attack : String,
    defense : String,
    speed : String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraAccessory {
    attack : String,
    defense : String,
    speed : String
}

lazy_static! {
    static ref USER_TOKEN_HASHMAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref CONV_MAP: HashMap<String, String> = {
        let mut map = HashMap::new();  
        map
    };
}

impl Default for Contract {
    fn default( ) -> Self {      
        let meta = NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Burrito Battle Accessories".to_string(),
            symbol: "BB".to_string(),
            icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            base_uri: None,
            reference: None,
            reference_hash: None,
        };
        Self {
            tokens:NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                env::signer_account_id().try_into().unwrap(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            accessories: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                env::signer_account_id().try_into().unwrap(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&meta)),
            n_tokens: 0,
            n_accessories: 0,
            accessories_hash_map:HashMap::new()
        }   
    }
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init_contract(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Burrito Battle NFT".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            accessories: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            n_tokens: 0,
            n_accessories: 0,
            accessories_hash_map:HashMap::new()
        }
    }

    // Obtener cantidad de accesorios creados
    pub fn get_number_accessories(&self) -> u64 {
        self.n_accessories
    }

    //Minar un nuevo accesorio  
    #[payable]
    pub fn new_accessory(&mut self,accessory_id: TokenId,receiver_id: ValidAccountId,accessory_metadata: TokenMetadata) -> Accessory {
        self.accessories.mint(accessory_id.clone(), receiver_id, Some(accessory_metadata.clone()));
        self.n_accessories += 1;

        let newextradata = str::replace(&accessory_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraAccessory = serde_json::from_str(&newextradata).unwrap();
        let owner_id = self.accessories.owner_by_id.get(&accessory_id.clone()).unwrap();

        let accessory = Accessory {
            owner_id : owner_id.to_string(),
            name : accessory_metadata.title.as_ref().unwrap().to_string(),
            description : accessory_metadata.description.as_ref().unwrap().to_string(),
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed
        };

        //Insertar nuevo token a Hashmap
        let mut info:Vec<String>=Vec::new();
        //info[0] owner_id
        info.push(accessory.owner_id.clone());
        //info[1] name
        info.push(accessory.name.clone());
        let mut _map =self.accessories_hash_map.clone();
        _map.insert(accessory_id.clone(),info);
        self.accessories_hash_map=_map.clone();
        
        accessory
    }

    // Obtener accesorio
    pub fn get_accessory(&self, accessory_id: TokenId) -> Accessory {
        let metadata = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accessory_id))
            .unwrap();
        
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraAccessory = serde_json::from_str(&newextradata).unwrap();
        let owner_id = self.accessories.owner_by_id.get(&accessory_id.clone()).unwrap();

        let accessory = Accessory {
            owner_id : owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
        };

        accessory
    }

    //Obtener paginación de los accesorios (Max 25 elementos por página)
    pub fn get_pagination(&self,tokens:u64) ->  Vec<u64> {
        let mut vectIDs = vec![];
        vectIDs.push(0);
        let mut _tokfound = 0;
        let mut _map =self.accessories_hash_map.clone();
        let mut i = 0;
        let mut toksfilted: Vec<u64> = vec![];
        log!("{:?}",_map);
        toksfilted = _map.iter()
        .map(|p| p.0.clone().parse::<u64>().unwrap() )
        .collect() ;
        toksfilted.sort();

        for x in 0..toksfilted.clone().len()-1 { 
                 _tokfound+=1;
                if _tokfound == tokens {   
                    vectIDs.push( toksfilted[x].clone()+1 );  
                    _tokfound = 0;  
                }
            if _tokfound == tokens { break; }            
        }
        vectIDs
    }

    // Obtener rango de items creados
    pub fn get_items_page(& self,tokens: u64,_start_index: u64) -> Vec<Accessory>  {
        let mut _map =self.accessories_hash_map.clone();
        let mut vectIDs = vec![];
        let mut vectMEta = vec![];
        let ends= _map.len().to_string().parse::<u64>();
        let mut _tokfound =0;
        let mut i=0;
        let mut toksfilted: Vec<u64> = vec![];
        log!("{:?}",_map);
        toksfilted = _map.iter()
        .map(|p| p.0.clone().parse::<u64>().unwrap() )
        .collect() ;
        toksfilted.sort();    
        
        for x in _start_index..ends.unwrap()  {
                _tokfound+=1;
                if _tokfound > tokens  {break;}      
            let tok = toksfilted[x as usize];
            vectIDs.push(tok );
                
        }  

        let endmeta = vectIDs.len().to_string().parse::<u64>().unwrap();
            for x in 0..endmeta { 
            let tokenid =  vectIDs[x as usize];
            let  token =self.get_accessory(tokenid.to_string());        
            vectMEta.push(token);
        }  

        return vectMEta ;   
    }

    // Obtener items que tiene un usuario
    pub fn get_items_owner(&self,accountId: ValidAccountId) -> Vec<Accessory>  {
        let mut _map = self.accessories_hash_map.clone();
        let mut vectIDs = vec![];
        let mut vectMEta = vec![];
        let ends = _map.len().to_string().parse::<u64>();
        for x in 0..ends.unwrap()  {
           let tok = _map.get(&x.to_string() ).unwrap();
           log!("{:?}",tok);
            if tok[0] == accountId.to_string()  {
                 vectIDs.push(x.to_string().parse::<u64>().unwrap() );
            }                  
        }

        let endmeta = vectIDs.len().to_string().parse::<u64>().unwrap();
        for x in 0..endmeta { 
            let tokenid =  vectIDs[x as usize];
            let mut token =self.get_accessory(tokenid.to_string());
            vectMEta.push(token);     
        }  
        return vectMEta ;     
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
