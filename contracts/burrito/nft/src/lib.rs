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
    env, log, near_bindgen, ext_contract, AccountId, BorshStorageKey, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult,};
near_sdk::setup_alloc!();
use std::convert::TryInto;

const ITEMS_CONTRACT = "dev-1640297267245-16523317752149";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: NonFungibleToken,
    burritos: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u64,
    n_burritos: u64,
    burritos_hash_map:HashMap<TokenId, Vec<String>>
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Burrito {
    // token_id : String,
    owner_id : String,
    name : String,
    description : String,
    burrito_type : String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraBurrito {
    burrito_type: String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String
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
            name: "Burrito Battle".to_string(),
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
            burritos: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                env::signer_account_id().try_into().unwrap(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&meta)),
            n_tokens: 0,
            n_burritos: 0,
            burritos_hash_map:HashMap::new()
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
            burritos: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            n_tokens: 0,
            n_burritos: 0,
            burritos_hash_map:HashMap::new()
        }
    }

    // Obtener cantidad de burritos creados
    pub fn get_number_burritos(&self) -> u64 {
        self.n_burritos
    }

    // Minar un nuevo burrito
    #[payable]
    pub fn new_burrito(&mut self,burrito_id: TokenId,receiver_id: ValidAccountId,burrito_metadata: TokenMetadata) -> Burrito {
        let mut new_burrito = burrito_metadata;

        let mut burrito_data = ExtraBurrito {
            hp : "5".to_string(),
            attack : "".to_string(),
            defense : "".to_string(),
            speed : "".to_string(),
            win : "0".to_string(),
            burrito_type : "".to_string()
        };

        // Generar estadísticas random

        let rand_attack = *env::random_seed().get(0).unwrap();
        let rand_defense = *env::random_seed().get(1).unwrap();
        let rand_speed = *env::random_seed().get(2).unwrap();
        let rand_type = *env::random_seed().get(3).unwrap();

        let mut attack: u8 = 0;
        let mut defense: u8 = 0;
        let mut speed: u8 = 0;
        let mut burrito_type: String = "".to_string();

        // Obtener ataque aleatorio
        if rand_attack >= 0 &&  rand_attack <= 70 {
            attack = 5;
        }
        if rand_attack >= 71 &&  rand_attack <= 130 {
            attack = 6;
        }
        if rand_attack >= 131 &&  rand_attack <= 180 {
            attack = 7;
        }
        if rand_attack >= 181 &&  rand_attack <= 220 {
            attack = 8;
        }
        if rand_attack >= 221 &&  rand_attack <= 250 {
            attack = 9;
        }
        if rand_attack >= 251 &&  rand_attack <= 255 {
            attack = 10;
        }

        // Obtener defensa aleatoria
        if rand_defense >= 0 &&  rand_defense <= 70 {
            defense = 5;
        }
        if rand_defense >= 71 &&  rand_defense <= 130 {
            defense = 6;
        }
        if rand_defense >= 131 &&  rand_defense <= 180 {
            defense = 7;
        }
        if rand_defense >= 181 &&  rand_defense <= 220 {
            defense = 8;
        }
        if rand_defense >= 221 &&  rand_defense <= 250 {
            defense = 9;
        }
        if rand_defense >= 251 &&  rand_defense <= 255 {
            defense = 10;
        }

        // Obtener velociad aleatoria
        if rand_speed >= 0 &&  rand_speed <= 70 {
            speed = 5;
        }
        if rand_speed >= 71 &&  rand_speed <= 130 {
            speed = 6;
        }
        if rand_speed >= 131 &&  rand_speed <= 180 {
            speed = 7;
        }
        if rand_speed >= 181 &&  rand_speed <= 220 {
            speed = 8;
        }
        if rand_speed >= 221 &&  rand_speed <= 250 {
            speed = 9;
        }
        if rand_speed >= 251 &&  rand_speed <= 255 {
            speed = 10;
        }

        // Obtener tipo
        if rand_type >= 0 &&  rand_type <= 51 {
            burrito_type = "Fuego".to_string();
        }
        if rand_type >= 52 &&  rand_type <= 102 {
            burrito_type = "Agua".to_string();
        }
        if rand_type >= 103 &&  rand_type <= 153 {
            burrito_type = "Planta".to_string();
        }
        if rand_type >= 154 &&  rand_type <= 204 {
            burrito_type = "Eléctrico".to_string();
        }
        if rand_type >= 205 &&  rand_type <= 255 {
            burrito_type = "Volador".to_string();
        }

        // Asignamos valores a las estadisticas del burrito
        burrito_data.attack = attack.to_string();
        burrito_data.defense = defense.to_string();
        burrito_data.speed = speed.to_string();
        burrito_data.burrito_type = burrito_type.to_string();

        let mut extra_data_string = serde_json::to_string(&burrito_data).unwrap();
        extra_data_string = str::replace(&extra_data_string, "\"", "'");
        new_burrito.extra = Some(extra_data_string);

        self.burritos.mint(burrito_id.clone(), receiver_id, Some(new_burrito.clone()));

        self.n_burritos += 1;
        let owner_id = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();

        let burrito = Burrito {
            owner_id : owner_id.to_string(),
            name : new_burrito.title.as_ref().unwrap().to_string(),
            description : new_burrito.description.as_ref().unwrap().to_string(),
            burrito_type : burrito_data.burrito_type,
            hp : burrito_data.hp,
            attack : burrito_data.attack,
            defense : burrito_data.defense,
            speed : burrito_data.speed,
            win : burrito_data.win
        };

        //Insertar nuevo token a Hashmap
        let mut info:Vec<String>=Vec::new();
        //info[0] owner_id
        info.push(burrito.owner_id.clone());
        //info[1] name
        info.push(burrito.name.clone());
        let mut _map =self.burritos_hash_map.clone();
        _map.insert(burrito_id.clone(),info);
        self.burritos_hash_map=_map.clone();

        burrito
    }

    // Obtener burrito
    pub fn get_burrito(&self, burrito_id: TokenId) -> Burrito {
        let metadata = self
            .burritos
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&burrito_id))
            .unwrap();
        
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();
        let owner_id = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();

        let burrito = Burrito {
            owner_id : owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win
        };

        burrito
    }

    // Modificar burrito
    pub fn update_burrito(&mut self, burrito_id: TokenId, extra: String) -> Burrito {
        let mut metadata = self
            .burritos
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&burrito_id))
            .unwrap();
        
        metadata.extra = Some(extra);

        self.burritos
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&burrito_id, &metadata));

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();
        let owner_id = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();

        let burrito = Burrito {
            owner_id : owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win
        };

        burrito
    }

    // Pelear
    // Sacar ddel contrato general

    //Obtener paginación de los accesorios (Max 25 elementos por página)
    pub fn get_pagination(&self,tokens:u64) ->  Vec<u64> {
        let mut vectIDs = vec![];
        vectIDs.push(0);
        let mut _tokfound = 0;
        let mut _map =self.burritos_hash_map.clone();
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
    pub fn get_burritos_page(& self,tokens: u64,_start_index: u64) -> Vec<Burrito>  {
        let mut _map =self.burritos_hash_map.clone();
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
            let  token =self.get_burrito(tokenid.to_string());        
            vectMEta.push(token);
        }  

        return vectMEta ;   
    }

    // Obtener items que tiene un usuario
    pub fn get_burritos_owner(&self,accountId: ValidAccountId) -> Vec<Burrito>  {
        let mut _map = self.burritos_hash_map.clone();
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
            let mut token =self.get_burrito(tokenid.to_string());
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
