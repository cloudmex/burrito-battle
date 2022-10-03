use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
    PromiseResult, Gas, require
};

pub use crate::fights_cpu::*;
pub use crate::xcc::*;
pub use crate::enumerations::*;
pub use crate::migrate::*;

mod fights_cpu;
mod xcc;
mod enumerations;
mod migrate;

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
    level : String,
    media : String
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

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BattleCPU {
    status : String, // 1 = On Hold , 2 = In Battle , 3 = Finish
    player_id : String,
    burrito_id : String,
    attack_b1 : String,
    defense_b1 : String,
    speed_b1 : String,
    level_b1 : String,
    turn : String, // Player or CPU
    strong_attack_player : String, // 0-3
    shields_player : String, // 0-3
    start_health_player : String,
    health_player : String,
    strong_attack_cpu : String, // 0-3
    shields_cpu : String, // 0-3
    start_health_cpu : String,
    health_cpu : String,
    burrito_cpu_level : String,
    burrito_cpu_type : String,
    burrito_cpu_attack : String,
    burrito_cpu_defense : String,
    burrito_cpu_speed : String,
    rewards : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct BattlesHistory {
    player1_id : String,
    player2_id : String,
    winner : String,
    status : String, // Battle, Surrender
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldContract {
    //contract owner
    pub owner_id: AccountId,
    pub battle_rooms: HashMap<String,BattleCPU>,
    pub battle_history: HashMap<String,BattlesHistory>,

    pub burrito_contract: String,
    pub strw_contract: String,
    pub pve_contract: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,
    pub battle_rooms: HashMap<String,BattleCPU>,
    pub battle_history: HashMap<String,BattlesHistory>,

    pub burrito_contract: String,
    pub strw_contract: String,
    pub pve_contract: String

}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init_contract(owner_id: AccountId, burrito_contract: String, strw_contract: String, pve_contract: String) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            burrito_contract,
            strw_contract,
            pve_contract
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, burrito_contract: String, strw_contract: String, pve_contract: String) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            owner_id,
            battle_rooms:HashMap::new(),
            battle_history:HashMap::new(),
            burrito_contract : burrito_contract,
            strw_contract : strw_contract,
            pve_contract : pve_contract
        };

        //return the Contract object
        this
    }

    pub fn change_contracts(&mut self, burrito_contract: String, strw_contract: String, pve_contract: String) {
        self.assert_owner();
        self.burrito_contract = burrito_contract;
        self.strw_contract = strw_contract;
        self.pve_contract = pve_contract;
    }

    pub fn change_owner(&mut self, owner_id: AccountId) {
        self.assert_owner();
        self.owner_id = owner_id;
    }

    fn assert_owner(&self) {
        require!(self.signer_is_owner(), "Method is private to owner")
    }

    fn signer_is_owner(&self) -> bool {
        self.is_owner(&env::predecessor_account_id())
    }

    fn is_owner(&self, minter: &AccountId) -> bool {
        minter.as_str() == self.owner_id.as_str()
    }

    pub fn show_contracts(&self) {
        log!("burrito_contract: {}",self.burrito_contract);
        log!("strw_contract: {}",self.strw_contract);
        log!("pve_contract: {}",self.pve_contract);
    }

}