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

pub const BURRITO_CONTRACT: &str = "bb-burritos.testnet";
pub const ITEMS_CONTRACT: &str = "bb-items.testnet";
pub const STRWTOKEN_CONTRACT: &str = "bb-strw.testnet";
pub const PVE_CONTRACT: &str = "bb-pve.testnet";

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
    accesories_attack_b1 : String,
    accesories_defense_b1 : String,
    accesories_speed_b1 : String,
    attack_b1 : String,
    defense_b1 : String,
    speed_b1 : String,
    level_b1 : String,
    accesories_attack_b2 : String,
    accesories_defense_b2 : String,
    accesories_speed_b2 : String,
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
    pub battle_history: HashMap<String,BattlesHistory>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,
    pub battle_rooms: HashMap<String,BattleCPU>,
    pub battle_history: HashMap<String,BattlesHistory>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init_contract(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id
        )
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            owner_id,
            battle_rooms:HashMap::new(),
            battle_history:HashMap::new()
        };

        //return the Contract object
        this
    }

    pub fn change_owner(&mut self, owner_id: AccountId) {
        self.assert_owner();
        self.owner_id = owner_id;
    }

    fn assert_owner(&self) {
        require!(self.signer_is_owner(), "Method is private to owner")
    }

    fn signer_is_owner(&self) -> bool {
        self.is_owner(&env::signer_account_id())
    }

    fn is_owner(&self, minter: &AccountId) -> bool {
        minter.as_str() == self.owner_id.as_str()
    }

}