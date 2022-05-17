use near_sdk::{
    env, serde_json::json
};

use crate::*;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {

    // Obtener sala de batalla creada por account_id
    pub fn get_battle_active_pvp(&self) -> BattlePVP {
        let token_owner_id = env::signer_account_id();

        let rooms_pvp = self.battle_room_pvp.clone();
        let filter_rooms : HashMap<String,BattlePVP> = rooms_pvp
        .into_iter()
        .filter(|(_, v)| 
            (v.payer1_id == token_owner_id.to_string() || v.payer2_id == token_owner_id.to_string()))
        .collect();

        if filter_rooms.len() == 0 {
            env::panic_str("No existe sala creada a la que pertenezca esta cuenta");
        }
        
        let mut key = "";

        for (k, v) in filter_rooms.iter() {
            key = k;
        }

        let info = filter_rooms.get(key).unwrap();

        env::log(
            json!(info.clone())
            .to_string()
            .as_bytes(),
        );

        let battle_room = BattlePVP {
            status : info.status.to_string(),
            payer1_id : info.payer1_id.to_string(),
            payer2_id : info.payer2_id.to_string(),
            burrito_player1_id : info.burrito_player1_id.to_string(),
            burrito_player2_id : info.burrito_player2_id.to_string(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player1 : info.strong_attack_player1.to_string(),
            shields_player1 : info.shields_player1.to_string(),
            health_player1 : info.health_player1.to_string(),
            strong_attack_player2 : info.strong_attack_player2.to_string(),
            shields_player2 : info.shields_player2.to_string(),
            health_player2 : info.health_player2.to_string(),
            selected_move_player1 : info.selected_move_player1.to_string(),
            selected_move_player2 : info.selected_move_player2.to_string(),
        };

        battle_room
    }

    // Guardar sala de combate Player vs Player
    pub fn create_battle_player_pvp(&mut self, burrito_id: TokenId, accesorio1_id: TokenId, accesorio2_id: TokenId, accesorio3_id: TokenId) -> Promise {
        let token_owner_id = env::signer_account_id();

        // Verificar si ya tienes una partida PVP creada
        let rooms_pvp = self.battle_room_pvp.clone();
        let filter_rooms : HashMap<String,BattlePVP> = rooms_pvp
        .into_iter()
        .filter(|(_, v)| 
            (v.payer1_id == token_owner_id.to_string() || v.payer2_id == token_owner_id.to_string()))
        .collect();

        if filter_rooms.len() > 0 {
            env::panic_str("Ya tienes una partida iniciada, debes terminarla o rendirte");
        }

        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&burrito_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        if account_id.clone() != owner_id.clone().parse::<AccountId>().unwrap() {
            env::panic_str("El burrito no te pertenece");
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
            env::panic_str("Los 3 Items a equipar deben ser diferentes");
        }

        // Obtener información de los accesorios para ver si existen y recuperar las estadísticas a aumentar
        let p = ext_nft::get_items_for_battle_cpu(
            accesorio1_id.to_string(), // Id el item 1 del burrito
            accesorio2_id.to_string(), // Id el item 2 del burrito
            accesorio3_id.to_string(), // Id el item 3 del burrito
            ITEMS_CONTRACT.parse::<AccountId>().unwrap(), // Contrato de items
            NO_DEPOSIT, // yocto NEAR a ajuntar
            MIN_GAS_FOR_NFT_TRANSFER_CALL // gas a ajuntar
        )
        .then(ext_self::save_battle_player_pvp(
            burrito_id,
            BURRITO_CONTRACT.parse::<AccountId>().unwrap(), // Contrato de burritos
            NO_DEPOSIT, // yocto NEAR a ajuntar al callback
            GAS_FOR_NFT_TRANSFER_CALL // gas a ajuntar al callback
        ));

        p
    }


    // Guardar sala de combate Player vs Player
    pub fn save_battle_player_pvp(&mut self, burrito_id: TokenId) -> String {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => "oops!".to_string(),
            PromiseResult::Successful(result) => {
                let value = std::str::from_utf8(&result).unwrap();
                let accessories_for_battle: AccessoriesForBattle = serde_json::from_str(&value).unwrap();

                let token_owner_id = env::signer_account_id();

                // Obtener metadata burrito
                let metadata_burrito = self.token_metadata_by_id.get(&burrito_id.clone()).unwrap();
        
                // Extraer extras del token burrito
                let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        
                // Crear json burrito
                let extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();

                let token = self.tokens_by_id.get(&burrito_id.clone());        
                let owner_id_burrito = token.unwrap().owner_id.to_string();
                
                if extradatajson_burrito.hp.clone().parse::<u8>().unwrap() == 0 {
                    env::panic_str("El Burrito a utilizar no tiene vidas");
                }
        
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
                    level : extradatajson_burrito.level.clone(),
                    media : metadata_burrito.media.as_ref().unwrap().to_string()
                };
        
                // Verificar si existen salas en estatus de espera
                let rooms_pvp = self.battle_room_pvp.clone();
                let filter_rooms : HashMap<String,BattlePVP> = rooms_pvp
                .into_iter()
                .filter(|(_, v)| 
                    v.status.parse::<u128>().unwrap_or_default() == 1)
                .collect();

                // Si existe una sala en espera entonces se guardan los datos en dicha sala
                if filter_rooms.len() > 0 {

                    let mut key = "";

                    for (k, v) in filter_rooms.iter() {
                        key = k;
                    }

                    // Obtener informacion de una sala de espera
                    let info = filter_rooms.get(key).unwrap();

                    // Obtener informacion del burrito en sala de espera para determinar cual es el mas rapido y asignar el turno
                    let metadata_other_burrito = self.token_metadata_by_id.get(&info.burrito_player1_id.clone()).unwrap();
        
                    // Extraer extras del token burrito
                    let newextradata_other_burrito = str::replace(&metadata_other_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
            
                    // Crear json burrito
                    let extradatajson_other_burrito: ExtraBurrito = serde_json::from_str(&newextradata_other_burrito).unwrap();                    
            
                    // Crear estructura burrito
                    let other_burrito = Burrito {
                        owner_id : info.payer1_id.clone().to_string(),
                        name : metadata_other_burrito.title.as_ref().unwrap().to_string(),
                        description : metadata_other_burrito.description.as_ref().unwrap().to_string(),
                        burrito_type : extradatajson_other_burrito.burrito_type.clone(),
                        hp : extradatajson_other_burrito.hp.clone(),
                        attack : extradatajson_other_burrito.attack.clone(),
                        defense : extradatajson_other_burrito.defense.clone(),
                        speed : extradatajson_other_burrito.speed.clone(),
                        win : extradatajson_other_burrito.win.clone(),
                        global_win : extradatajson_other_burrito.global_win.clone(),
                        level : extradatajson_other_burrito.level.clone(),
                        media : metadata_other_burrito.media.as_ref().unwrap().to_string()
                    };

                    let mut battle_room = BattlePVP {
                        status : "2".to_string(),
                        payer1_id : info.payer1_id.to_string(),
                        payer2_id : token_owner_id.clone().to_string(),
                        burrito_player1_id : info.burrito_player1_id.to_string(),
                        burrito_player2_id : burrito_id.clone().to_string(),
                        accesories_attack_b1 : info.accesories_attack_b1.to_string(),
                        accesories_defense_b1 : info.accesories_defense_b1.to_string(),
                        accesories_speed_b1 : info.accesories_speed_b1.to_string(),
                        accesories_attack_b2 : accessories_for_battle.final_attack_b1.clone().to_string(),
                        accesories_defense_b2 : accessories_for_battle.final_defense_b1.clone().to_string(),
                        accesories_speed_b2 : accessories_for_battle.final_speed_b1.clone().to_string(),
                        turn : "".to_string(),
                        strong_attack_player1 : info.strong_attack_player1.to_string(),
                        shields_player1 : info.shields_player1.to_string(),
                        health_player1 : info.health_player1.to_string(),
                        strong_attack_player2 : "3".to_string(),
                        shields_player2 : "3".to_string(),
                        health_player2 : (burrito.attack.parse::<u8>().unwrap()+burrito.defense.parse::<u8>().unwrap()+burrito.speed.parse::<u8>().unwrap()).to_string(),
                        selected_move_player1 : "".to_string(),
                        selected_move_player2 : "".to_string()
                    };

                    if (burrito.speed.clone().parse::<u8>().unwrap() + accessories_for_battle.final_speed_b1.clone().parse::<u8>().unwrap()) > (other_burrito.speed.clone().parse::<u8>().unwrap() + info.accesories_speed_b1.clone().parse::<u8>().unwrap()) {
                        battle_room.turn = battle_room.payer2_id.clone();
                    } else {
                        battle_room.turn = battle_room.payer1_id.clone();
                    }

                    self.battle_room_pvp.remove(&key.to_string());
                    self.battle_room_pvp.insert(key.to_string(),battle_room.clone());

                    return serde_json::to_string(&battle_room).unwrap();

                } else {
                    // Se crea una sala con estatus de en espera
                    let info = BattlePVP {
                        status : "1".to_string(),
                        payer1_id : token_owner_id.clone().to_string(),
                        payer2_id : "".to_string(),
                        burrito_player1_id : burrito_id.clone().to_string(),
                        burrito_player2_id : "".to_string(),
                        accesories_attack_b1 : accessories_for_battle.final_attack_b1.clone().to_string(),
                        accesories_defense_b1 : accessories_for_battle.final_defense_b1.clone().to_string(),
                        accesories_speed_b1 : accessories_for_battle.final_speed_b1.clone().to_string(),
                        accesories_attack_b2 : "".to_string(),
                        accesories_defense_b2 : "".to_string(),
                        accesories_speed_b2 : "".to_string(),
                        turn : "".to_string(),
                        strong_attack_player1 : "3".to_string(),
                        shields_player1 : "3".to_string(),
                        health_player1 : (burrito.attack.parse::<u8>().unwrap()+burrito.defense.parse::<u8>().unwrap()+burrito.speed.parse::<u8>().unwrap()).to_string(),
                        strong_attack_player2 : "".to_string(),
                        shields_player2 : "".to_string(),
                        health_player2 : "".to_string(),
                        selected_move_player1 : "".to_string(),
                        selected_move_player2 : "".to_string()
                    };
            
                    self.battle_room_pvp.insert((self.battle_room_pvp.len()+1).to_string(),info.clone());
            
                    env::log(
                        json!(info.clone())
                        .to_string()
                        .as_bytes(),
                    );
    
                    return serde_json::to_string(&info).unwrap();
                }

            }
        }

    }

    // Rendirse y finalizar batalla Player vs Player
    pub fn surrender_pvp(&mut self) -> String {
        let token_owner_id = env::signer_account_id();

        let rooms_pvp = self.battle_room_pvp.clone();
        let filter_rooms : HashMap<String,BattlePVP> = rooms_pvp
        .into_iter()
        .filter(|(_, v)| 
            (v.payer1_id == token_owner_id.to_string() || v.payer2_id == token_owner_id.to_string()))
        .collect();

        if filter_rooms.len() == 0 {
            env::panic_str("No existe sala creada a la que pertenezca esta cuenta");
        }
        
        let mut key = "";

        for (k, v) in filter_rooms.iter() {
            key = k;
        }

        let info = filter_rooms.get(key).unwrap();

        let battle_room = BattlePVP {
            status : info.status.to_string(),
            payer1_id : info.payer1_id.to_string(),
            payer2_id : info.payer2_id.to_string(),
            burrito_player1_id : info.burrito_player1_id.to_string(),
            burrito_player2_id : info.burrito_player2_id.to_string(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player1 : info.strong_attack_player1.to_string(),
            shields_player1 : info.shields_player1.to_string(),
            health_player1 : info.health_player1.to_string(),
            strong_attack_player2 : info.strong_attack_player2.to_string(),
            shields_player2 : info.shields_player2.to_string(),
            health_player2 : info.health_player2.to_string(),
            selected_move_player1 : info.selected_move_player1.to_string(),
            selected_move_player2 : info.selected_move_player2.to_string(),
        };

        // Restar una vida del burrito utilizado en el combate del usuario que se rindió

        // Se rindio el jugador 1
        if( battle_room.payer1_id.clone().to_string() == token_owner_id.clone().to_string()) {
            // Restar una vida al burrito que se rindio
            // Obtener metadata burrito
            let mut metadata_burrito1 = self.token_metadata_by_id.get(&battle_room.burrito_player1_id.clone()).unwrap();

            // Extraer extras del token burrito 1
            let newextradata_burrito1 = str::replace(&metadata_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
    
            // Crear json burrito 1
            let mut extradatajson_burrito1: ExtraBurrito = serde_json::from_str(&newextradata_burrito1).unwrap();
    
            let token1 = self.tokens_by_id.get(&battle_room.burrito_player1_id.clone());        
            let owner_id_burrito1 = token1.unwrap().owner_id.to_string();
            
            // Crear estructura burrito
            let burrito1 = Burrito {
                owner_id : owner_id_burrito1.clone().to_string(),
                name : metadata_burrito1.title.as_ref().unwrap().to_string(),
                description : metadata_burrito1.description.as_ref().unwrap().to_string(),
                burrito_type : extradatajson_burrito1.burrito_type.clone(),
                hp : extradatajson_burrito1.hp.clone(),
                attack : extradatajson_burrito1.attack.clone(),
                defense : extradatajson_burrito1.defense.clone(),
                speed : extradatajson_burrito1.speed.clone(),
                win : extradatajson_burrito1.win.clone(),
                global_win : extradatajson_burrito1.global_win.clone(),
                level : extradatajson_burrito1.level.clone(),
                media : metadata_burrito1.media.as_ref().unwrap().to_string()
            };
    
            let new_hp_burrito1 = burrito1.hp.parse::<u8>().unwrap()-1;
            extradatajson_burrito1.hp = new_hp_burrito1.to_string();
    
            let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
            extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
            metadata_burrito1.extra = Some(extra_string_burrito1.clone());
    
            self.token_metadata_by_id.insert(&battle_room.burrito_player1_id.clone(), &metadata_burrito1);    

            //Incremetar el contador de victorias del burrito ganador
            // Obtener metadata burrito
            let mut metadata_burrito2 = self.token_metadata_by_id.get(&battle_room.burrito_player2_id.clone()).unwrap();

            // Extraer extras del token burrito 1
            let newextradata_burrito2 = str::replace(&metadata_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
    
            // Crear json burrito 1
            let mut extradatajson_burrito2: ExtraBurrito = serde_json::from_str(&newextradata_burrito2).unwrap();
    
            let token2 = self.tokens_by_id.get(&battle_room.burrito_player2_id.clone());        
            let owner_id_burrito2 = token2.unwrap().owner_id.to_string();
            
            // Crear estructura burrito
            let burrito2 = Burrito {
                owner_id : owner_id_burrito2.clone().to_string(),
                name : metadata_burrito2.title.as_ref().unwrap().to_string(),
                description : metadata_burrito2.description.as_ref().unwrap().to_string(),
                burrito_type : extradatajson_burrito2.burrito_type.clone(),
                hp : extradatajson_burrito2.hp.clone(),
                attack : extradatajson_burrito2.attack.clone(),
                defense : extradatajson_burrito2.defense.clone(),
                speed : extradatajson_burrito2.speed.clone(),
                win : extradatajson_burrito2.win.clone(),
                global_win : extradatajson_burrito2.global_win.clone(),
                level : extradatajson_burrito2.level.clone(),
                media : metadata_burrito2.media.as_ref().unwrap().to_string()
            };
    
            // Incrementar victorias del burrito si son < 10
            let mut new_win_burrito2 = extradatajson_burrito2.win.parse::<u8>().unwrap();
            let new_global_win_burrito2 = extradatajson_burrito2.global_win.parse::<u8>().unwrap()+1;

            if new_win_burrito2 < 10 {
                new_win_burrito2 += 1;
            }

            extradatajson_burrito2.win = new_win_burrito2.to_string();
            extradatajson_burrito2.global_win = new_global_win_burrito2.to_string();
    
            let mut extra_string_burrito2 = serde_json::to_string(&extradatajson_burrito2).unwrap();
            extra_string_burrito2 = str::replace(&extra_string_burrito2, "\"", "'");
            metadata_burrito2.extra = Some(extra_string_burrito2.clone());
    
            self.token_metadata_by_id.insert(&battle_room.burrito_player2_id.clone(), &metadata_burrito2);    
            
            // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
            let info = BattlesHistory {
                payer1_id : battle_room.payer1_id.clone().to_string(),
                payer2_id : battle_room.payer2_id.clone().to_string(),
                winner : battle_room.payer2_id.clone().to_string(),
                status : "Surrender".to_string()
            };

            self.battle_history.insert(self.battle_history.len().to_string(),info);

            // Eliminar sala
            self.battle_room_pvp.remove(&key.to_string());
        } 

        // Se rindio el jugador 2
        if( battle_room.payer2_id.clone().to_string() == token_owner_id.clone().to_string()) {
            // Restar una vida al burrito que se rindio
            // Obtener metadata burrito
            let mut metadata_burrito2 = self.token_metadata_by_id.get(&battle_room.burrito_player2_id.clone()).unwrap();

            // Extraer extras del token burrito 1
            let newextradata_burrito2 = str::replace(&metadata_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
    
            // Crear json burrito 1
            let mut extradatajson_burrito2: ExtraBurrito = serde_json::from_str(&newextradata_burrito2).unwrap();
    
            let token2 = self.tokens_by_id.get(&battle_room.burrito_player2_id.clone());        
            let owner_id_burrito2 = token2.unwrap().owner_id.to_string();
            
            // Crear estructura burrito
            let burrito2 = Burrito {
                owner_id : owner_id_burrito2.clone().to_string(),
                name : metadata_burrito2.title.as_ref().unwrap().to_string(),
                description : metadata_burrito2.description.as_ref().unwrap().to_string(),
                burrito_type : extradatajson_burrito2.burrito_type.clone(),
                hp : extradatajson_burrito2.hp.clone(),
                attack : extradatajson_burrito2.attack.clone(),
                defense : extradatajson_burrito2.defense.clone(),
                speed : extradatajson_burrito2.speed.clone(),
                win : extradatajson_burrito2.win.clone(),
                global_win : extradatajson_burrito2.global_win.clone(),
                level : extradatajson_burrito2.level.clone(),
                media : metadata_burrito2.media.as_ref().unwrap().to_string()
            };
    
            let new_hp_burrito2 = burrito2.hp.parse::<u8>().unwrap()-1;
            extradatajson_burrito2.hp = new_hp_burrito2.to_string();
    
            let mut extra_string_burrito2 = serde_json::to_string(&extradatajson_burrito2).unwrap();
            extra_string_burrito2 = str::replace(&extra_string_burrito2, "\"", "'");
            metadata_burrito2.extra = Some(extra_string_burrito2.clone());
    
            self.token_metadata_by_id.insert(&battle_room.burrito_player2_id.clone(), &metadata_burrito2);    

            //Incremetar el contador de victorias del burrito ganador
            // Obtener metadata burrito
            let mut metadata_burrito1 = self.token_metadata_by_id.get(&battle_room.burrito_player1_id.clone()).unwrap();

            // Extraer extras del token burrito 1
            let newextradata_burrito1 = str::replace(&metadata_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
    
            // Crear json burrito 1
            let mut extradatajson_burrito1: ExtraBurrito = serde_json::from_str(&newextradata_burrito1).unwrap();
    
            let token1 = self.tokens_by_id.get(&battle_room.burrito_player1_id.clone());        
            let owner_id_burrito1 = token1.unwrap().owner_id.to_string();
            
            // Crear estructura burrito
            let burrito1 = Burrito {
                owner_id : owner_id_burrito1.clone().to_string(),
                name : metadata_burrito1.title.as_ref().unwrap().to_string(),
                description : metadata_burrito1.description.as_ref().unwrap().to_string(),
                burrito_type : extradatajson_burrito1.burrito_type.clone(),
                hp : extradatajson_burrito1.hp.clone(),
                attack : extradatajson_burrito1.attack.clone(),
                defense : extradatajson_burrito1.defense.clone(),
                speed : extradatajson_burrito1.speed.clone(),
                win : extradatajson_burrito1.win.clone(),
                global_win : extradatajson_burrito1.global_win.clone(),
                level : extradatajson_burrito1.level.clone(),
                media : metadata_burrito1.media.as_ref().unwrap().to_string()
            };
    
            // Incrementar victorias del burrito si son < 10
            let mut new_win_burrito1 = extradatajson_burrito1.win.parse::<u8>().unwrap();
            let new_global_win_burrito1 = extradatajson_burrito1.global_win.parse::<u8>().unwrap()+1;

            if new_win_burrito1 < 10 {
                new_win_burrito1 += 1;
            }

            extradatajson_burrito1.win = new_win_burrito1.to_string();
            extradatajson_burrito1.global_win = new_global_win_burrito1.to_string();
    
            let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
            extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
            metadata_burrito1.extra = Some(extra_string_burrito1.clone());
    
            self.token_metadata_by_id.insert(&battle_room.burrito_player1_id.clone(), &metadata_burrito1);    

            // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
            let info = BattlesHistory {
                payer1_id : battle_room.payer1_id.clone().to_string(),
                payer2_id : battle_room.payer2_id.clone().to_string(),
                winner : battle_room.payer1_id.clone().to_string(),
                status : "Surrender".to_string()
            };

            self.battle_history.insert(self.battle_history.len().to_string(),info);

            // Eliminar sala
            self.battle_room_pvp.remove(&key.to_string());
        }

        "Finalizó batalla".to_string()
    }

    // Método combate player vs player (type_move 1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse 4 = Defenderse)
    pub fn battle_player_pvp(&mut self, type_move: String) -> String {
        let token_owner_id = env::signer_account_id();
        let rooms_pvp = self.battle_room_pvp.clone();
        let filter_rooms : HashMap<String,BattlePVP> = rooms_pvp
        .into_iter()
        .filter(|(_, v)| 
            (v.player1_id == token_owner_id.clone().to_string() || v.player2_id == token_owner_id.clone().to_string()))
        .collect();

        if filter_rooms.len() == 0 {
            env::panic_str("No tienes una batalla activa");
        }
        
        let mut key = "";

        for (k, v) in filter_rooms.iter() {
            key = k;
        }

        let info = filter_rooms.get(key).unwrap();

        let battle_room = BattlePVP {
            status : info.status.to_string(),
            player1_id : info.player1_id.to_string(),
            player2_id : info.player2_id.to_string(),
            burrito_player1_id : info.burrito_player1_id.to_string(),
            burrito_player2_id : info.burrito_player2_id.to_string(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player1 : info.strong_attack_player1.to_string(),
            shields_player1 : info.shields_player1.to_string(),
            health_player1 : info.health_player1.to_string(),
            strong_attack_player2 : info.strong_attack_player2.to_string(),
            shields_player2 : info.shields_player2.to_string(),
            health_player2 : info.health_player2.to_string(),
            selected_move_player1 : info.selected_move_player1.to_string(),
            selected_move_player2 : info.selected_move_player2.to_string(),
        };

        // Verificar de quien es el turno
        if( battle_room.turn.clone().to_string() == token_owner_id.clone().to_string()) {
            // Es turno de atacar
            if (type_move == "3" || type_move == "4") && battle_room.turn == battle_room.player1_id.clone().to_string(){
                env::panic_str("No puedes defenderte, debes realizar un ataque");
            }

            if type_move == "2" && battle_room.strong_attack_player1.parse::<u8>().unwrap() == 0 {
                env::panic_str("No tienes mas ataques fuertes, debes realizar uno normal");
            }

        } else {
            // Es turno de defender
            if (type_move == "1" || type_move == "2") && battle_room.turn == battle_room.player2_id.clone().to_string(){
                env::panic_str("No puedes realizar un ataque, debes elegir si defenderte o no");
            }

            if type_move == "4" && battle_room.shields_player2.parse::<u8>().unwrap() == 0 {
                env::panic_str("No tienes mas escudos, no puedes defenderte");
            }

        }

        let mut new_battle_room = battle_room;

        // Guardar el ataque seleccionado del jugador

        if token_owner_id.clone().to_string() == battle_room.player1_id.clone().to_string(){
            new_battle_room.selected_move_player1 = type_move;
        }
        if token_owner_id.clone().to_string() == battle_room.player2_id.clone().to_string(){
            new_battle_room.selected_move_player2 = type_move;
        }

        self.battle_room_pvp.remove(&key.to_string());
        self.battle_room_pvp.insert(key.to_string(),new_battle_room.clone());

        // Verificamos si ambos jugadores ya seleccionaron el movimiento correspondiente
        if new_battle_room.selected_move_player1.to_string() == '' || new_battle_room.selected_move_player1.to_string() == '' {
            env::panic_str("Los jugadores aun no seleccionan los movimientos correspondientes");
        }

        // Restamos los contadores a ataques fuertes en caso de que se utilizara alguno
        if new_battle_room.selected_move_player1.clone().to_string() == "2"{
            new_battle_room.strong_attack_player1 = (new_battle_room.strong_attack_player1.parse::<u8>().unwrap()-1).to_string();
        }
        if new_battle_room.selected_move_player2.clone().to_string() == "2"{
            new_battle_room.strong_attack_player2 = (new_battle_room.strong_attack_player2.parse::<u8>().unwrap()-1).to_string();
        }
    
        // Verificamos si se utilizó algun escudo para finalizar la ronda
        if new_battle_room.selected_move_player1.clone().to_string() == "4".to_string(){
            new_battle_room.shields_player1 = (new_battle_room.shields_player1.parse::<u8>().unwrap()-1).to_string();
            new_battle_room.turn = battle_room.player1_id.clone().to_string();
            new_battle_room.selected_move_player1 = "".to_string();
            new_battle_room.selected_move_player2 = "".to_string();
            return str::replace(&serde_json::to_string(&new_battle_room.clone()).unwrap(), "\"", "'");
        }
        if new_battle_room.selected_move_player2.clone().to_string() == "4".to_string(){
            new_battle_room.shields_player2 = (new_battle_room.shields_player1.parse::<u8>().unwrap()-1).to_string();
            new_battle_room.turn = battle_room.player2_id.clone().to_string();
            new_battle_room.selected_move_player1 = "".to_string();
            new_battle_room.selected_move_player2 = "".to_string();
            return str::replace(&serde_json::to_string(&new_battle_room.clone()).unwrap(), "\"", "'");

        }

        self.battle_room_pvp.remove(&key.to_string());
        self.battle_room_pvp.insert(key.to_string(),new_battle_room.clone());

        //Obtener información de los burritos

        // Obtener metadata burrito
        let mut metadata_burrito1 = self.token_metadata_by_id.get(&new_battle_room.burrito_player1_id.clone()).unwrap();

        // Extraer extras del token burrito 1
        let newextradata_burrito1 = str::replace(&metadata_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Crear json burrito 1
        let mut extradatajson_burrito1: ExtraBurrito = serde_json::from_str(&newextradata_burrito1).unwrap();

        let token1 = self.tokens_by_id.get(&new_battle_room.burrito_player1_id.clone());        
        let owner_id_burrito1 = token1.unwrap().owner_id.to_string();
        
        // Crear estructura burrito
        let burrito1 = Burrito {
            owner_id : owner_id_burrito1.clone().to_string(),
            name : metadata_burrito1.title.as_ref().unwrap().to_string(),
            description : metadata_burrito1.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson_burrito1.burrito_type.clone(),
            hp : extradatajson_burrito1.hp.clone(),
            attack : extradatajson_burrito1.attack.clone(),
            defense : extradatajson_burrito1.defense.clone(),
            speed : extradatajson_burrito1.speed.clone(),
            win : extradatajson_burrito1.win.clone(),
            global_win : extradatajson_burrito1.global_win.clone(),
            level : extradatajson_burrito1.level.clone(),
            media : metadata_burrito1.media.as_ref().unwrap().to_string()
        };

        // Obtener metadata burrito
        let mut metadata_burrito2 = self.token_metadata_by_id.get(&new_battle_room.burrito_player2_id.clone()).unwrap();

        // Extraer extras del token burrito 1
        let newextradata_burrito2 = str::replace(&metadata_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Crear json burrito 1
        let mut extradatajson_burrito2: ExtraBurrito = serde_json::from_str(&newextradata_burrito2).unwrap();

        let token2 = self.tokens_by_id.get(&new_battle_room.burrito_player2_id.clone());        
        let owner_id_burrito2 = token2.unwrap().owner_id.to_string();
        
        // Crear estructura burrito
        let burrito2 = Burrito {
            owner_id : owner_id_burrito2.clone().to_string(),
            name : metadata_burrito2.title.as_ref().unwrap().to_string(),
            description : metadata_burrito2.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson_burrito2.burrito_type.clone(),
            hp : extradatajson_burrito2.hp.clone(),
            attack : extradatajson_burrito2.attack.clone(),
            defense : extradatajson_burrito2.defense.clone(),
            speed : extradatajson_burrito2.speed.clone(),
            win : extradatajson_burrito2.win.clone(),
            global_win : extradatajson_burrito2.global_win.clone(),
            level : extradatajson_burrito2.level.clone(),
            media : metadata_burrito2.media.as_ref().unwrap().to_string()
        };
        
        // Realizar calculos de daño

        let rand_attack: u8 = *env::random_seed().get(0).unwrap();

        let mut attack_mult: f32 = 0.0;
        let mut type_mult: f32 = 0.0;

        let burrito_attacker;
        let burrito_defender;
        let mut old_health_burrito_defender: f32 = 0.0;

        if new_battle_room.turn == new_battle_room.player1_id.clone().to_string() {
            burrito_attacker = burrito1.clone();
            burrito_defender = burrito2.clone();
            old_health_burrito_defender = new_battle_room.health_player2.parse::<f32>().unwrap();
        }
        if new_battle_room.turn == new_battle_room.player2_id.clone().to_string() {
        {
            burrito_attacker = burrito2.clone();
            burrito_defender = burrito1.clone();
            old_health_burrito_defender = new_battle_room.health_player1.parse::<f32>().unwrap();
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

        log!("Vida vieja burrito defensor: {}",old_health_burrito_defender);

        let mut attack = 0.0;
        if new_battle_room.turn == new_battle_room.player1_id.clone().to_string(){
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult+new_battle_room.accesories_attack_b1.parse::<f32>().unwrap();
        }  
        if new_battle_room.turn == new_battle_room.player2_id.clone().to_string(){
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult+new_battle_room.accesories_attack_b2.parse::<f32>().unwrap();
        }
        log!("Cantidad de daño a realizar: {}",attack);

         // Verificar el tipo de ataque
        if new_battle_room.turn == new_battle_room.player1_id.clone().to_string(){
            if type_move == "2"{
                attack += attack*0.5;
                log!("Cantidad de daño fuerte a realizar: {}",attack);
            }
        } 
        if new_battle_room.turn == new_battle_room.player2_id.clone().to_string(){
            if type_move == "2"{
                attack += attack*0.5;
                log!("Cantidad de daño fuerte a realizar: {}",attack);
            }
        } 

        let new_health_burrito_defender = old_health_burrito_defender - attack;
        log!("Vida nueva burrito defensor: {}",new_health_burrito_defender);


        // Actualizar registro de sala de batalla
        // Es turno del jugador 1
        if new_battle_room.turn == new_battle_room.player1_id.clone().to_string(){
            if new_health_burrito_defender <= 0.0 {
                // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
                let info = BattlesHistory {
                    player1_id : new_battle_room.player1_id.to_string(),
                    player2_id : new_battle_room.player2_id.to_string(),
                    winner : new_battle_room.player1_id.to_string(),
                    status : "Battle".to_string()
                };
                self.battle_history.insert(new_battle_room.player1_id.to_string()+&new_battle_room.player2_id.to_string()+&"-".to_string()+ &self.battle_history.len().to_string(),info);
                // Eliminar sala activa
                self.battle_room_pvp.remove(&key.to_string());
                log!("Batalla Finalizada, Ganó Jugador");
                
                // Incrementar victorias del burrito si son < 10
                let mut new_win_burrito1 = extradatajson_burrito1.win.parse::<u8>().unwrap();
                let new_global_win_burrito1 = extradatajson_burrito1.global_win.parse::<u8>().unwrap()+1;

                if new_win_burrito1 < 10 {
                    new_win_burrito1 += 1;
                }

                extradatajson_burrito1.win = new_win_burrito1.to_string();
                extradatajson_burrito1.global_win = new_global_win_burrito1.to_string();

                let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
                extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
                metadata_burrito1.extra = Some(extra_string_burrito1.clone());

                self.token_metadata_by_id.insert(&new_battle_room.burrito_player1_id.clone(), &metadata_burrito1);

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
                    new_battle_room.player1_id.clone().to_string(),
                    tokens_to_mint.to_string(),
                    STRWTOKEN_CONTRACT.parse::<AccountId>().unwrap(),
                    0000000000000000000000001,
                    GAS_FOR_NFT_TRANSFER_CALL
                );

                return str::replace(&serde_json::to_string(&new_battle_room.clone()).unwrap(), "\"", "'");
            } else {
                new_battle_room.health_player2 = new_health_burrito_defender.to_string();
                new_battle_room.turn = new_battle_room.player2_id.clone().to_string();
                self.battle_room_pvp.remove(&key.to_string());
                self.battle_room_pvp.insert(key.to_string(),new_battle_room.clone());
            }
        } 
        // Es turno del jugador 2
        if new_battle_room.turn == new_battle_room.player2_id.clone().to_string(){
            if new_health_burrito_defender <= 0.0 {
                // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
                let info = BattlesHistory {
                    player1_id : new_battle_room.player1_id.to_string(),
                    player2_id : new_battle_room.player2_id.to_string(),
                    winner : new_battle_room.player2_id.to_string(),
                    status : "Battle".to_string()
                };
                self.battle_history.insert(new_battle_room.player1_id.to_string()+&new_battle_room.player2_id.to_string()+&"-".to_string()+ &self.battle_history.len().to_string(),info);
                // Eliminar sala activa
                self.battle_room_pvp.remove(&key.to_string());
                log!("Batalla Finalizada, Ganó Jugador");
                
                // Incrementar victorias del burrito si son < 10
                let mut new_win_burrito1 = extradatajson_burrito1.win.parse::<u8>().unwrap();
                let new_global_win_burrito1 = extradatajson_burrito1.global_win.parse::<u8>().unwrap()+1;

                if new_win_burrito1 < 10 {
                    new_win_burrito1 += 1;
                }

                extradatajson_burrito1.win = new_win_burrito1.to_string();
                extradatajson_burrito1.global_win = new_global_win_burrito1.to_string();

                let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
                extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
                metadata_burrito1.extra = Some(extra_string_burrito1.clone());

                self.token_metadata_by_id.insert(&new_battle_room.burrito_player1_id.clone(), &metadata_burrito1);

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
                    new_battle_room.player1_id.clone().to_string(),
                    tokens_to_mint.to_string(),
                    STRWTOKEN_CONTRACT.parse::<AccountId>().unwrap(),
                    0000000000000000000000001,
                    GAS_FOR_NFT_TRANSFER_CALL
                );

                return str::replace(&serde_json::to_string(&new_battle_room.clone()).unwrap(), "\"", "'");
            } else {
                new_battle_room.health_player2 = new_health_burrito_defender.to_string();
                new_battle_room.turn = new_battle_room.player2_id.clone().to_string();
                self.battle_room_pvp.remove(&key.to_string());
                self.battle_room_pvp.insert(key.to_string(),new_battle_room.clone());
            }
        } 





        "Ronda Finalizada".to_string()
        str::replace(&serde_json::to_string(&new_battle_room.clone()).unwrap(), "\"", "'")

    }


}

