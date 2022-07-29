use near_sdk::{
    env
};

use crate::*;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {
    // Obtener items para batalla player vs cpu
    pub fn get_items_for_battle_cpu(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId) -> AccessoriesForBattle  {

        let mut accessories_for_battle = AccessoriesForBattle {
            final_attack_b1 : "0".to_string(),
            final_defense_b1 : "0".to_string(),
            final_speed_b1 : "0".to_string(),
            final_attack_b2 : "0".to_string(),
            final_defense_b2 : "0".to_string(),
            final_speed_b2 : "0".to_string()
        };
            
        let mut accesories_attack_burrito1 : f32 = 0.0;
        let mut accesories_defense_burrito1 : f32 = 0.0;
        let mut accesories_speed_burrito1 : f32 = 0.0;

        if self.token_metadata_by_id.len() == 0 {
            return accessories_for_battle;
        }

        // Validar que exista el id
        if accesorio1_burrito1_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 1");
        }
        if accesorio2_burrito1_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 2");
        }
        if accesorio3_burrito1_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 3");
        }
        
        if accesorio1_burrito1_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio1_burrito1_id.clone());   
            let owner_id_a1 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a1.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 1 no te pertenece");
            //}
            let metadata_accesorio1_burrito1 = self.token_metadata_by_id.get(&accesorio1_burrito1_id.clone()).unwrap();
               
            let newextradata_accesorio1_burrito1 = str::replace(&metadata_accesorio1_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio1_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito1).unwrap();
            accesories_attack_burrito1 += extradatajson_accesorio1_burrito1.attack.parse::<f32>().unwrap();
            accesories_defense_burrito1 += extradatajson_accesorio1_burrito1.defense.parse::<f32>().unwrap();
            accesories_speed_burrito1 += extradatajson_accesorio1_burrito1.speed.parse::<f32>().unwrap();
        }

        if accesorio2_burrito1_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio2_burrito1_id.clone());   
            let owner_id_a2 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a2.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 2 no te pertenece");
            //}
            let metadata_accesorio2_burrito1 = self.token_metadata_by_id.get(&accesorio2_burrito1_id.clone()).unwrap();

            let newextradata_accesorio2_burrito1 = str::replace(&metadata_accesorio2_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio2_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito1).unwrap();
            accesories_attack_burrito1 += extradatajson_accesorio2_burrito1.attack.parse::<f32>().unwrap();
            accesories_defense_burrito1 += extradatajson_accesorio2_burrito1.defense.parse::<f32>().unwrap();
            accesories_speed_burrito1 += extradatajson_accesorio2_burrito1.speed.parse::<f32>().unwrap();
        }

        if accesorio3_burrito1_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio3_burrito1_id.clone());   
            let owner_id_a3 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a3.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 3 no te pertenece");
            //}
            let metadata_accesorio3_burrito1 = self.token_metadata_by_id.get(&accesorio3_burrito1_id.clone()).unwrap();

            let newextradata_accesorio3_burrito1 = str::replace(&metadata_accesorio3_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio3_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito1).unwrap();
            accesories_attack_burrito1 += extradatajson_accesorio3_burrito1.attack.parse::<f32>().unwrap();
            accesories_defense_burrito1 += extradatajson_accesorio3_burrito1.defense.parse::<f32>().unwrap();
            accesories_speed_burrito1 += extradatajson_accesorio3_burrito1.speed.parse::<f32>().unwrap();
        }

        accessories_for_battle.final_attack_b1 = accesories_attack_burrito1.to_string();
        accessories_for_battle.final_defense_b1 = accesories_defense_burrito1.to_string();
        accessories_for_battle.final_speed_b1 = accesories_speed_burrito1.to_string();

        return accessories_for_battle;

    }

    pub fn get_items_for_battle_pvp(&self, accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId,
        accesorio1_burrito2_id: TokenId, accesorio2_burrito2_id: TokenId, accesorio3_burrito2_id: TokenId) -> AccessoriesForBattle  {

        // Validar que exista el id
        if accesorio1_burrito1_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 1 del burrito 1");
        }
        if accesorio2_burrito1_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 2");
        }
        if accesorio3_burrito1_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 3");
        }
        if accesorio1_burrito2_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 1");
        }
        if accesorio2_burrito2_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 2");
        }
        if accesorio3_burrito2_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic(b"No existe el id del accesorio 3");
        }

        let mut accessories_for_battle = AccessoriesForBattle {
            final_attack_b1 : "0".to_string(),
            final_defense_b1 : "0".to_string(),
            final_speed_b1 : "0".to_string(),
            final_attack_b2 : "0".to_string(),
            final_defense_b2 : "0".to_string(),
            final_speed_b2 : "0".to_string()
        };

        //let token_owner_id = env::signer_account_id();
        let mut accesories_attack_burrito1 : f32 = 0.0;
        let mut accesories_defense_burrito1 : f32 = 0.0;
        let mut accesories_speed_burrito1 : f32 = 0.0;
        let mut accesories_attack_burrito2 : f32 = 0.0;
        let mut accesories_defense_burrito2 : f32 = 0.0;
        let mut accesories_speed_burrito2 : f32 = 0.0;
        
        if accesorio1_burrito1_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio1_burrito1_id.clone());   
            let owner_id_a1 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a1.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 1 no te pertenece");
            //}
            let metadata_accesorio1_burrito1 = self.token_metadata_by_id.get(&accesorio1_burrito1_id.clone()).unwrap();
               
            let newextradata_accesorio1_burrito1 = str::replace(&metadata_accesorio1_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio1_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito1).unwrap();
            accesories_attack_burrito1 += extradatajson_accesorio1_burrito1.attack.parse::<f32>().unwrap();
            accesories_defense_burrito1 += extradatajson_accesorio1_burrito1.defense.parse::<f32>().unwrap();
            accesories_speed_burrito1 += extradatajson_accesorio1_burrito1.speed.parse::<f32>().unwrap();
        }

        if accesorio2_burrito1_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio2_burrito1_id.clone());   
            let owner_id_a2 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a2.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 2 no te pertenece");
            //}
            let metadata_accesorio2_burrito1 = self.token_metadata_by_id.get(&accesorio2_burrito1_id.clone()).unwrap();

            let newextradata_accesorio2_burrito1 = str::replace(&metadata_accesorio2_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio2_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito1).unwrap();
            accesories_attack_burrito1 += extradatajson_accesorio2_burrito1.attack.parse::<f32>().unwrap();
            accesories_defense_burrito1 += extradatajson_accesorio2_burrito1.defense.parse::<f32>().unwrap();
            accesories_speed_burrito1 += extradatajson_accesorio2_burrito1.speed.parse::<f32>().unwrap();
        }

        if accesorio3_burrito1_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio3_burrito1_id.clone());   
            let owner_id_a3 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a3.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 3 no te pertenece");
            //}
            let metadata_accesorio3_burrito1 = self.token_metadata_by_id.get(&accesorio3_burrito1_id.clone()).unwrap();

            let newextradata_accesorio3_burrito1 = str::replace(&metadata_accesorio3_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio3_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito1).unwrap();
            accesories_attack_burrito1 += extradatajson_accesorio3_burrito1.attack.parse::<f32>().unwrap();
            accesories_defense_burrito1 += extradatajson_accesorio3_burrito1.defense.parse::<f32>().unwrap();
            accesories_speed_burrito1 += extradatajson_accesorio3_burrito1.speed.parse::<f32>().unwrap();
        }


        if accesorio1_burrito2_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio1_burrito2_id.clone());   
            let owner_id_a1 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a1.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 1 no te pertenece");
            //}
            let metadata_accesorio1_burrito2 = self.token_metadata_by_id.get(&accesorio1_burrito2_id.clone()).unwrap();
               
            let newextradata_accesorio1_burrito2 = str::replace(&metadata_accesorio1_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio1_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito2).unwrap();
            accesories_attack_burrito2 += extradatajson_accesorio1_burrito2.attack.parse::<f32>().unwrap();
            accesories_defense_burrito2 += extradatajson_accesorio1_burrito2.defense.parse::<f32>().unwrap();
            accesories_speed_burrito2 += extradatajson_accesorio1_burrito2.speed.parse::<f32>().unwrap();
        }

        if accesorio2_burrito2_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio2_burrito2_id.clone());   
            let owner_id_a2 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a2.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 2 no te pertenece");
            //}
            let metadata_accesorio2_burrito2 = self.token_metadata_by_id.get(&accesorio2_burrito2_id.clone()).unwrap();

            let newextradata_accesorio2_burrito2 = str::replace(&metadata_accesorio2_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio2_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito2).unwrap();
            accesories_attack_burrito2 += extradatajson_accesorio2_burrito2.attack.parse::<f32>().unwrap();
            accesories_defense_burrito2 += extradatajson_accesorio2_burrito2.defense.parse::<f32>().unwrap();
            accesories_speed_burrito2 += extradatajson_accesorio2_burrito2.speed.parse::<f32>().unwrap();
        }

        if accesorio3_burrito2_id.clone().parse::<u128>().unwrap() > 0 {
            let token = self.tokens_by_id.get(&accesorio3_burrito2_id.clone());   
            let owner_id_a3 = token.unwrap().owner_id.to_string();
            //if token_owner_id.clone() != owner_id_a3.clone().parse::<AccountId>().unwrap() {
            //    env::panic(b"El accesorio 3 no te pertenece");
            //}
            let metadata_accesorio3_burrito2 = self.token_metadata_by_id.get(&accesorio3_burrito2_id.clone()).unwrap();

            let newextradata_accesorio3_burrito2 = str::replace(&metadata_accesorio3_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
            let extradatajson_accesorio3_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito2).unwrap();
            accesories_attack_burrito2 += extradatajson_accesorio3_burrito2.attack.parse::<f32>().unwrap();
            accesories_defense_burrito2 += extradatajson_accesorio3_burrito2.defense.parse::<f32>().unwrap();
            accesories_speed_burrito2 += extradatajson_accesorio3_burrito2.speed.parse::<f32>().unwrap();
        }

        accessories_for_battle.final_attack_b1 = accesories_attack_burrito1.to_string();
        accessories_for_battle.final_defense_b1 = accesories_defense_burrito1.to_string();
        accessories_for_battle.final_speed_b1 = accesories_speed_burrito1.to_string();
        accessories_for_battle.final_attack_b2 = accesories_attack_burrito2.to_string();
        accessories_for_battle.final_defense_b2 = accesories_defense_burrito2.to_string();
        accessories_for_battle.final_speed_b2 = accesories_speed_burrito2.to_string();

        accessories_for_battle

    }
}