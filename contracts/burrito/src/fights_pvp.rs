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
    // Obtener cantidad de batallas activas Player vs Player
    pub fn get_number_battles_actives_pvp(&self) -> u128 {
        self.battle_room_pvp.len().try_into().unwrap()
    }

    pub fn delete_battles_actives(&mut self) {
        self.battle_room_pvp = HashMap::new();
        self.battle_room_cpu = HashMap::new();
    }

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

    // Método combate player vs player (type_move 1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse 4 = Defenderse)
    pub fn battle_player_pvp(&mut self, type_move: String) -> String {
        let token_owner_id = env::signer_account_id();
        let rooms_pvp = self.battle_room_pvp.clone();
        let filter_rooms : HashMap<String,BattlePVP> = rooms_pvp
        .into_iter()
        .filter(|(_, v)| 
            (v.payer1_id == token_owner_id.to_string() || v.payer2_id == token_owner_id.to_string()))
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

        if (type_move == "1" || type_move == "2") && battle_room.turn == battle_room.payer2_id.clone().to_string(){
            env::panic_str("No puedes realizar un ataque, debes elegir si defenderte o no");
        }

        if (type_move == "3" || type_move == "4") && battle_room.turn == battle_room.payer1_id.clone().to_string(){
            env::panic_str("No puedes defenderte, debes realizar un ataque");
        }

        let mut old_battle_room = battle_room;

        "Ronda Finalizada".to_string()

    }

}
