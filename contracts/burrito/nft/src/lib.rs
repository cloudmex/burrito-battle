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
use near_sdk::utils::promise_result_as_success;
use std::sync::{Mutex};
use lazy_static::lazy_static;
use std::str;
use near_sdk::{
    env, log, near_bindgen, ext_contract, AccountId, BorshStorageKey, PanicOnDefault,
    Promise, PromiseOrValue, PromiseResult, Balance, Gas};
near_sdk::setup_alloc!();
use std::convert::TryInto;
// Contrato de items
const BURRITO_CONTRACT: &str = "dev-1641852649467-90519282117752";
const ITEMS_CONTRACT: &str = "dev-1641232107829-62553275563586";
const NO_DEPOSIT: Balance = 0;
const BASE_GAS: Gas = 50_000_000_000_000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: NonFungibleToken,
    burritos: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u128,
    n_burritos: u128,
    n_battles: u128,
    burritos_hash_map:HashMap<TokenId, Vec<String>>,
    users_hash_map:HashMap<u128,String>,
    battle_room_map:HashMap::<u128,Vec<String>>

    // Estatus = Espera,Ocupado,Finalizado
    // NumeroBatalla,[Estatus,
    //                        Jugador1,Burrito1,Burrito2,Burrito3,
    //                        Jugador2,Burrito1,Burrito2,Burrito3,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda1,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda2,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda3,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda4,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda5,
    //                Ganador]

    // Ejemplo Inicio de batalla
    // 1,[Espera,
    //                        yairnava.testnet,1,2,3,
    //                        Jugador2,Burrito1,Burrito2,Burrito3,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda1,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda2,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda3,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda4,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda5,
    //                Ganador]

    // Ejemplo Proceso de batalla
    // 1,[Batallando,
    //                        yairnava.testnet,1,2,3,
    //                        missael.testnet,4,5,6,
    //                        1,1,2,3,4,4,5,6,yairnava.testnet,
    //                        1,1,2,3,4,4,5,6,missael.testnet,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda3,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda4,
    //                        BurritoJ1,Accesorio1J1,Accesorio2J1,Accesorio3J1,BurritoJ2,Accesorio1J2,Accesorio2J2,Accesorio3J2,GanadorRonda5,
    //                Ganador]

    // Ejemplo Final de batalla
    // 1,[Finalizado,
    //                        yairnava.testnet,1,2,3,
    //                        missael.testnet,4,5,6,
    //                        1,1,2,3,4,4,5,6,yairnava.testnet,
    //                        1,1,2,3,4,4,5,6,missael.testnet,
    //                        2,1,2,3,4,4,5,6,yairnava.testnet,
    //                        1,1,2,3,4,4,5,6,missael.testnet,
    //                        3,1,2,3,5,4,5,6,yairnava.testnet,
    //                yairnava.testnet]

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
            n_battles: 0,
            burritos_hash_map:HashMap::new(),
            battle_room_map:HashMap::new(),
            users_hash_map:HashMap::new(),
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
#[ext_contract(ext_ft)]
pub trait ItemsContract {
    fn get_items_for_battle(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId,
        accesorio1_burrito2_id: TokenId, accesorio2_burrito2_id: TokenId, accesorio3_burrito2_id: TokenId
    ) -> AccessoriesForBattle;
}

