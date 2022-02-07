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
    env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,ext_contract,
    Promise, PromiseOrValue,};
near_sdk::setup_alloc!();
use std::convert::TryInto;

const BURRITO_CONTRACT: &str = "dev-1643951075935-27022974276068";
const ITEMS_CONTRACT: &str = "dev-1643957848449-43046979351328";
const MK_CONTRACT: &str = "dev-1643331107973-95015694722073";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: NonFungibleToken,
    accessories: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u128,
    n_accessories: u128,
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

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccessoriesForBattle {
    final_attack_b1 : String,
    final_defense_b1 : String,
    final_speed_b1 : String,
    final_attack_b2 : String,
    final_defense_b2 : String,
    final_speed_b2 : String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Thegraphstructure {
    colecction:String,
    contract_name:String,
    token_id : String,
    owner_id : String,
    title : String,
    description : String,
    media : String,
    creator : String,
    price : String,
    status: String, // sale status
    adressbidder: String,
    highestbid: String,
    lowestbid: String,
    expires_at: String,
    starts_at: String,
    extra: String,
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

// Métodos de otro contrato
#[ext_contract(ext_nft)]
pub trait MarketPlaceContract {
    fn saveToTheGraph(&self, info: String) -> Option<Token>;
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
        }
    }

    // Obtener cantidad de accesorios creados
    pub fn get_number_accessories(&self) -> u128 {
        self.n_accessories
    }

    // Minar un nuevo token
    #[payable]
    pub fn nft_mint_token(&mut self,token_owner_id: ValidAccountId, colecction: String, token_metadata: TokenMetadata) -> Accessory {
        let accessory_id: TokenId = self.n_accessories.to_string();

        self.accessories.mint(accessory_id.clone(), token_owner_id, Some(token_metadata.clone()));
        self.n_accessories += 1;

        let newextradata = str::replace(&token_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraAccessory = serde_json::from_str(&newextradata).unwrap();
        let owner_id = self.accessories.owner_by_id.get(&accessory_id.clone()).unwrap();

        let accessory = Accessory {
            owner_id : owner_id.to_string(),
            name : token_metadata.title.as_ref().unwrap().to_string(),
            description : token_metadata.description.as_ref().unwrap().to_string(),
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed
        };

        let ext : String =  "".to_string()+&accessory.attack.clone()+&":".to_string()+
                            &accessory.defense.clone()+&":".to_string()+
                            &accessory.speed.clone();

        let mut graphdata = Thegraphstructure {
            contract_name: ITEMS_CONTRACT.to_string(),
            colecction: colecction.clone().to_string(),
            token_id : accessory_id.to_string(),
            owner_id : owner_id.to_string(),
            title : token_metadata.title.as_ref().unwrap().to_string(),
            description : token_metadata.description.as_ref().unwrap().to_string(),
            media : token_metadata.media.as_ref().unwrap().to_string(),
            creator : owner_id.to_string(),
            price : "0".to_string(),
            status: "U".to_string(),
            adressbidder: owner_id.to_string(),
            highestbid: "0".to_string(),
            lowestbid: "0".to_string(),
            expires_at: "".to_string(),
            starts_at: "".to_string(),
            extra: ext
        };

        let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.colecction.to_string(); 
        
        let p = ext_nft::saveToTheGraph(
            rett.clone(),
            &MK_CONTRACT, //  account_id as a parameter
            env::attached_deposit(), // yocto NEAR to attach
            10_000_000_000_000 // gas to attach
        );
        
        accessory
    }

    // Minar un nuevo token desde contrato externo
    #[payable]
    pub fn nft_mint_token_ext(&mut self,token_owner_id: ValidAccountId, colecction: String, token_metadata: TokenMetadata) -> String {
        let accessory_id: TokenId = self.n_accessories.to_string();

        self.accessories.mint(accessory_id.clone(), token_owner_id.clone(), Some(token_metadata.clone()));
        self.n_accessories += 1;

        let newextradata = str::replace(&token_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraAccessory = serde_json::from_str(&newextradata).unwrap();

        let ext : String =  "".to_string()+&extradatajson.attack.clone()+&":".to_string()+
                            &extradatajson.defense.clone()+&":".to_string()+
                            &extradatajson.speed.clone();

        let mut graphdata = Thegraphstructure {
            contract_name: ITEMS_CONTRACT.to_string(),
            colecction: colecction.clone().to_string(),
            token_id : accessory_id.clone().to_string(),
            owner_id : token_owner_id.clone().to_string(),
            title : token_metadata.title.as_ref().unwrap().to_string(),
            description : token_metadata.description.as_ref().unwrap().to_string(),
            media : token_metadata.media.as_ref().unwrap().to_string(),
            creator : token_owner_id.clone().to_string(),
            price : "0".to_string(),
            status: "U".to_string(),
            adressbidder: token_owner_id.clone().to_string(),
            highestbid: "0".to_string(),
            lowestbid: "0".to_string(),
            expires_at: "".to_string(),
            starts_at: "".to_string(),
            extra: ext
        };

        let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.colecction.to_string(); 
        
        rett
    }

    // Obtener accesorio
    pub fn get_accessory(&self, accessory_id: TokenId) -> Accessory {
        if accessory_id.clone().parse::<u128>().unwrap() > self.n_accessories-1 {
            env::panic(b"No existe el accesorio con el id ingresado");
        }

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

    // Obtener items para batalla pvp
    pub fn get_items_for_battle(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId,
        accesorio1_burrito2_id: TokenId, accesorio2_burrito2_id: TokenId, accesorio3_burrito2_id: TokenId) -> AccessoriesForBattle  {

         // Obtener metadata accesorio 1 burrito 1
         let mut metadata_accesorio1_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio1_burrito1_id))
            .unwrap();

        // Obtener metadata accesorio 2 burrito 1
        let mut metadata_accesorio2_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio2_burrito1_id))
            .unwrap();

        // Obtener metadata accesorio 3 burrito 1
        let mut metadata_accesorio3_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio3_burrito1_id))
            .unwrap();

        
        // Obtener metadata accesorio 1 burrito 2
        let mut metadata_accesorio1_burrito2 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio1_burrito2_id))
            .unwrap();

        // Obtener metadata accesorio 2 burrito 2
        let mut metadata_accesorio2_burrito2 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio2_burrito2_id))
            .unwrap();

        // Obtener metadata accesorio 3 burrito 2
        let mut metadata_accesorio3_burrito2 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio3_burrito2_id))
            .unwrap();

        // Extraer extras del token accesorios burrito 1
        let newextradata_accesorio1_burrito1 = str::replace(&metadata_accesorio1_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio2_burrito1 = str::replace(&metadata_accesorio2_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio3_burrito1 = str::replace(&metadata_accesorio3_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        
        // Extraer extras del token accesorios burrito 2
        let newextradata_accesorio1_burrito2 = str::replace(&metadata_accesorio1_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio2_burrito2 = str::replace(&metadata_accesorio2_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio3_burrito2 = str::replace(&metadata_accesorio3_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
       
        // Crear json accesorios burrito 1
        let mut extradatajson_accesorio1_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito1).unwrap();
        let mut extradatajson_accesorio2_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito1).unwrap();
        let mut extradatajson_accesorio3_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito1).unwrap();

        // Crear json accesorios burrito 2
        let mut extradatajson_accesorio1_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito2).unwrap();
        let mut extradatajson_accesorio2_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito2).unwrap();
        let mut extradatajson_accesorio3_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito2).unwrap();
        
        // Obtener puntos totales a sumar de cada estadística de los accesorios del burrito 1
        let accesories_attack_burrito1 : f32 = (extradatajson_accesorio1_burrito1.attack.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.attack.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.attack.parse::<f32>().unwrap());
        let accesories_defense_burrito1 : f32 = (extradatajson_accesorio1_burrito1.defense.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.defense.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.defense.parse::<f32>().unwrap());
        let accesories_speed_burrito1 : f32 = (extradatajson_accesorio1_burrito1.speed.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.speed.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.speed.parse::<f32>().unwrap());
        
        // Obtener puntos totales a sumar de cada estadística de los accesorios del burrito 2
        let accesories_attack_burrito2 : f32 = (extradatajson_accesorio1_burrito2.attack.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito2.attack.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito2.attack.parse::<f32>().unwrap());
        let accesories_defense_burrito2 : f32 = (extradatajson_accesorio1_burrito2.defense.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito2.defense.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito2.defense.parse::<f32>().unwrap());
        let accesories_speed_burrito2 : f32 = (extradatajson_accesorio1_burrito2.speed.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito2.speed.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito2.speed.parse::<f32>().unwrap());
        
        let accessories_for_battle = AccessoriesForBattle {
            final_attack_b1 : accesories_attack_burrito1.to_string(),
            final_defense_b1 : accesories_defense_burrito1.to_string(),
            final_speed_b1 : accesories_speed_burrito1.to_string(),
            final_attack_b2 : accesories_attack_burrito2.to_string(),
            final_defense_b2 : accesories_defense_burrito2.to_string(),
            final_speed_b2 : accesories_speed_burrito2.to_string()
        };

        accessories_for_battle

    }

    // Obtener items para batalla player vs cpu
    pub fn get_items_for_battle_cpu(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId) -> AccessoriesForBattle  {

        // Validar que exista el id
        if accesorio1_burrito1_id.clone().parse::<u128>().unwrap() > self.n_accessories-1 {
            env::panic(b"No existe el id del accesorio 1");
        }
        if accesorio2_burrito1_id.clone().parse::<u128>().unwrap() > self.n_accessories-1 {
            env::panic(b"No existe el id del accesorio 2");
        }
        if accesorio3_burrito1_id.clone().parse::<u128>().unwrap() > self.n_accessories-1 {
            env::panic(b"No existe el id del accesorio 3");
        }

        let token_owner_id = env::signer_account_id();
        let owner_id_a1 = self.accessories.owner_by_id.get(&accesorio1_burrito1_id.clone()).unwrap();
        let owner_id_a2 = self.accessories.owner_by_id.get(&accesorio2_burrito1_id.clone()).unwrap();
        let owner_id_a3 = self.accessories.owner_by_id.get(&accesorio3_burrito1_id.clone()).unwrap();

        if token_owner_id.clone() != owner_id_a1.clone() {
            env::panic(b"El accesorio 1 no te pertenece");
        }
        if token_owner_id.clone() != owner_id_a2.clone() {
            env::panic(b"El accesorio 2 no te pertenece");
        }
        if token_owner_id.clone() != owner_id_a3.clone() {
            env::panic(b"El accesorio 3 no te pertenece");
        }
        if (accesorio1_burrito1_id.clone().parse::<u128>().unwrap() == accesorio2_burrito1_id.clone().parse::<u128>().unwrap()) || (accesorio1_burrito1_id.clone().parse::<u128>().unwrap() == accesorio3_burrito1_id.clone().parse::<u128>().unwrap()) || (accesorio2_burrito1_id.clone().parse::<u128>().unwrap() == accesorio3_burrito1_id.clone().parse::<u128>().unwrap()){
            env::panic(b"Los 3 accesorio deben ser diferentes");
        }

        // Obtener metadata accesorio 1 burrito 1
        let mut metadata_accesorio1_burrito1 = self
        .accessories
        .token_metadata_by_id
        .as_ref()
        .and_then(|by_id| by_id.get(&accesorio1_burrito1_id))
        .unwrap();

        // Obtener metadata accesorio 2 burrito 1
        let mut metadata_accesorio2_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio2_burrito1_id))
            .unwrap();

        // Obtener metadata accesorio 3 burrito 1
        let mut metadata_accesorio3_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio3_burrito1_id))
            .unwrap();

        // Extraer extras del token accesorios burrito 1
        let newextradata_accesorio1_burrito1 = str::replace(&metadata_accesorio1_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio2_burrito1 = str::replace(&metadata_accesorio2_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio3_burrito1 = str::replace(&metadata_accesorio3_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        
        // Crear json accesorios burrito 1
        let mut extradatajson_accesorio1_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito1).unwrap();
        let mut extradatajson_accesorio2_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito1).unwrap();
        let mut extradatajson_accesorio3_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito1).unwrap();

        // Obtener puntos totales a sumar de cada estadística de los accesorios del burrito 1
        let accesories_attack_burrito1 : f32 = (extradatajson_accesorio1_burrito1.attack.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.attack.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.attack.parse::<f32>().unwrap());
        let accesories_defense_burrito1 : f32 = (extradatajson_accesorio1_burrito1.defense.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.defense.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.defense.parse::<f32>().unwrap());
        let accesories_speed_burrito1 : f32 = (extradatajson_accesorio1_burrito1.speed.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.speed.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.speed.parse::<f32>().unwrap());
        
        let mut accessories_for_battle = AccessoriesForBattle {
            final_attack_b1 : accesories_attack_burrito1.to_string(),
            final_defense_b1 : accesories_defense_burrito1.to_string(),
            final_speed_b1 : accesories_speed_burrito1.to_string(),
            final_attack_b2 : accesories_attack_burrito1.to_string(),
            final_defense_b2 : accesories_defense_burrito1.to_string(),
            final_speed_b2 : accesories_speed_burrito1.to_string()
        };

        // Generamos incremento o decremento de los accesorios del burrito del CPU
        let rand_attack = *env::random_seed().get(0).unwrap();
        let rand_defense = *env::random_seed().get(1).unwrap();
        let rand_speed = *env::random_seed().get(2).unwrap();
        let mut attack: f32 = 0.0;
        let mut defense: f32 = 0.0;
        let mut speed: f32 = 0.0;

        if rand_attack >= 0 &&  rand_attack <= 127 {
            attack = 3.0;
        } else {
            attack = -3.0;
        }
        if rand_defense >= 0 &&  rand_defense <= 127 {
            defense = 3.0;
        } else {
            defense = -3.0;
        }
        if rand_speed >= 0 &&  rand_speed <= 127 {
            speed = 3.0;
        } else {
            speed = -3.0;
        }

        accessories_for_battle.final_attack_b2 = (accessories_for_battle.final_attack_b2.parse::<f32>().unwrap()+attack).to_string();
        accessories_for_battle.final_defense_b2 = (accessories_for_battle.final_defense_b2.parse::<f32>().unwrap()+defense).to_string();
        accessories_for_battle.final_speed_b2 = (accessories_for_battle.final_speed_b2.parse::<f32>().unwrap()+speed).to_string();

        accessories_for_battle

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
