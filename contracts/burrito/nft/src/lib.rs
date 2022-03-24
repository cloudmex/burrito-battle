//Implementación de los standards NFT de near
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{ValidAccountId};
use std::str;
use near_sdk::{
    env, log, near_bindgen, ext_contract, AccountId, BorshStorageKey,
    Promise, PromiseOrValue, PromiseResult, Balance, Gas, serde_json::json};
near_sdk::setup_alloc!();
use std::convert::TryInto;
use near_sdk::env::BLOCKCHAIN_INTERFACE;

// Contrato de items
const BURRITO_CONTRACT: &str = "dev-1648154149121-91041924696178";
const ITEMS_CONTRACT: &str = "dev-1647986467816-61735125036881";
const MK_CONTRACT: &str = "dev-1646163482135-99250841517221";
const STRWTOKEN_CONTRACT: &str = "dev-1645837411235-48460272126519";

const NO_DEPOSIT: Balance = 0;
const BASE_GAS: Gas = 10_000_000_000_000;
pub const TGAS: u64 = 1_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OldContract {
    tokens: NonFungibleToken,
    burritos: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u128,
    n_burritos: u128,
    n_battle_rooms_cpu: u128,
    battle_room_cpu: HashMap<String,BattleCPU>,
    battle_history: HashMap<String,BattlesHistory>,
    n_battles: u128
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    tokens: NonFungibleToken,
    burritos: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u128,
    n_burritos: u128,
    n_battle_rooms_cpu: u128,
    battle_room_cpu: HashMap<String,BattleCPU>,
    battle_history: HashMap<String,BattlesHistory>,
    n_battles: u128
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BattleCPU {
    status : String, // 1 = On Hold , 2 = In Battle , 3 = Finish
    payer_id : String,
    burrito_id : String,
    accesories_attack_b1 : String,
    accesories_defense_b1 : String,
    accesories_speed_b1 : String,
    accesories_attack_b2 : String,
    accesories_defense_b2 : String,
    accesories_speed_b2 : String,
    turn : String, // Player or CPU
    strong_attack_player : String, // 0-3
    shields_player : String, // 0-3
    health_player : String,
    strong_attack_cpu : String, // 0-3
    shields_cpu : String, // 0-3
    health_cpu : String,
    burrito_cpu_level : String,
    burrito_cpu_type : String,
    burrito_cpu_attack : String,
    burrito_cpu_defense : String,
    burrito_cpu_speed : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BattlesHistory {
    payer1_id : String,
    payer2_id : String,
    winner : String,
    status : String, // Battle, Surrender
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
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
    win : String,
    global_win : String,
    level : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraBurrito {
    burrito_type: String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String,
    global_win : String,
    level : String
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
    collection:String,
    collection_id:String,
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
            n_battle_rooms_cpu: 0,
            battle_room_cpu:HashMap::new(),
            battle_history:HashMap::new(),
            n_battles: 0
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
pub trait ExternsContract {
    fn get_items_for_battle(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId,
        accesorio1_burrito2_id: TokenId, accesorio2_burrito2_id: TokenId, accesorio3_burrito2_id: TokenId
    ) -> AccessoriesForBattle;
    fn get_items_for_battle_cpu(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId
    ) -> AccessoriesForBattle;
    fn save_mint_ttg(&self, info: String) -> Option<Token>;
    fn reward_player(&self,player_owner_id: String,tokens_mint: String) -> String;
    fn get_balance_and_transfer(&self,account_id: String, action: String) -> U128;

}

// Métodos del mismo contrato para los callback
#[ext_contract(ext_self)]
pub trait MyContract {
    fn get_winner(&mut self,burrito1_id: TokenId,burrito2_id: TokenId) -> String;
    fn burrito_level_up(&mut self,burrito_id: TokenId) -> String;
    fn new_burrito(&mut self,token_owner_id: ValidAccountId, colecction: String, token_metadata: TokenMetadata) -> String;
    fn reset_conditions(&mut self,burrito_id: TokenId) -> String;
    fn save_battle_player_cpu(&mut self,burrito_id: TokenId) -> String;
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
            }
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
            n_battle_rooms_cpu: 0 ,
            battle_room_cpu:HashMap::new(),
            battle_history:HashMap::new(),
            n_battles: 0
        }
    }

    // Obtener cantidad de burritos creados
    pub fn get_number_burritos(&self) -> u128 {
        self.n_burritos
    }

    // Minar un nuevo token 600,000 $STRW tokens + 5 $NEAR tokens
    #[payable]
    pub fn mint_token(&mut self,token_owner_id: ValidAccountId, colecction: String, token_metadata: TokenMetadata) -> Promise {
        let account_id = env::signer_account_id();
        
        let deposit = env::attached_deposit();
        // 5 Nears
        let deposit_to_treasury = deposit.clone() - 100000000000000000000000;
        log!("Deposito en mint_token treasury {}",deposit_to_treasury);
        // 0.1 Nears
        let deposit_to_mint = deposit.clone() - 4900000000000000000000000;
        log!("Deposito en mint_token mint {}",deposit_to_mint);

        ext_nft::get_balance_and_transfer(
            account_id.clone().to_string(),
            "Mint".to_string(),
            &STRWTOKEN_CONTRACT,
            deposit_to_treasury,
            100_000_000_000_000
        ).then(ext_self::new_burrito(
            token_owner_id,
            colecction,
            token_metadata,
            &BURRITO_CONTRACT, // Contrato de burritos
            deposit_to_mint, // yocto NEAR a ajuntar al callback
            100_000_000_000_000 // gas a ajuntar al callback
        ))
    }

    #[payable]
    pub fn new_burrito(&mut self,token_owner_id: ValidAccountId, colecction: String, token_metadata: TokenMetadata) -> String{
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(_result) => {
                let deposit = env::attached_deposit();   
                log!("Deposito en new_burrito {}",deposit);

                let mut new_burrito = token_metadata;
                let burrito_id: TokenId = self.n_burritos.to_string();
                let mut burrito_data = ExtraBurrito {
                    hp : "5".to_string(),
                    attack : "5".to_string(),
                    defense : "5".to_string(),
                    speed : "5".to_string(),
                    win : "0".to_string(),
                    global_win : "0".to_string(),
                    burrito_type : "Fuego".to_string(),
                    level : "1".to_string()
                };

                // Generar estadísticas random

                let rand_attack = *env::random_seed().get(0).unwrap();
                let rand_defense = *env::random_seed().get(1).unwrap();
                let rand_speed = *env::random_seed().get(2).unwrap();
                let rand_type = *env::random_seed().get(3).unwrap();

                let mut attack: u8 = 5;
                let mut defense: u8 = 5;
                let mut speed: u8 = 5;
                let mut burrito_type: String = "Fuego".to_string();

                // Obtener ataque aleatorio
                if rand_attack > 0 &&  rand_attack <= 70 {
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
                if rand_attack >= 251 &&  rand_attack < 255 {
                    attack = 10;
                }

                // Obtener defensa aleatoria
                if rand_defense > 0 &&  rand_defense <= 70 {
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
                if rand_defense >= 251 &&  rand_defense < 255 {
                    defense = 10;
                }

                // Obtener velociad aleatoria
                if rand_speed > 0 &&  rand_speed <= 70 {
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
                if rand_speed >= 251 &&  rand_speed < 255 {
                    speed = 10;
                }

                // Obtener tipo
                if rand_type > 0 &&  rand_type <= 51 {
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
                if rand_type >= 205 &&  rand_type < 255 {
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
        
                self.burritos.mint(burrito_id.clone(), token_owner_id.clone(), Some(new_burrito.clone()));
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
                    win : burrito_data.win,
                    global_win : burrito_data.global_win,
                    level : burrito_data.level
                };

                let ext : String =  "".to_string()+&burrito.hp.clone()+&":".to_string()+
                                                &burrito.attack.clone()+&":".to_string()+
                                                &burrito.defense.clone()+&":".to_string()+
                                                &burrito.speed.clone()+&":".to_string()+
                                                &burrito.win.clone()+&":".to_string()+
                                                &burrito.global_win.clone()+&":".to_string()+
                                                &burrito.burrito_type.clone()+&":".to_string()+
                                                &burrito.level.clone();

                let graphdata = Thegraphstructure {
                    contract_name: BURRITO_CONTRACT.to_string(),
                    collection: colecction.clone().to_string(),
                    collection_id: "4".to_string(),
                    token_id : burrito_id.to_string(),
                    owner_id : owner_id.to_string(),
                    title : new_burrito.title.as_ref().unwrap().to_string(),
                    description : new_burrito.description.as_ref().unwrap().to_string(),
                    media : "imagen".to_string(),
                    creator : owner_id.to_string(),
                    price : "0".to_string(),
                    status: "U".to_string(),
                    adressbidder: owner_id.to_string(),
                    highestbid: "0".to_string(),
                    lowestbid: "0".to_string(),
                    expires_at: "null".to_string(),
                    starts_at: "null".to_string(),
                    extra: ext.clone()
                };

                // Mandar a guardar a TheGraph

                // let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string()+","+&graphdata.collection_id.to_string();
        
                // ext_nft::save_mint_ttg(
                //     rett.clone(),
                //     &MK_CONTRACT,
                //     env::attached_deposit(),
                //     100_000_000_000_000
                // );

                // rett.to_string()

                serde_json::to_string(&burrito).unwrap()

            }
        }

    }

    // Minar un nuevo token desde contrato externo
    #[payable]
    pub fn mint_token_ext(&mut self,token_owner_id: ValidAccountId, colecction: String, token_metadata: TokenMetadata) -> String {
        let mut new_burrito = token_metadata;
        let burrito_id: TokenId = self.n_burritos.to_string();
        let mut burrito_data = ExtraBurrito {
            hp : "5".to_string(),
            attack : "".to_string(),
            defense : "".to_string(),
            speed : "".to_string(),
            win : "0".to_string(),
            global_win : "0".to_string(),
            burrito_type : "".to_string(),
            level : "1".to_string()
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
        if rand_attack > 0 &&  rand_attack <= 70 {
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
        if rand_attack >= 251 &&  rand_attack < 255 {
            attack = 10;
        }

        // Obtener defensa aleatoria
        if rand_defense > 0 &&  rand_defense <= 70 {
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
        if rand_defense >= 251 &&  rand_defense < 255 {
            defense = 10;
        }

        // Obtener velociad aleatoria
        if rand_speed > 0 &&  rand_speed <= 70 {
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
        if rand_speed >= 251 &&  rand_speed < 255 {
            speed = 10;
        }

        // Obtener tipo
        if rand_type > 0 &&  rand_type <= 51 {
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
        if rand_type >= 205 &&  rand_type < 255 {
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

        self.burritos.mint(burrito_id.clone(), token_owner_id.clone(), Some(new_burrito.clone()));

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
            win : burrito_data.win,
            global_win : burrito_data.global_win,
            level : burrito_data.level
        };

        let ext : String =  "".to_string()+&burrito.hp.clone()+&":".to_string()+
                                        &burrito.attack.clone()+&":".to_string()+
                                        &burrito.defense.clone()+&":".to_string()+
                                        &burrito.speed.clone()+&":".to_string()+
                                        &burrito.win.clone()+&":".to_string()+
                                        &burrito.global_win.clone()+&":".to_string()+
                                        &burrito.burrito_type.clone()+&":".to_string()+
                                        &burrito.level.clone();

        let graphdata = Thegraphstructure {
            contract_name: BURRITO_CONTRACT.to_string(),
            collection: colecction.clone().to_string(),
            collection_id: "4".to_string(),
            token_id : burrito_id.to_string(),
            owner_id : owner_id.to_string(),
            title : new_burrito.title.as_ref().unwrap().to_string(),
            description : new_burrito.description.as_ref().unwrap().to_string(),
            media : "imagen".to_string(),
            creator : owner_id.to_string(),
            price : "0".to_string(),
            status: "U".to_string(),
            adressbidder: owner_id.to_string(),
            highestbid: "0".to_string(),
            lowestbid: "0".to_string(),
            expires_at: "null".to_string(),
            starts_at: "null".to_string(),
            extra: ext.clone()
        };

        let rett : String = graphdata.contract_name.to_string()+","+&graphdata.token_id.to_string()+","+&graphdata.owner_id.to_string()+","+ &graphdata.title.to_string()+","+&graphdata.description.to_string()+","+ &graphdata.media.to_string()+","+&graphdata.creator.to_string()+","+&graphdata.price.to_string()+","+ &graphdata.status.to_string()+","+ &graphdata.adressbidder.to_string()+","+ &graphdata.highestbid.to_string()+","+ &graphdata.lowestbid.to_string()+","+&graphdata.expires_at.to_string()+","+ &graphdata.starts_at.to_string()+","+&graphdata.extra.to_string()+","+&graphdata.collection.to_string()+","+&graphdata.collection_id.to_string();
        rett
    }

    // Obtener burrito
    pub fn get_burrito(&self, burrito_id: TokenId) -> Burrito {

        if burrito_id.clone().parse::<u128>().unwrap() > self.n_burritos-1 {
            env::panic(b"No existe el burrito con el id ingresado");
        }

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
            win : extradatajson.win,
            global_win : extradatajson.global_win,
            level : extradatajson.level
        };

        burrito
    }

    // Modificar burrito
    pub fn update_burrito(&mut self, burrito_id: TokenId, extra: String) -> Burrito {

        // Validar que exista el id
        if burrito_id.clone().parse::<u128>().unwrap() > self.n_burritos-1 {
            env::panic(b"No existe el burrito con el id ingresado");
        }

        // Validar que el burrito pertenezca al signer
        let token_owner_id = env::signer_account_id();
        let owner_id = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();

        if token_owner_id.clone() != owner_id.clone() {
            env::panic(b"El burrito no te pertenece");
        }

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
            win : extradatajson.win,
            global_win : extradatajson.global_win,
            level : extradatajson.level
        };

        burrito
    }

    // Evolucionar burrito 100,000 $STRW tokens + 2 $NEAR tokens
    #[payable]
    pub fn evolve_burrito(&mut self, burrito_id: TokenId) -> Promise {
        if burrito_id.clone().parse::<u128>().unwrap() > self.n_burritos-1 {
            env::panic(b"No existe el burrito con el id ingresado");
        }

        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();        
        let owner_id = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();

        if account_id.clone() != owner_id.clone() {
            env::panic(b"El burrito no te pertenece");
        }

        let metadata_burrito = self
            .burritos
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&burrito_id.clone()))
            .unwrap();

        let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();
        let win_burrito = extradatajson_burrito.win.clone().parse::<u8>().unwrap();
        let level_burrito = extradatajson_burrito.level.clone().parse::<u8>().unwrap();

        if level_burrito == 40 {
            env::panic(
                format!("El burrito ya no puede evolucionar, el nivel máximo es el 40").as_bytes(),
            );
        }

        if win_burrito < 10 {
            env::panic(
                format!("El burrito no cumple las victorias para evolucionar, tiene {} y deben ser 10", &win_burrito).as_bytes(),
            );
        }

        ext_nft::get_balance_and_transfer(
            account_id.clone().to_string(),
            "Evolve".to_string(),
            &STRWTOKEN_CONTRACT,
            deposit,
            BASE_GAS
        ).then(ext_self::burrito_level_up(
            burrito_id.to_string(),
            &BURRITO_CONTRACT, // Contrato de burritos
            NO_DEPOSIT, // yocto NEAR a ajuntar al callback
            BASE_GAS // gas a ajuntar al callback
        ))
    }

    pub fn burrito_level_up(&mut self, burrito_id: TokenId) -> String{
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(_result) => {
                // let value = str::from_utf8(&result).unwrap();
                // let strw_user_tokens_str = str::replace(&value.clone().to_string(), "\"", "");
                // let strw_user_tokens = strw_user_tokens_str.trim().parse::<u128>().unwrap();
        
                let mut metadata_burrito = self
                .burritos
                .token_metadata_by_id
                .as_ref()
                .and_then(|by_id| by_id.get(&burrito_id.clone()))
                .unwrap();
        
                let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        
                let mut extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();
        
                let owner_id_burrito = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();
        
                // Crear estructura burrito
                let mut burrito = Burrito {
                    owner_id : owner_id_burrito.clone().to_string(),
                    name : metadata_burrito.title.as_ref().unwrap().to_string(),
                    description : metadata_burrito.description.as_ref().unwrap().to_string(),
                    burrito_type : extradatajson_burrito.burrito_type.clone(),
                    hp : extradatajson_burrito.hp.clone(),
                    attack : extradatajson_burrito.attack.clone(),
                    defense : extradatajson_burrito.defense.clone(),
                    speed : extradatajson_burrito.speed.clone(),
                    win : extradatajson_burrito.win.clone(),
                    global_win : extradatajson_burrito.global_win.clone(),
                    level : extradatajson_burrito.level.clone()
                };
                
                let win_burrito = 0;                
                let new_level_burrito = burrito.level.parse::<u8>().unwrap()+1;
                extradatajson_burrito.level = new_level_burrito.to_string();
                burrito.level = extradatajson_burrito.level.clone();
                burrito.win = "0".to_string();
    
                // Incrementar estadísticas
                // Generar número aleatorio para ver a cuál estadística aumentarle 3 puntos
                // De las 2 estadísticas que no fueron aumentadas, seleccionar 1 para aumentarle 2 puntos
                let rand_prop1 = *env::random_seed().get(4).unwrap();
                let rand_prop2 = *env::random_seed().get(5).unwrap();
    
                if rand_prop1 < 86 {
                    extradatajson_burrito.attack = (extradatajson_burrito.attack.clone().parse::<u8>().unwrap()+3).to_string();
                    burrito.attack = extradatajson_burrito.attack.clone();
                    if rand_prop2 < 128 {
                        extradatajson_burrito.defense = (extradatajson_burrito.defense.clone().parse::<u8>().unwrap()+2).to_string();
                        burrito.defense = extradatajson_burrito.defense.clone();
                    }
                    if rand_prop2 >= 128 && rand_prop2 < 255 {
                        extradatajson_burrito.speed = (extradatajson_burrito.speed.clone().parse::<u8>().unwrap()+2).to_string();
                        burrito.speed = extradatajson_burrito.speed.clone();
                    }
                }
                if rand_prop1 >= 86 && rand_prop1 < 171 {
                    extradatajson_burrito.defense = (extradatajson_burrito.defense.clone().parse::<u8>().unwrap()+3).to_string();
                    burrito.defense = extradatajson_burrito.defense.clone();
                    if rand_prop2 < 128 {
                        extradatajson_burrito.attack = (extradatajson_burrito.attack.clone().parse::<u8>().unwrap()+2).to_string();
                        burrito.attack = extradatajson_burrito.attack.clone();
                    }
                    if rand_prop2 >= 128 && rand_prop2 < 255 {
                        extradatajson_burrito.speed = (extradatajson_burrito.speed.clone().parse::<u8>().unwrap()+2).to_string();
                        burrito.speed = extradatajson_burrito.speed.clone();
                    }
                }
                if rand_prop1 >= 171 && rand_prop1 < 255 {
                    extradatajson_burrito.speed = (extradatajson_burrito.speed.clone().parse::<u8>().unwrap()+3).to_string();
                    burrito.speed = extradatajson_burrito.speed.clone();
                    if rand_prop2 < 128 {
                        extradatajson_burrito.attack = (extradatajson_burrito.attack.clone().parse::<u8>().unwrap()+2).to_string();
                        burrito.attack = extradatajson_burrito.attack.clone();
                    }
                    if rand_prop2 >= 128 && rand_prop2 < 255 {
                        extradatajson_burrito.defense = (extradatajson_burrito.defense.clone().parse::<u8>().unwrap()+2).to_string();
                        burrito.defense = extradatajson_burrito.defense.clone();
                    }
                }
    
                extradatajson_burrito.win = win_burrito.to_string();
    
                let mut extra_string_burrito = serde_json::to_string(&extradatajson_burrito).unwrap();
                extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
                metadata_burrito.extra = Some(extra_string_burrito.clone());
        
                self.burritos
                    .token_metadata_by_id
                    .as_mut()
                    .and_then(|by_id| by_id.insert(&burrito_id, &metadata_burrito));
        
                // Mandar a actualizar a TheGraph

                "Burrito Evolucionado".to_string()

            }
        }

    }

    // Restaurar burrito 30,000 $STRW tokens + 1 $NEAR tokens
    #[payable]
    pub fn reset_burrito(&mut self, burrito_id: TokenId) -> Promise {
        if burrito_id.clone().parse::<u128>().unwrap() > self.n_burritos-1 {
            env::panic(b"No existe el burrito con el id ingresado");
        }

        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let deposit = env::attached_deposit();        
        let owner_id = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();

        if account_id.clone() != owner_id.clone() {
            env::panic(b"El burrito no te pertenece");
        }

        ext_nft::get_balance_and_transfer(
            account_id.clone().to_string(),
            "Reset".to_string(),
            &STRWTOKEN_CONTRACT,
            deposit,
            BASE_GAS
        ).then(ext_self::reset_conditions(
            burrito_id.to_string(),
            &BURRITO_CONTRACT, // Contrato de burritos
            NO_DEPOSIT, // yocto NEAR a ajuntar al callback
            BASE_GAS // gas a ajuntar al callback
        ))
    }

    pub fn reset_conditions(&mut self, burrito_id: TokenId) -> String{
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(_result) => {
                // let value = str::from_utf8(&result).unwrap();
                // let strw_user_tokens_str = str::replace(&value.clone().to_string(), "\"", "");
                // let strw_user_tokens = strw_user_tokens_str.trim().parse::<u128>().unwrap();
        
                let mut metadata_burrito = self
                .burritos
                .token_metadata_by_id
                .as_ref()
                .and_then(|by_id| by_id.get(&burrito_id.clone()))
                .unwrap();
        
                let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        
                let mut extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();
                                       
                extradatajson_burrito.hp = "5".to_string();
        
                let mut extra_string_burrito = serde_json::to_string(&extradatajson_burrito).unwrap();
                extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
                metadata_burrito.extra = Some(extra_string_burrito.clone());
        
                self.burritos
                    .token_metadata_by_id
                    .as_mut()
                    .and_then(|by_id| by_id.insert(&burrito_id, &metadata_burrito));
        
                // Mandar a actualizar a TheGraph

                "Burrito Restaurado".to_string()

            }
        }

    }

    // Obtener cantidad de batallas activas Player vs CPU
    pub fn get_number_battles_actives_cpu(&self) -> u128 {
        self.n_battle_rooms_cpu
    }

    // Mostrar todo el historial de batallas finalizadas del jugador
    pub fn get_battle_rooms_history(&mut self) {
        let token_owner_id = env::signer_account_id();
        self.battle_history.retain(|k, _v| {
            if k.to_string().contains(&token_owner_id.to_string()) {
                log!("{}",k);
                env::log(
                    json!(_v)
                    .to_string()
                    .as_bytes(),
                );
            }
            true
         });
    }

    // Obtener numero de batallas finalizadas
    pub fn get_number_battles(&self) -> u128 {
        self.n_battles
    }

    // Obtener sala de batalla creada por account_id
    pub fn get_battle_active_cpu(&self) -> BattleCPU {
        let token_owner_id = env::signer_account_id();

        let br = self.battle_room_cpu.get(&token_owner_id.to_string());
        
        if br.is_none() {
            env::panic(b"No existe sala creada de esta cuenta");
        }

        let info = br.unwrap();

        let battle_room = BattleCPU {
            status : info.status.to_string(),
            payer_id : info.payer_id.to_string(),
            burrito_id : info.burrito_id.to_string(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player : info.strong_attack_player.to_string(),
            shields_player : info.shields_player.to_string(),
            health_player : info.health_player.to_string(),
            strong_attack_cpu : info.strong_attack_cpu.to_string(),
            shields_cpu : info.shields_cpu.to_string(),
            health_cpu : info.health_cpu.to_string(),
            burrito_cpu_level : info.burrito_cpu_level.to_string(),
            burrito_cpu_type : info.burrito_cpu_type.to_string(),
            burrito_cpu_attack : info.burrito_cpu_attack.to_string(),
            burrito_cpu_defense : info.burrito_cpu_defense.to_string(),
            burrito_cpu_speed : info.burrito_cpu_speed.to_string()
        };

        battle_room
    }

    // Guardar sala de combate Player vs CPU
    pub fn create_battle_player_cpu(&mut self, burrito_id: TokenId, accesorio1_id: TokenId, accesorio2_id: TokenId, accesorio3_id: TokenId) -> Promise {
        let token_owner_id = env::signer_account_id();

        let br = self.battle_room_cpu.get(&token_owner_id.to_string());
        
        if br.is_some() {
            env::panic(b"Ya tienes una partida iniciada, debes terminarla o rendirte");
        }
        
        // Validar que exista el id
        if burrito_id.clone().parse::<u128>().unwrap() > self.n_burritos-1 {
            env::panic(b"No existe el Burrito a utilizar para el combate");
        }

        // Validar que el burrito pertenezca al signer
        let token_owner_id = env::signer_account_id();
        let owner_id = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();

        if token_owner_id.clone() != owner_id.clone() {
            env::panic(b"El Burrito a utilizar no te pertenece");
        }

        // Validar que los 3 accesorios sean diferentes
        if (accesorio1_id.clone().parse::<u128>().unwrap() == accesorio2_id.clone().parse::<u128>().unwrap() && 
            accesorio1_id.clone().parse::<u128>().unwrap() != 0 && accesorio2_id.clone().parse::<u128>().unwrap() != 0) 
            || 
            (accesorio1_id.clone().parse::<u128>().unwrap() == accesorio3_id.clone().parse::<u128>().unwrap() &&
            accesorio1_id.clone().parse::<u128>().unwrap() != 0 && accesorio3_id.clone().parse::<u128>().unwrap() != 0) 
            || 
            (accesorio2_id.clone().parse::<u128>().unwrap() == accesorio3_id.clone().parse::<u128>().unwrap() &&
            accesorio2_id.clone().parse::<u128>().unwrap() != 0 && accesorio3_id.clone().parse::<u128>().unwrap() != 0) 
        {
            env::panic(b"Los 3 Items a equipar deben ser diferentes");
        }

        // Obtener información de los accesorios para ver si existen y recuperar las estadísticas a aumentar
        let p = ext_nft::get_items_for_battle_cpu(
            accesorio1_id.to_string(), // Id el item 1 del burrito
            accesorio2_id.to_string(), // Id el item 2 del burrito
            accesorio3_id.to_string(), // Id el item 3 del burrito
            &ITEMS_CONTRACT, // Contrato de items
            NO_DEPOSIT, // yocto NEAR a ajuntar
            100_000_000_000_000 // gas a ajuntar
        )
        .then(ext_self::save_battle_player_cpu(
            burrito_id,
            &BURRITO_CONTRACT, // Contrato de burritos
            NO_DEPOSIT, // yocto NEAR a ajuntar al callback
            20_000_000_000_000 // gas a ajuntar al callback
        ));

        p

    }

    // Guardar sala de combate Player vs CPU
    pub fn save_battle_player_cpu(&mut self, burrito_id: TokenId) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let value = str::from_utf8(&result).unwrap();
                let mut accessories_for_battle: AccessoriesForBattle = serde_json::from_str(&value).unwrap();

                let token_owner_id = env::signer_account_id();

                // Obtener metadata burrito 1
                let metadata_burrito = self
                .burritos
                .token_metadata_by_id
                .as_ref()
                .and_then(|by_id| by_id.get(&burrito_id.clone()))
                .unwrap();
        
                // Extraer extras del token burrito 1
                let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        
                // Crear json burrito 1
                let extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();
                let owner_id_burrito = self.burritos.owner_by_id.get(&burrito_id.clone()).unwrap();
                
                if extradatajson_burrito.hp.clone().parse::<u8>().unwrap() == 0 {
                    env::panic(b"El Burrito a utilizar no tiene vidas");
                }
        
                // Crear estructura burrito 1
                let burrito = Burrito {
                    owner_id : owner_id_burrito.clone().to_string(),
                    name : metadata_burrito.title.as_ref().unwrap().to_string(),
                    description : metadata_burrito.description.as_ref().unwrap().to_string(),
                    burrito_type : extradatajson_burrito.burrito_type.clone(),
                    hp : extradatajson_burrito.hp.clone(),
                    attack : extradatajson_burrito.attack.clone(),
                    defense : extradatajson_burrito.defense.clone(),
                    speed : extradatajson_burrito.speed.clone(),
                    win : extradatajson_burrito.win.clone(),
                    global_win : extradatajson_burrito.global_win.clone(),
                    level : extradatajson_burrito.level.clone()
                };
        
                // Generar nivel del burrito cpu --> nivel del burrito como minimo + 5 maximo
                let rand_level = *env::random_seed().get(4).unwrap();
                let mut level_cpu: u8 = 0;

                if rand_level > 0 &&  rand_level <= 70 {
                    level_cpu = extradatajson_burrito.level.clone().parse::<u8>().unwrap();
                }
                if rand_level >= 71 &&  rand_level <= 130 {
                    level_cpu = extradatajson_burrito.level.clone().parse::<u8>().unwrap() + 1;
                }
                if rand_level >= 131 &&  rand_level <= 180 {
                    level_cpu = extradatajson_burrito.level.clone().parse::<u8>().unwrap() + 2;
                }
                if rand_level >= 181 &&  rand_level <= 220 {
                    level_cpu = extradatajson_burrito.level.clone().parse::<u8>().unwrap() + 3;
                }
                if rand_level >= 221 &&  rand_level <= 250 {
                    level_cpu = extradatajson_burrito.level.clone().parse::<u8>().unwrap() + 4;
                }
                if rand_level >= 251 &&  rand_level < 255 {
                    level_cpu = extradatajson_burrito.level.clone().parse::<u8>().unwrap() + 5;
                }

                if level_cpu > 40 {
                    level_cpu = 40;
                }

                // Generar burrito aleatorio
                let mut burrito_cpu = Burrito {
                    owner_id : "BB CPU".to_string(),
                    name : "Burrito CPU".to_string(),
                    description : "This is a random burrito cpu".to_string(),
                    burrito_type : "Fuego".to_string(),
                    hp : "5".to_string(),
                    attack : "5".to_string(),
                    defense : "5".to_string(),
                    speed : "5".to_string(),
                    win : "0".to_string(),
                    global_win : "0".to_string(),
                    level : level_cpu.clone().to_string()
                };
        
                // Crear estadisticas aleatorias para burrito cpu
        
                let rand_attack = *env::random_seed().get(0).unwrap();
                let rand_defense = *env::random_seed().get(1).unwrap();
                let rand_speed = *env::random_seed().get(2).unwrap();
                let rand_type = *env::random_seed().get(3).unwrap();
        
                let mut attack: u8 = 0;
                let mut defense: u8 = 0;
                let mut speed: u8 = 0;
                let mut burrito_type: String = "Fuego".to_string();
                let burrito_cpu_level = burrito_cpu.level.clone().parse::<u8>().unwrap();
        
                // Obtener ataque aleatorio
                if rand_attack > 0 &&  rand_attack <= 70 {
                    if rand_attack % 2 == 1 {
                        attack = 5+(burrito_cpu_level.clone());
                    } else {
                        attack = 5+(burrito_cpu_level.clone()*2);
                    }    
                }
                if rand_attack >= 71 &&  rand_attack <= 130 {
                    if rand_attack % 2 == 1 {
                        attack = 6+(burrito_cpu_level.clone());
                    } else {
                        attack = 6+(burrito_cpu_level.clone()*2);
                    }                 
                }
                if rand_attack >= 131 &&  rand_attack <= 180 {
                    if rand_attack % 2 == 1 {
                        attack = 7+(burrito_cpu_level.clone());
                    } else {
                        attack = 7+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_attack >= 181 &&  rand_attack <= 220 {
                    if rand_attack % 2 == 1 {
                        attack = 8+(burrito_cpu_level.clone());
                    } else {
                        attack = 8+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_attack >= 221 &&  rand_attack <= 250 {
                    if rand_attack % 2 == 1 {
                        attack = 9+(burrito_cpu_level.clone());
                    } else {
                        attack = 9+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_attack >= 251 &&  rand_attack < 255 {
                    if rand_attack % 2 == 1 {
                        attack = 10+(burrito_cpu_level.clone());
                    } else {
                        attack = 10+(burrito_cpu_level.clone()*2);
                    } 
                }
        
                // Obtener defensa aleatoria
                if rand_defense > 0 &&  rand_defense <= 70 {
                    if rand_defense % 2 == 1 {
                        defense = 5+(burrito_cpu_level.clone());
                    } else {
                        defense = 5+(burrito_cpu_level.clone()*2);
                    }
                }
                if rand_defense >= 71 &&  rand_defense <= 130 {
                    if rand_defense % 2 == 1 {
                        defense = 6+(burrito_cpu_level.clone());
                    } else {
                        defense = 6+(burrito_cpu_level.clone()*2);
                    }                }
                if rand_defense >= 131 &&  rand_defense <= 180 {
                    if rand_defense % 2 == 1 {
                        defense = 7+(burrito_cpu_level.clone());
                    } else {
                        defense = 7+(burrito_cpu_level.clone()*2);
                    }                
                }
                if rand_defense >= 181 &&  rand_defense <= 220 {
                    if rand_defense % 2 == 1 {
                        defense = 8+(burrito_cpu_level.clone());
                    } else {
                        defense = 8+(burrito_cpu_level.clone()*2);
                    }                
                }
                if rand_defense >= 221 &&  rand_defense <= 250 {
                    if rand_defense % 2 == 1 {
                        defense = 9+(burrito_cpu_level.clone());
                    } else {
                        defense = 9+(burrito_cpu_level.clone()*2);
                    }                
                }
                if rand_defense >= 251 &&  rand_defense < 255 {
                    if rand_defense % 2 == 1 {
                        defense = 10+(burrito_cpu_level.clone());
                    } else {
                        defense = 10+(burrito_cpu_level.clone()*2);
                    }                
                }
        
                // Obtener velociad aleatoria
                if rand_speed > 0 &&  rand_speed <= 70 {
                    if rand_speed % 2 == 1 {
                        speed = 5+(burrito_cpu_level.clone());
                    } else {
                        speed = 5+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_speed >= 71 &&  rand_speed <= 130 {
                    if rand_speed % 2 == 1 {
                        speed = 6+(burrito_cpu_level.clone());
                    } else {
                        speed = 6+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_speed >= 131 &&  rand_speed <= 180 {
                    if rand_speed % 2 == 1 {
                        speed = 7+(burrito_cpu_level.clone());
                    } else {
                        speed = 7+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_speed >= 181 &&  rand_speed <= 220 {
                    if rand_speed % 2 == 1 {
                        speed = 8+(burrito_cpu_level.clone());
                    } else {
                        speed = 8+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_speed >= 221 &&  rand_speed <= 250 {
                    if rand_speed % 2 == 1 {
                        speed = 9+(burrito_cpu_level.clone());
                    } else {
                        speed = 9+(burrito_cpu_level.clone()*2);
                    } 
                }
                if rand_speed >= 251 &&  rand_speed < 255 {
                    if rand_speed % 2 == 1 {
                        speed = 10+(burrito_cpu_level.clone());
                    } else {
                        speed = 10+(burrito_cpu_level.clone()*2);
                    } 
                }
        
                // Obtener tipo
                if rand_type > 0 &&  rand_type <= 51 {
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
                if rand_type >= 205 &&  rand_type < 255 {
                    burrito_type = "Volador".to_string();
                }
        
                // Asignamos valores a las estadisticas del burrito
                burrito_cpu.attack = attack.to_string();
                burrito_cpu.defense = defense.to_string();
                burrito_cpu.speed = speed.to_string();
                burrito_cpu.burrito_type = burrito_type.to_string();
        
                // Determinar burrito mas veloz
                let mut burrito_first_atack = "";
        
                if burrito_cpu.speed.parse::<u8>().unwrap() > (burrito.speed.parse::<u8>().unwrap() + accessories_for_battle.final_speed_b1.clone().parse::<u8>().unwrap()) {
                    burrito_first_atack = "CPU";
                } else {
                    burrito_first_atack = "Player";
                }
                
                let info = BattleCPU {
                    status : "1".to_string(),
                    payer_id : token_owner_id.clone().to_string(),
                    burrito_id : burrito_id.clone().to_string(),
                    accesories_attack_b1 : accessories_for_battle.final_attack_b1.clone().to_string(),
                    accesories_defense_b1 : accessories_for_battle.final_defense_b1.clone().to_string(),
                    accesories_speed_b1 : accessories_for_battle.final_speed_b1.clone().to_string(),
                    accesories_attack_b2 : accessories_for_battle.final_attack_b2.clone().to_string(),
                    accesories_defense_b2 : accessories_for_battle.final_defense_b2.clone().to_string(),
                    accesories_speed_b2 : accessories_for_battle.final_speed_b2.clone().to_string(),
                    turn : burrito_first_atack.to_string(),
                    strong_attack_player : "3".to_string(),
                    shields_player : "3".to_string(),
                    health_player : (burrito.attack.parse::<u8>().unwrap()+burrito.defense.parse::<u8>().unwrap()+burrito.speed.parse::<u8>().unwrap()).to_string(),
                    strong_attack_cpu : "3".to_string(),
                    shields_cpu : "3".to_string(),
                    health_cpu : (burrito_cpu.attack.parse::<u8>().unwrap()+burrito_cpu.defense.parse::<u8>().unwrap()+burrito_cpu.speed.parse::<u8>().unwrap()).to_string(),
                    burrito_cpu_level : level_cpu.clone().to_string(),
                    burrito_cpu_type : burrito_cpu.burrito_type.to_string(),
                    burrito_cpu_attack : burrito_cpu.attack.to_string(),
                    burrito_cpu_defense : burrito_cpu.defense.to_string(),
                    burrito_cpu_speed : burrito_cpu.speed.to_string()
                };
        
                self.battle_room_cpu.insert(token_owner_id.clone().to_string(),info.clone());
                self.n_battle_rooms_cpu += 1;
        
                serde_json::to_string(&info).unwrap()

            }
        }

    }

    
    // Rendirse y finalizar batalla Player vs CPU
    pub fn surrender_cpu(&mut self) -> String {
        let token_owner_id = env::signer_account_id();

        let br = self.battle_room_cpu.get(&token_owner_id.to_string());
        
        if br.is_none() {
            env::panic(b"No tienes una batalla activa");
        }

        let info = br.unwrap();

        let battle_room = BattleCPU {
            status : info.status.to_string(),
            payer_id : info.payer_id.to_string(),
            burrito_id : info.burrito_id.to_string(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player : info.strong_attack_player.to_string(),
            shields_player : info.shields_player.to_string(),
            health_player : info.health_player.to_string(),
            strong_attack_cpu : info.strong_attack_cpu.to_string(),
            shields_cpu : info.shields_cpu.to_string(),
            health_cpu : info.health_cpu.to_string(),
            burrito_cpu_level : info.burrito_cpu_level.to_string(),
            burrito_cpu_type : info.burrito_cpu_type.to_string(),
            burrito_cpu_attack : info.burrito_cpu_attack.to_string(),
            burrito_cpu_defense : info.burrito_cpu_defense.to_string(),
            burrito_cpu_speed : info.burrito_cpu_speed.to_string()
        };

        // Restar una vida del burrito utilizado en el combate

         // Obtener metadata burrito
         let mut metadata_burrito = self
         .burritos
         .token_metadata_by_id
         .as_ref()
         .and_then(|by_id| by_id.get(&battle_room.burrito_id.clone()))
         .unwrap();

         // Extraer extras del token burrito 1
         let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");

         // Crear json burrito 1
         let mut extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();

         let owner_id_burrito = self.burritos.owner_by_id.get(&battle_room.burrito_id.clone()).unwrap();
         
         // Crear estructura burrito
         let burrito = Burrito {
             owner_id : owner_id_burrito.clone().to_string(),
             name : metadata_burrito.title.as_ref().unwrap().to_string(),
             description : metadata_burrito.description.as_ref().unwrap().to_string(),
             burrito_type : extradatajson_burrito.burrito_type.clone(),
             hp : extradatajson_burrito.hp.clone(),
             attack : extradatajson_burrito.attack.clone(),
             defense : extradatajson_burrito.defense.clone(),
             speed : extradatajson_burrito.speed.clone(),
             win : extradatajson_burrito.win.clone(),
             global_win : extradatajson_burrito.global_win.clone(),
             level : extradatajson_burrito.level.clone()
         };

        let new_hp_burrito = burrito.hp.parse::<u8>().unwrap()-1;
        extradatajson_burrito.hp = new_hp_burrito.to_string();

        let mut extra_string_burrito = serde_json::to_string(&extradatajson_burrito).unwrap();
        extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
        metadata_burrito.extra = Some(extra_string_burrito.clone());

        self.burritos
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&battle_room.burrito_id.clone(), &metadata_burrito));

        // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
        let info = BattlesHistory {
            payer1_id : battle_room.payer_id.to_string(),
            payer2_id : "CPU".to_string(),
            winner : "CPU".to_string(),
            status : "Surrender".to_string()
        };

        self.battle_history.insert(battle_room.payer_id.to_string()+&"-".to_string()+ &self.n_battles.to_string(),info);
        self.n_battles += 1;

        // Eliminar sala
        self.battle_room_cpu.remove(&token_owner_id);
        self.n_battle_rooms_cpu -= 1;

        "Finalizó batalla".to_string()
    }
    
    // Método combate player vs cpu (type_move 1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse 4 = Defenderse)
    pub fn battle_player_cpu(&mut self, type_move: String) -> String {
        let token_owner_id = env::signer_account_id();

        let br = self.battle_room_cpu.get(&token_owner_id.to_string());
        
        if br.is_none() {
            env::panic(b"No tienes una batalla activa");
        }

        let info = br.unwrap();

        let battle_room = BattleCPU {
            status : info.status.to_string(),
            payer_id : info.payer_id.to_string(),
            burrito_id : info.burrito_id.clone().to_string(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player : info.strong_attack_player.to_string(),
            shields_player : info.shields_player.to_string(),
            health_player : info.health_player.to_string(),
            strong_attack_cpu : info.strong_attack_cpu.to_string(),
            shields_cpu : info.shields_cpu.to_string(),
            health_cpu : info.health_cpu.to_string(),
            burrito_cpu_level : info.burrito_cpu_level.to_string(),
            burrito_cpu_type : info.burrito_cpu_type.to_string(),
            burrito_cpu_attack : info.burrito_cpu_attack.to_string(),
            burrito_cpu_defense : info.burrito_cpu_defense.to_string(),
            burrito_cpu_speed : info.burrito_cpu_speed.to_string()
        };


        if (type_move == "1" || type_move == "2") && battle_room.turn == "CPU"{
            env::panic(b"No puedes realizar un ataque, debes elegir si defenderte o no");
        }

        if (type_move == "3" || type_move == "4") && battle_room.turn == "Player"{
            env::panic(b"No puedes defenderte, debes realizar un ataque");
        }

        if type_move == "2" && battle_room.strong_attack_player.parse::<u8>().unwrap() == 0 {
            env::panic(b"No tienes mas ataques fuertes, debes realizar uno normal");
        }

        if type_move == "4" && battle_room.shields_player.parse::<u8>().unwrap() == 0 {
            env::panic(b"No tienes mas escudos, no puedes defenderte");
        }

        let mut old_battle_room = battle_room;
        let mut cpu_type_move = "1";

        // Verificar si se utilizo un escudo para finalizar la ronda
        if old_battle_room.turn == "Player"{
            if type_move == "2"{
                old_battle_room.strong_attack_player = (old_battle_room.strong_attack_player.parse::<u8>().unwrap()-1).to_string();
                log!("Jugador utilizó ataque fuerte");
            }
            // Validar si el CPU aun tiene escudos y elegir aleatoriamente si utilizara uno o no
            if old_battle_room.shields_cpu.parse::<u8>().unwrap() > 0 {
                let use_shield: u8 = *env::random_seed().get(0).unwrap();
                if use_shield % 2 == 1 {
                    old_battle_room.shields_cpu = (old_battle_room.shields_cpu.parse::<u8>().unwrap()-1).to_string();
                    old_battle_room.turn = "CPU".to_string();
                    self.battle_room_cpu.remove(&old_battle_room.payer_id);
                    self.battle_room_cpu.insert(old_battle_room.payer_id.to_string(),old_battle_room.clone());
                    log!("CPU utilizó escudo");
                    return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");
                }
            }
        } else {
            if old_battle_room.strong_attack_cpu.parse::<u8>().unwrap() > 0 {
                let use_strong_attack: u8 = *env::random_seed().get(0).unwrap();
                if old_battle_room.shields_player.parse::<u8>().unwrap() == 0 {
                    old_battle_room.strong_attack_cpu = (old_battle_room.strong_attack_cpu.parse::<u8>().unwrap()-1).to_string();
                    cpu_type_move = "2";
                    log!("CPU utilizó ataque fuerte");
                } else {
                    if use_strong_attack % 2 == 1 {
                        old_battle_room.strong_attack_cpu = (old_battle_room.strong_attack_cpu.parse::<u8>().unwrap()-1).to_string();
                        cpu_type_move = "2";
                        log!("CPU utilizó ataque fuerte");
                    }
                }
            }
            if type_move == "4"{
                old_battle_room.shields_player = (old_battle_room.shields_player.parse::<u8>().unwrap()-1).to_string();
                old_battle_room.turn = "Player".to_string();
                self.battle_room_cpu.remove(&old_battle_room.payer_id);
                self.battle_room_cpu.insert(old_battle_room.payer_id.to_string(),old_battle_room.clone());
                log!("Jugador utilizó escudo");
                return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");
            }
        }

        // Obtener metadata burrito
        let mut metadata_burrito = self
        .burritos
        .token_metadata_by_id
        .as_ref()
        .and_then(|by_id| by_id.get(&old_battle_room.burrito_id.clone()))
        .unwrap();

        // Extraer extras del token burrito 1
        let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Crear json burrito 1
        let mut extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();

        let owner_id_burrito = self.burritos.owner_by_id.get(&old_battle_room.burrito_id.clone()).unwrap();
        
        // Crear estructura burrito
        let burrito = Burrito {
            owner_id : owner_id_burrito.clone().to_string(),
            name : metadata_burrito.title.as_ref().unwrap().to_string(),
            description : metadata_burrito.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson_burrito.burrito_type.clone(),
            hp : extradatajson_burrito.hp.clone(),
            attack : extradatajson_burrito.attack.clone(),
            defense : extradatajson_burrito.defense.clone(),
            speed : extradatajson_burrito.speed.clone(),
            win : extradatajson_burrito.win.clone(),
            global_win : extradatajson_burrito.global_win.clone(),
            level : extradatajson_burrito.level.clone()
        };

        // env::log(
        //     json!(burrito)
        //     .to_string()
        //     .as_bytes(),
        // );

        // Crear estructura burrito cpu
        let burrito_cpu = Burrito {
            owner_id : "BB CPU".to_string(),
            name : "Burrito CPU".to_string(),
            description : "This is a random burrito cpu".to_string(),
            burrito_type : old_battle_room.burrito_cpu_type.to_string(),
            hp : "5".to_string(),
            attack : old_battle_room.burrito_cpu_attack.to_string(),
            defense : old_battle_room.burrito_cpu_defense.to_string(),
            speed : old_battle_room.burrito_cpu_speed.to_string(),
            win : "0".to_string(),
            global_win : "0".to_string(),
            level : old_battle_room.burrito_cpu_level.to_string()
        };

        // env::log(
        //     json!(burrito_cpu)
        //     .to_string()
        //     .as_bytes(),
        // );

        // Calculos de daño

        let rand_attack: u8 = *env::random_seed().get(0).unwrap();

        let mut attack_mult: f32 = 0.0;
        let mut type_mult: f32 = 0.0;

        let burrito_attacker;
        let burrito_defender;
        let mut old_health_burrito_defender: f32 = 0.0;

        if old_battle_room.turn == "Player"{
            burrito_attacker = burrito.clone();
            burrito_defender = burrito_cpu;
            old_health_burrito_defender = old_battle_room.health_cpu.parse::<f32>().unwrap();
        } else {
            burrito_attacker = burrito_cpu;
            burrito_defender = burrito.clone();
            old_health_burrito_defender = old_battle_room.health_player.parse::<f32>().unwrap();
        }

        if rand_attack < 10 {
            attack_mult = rand_attack as f32 * 0.1;
        }
        if rand_attack >= 10 && rand_attack < 100 {
            attack_mult = rand_attack as f32 * 0.01;
        }
        if rand_attack >= 100 && rand_attack < 255 {
            attack_mult = rand_attack as f32 * 0.001;
        }
        if burrito_attacker.burrito_type == "Fuego" && burrito_defender.burrito_type == "Planta"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Agua" && burrito_defender.burrito_type == "Fuego"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Planta" && burrito_defender.burrito_type == "Eléctrico"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Eléctrico" && burrito_defender.burrito_type == "Volador"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }
        if burrito_attacker.burrito_type == "Volador" && burrito_defender.burrito_type == "Agua"{
            type_mult = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)*0.25
        }

        // env::log(
        //     json!(accessories_for_battle)
        //     .to_string()
        //     .as_bytes(),
        // );

        log!("Vida vieja burrito defensor: {}",old_health_burrito_defender);

        let mut attack = 0.0;
        if old_battle_room.turn == "Player"{
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult+old_battle_room.accesories_attack_b1.parse::<f32>().unwrap();
        } else {
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult+old_battle_room.accesories_attack_b2.parse::<f32>().unwrap();
        }
        log!("Cantidad de daño a realizar: {}",attack);

        // Verificar el tipo de ataque
        if old_battle_room.turn == "Player"{
            if type_move == "2"{
                attack += attack*0.5;
                log!("Cantidad de daño fuerte a realizar: {}",attack);
            }
        } else {
            if cpu_type_move == "2"{
                attack += attack*0.5;
                log!("Cantidad de daño fuerte a realizar: {}",attack);
            }
        }
        
        let new_health_burrito_defender = old_health_burrito_defender - attack;
        log!("Vida nueva burrito defensor: {}",new_health_burrito_defender);
        
        // Actualizar registro de sala de batalla
        if old_battle_room.turn == "Player"{
            if new_health_burrito_defender <= 0.0 {
                // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
                let info = BattlesHistory {
                    payer1_id : old_battle_room.payer_id.to_string(),
                    payer2_id : "CPU".to_string(),
                    winner : old_battle_room.payer_id.to_string(),
                    status : "Battle".to_string()
                };
                self.battle_history.insert(old_battle_room.payer_id.to_string()+&"-".to_string()+ &self.n_battles.to_string(),info);
                self.n_battles += 1;
                // Eliminar sala activa
                self.battle_room_cpu.remove(&old_battle_room.payer_id);
                self.n_battle_rooms_cpu -= 1;
                log!("Batalla Finalizada, Ganó Jugador");
                
                // Incrementar victorias del burrito si son < 10
                let mut new_win_burrito1 = extradatajson_burrito.win.parse::<u8>().unwrap();
                let new_global_win_burrito1 = extradatajson_burrito.global_win.parse::<u8>().unwrap()+1;

                if new_win_burrito1 < 10 {
                    new_win_burrito1 += 1;
                }

                extradatajson_burrito.win = new_win_burrito1.to_string();
                extradatajson_burrito.global_win = new_global_win_burrito1.to_string();

                let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito).unwrap();
                extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
                metadata_burrito.extra = Some(extra_string_burrito1.clone());
    
                self.burritos
                    .token_metadata_by_id
                    .as_mut()
                    .and_then(|by_id| by_id.insert(&old_battle_room.burrito_id.clone(), &metadata_burrito));

                // Minar recompensa STRW Tokens
                log!("Nivel burrito cpu {}",burrito_defender.level.clone().to_string().parse::<f32>().unwrap());
                let mut tokens_mint : f32 = 0.0;

                if burrito_attacker.level.clone().parse::<u8>().unwrap() < 10 {
                    tokens_mint = 5.0*(burrito_defender.level.clone().parse::<f32>().unwrap()/burrito_attacker.level.clone().parse::<f32>().unwrap());
                }
                if burrito_attacker.level.clone().parse::<u8>().unwrap() >= 10 && burrito_attacker.level.clone().parse::<u8>().unwrap() <= 14 {
                    tokens_mint = 10.0*(burrito_defender.level.clone().parse::<f32>().unwrap()/burrito_attacker.level.clone().parse::<f32>().unwrap());
                }
                if burrito_attacker.level.clone().parse::<u8>().unwrap() >= 15 && burrito_attacker.level.clone().parse::<u8>().unwrap() <= 19 {
                    tokens_mint = 15.0*(burrito_defender.level.clone().parse::<f32>().unwrap()/burrito_attacker.level.clone().parse::<f32>().unwrap());
                }
                if burrito_attacker.level.clone().parse::<u8>().unwrap() >= 20 && burrito_attacker.level.clone().parse::<u8>().unwrap() <= 24 {
                    tokens_mint = 25.0*(burrito_defender.level.clone().parse::<f32>().unwrap()/burrito_attacker.level.clone().parse::<f32>().unwrap());
                }
                if burrito_attacker.level.clone().parse::<u8>().unwrap() >= 25 && burrito_attacker.level.clone().parse::<u8>().unwrap() <= 29 {
                    tokens_mint = 40.0*(burrito_defender.level.clone().parse::<f32>().unwrap()/burrito_attacker.level.clone().parse::<f32>().unwrap());
                }
                if burrito_attacker.level.clone().parse::<u8>().unwrap() >= 30 && burrito_attacker.level.clone().parse::<u8>().unwrap() <= 34 {
                    tokens_mint = 50.0*(burrito_defender.level.clone().parse::<f32>().unwrap()/burrito_attacker.level.clone().parse::<f32>().unwrap());
                }
                if burrito_attacker.level.clone().parse::<u8>().unwrap() >= 35 && burrito_attacker.level.clone().parse::<u8>().unwrap() <= 39 {
                    tokens_mint = 55.0*(burrito_defender.level.clone().parse::<f32>().unwrap()/burrito_attacker.level.clone().parse::<f32>().unwrap());
                }
                if burrito_attacker.level.clone().parse::<u8>().unwrap() == 40 {
                    tokens_mint = 60.0;
                }

                log!("Tokens a minar {}",tokens_mint*1000000000000000000000000.0);
                let tokens_to_mint = tokens_mint*1000000000000000000000000.0;
                ext_nft::reward_player(
                    old_battle_room.payer_id.clone().to_string(),
                    tokens_to_mint.to_string(),
                    &STRWTOKEN_CONTRACT,
                    0000000000000000000000001,
                    100_000_000_000_000
                );

                return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");
            } else {
                old_battle_room.health_cpu = new_health_burrito_defender.to_string();
                old_battle_room.turn = "CPU".to_string();
                self.battle_room_cpu.remove(&old_battle_room.payer_id);
                self.battle_room_cpu.insert(old_battle_room.payer_id.to_string(),old_battle_room.clone());
            }
        } else {
            if new_health_burrito_defender <= 0.0 {
                // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
                let info = BattlesHistory {
                    payer1_id : old_battle_room.payer_id.to_string(),
                    payer2_id : "CPU".to_string(),
                    winner : "CPU".to_string(),
                    status : "Battle".to_string()
                };
                self.battle_history.insert(old_battle_room.payer_id.to_string()+&"-".to_string()+ &self.n_battles.to_string(),info);
                self.n_battles += 1;
                // Eliminar sala activa
                self.battle_room_cpu.remove(&old_battle_room.payer_id);
                self.n_battle_rooms_cpu -= 1;
                log!("Batalla Finalizada, Ganó CPU");

                // Restar una vida al burrito
                let new_hp_burrito = burrito.hp.parse::<u8>().unwrap()-1;
                extradatajson_burrito.hp = new_hp_burrito.to_string();
    
                let mut extra_string_burrito = serde_json::to_string(&extradatajson_burrito).unwrap();
                extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
                metadata_burrito.extra = Some(extra_string_burrito.clone());
                
                self.burritos
                .token_metadata_by_id
                .as_mut()
                .and_then(|by_id| by_id.insert(&old_battle_room.burrito_id.clone(), &metadata_burrito));

                return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");
            } else {
                old_battle_room.health_player = new_health_burrito_defender.to_string();
                old_battle_room.turn = "Player".to_string();
                self.battle_room_cpu.remove(&old_battle_room.payer_id);
                self.battle_room_cpu.insert(old_battle_room.payer_id.to_string(),old_battle_room.clone());
            }                
        }

        log!("Ronda Finalizada");
        str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'")
    }

    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        // assert!(env::predecessor_account_id() == self.minter_account_id);
        //input is code:<Vec<u8> on REGISTER 0
        //log!("bytes.length {}", code.unwrap().len());
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
        const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id().into_bytes();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE;
        unsafe {
            BLOCKCHAIN_INTERFACE.with(|b| {
                // Load input (new contract code) into register 0
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .input(0);
                //prepare self-call promise
                let promise_id = b
                    .borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_create(current_id.len() as _, current_id.as_ptr() as _);
                //1st action, deploy/upgrade code (takes code from register 0)
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);
                //2nd action, schedule a call to "migrate()".
                //Will execute on the **new code**
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_function_call(
                        promise_id,
                        migrate_method_name.len() as _,
                        migrate_method_name.as_ptr() as _,
                        0 as _,
                        0 as _,
                        0 as _,
                        attached_gas,
                    );
            });
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldContract = env::state_read().expect("failed");
        log!("old state readed {}", old_state.n_burritos);
        Self {
            tokens: old_state.tokens,
            burritos: old_state.burritos,
            metadata: old_state.metadata,
            n_tokens: old_state.n_tokens,
            n_burritos: old_state.n_burritos,
            n_battle_rooms_cpu: old_state.n_battle_rooms_cpu,
            battle_room_cpu: old_state.battle_room_cpu,
            battle_history: old_state.battle_history,
            n_battles: old_state.n_battles
        }
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