// Métodos del mismo contrato para los callback
#[ext_contract(ext_self)]
pub trait MyContract {
    fn get_winner(&mut self,burrito1_id: TokenId,burrito2_id: TokenId) -> String;
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
            n_battles: 0 ,
            burritos_hash_map:HashMap::new(),
            battle_room_map:HashMap::new(),
            users_hash_map:HashMap::new(),
        }
    }

    // Obtener cantidad de burritos creados
    pub fn get_number_burritos(&self) -> u128 {
        self.n_burritos
    }

    // Obtener cantidad de batallas creadas
    pub fn get_number_battles(&self) -> u128 {
        self.n_battles
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

        self.burritos.mint(burrito_id.clone(), receiver_id.clone(), Some(new_burrito.clone()));

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
        let mut _map_burritos =self.burritos_hash_map.clone();
        _map_burritos.insert(burrito_id.clone(),info);
        self.burritos_hash_map=_map_burritos.clone();

        // Consultamos usuarios registrados en el Hashmap
        let mut _map_users = self.users_hash_map.clone();
        let users_number = (_map_users.len()+1).to_string().parse::<u128>();            
        let ends = _map_users.len().to_string().parse::<u128>();
        let mut exist_user = false;
        for x in 0..ends.unwrap()  {
            let tok = _map_users.get(&(&x.to_string().parse::<u128>().unwrap()+1));
            if tok == Some(&receiver_id.to_string()){
                exist_user = true;
            }             
        }

        // Verificar si ya existe usuario en Hashmap
        if !exist_user {
            //Insertar nuevo token a Hashmap
            _map_users.insert(users_number.unwrap(),receiver_id.to_string());
            self.users_hash_map=_map_users.clone();
        }

        burrito
    }

    pub fn get_hashmap_users(&self){
        log!("{:?}",&self.users_hash_map);
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
    
    // Método para pelea de 2 burritos
    pub fn fight_burritos(&self,
        burrito1_id: TokenId, accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId, 
        burrito2_id: TokenId, accesorio1_burrito2_id: TokenId, accesorio2_burrito2_id: TokenId, accesorio3_burrito2_id: TokenId) -> Promise {
        log!("Llamando contrato @{} desde @{}",ITEMS_CONTRACT,BURRITO_CONTRACT);

        // Invocar un método en otro contrato
        let p = ext_ft::get_items_for_battle(
            accesorio1_burrito1_id.to_string(), // Id el item 1 del burrito 1
            accesorio2_burrito1_id.to_string(), // Id el item 2 del burrito 1
            accesorio3_burrito1_id.to_string(), // Id el item 3 del burrito 1
            accesorio1_burrito2_id.to_string(), // Id el item 1 del burrito 2
            accesorio2_burrito2_id.to_string(), // Id el item 2 del burrito 2
            accesorio3_burrito2_id.to_string(), // Id el item 3 del burrito 2
            &ITEMS_CONTRACT, // Contrato de items
            NO_DEPOSIT, // yocto NEAR a ajuntar
            BASE_GAS // gas a ajuntar
        )
        .then(ext_self::get_winner(
            burrito1_id.to_string(),
            burrito2_id.to_string(),
            &BURRITO_CONTRACT, // Contrato de burritos
            NO_DEPOSIT, // yocto NEAR a ajuntar al callback
            BASE_GAS // gas a ajuntar al callback
        ));

        p
    } 

    // Obtener al ganador de una pelea
    pub fn get_winner(&mut self,burrito1_id: TokenId,burrito2_id: TokenId) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let value = str::from_utf8(&result).unwrap();
                let accessories_for_battle: AccessoriesForBattle = serde_json::from_str(&value).unwrap();

                // Obtenemos los datos de los burritos

                // Obtener metadata burrito 1
                let mut metadata_burrito1 = self
                .burritos
                .token_metadata_by_id
                .as_ref()
                .and_then(|by_id| by_id.get(&burrito1_id.clone()))
                .unwrap();

                // Obtener metadata burrito 2
                let mut metadata_burrito2 = self
                .burritos
                .token_metadata_by_id
                .as_ref()
                .and_then(|by_id| by_id.get(&burrito2_id.clone()))
                .unwrap();

                // Extraer extras del token burrito 1
                let newextradata_burrito1 = str::replace(&metadata_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");

                // Extraer extras del token burrito 2
                let newextradata_burrito2 = str::replace(&metadata_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");

                // Crear json burrito 1
                let mut extradatajson_burrito1: ExtraBurrito = serde_json::from_str(&newextradata_burrito1).unwrap();

                // Crear json burrito 2
                let mut extradatajson_burrito2: ExtraBurrito = serde_json::from_str(&newextradata_burrito2).unwrap();


                let owner_id_burrito1 = self.burritos.owner_by_id.get(&burrito1_id.clone()).unwrap();
                // Crear estructura burrito 1
                let burrito1 = Burrito {
                    owner_id : owner_id_burrito1.to_string(),
                    name : metadata_burrito1.title.as_ref().unwrap().to_string(),
                    description : metadata_burrito1.description.as_ref().unwrap().to_string(),
                    burrito_type : extradatajson_burrito1.burrito_type.clone(),
                    hp : extradatajson_burrito1.hp.clone(),
                    attack : extradatajson_burrito1.attack.clone(),
                    defense : extradatajson_burrito1.defense.clone(),
                    speed : extradatajson_burrito1.speed.clone(),
                    win : extradatajson_burrito1.win.clone()
        
                };
        
                let owner_id_burrito2 = self.burritos.owner_by_id.get(&burrito2_id.clone()).unwrap();
                // Crear estructura burrito 2
                let burrito2 = Burrito {
                    owner_id : owner_id_burrito2.to_string(),
                    name : metadata_burrito2.title.as_ref().unwrap().to_string(),
                    description : metadata_burrito2.description.as_ref().unwrap().to_string(),
                    burrito_type : extradatajson_burrito2.burrito_type.clone(),
                    hp : extradatajson_burrito2.hp.clone(),
                    attack : extradatajson_burrito2.attack.clone(),
                    defense : extradatajson_burrito2.defense.clone(),
                    speed : extradatajson_burrito2.speed.clone(),
                    win : extradatajson_burrito2.win.clone()
                };
                
                // Validamos que ambos burritos tengan vidas para combatir
                assert!(burrito1.hp.parse::<u8>().unwrap() > 0, "{} no tiene vidas para combatir",metadata_burrito1.title.as_ref().unwrap().to_string());
                assert!(burrito2.hp.parse::<u8>().unwrap() > 0, "{} no tiene vidas para combatir",metadata_burrito2.title.as_ref().unwrap().to_string());

                let logname1 = format!("Nombre Burrito 1: {}", metadata_burrito1.title.as_ref().unwrap().to_string() );
                env::log(logname1.as_bytes());
                
                let logname2 = format!("Nombre Burrito 2: {}", metadata_burrito2.title.as_ref().unwrap().to_string() );
                env::log(logname2.as_bytes());

                // Variable que almacenará al ganador
                let burrito_winner : Burrito;

                //let burrito_winner : Burrito;
                let mut winner : i32 = 1;
                
                // Defensa total del burrito 1
                let mut old_defense_burrito1 = (burrito1.defense.parse::<f32>().unwrap()+accessories_for_battle.final_defense_b1.parse::<f32>().unwrap());
                
                // Defensa total del burrito 2
                let mut old_defense_burrito2 = (burrito2.defense.parse::<f32>().unwrap()+accessories_for_battle.final_defense_b2.parse::<f32>().unwrap());
                
                // Generar números aleatorios para multiplicadores de velocidad y ataque
                let mut rands1: u8 = *env::random_seed().get(0).unwrap();;
                let mut rands2: u8 = *env::random_seed().get(1).unwrap();;
                let mut randa1: u8 = *env::random_seed().get(2).unwrap();
                let mut randa2: u8 = *env::random_seed().get(3).unwrap();

                let mut speed_mult1: f32 = 0.0;
                let mut speed_mult2: f32 = 0.0;
                let mut attack_mult1: f32 = 0.0;
                let mut attack_mult2: f32 = 0.0;
                let mut type_mult1: f32 = 0.0;
                let mut type_mult2: f32 = 0.0;

                if rands1 < 10 {
                    speed_mult1 = rands1 as f32 * 0.1;
                }
                if rands1 >= 10 && rands1 < 100 {
                    speed_mult1 = rands1 as f32 * 0.01;
                }
                if rands1 >= 100 && rands1 < 255 {
                    speed_mult1 = rands1 as f32 * 0.001;
                }
                if rands2 < 10 {
                    speed_mult2 = rands2 as f32 * 0.1;
                }
                if rands2 >= 10 && rands2 < 100 {
                    speed_mult2 = rands2 as f32 * 0.01;
                }
                if rands2 >= 100 && rands2 < 255 {
                    speed_mult2 = rands2 as f32 * 0.001;
                }
                if randa1 < 10 {
                    attack_mult1 = randa1 as f32 * 0.1;
                }
                if randa1 >= 10 && randa1 < 100 {
                    attack_mult1 = randa1 as f32 * 0.01;
                }
                if randa1 >= 100 && randa1 < 255 {
                    attack_mult1 = randa1 as f32 * 0.001;
                }
                if randa2 < 10 {
                    attack_mult2 = randa2 as f32 * 0.1;
                }
                if randa2 >= 10 && randa2 < 100 {
                    attack_mult2 = randa2 as f32 * 0.01;
                }
                if randa2 >= 100 && randa2 < 255 {
                    attack_mult2 = randa2 as f32 * 0.001;
                }

                loop {
                    // Verificar cuál burrito tiene mayor velocidad
                    if ((burrito1.speed.parse::<f32>().unwrap()*speed_mult1)+accessories_for_battle.final_speed_b1.parse::<f32>().unwrap()) > ((burrito2.speed.parse::<f32>().unwrap()*speed_mult2)+accessories_for_battle.final_speed_b2.parse::<f32>().unwrap()) {
                        //Obtener multiplicador de tipo
                        if(burrito1.burrito_type == "Fuego" && burrito2.burrito_type == "Planta"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Agua" && burrito2.burrito_type == "Fuego"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Planta" && burrito2.burrito_type == "Eléctrico"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Eléctrico" && burrito2.burrito_type == "Volador"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Volador" && burrito2.burrito_type == "Agua"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
    
                        old_defense_burrito2 = old_defense_burrito2 - ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)+type_mult1+accessories_for_battle.final_attack_b1.parse::<f32>().unwrap());
                        type_mult1 = 0.0;
                        if old_defense_burrito2 < 0.0 {
                            winner = 1;
                        }
                        if winner == 0 {
                            //Obtener multiplicador de tipo
                            if(burrito2.burrito_type == "Fuego" && burrito1.burrito_type == "Planta"){
                                type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito2.burrito_type == "Agua" && burrito1.burrito_type == "Fuego"){
                                type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito2.burrito_type == "Planta" && burrito1.burrito_type == "Eléctrico"){
                                type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito2.burrito_type == "Eléctrico" && burrito1.burrito_type == "Volador"){
                                type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito2.burrito_type == "Volador" && burrito1.burrito_type == "Agua"){
                                type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            
                            old_defense_burrito1 = old_defense_burrito1 - ((burrito2.attack.parse::<f32>().unwrap()*attack_mult2)+type_mult2+accessories_for_battle.final_attack_b2.parse::<f32>().unwrap());
                            type_mult2 = 0.0;
                            if old_defense_burrito1 < 0.0 {
                                winner = 2;
                            }
                        }
                    } else {
                        //Obtener multiplicador de tipo
                        if(burrito2.burrito_type == "Fuego" && burrito1.burrito_type == "Planta"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Agua" && burrito1.burrito_type == "Fuego"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Planta" && burrito1.burrito_type == "Eléctrico"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Eléctrico" && burrito1.burrito_type == "Volador"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Volador" && burrito1.burrito_type == "Agua"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
    
                        old_defense_burrito1 = old_defense_burrito1 - ((burrito2.attack.parse::<f32>().unwrap()*attack_mult2)+type_mult2+accessories_for_battle.final_attack_b2.parse::<f32>().unwrap());
                        type_mult2 = 0.0;
                        if old_defense_burrito1 < 0.0 {
                            winner = 2;
                        }
                        if winner == 0 {
                            //Obtener multiplicador de tipo
                            if(burrito1.burrito_type == "Fuego" && burrito2.burrito_type == "Planta"){
                                type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito1.burrito_type == "Agua" && burrito2.burrito_type == "Fuego"){
                                type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito1.burrito_type == "Planta" && burrito2.burrito_type == "Eléctrico"){
                                type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito1.burrito_type == "Eléctrico" && burrito2.burrito_type == "Volador"){
                                type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
                            if(burrito1.burrito_type == "Volador" && burrito2.burrito_type == "Agua"){
                                type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                            }
    
                            old_defense_burrito2 = old_defense_burrito2 - ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)+type_mult1+accessories_for_battle.final_attack_b1.parse::<f32>().unwrap());
                            type_mult1 = 0.0;
                            if old_defense_burrito2 < 0.0 {
                                winner = 1;
                            }
                        }
                    }
    
                    if winner != 0 {
                        break;
                    }
                }

                if winner == 1 {
                    burrito_winner = burrito1;
        
                    let new_hp_burrito2 = burrito2.hp.parse::<u8>().unwrap()-1;
                    extradatajson_burrito2.hp = new_hp_burrito2.to_string();
        
                    let mut extra_string_burrito2 = serde_json::to_string(&extradatajson_burrito2).unwrap();
                    extra_string_burrito2 = str::replace(&extra_string_burrito2, "\"", "'");
                    metadata_burrito2.extra = Some(extra_string_burrito2.clone());
        
                    self.burritos
                        .token_metadata_by_id
                        .as_mut()
                        .and_then(|by_id| by_id.insert(&burrito2_id, &metadata_burrito2));
        
                    let new_win_burrito1 = burrito_winner.win.parse::<u8>().unwrap()+1;
                    extradatajson_burrito1.win = new_win_burrito1.to_string();
        
                    let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
                    extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
                    metadata_burrito1.extra = Some(extra_string_burrito1.clone());
        
                    self.burritos
                        .token_metadata_by_id
                        .as_mut()
                        .and_then(|by_id| by_id.insert(&burrito1_id, &metadata_burrito1));
                } 
                else {
                    burrito_winner = burrito2;
        
                    let new_hp_burrito1 = burrito1.hp.parse::<u8>().unwrap()-1;
                    extradatajson_burrito1.hp = new_hp_burrito1.to_string();
        
                    let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
                    extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
                    metadata_burrito1.extra = Some(extra_string_burrito1.clone());
        
                    self.burritos
                        .token_metadata_by_id
                        .as_mut()
                        .and_then(|by_id| by_id.insert(&burrito1_id, &metadata_burrito1));
        
                    let new_win_burrito2 = burrito_winner.win.parse::<u8>().unwrap()+1;
                    extradatajson_burrito2.win = new_win_burrito2.to_string();
        
                    let mut extra_string_burrito2 = serde_json::to_string(&extradatajson_burrito2).unwrap();
                    extra_string_burrito2 = str::replace(&extra_string_burrito2, "\"", "'");
                    metadata_burrito2.extra = Some(extra_string_burrito2.clone());
        
                    self.burritos
                        .token_metadata_by_id
                        .as_mut()
                        .and_then(|by_id| by_id.insert(&burrito2_id, &metadata_burrito2));
                }

                //Retornamos al burrito ganador
                burrito_winner.name


            }
        }
    }

    // Obtener rival de combate
    pub fn get_battle_room(&self, accountId: ValidAccountId) -> String {
        // Buscar en el hashmap de la salas de combate si ya hay un encuentro iniciado con otro jugador.
            // Verificar en que número de ronda va la batalla.
                // Cada jugador debe seleccionar su burrito y accesorios para el próximo combate.
                // Se manda a llamar el metodo de fight_burritos con los datos de los burritos y accesorios a combatir.
                // Una vez determinado al ganador se moifica el hashmap y se registra al ganador de la ronda.
                    // Se verifica si ya es la ronda final
                        // Se determina al ganador de 3 de 5 combates para determinar al ganador de la batalla y finalizarla en el hashmap.
                    // Se verifica si algún jugador ya ganó 3 combates
                        // Se registra al ganador y se finaliza la batalla en el hashmap.

        // Buscar en el hashmap de salas de combate si hay una sala disponible para combatir.
            // Si no hay una sala disponible, entonces se crea una nueva.
                // Se espera a que llegue un contrincante.
                    // Cada jugador debe seleccionar su burrito y accesorios para el próximo combate.
                    // Se manda a llamar el metodo de fight_burritos con los datos de los burritos y accesorios a combatir.
                    // Una vez determinado al ganador se moifica el hashmap y se registra al ganador de la ronda.
            // Si ya exíste una sala disponible, entonces se registra en esa sala disponible para combatir.
                // Cada jugador debe seleccionar su burrito y accesorios para el próximo combate.
                        // Se manda a llamar el metodo de fight_burritos con los datos de los burritos y accesorios a combatir.
                        // Una vez determinado al ganador se moifica el hashmap y se registra al ganador de la ronda.

            let mut _map = self.battle_room_map.clone();

            //let mut vectIDs = vec![];
            let ends = _map.len().to_string().parse::<u128>();
            for x in 0..ends.unwrap()  {
                log!("Sala: {:?}",&x+1);
                let tok = _map.get(&(&x.to_string().parse::<u128>().unwrap()+1));
                log!("{:?}",tok);                
            }

        "Obteniendo salas de batalla".to_string()
    }

    // Crear batalla player vs cpu
    pub fn save_battle_cpu(&mut self, accountId: ValidAccountId,burrito1_id: TokenId,burrito2_id: TokenId,burrito3_id: TokenId) {
        //Insertar nuevo token a Hashmap
        let mut _map_rooms =self.battle_room_map.clone();
        let battle_number = (_map_rooms.len()+1).to_string().parse::<u128>();
        let mut info:Vec<String>=Vec::new();

        //info[0] Estatus
        info.push("Espera".to_string());
        //info[1] Jugador1
        info.push(accountId.to_string());
        //info[2] Jugador1 Burrito1
        info.push(burrito1_id.to_string());
        //info[3] Jugador1 Burrito2
        info.push(burrito2_id.to_string());
        //info[4] Jugador1 Burrito3
        info.push(burrito3_id.to_string());

        // Consultar Hashmap de usuarios
        let mut _map_users = self.users_hash_map.clone();
        let mut users = vec![];
        let ends = _map_users.len().to_string().parse::<u128>();
        for x in 0..ends.unwrap()  {
            let tok = _map_users.get(&(&x.to_string().parse::<u128>().unwrap()+1));
            // Verificar que el usuario sea diferente al que está iniciando la batalla
            if tok != Some(&accountId.to_string()){
                users.push(tok);
            }             
        }

        log!("{:?}",&users);
        // Tomar usuario al azar
        let users_number = (users.len()).to_string().parse::<u128>();            
        log!("{:?}",&users_number);
        let num_rand = *env::random_seed().get(0).unwrap();
        log!("{:?}",&num_rand);

        // Obtener los burritos del jugador
        
        //info[5] Jugador2
        info.push("".to_string());
        //info[6] Jugador2 Burrito1
        info.push("".to_string());
        //info[7] Jugador2 Burrito2
        info.push("".to_string());
        //info[8] Jugador2 Burrito3
        info.push("".to_string());

        
        //info[9] BurritoJ1
        info.push("".to_string());
        //info[10] Accesorio1J1
        info.push("".to_string());
        //info[11] Accesorio2J1
        info.push("".to_string());
        //info[12] Accesorio3J1
        info.push("".to_string());
        //info[13] BurritoJ2
        info.push("".to_string());
        //info[14] Accesorio1J2
        info.push("".to_string());
        //info[15] Accesorio2J2
        info.push("".to_string());
        //info[16] Accesorio3J2
        info.push("".to_string());
        //info[17] GanadorRonda1
        info.push("".to_string());
        //info[18] BurritoJ1
        info.push("".to_string());
        //info[19] Accesorio1J1
        info.push("".to_string());
        //info[20] Accesorio2J1
        info.push("".to_string());
        //info[21] Accesorio3J1
        info.push("".to_string());
        //info[22] BurritoJ2
        info.push("".to_string());
        //info[23] Accesorio1J2
        info.push("".to_string());
        //info[24] Accesorio2J2
        info.push("".to_string());
        //info[25] Accesorio3J2
        info.push("".to_string());
        //info[26] GanadorRonda1
        info.push("".to_string());
        //info[27] BurritoJ1
        info.push("".to_string());
        //info[28] Accesorio1J1
        info.push("".to_string());
        //info[29] Accesorio2J1
        info.push("".to_string());
        //info[30] Accesorio3J1
        info.push("".to_string());
        //info[31] BurritoJ2
        info.push("".to_string());
        //info[32] Accesorio1J2
        info.push("".to_string());
        //info[33] Accesorio2J2
        info.push("".to_string());
        //info[34] Accesorio3J2
        info.push("".to_string());
        //info[35] GanadorRonda1
        info.push("".to_string());
        //info[36] BurritoJ1
        info.push("".to_string());
        //info[37] Accesorio1J1
        info.push("".to_string());
        //info[38] Accesorio2J1
        info.push("".to_string());
        //info[39] Accesorio3J1
        info.push("".to_string());
        //info[40] BurritoJ2
        info.push("".to_string());
        //info[41] Accesorio1J2
        info.push("".to_string());
        //info[42] Accesorio2J2
        info.push("".to_string());
        //info[43] Accesorio3J2
        info.push("".to_string());
        //info[44] GanadorRonda1
        info.push("".to_string());
        //info[45] BurritoJ1
        info.push("".to_string());
        //info[46] Accesorio1J1
        info.push("".to_string());
        //info[47] Accesorio2J1
        info.push("".to_string());
        //info[48] Accesorio3J1
        info.push("".to_string());
        //info[49] BurritoJ2
        info.push("".to_string());
        //info[50] Accesorio1J2
        info.push("".to_string());
        //info[51] Accesorio2J2
        info.push("".to_string());
        //info[52] Accesorio3J2
        info.push("".to_string());
        //info[53] GanadorRonda1
        info.push("".to_string());
        //info[54] Ganador
        info.push("".to_string());
        // _map_rooms.insert(battle_number.unwrap(),info);
        // self.battle_room_map=_map_rooms.clone();
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
