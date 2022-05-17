use near_sdk::{
    env, serde_json::json
};
pub type TokenId = String;
use crate::*;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {
    

    // Verificar si tiene una sala activa
    pub fn  is_in_battle(&self, account_id : AccountId) -> bool {
        let token_owner_id = env::signer_account_id();

        let rooms_pvp = self.battle_rooms.clone();

        let filter_rooms : HashMap<String,BattlePVP> = rooms_pvp
        .into_iter()
        .filter(|(_, v)| 
            (v.payer1_id == token_owner_id.to_string() || v.payer2_id == token_owner_id.to_string()))
        .collect();

        if filter_rooms.len() == 0 {
            return false;
        } else {
            return true;
        }
    }

    // Obtener sala de batalla creada por account_id
    pub fn get_battle_active_pvp(&self) -> BattlePVP {
        let token_owner_id = env::signer_account_id();

        let rooms_pvp = self.battle_rooms.clone();
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

    // METODOS PROBADOS //

    // Guardar sala de combate Player vs CPU
    pub fn create_battle_player_cpu(&mut self, burrito_id: TokenId, accesorio1_id: TokenId, accesorio2_id: TokenId, accesorio3_id: TokenId) {
        let token_owner_id = env::signer_account_id();

        let br = self.battle_rooms.get(&token_owner_id.to_string());
        
        if br.is_some() {
            env::panic_str("Ya tienes una partida iniciada, debes terminarla o rendirte");
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

        // Mandar a llamar al contrato de burritos para obtener la información del mismo
        ext_nft::get_burrito(
            token_owner_id.clone().to_string(),
            burrito_id.clone(),
            BURRITO_CONTRACT.parse::<AccountId>().unwrap(),
            NO_DEPOSIT,
            MIN_GAS_FOR_NFT_TRANSFER_CALL        
        )
        .then(ext_self::save_burritos_battle_room(
            burrito_id,
            accesorio1_id.to_string(),
            accesorio2_id.to_string(),
            accesorio3_id.to_string(), 
            PVE_CONTRACT.parse::<AccountId>().unwrap(),
            NO_DEPOSIT,
            GAS_FOR_NFT_TRANSFER_CALL
        ));
    }

    // Recuperar información de los burritos y guardarla en la sala de batalla
    pub fn save_burritos_battle_room(&mut self, burrito_id: TokenId, accesorio1_id: TokenId, accesorio2_id: TokenId, accesorio3_id: TokenId) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::log_str("Ops..."),
            PromiseResult::Successful(result) => {

                let value = std::str::from_utf8(&result).unwrap();
                let burrito_for_battle: Burrito = serde_json::from_str(&value).unwrap();

                if burrito_for_battle.hp.clone().parse::<u8>().unwrap() == 0 {
                    env::panic_str("El Burrito a utilizar no tiene vidas");
                }

                // Obtener información de los accesorios para ver si existen y recuperar las estadísticas a aumentar
                ext_nft::get_items_for_battle_cpu(
                    accesorio1_id.to_string(), // Id el item 1 del burrito
                    accesorio2_id.to_string(), // Id el item 2 del burrito
                    accesorio3_id.to_string(), // Id el item 3 del burrito
                    ITEMS_CONTRACT.parse::<AccountId>().unwrap(), // Contrato de items
                    NO_DEPOSIT, // yocto NEAR a ajuntar
                    GAS_FOR_RESOLVE_TRANSFER // gas a ajuntar
                )
                .then(ext_self::save_battle_player_cpu(
                    burrito_id,
                    burrito_for_battle,
                    PVE_CONTRACT.parse::<AccountId>().unwrap(), // Contrato de burritos
                    NO_DEPOSIT, // yocto NEAR a ajuntar al callback
                    GAS_FOR_RESOLVE_TRANSFER // gas a ajuntar al callback
                ));
            }
        }
    }

    // Guardar sala de combate Player vs CPU
    pub fn save_battle_player_cpu(&mut self, burrito_id: TokenId, burrito_for_battle: Burrito) -> String {
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

                let extradatajson_burrito = ExtraBurrito {
                    burrito_type: burrito_for_battle.burrito_type.clone().to_string(),
                    hp : burrito_for_battle.hp.clone().to_string(),
                    attack : burrito_for_battle.attack.clone().to_string(),
                    defense : burrito_for_battle.defense.clone().to_string(),
                    speed : burrito_for_battle.speed.clone().to_string(),
                    win : burrito_for_battle.win.clone().to_string(),
                    global_win : burrito_for_battle.global_win.clone().to_string(),
                    level : burrito_for_battle.level.clone().to_string()
                };
        
                // Crear estructura burrito 1
                let burrito = burrito_for_battle.clone();
        
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
                    level : level_cpu.clone().to_string(),
                    media : "".to_string()
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
                    status : "2".to_string(),
                    player_id : token_owner_id.clone().to_string(),
                    burrito_id : burrito_id.clone().to_string(),
                    attack_b1 : burrito.attack.clone(),
                    defense_b1 : burrito.defense.clone(),
                    speed_b1 : burrito.speed.clone(),
                    level_b1 : burrito.level.clone(),
                    accesories_attack_b1 : accessories_for_battle.final_attack_b1.clone().to_string(),
                    accesories_defense_b1 : accessories_for_battle.final_defense_b1.clone().to_string(),
                    accesories_speed_b1 : accessories_for_battle.final_speed_b1.clone().to_string(),
                    accesories_attack_b2 : accessories_for_battle.final_attack_b2.clone().to_string(),
                    accesories_defense_b2 : accessories_for_battle.final_defense_b2.clone().to_string(),
                    accesories_speed_b2 : accessories_for_battle.final_speed_b2.clone().to_string(),
                    turn : burrito_first_atack.to_string(),
                    strong_attack_player : "3".to_string(),
                    shields_player : "3".to_string(),
                    start_health_player : (burrito.attack.parse::<u8>().unwrap()+burrito.defense.parse::<u8>().unwrap()+burrito.speed.parse::<u8>().unwrap()).to_string(),
                    health_player : (burrito.attack.parse::<u8>().unwrap()+burrito.defense.parse::<u8>().unwrap()+burrito.speed.parse::<u8>().unwrap()).to_string(),
                    strong_attack_cpu : "3".to_string(),
                    shields_cpu : "3".to_string(),
                    start_health_cpu : (burrito_cpu.attack.parse::<u8>().unwrap()+burrito_cpu.defense.parse::<u8>().unwrap()+burrito_cpu.speed.parse::<u8>().unwrap()).to_string(),
                    health_cpu : (burrito_cpu.attack.parse::<u8>().unwrap()+burrito_cpu.defense.parse::<u8>().unwrap()+burrito_cpu.speed.parse::<u8>().unwrap()).to_string(),
                    burrito_cpu_level : level_cpu.clone().to_string(),
                    burrito_cpu_type : burrito_cpu.burrito_type.to_string(),
                    burrito_cpu_attack : burrito_cpu.attack.to_string(),
                    burrito_cpu_defense : burrito_cpu.defense.to_string(),
                    burrito_cpu_speed : burrito_cpu.speed.to_string()
                };
        
                self.battle_rooms.insert(token_owner_id.clone().to_string(),info.clone());
        
                env::log(
                    json!(info.clone())
                    .to_string()
                    .as_bytes(),
                );

                serde_json::to_string(&info).unwrap()

            }
        }

    }

    // Rendirse y finalizar batalla Player vs CPU
    pub fn surrender_cpu(&mut self) -> String {
        let token_owner_id = env::signer_account_id();

        let br = self.battle_rooms.get(&token_owner_id.to_string());
        
        if br.is_none() {
            env::panic_str("No tienes una batalla activa");
        }

        let info = br.unwrap();

        let battle_room = BattleCPU {
            status : info.status.to_string(),
            player_id : info.player_id.to_string(),
            burrito_id : info.burrito_id.to_string(),
            attack_b1 : info.attack_b1.clone(),
            defense_b1 : info.defense_b1.clone(),
            speed_b1 : info.speed_b1.clone(),
            level_b1 : info.level_b1.clone(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player : info.strong_attack_player.to_string(),
            shields_player : info.shields_player.to_string(),
            start_health_player : info.start_health_player.to_string(),
            health_player : info.health_player.to_string(),
            strong_attack_cpu : info.strong_attack_cpu.to_string(),
            shields_cpu : info.shields_cpu.to_string(),
            start_health_cpu : info.start_health_cpu.to_string(),
            health_cpu : info.health_cpu.to_string(),
            burrito_cpu_level : info.burrito_cpu_level.to_string(),
            burrito_cpu_type : info.burrito_cpu_type.to_string(),
            burrito_cpu_attack : info.burrito_cpu_attack.to_string(),
            burrito_cpu_defense : info.burrito_cpu_defense.to_string(),
            burrito_cpu_speed : info.burrito_cpu_speed.to_string()
        };

        // Mandar a llamar al contrato de burritos para modificar la informacion del burrito perdedor
        let p = ext_nft::decrease_burrito_hp(
            battle_room.burrito_id.clone().to_string(),
            BURRITO_CONTRACT.parse::<AccountId>().unwrap(),
            NO_DEPOSIT,
            MIN_GAS_FOR_NFT_TRANSFER_CALL
        );

        // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
        let info = BattlesHistory {
            player1_id : battle_room.player_id.to_string(),
            player2_id : "CPU".to_string(),
            winner : "CPU".to_string(),
            status : "Surrender".to_string()
        };

        self.battle_history.insert(battle_room.player_id.to_string()+&"-".to_string()+ &self.battle_history.len().to_string(),info);

        // Eliminar sala
        self.battle_rooms.remove(&token_owner_id.to_string());

        "Te rendiste".to_string()
        
    }

    // Método combate player vs cpu (type_move 1 = Ataque Debil, 2 = Ataque Fuerte, 3 = No Defenderse 4 = Defenderse)
    pub fn battle_player_cpu(&mut self, type_move: String) -> BattleCPU {
        let token_owner_id = env::signer_account_id();

        let br = self.battle_rooms.get(&token_owner_id.to_string());
        
        if br.is_none() {
            env::panic_str("No tienes una batalla activa");
        }

        let info = br.unwrap();

        let battle_room = BattleCPU {
            status : info.status.to_string(),
            player_id : info.player_id.to_string(),
            burrito_id : info.burrito_id.clone().to_string(),
            attack_b1 : info.attack_b1.clone(),
            defense_b1 : info.defense_b1.clone(),
            speed_b1 : info.speed_b1.clone(),
            level_b1 : info.level_b1.clone(),
            accesories_attack_b1 : info.accesories_attack_b1.to_string(),
            accesories_defense_b1 : info.accesories_defense_b1.to_string(),
            accesories_speed_b1 : info.accesories_speed_b1.to_string(),
            accesories_attack_b2 : info.accesories_attack_b2.to_string(),
            accesories_defense_b2 : info.accesories_defense_b2.to_string(),
            accesories_speed_b2 : info.accesories_speed_b2.to_string(),
            turn : info.turn.to_string(),
            strong_attack_player : info.strong_attack_player.to_string(),
            shields_player : info.shields_player.to_string(),
            start_health_player : info.start_health_player.to_string(),
            health_player : info.health_player.to_string(),
            strong_attack_cpu : info.strong_attack_cpu.to_string(),
            shields_cpu : info.shields_cpu.to_string(),
            start_health_cpu : info.start_health_cpu.to_string(),
            health_cpu : info.health_cpu.to_string(),
            burrito_cpu_level : info.burrito_cpu_level.to_string(),
            burrito_cpu_type : info.burrito_cpu_type.to_string(),
            burrito_cpu_attack : info.burrito_cpu_attack.to_string(),
            burrito_cpu_defense : info.burrito_cpu_defense.to_string(),
            burrito_cpu_speed : info.burrito_cpu_speed.to_string()
        };


        if (type_move == "1" || type_move == "2") && battle_room.turn == "CPU"{
            env::panic_str("No puedes realizar un ataque, debes elegir si defenderte o no");
        }

        if (type_move == "3" || type_move == "4") && battle_room.turn == "Player"{
            env::panic_str("No puedes defenderte, debes realizar un ataque");
        }

        if type_move == "2" && battle_room.strong_attack_player.parse::<u8>().unwrap() == 0 {
            env::panic_str("No tienes mas ataques fuertes, debes realizar uno normal");
        }

        if type_move == "4" && battle_room.shields_player.parse::<u8>().unwrap() == 0 {
            env::panic_str("No tienes mas escudos, no puedes defenderte");
        }

        let mut old_battle_room = battle_room;
        let mut cpu_type_move = "1";

        // Verificar si se utilizo un escudo para finalizar la ronda
        if old_battle_room.turn == "Player"{
            if type_move == "2"{
                old_battle_room.strong_attack_player = (old_battle_room.strong_attack_player.parse::<u8>().unwrap()-1).to_string();
                // log!("Jugador utilizó ataque fuerte");
            }
            // Validar si el CPU aun tiene escudos y elegir aleatoriamente si utilizara uno o no
            if old_battle_room.shields_cpu.parse::<u8>().unwrap() > 0 {
                let use_shield: u8 = *env::random_seed().get(0).unwrap();
                if use_shield % 2 == 1 {
                    old_battle_room.shields_cpu = (old_battle_room.shields_cpu.parse::<u8>().unwrap()-1).to_string();
                    old_battle_room.turn = "CPU".to_string();
                    self.battle_rooms.remove(&old_battle_room.player_id);
                    self.battle_rooms.insert(old_battle_room.player_id.to_string(),old_battle_room.clone());
                    // log!("CPU utilizó escudo");
                    // return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");
                    env::log(
                        json!(old_battle_room)
                        .to_string()
                        .as_bytes(),
                    );
                    return old_battle_room;
                }
            }
        } else {
            if old_battle_room.strong_attack_cpu.parse::<u8>().unwrap() > 0 {
                let use_strong_attack: u8 = *env::random_seed().get(0).unwrap();
                if old_battle_room.shields_player.parse::<u8>().unwrap() == 0 {
                    old_battle_room.strong_attack_cpu = (old_battle_room.strong_attack_cpu.parse::<u8>().unwrap()-1).to_string();
                    cpu_type_move = "2";
                    // log!("CPU utilizó ataque fuerte");
                } else {
                    if use_strong_attack % 2 == 1 {
                        old_battle_room.strong_attack_cpu = (old_battle_room.strong_attack_cpu.parse::<u8>().unwrap()-1).to_string();
                        cpu_type_move = "2";
                        // log!("CPU utilizó ataque fuerte");
                    }
                }
            }
            if type_move == "4"{
                old_battle_room.shields_player = (old_battle_room.shields_player.parse::<u8>().unwrap()-1).to_string();
                old_battle_room.turn = "Player".to_string();
                self.battle_rooms.remove(&old_battle_room.player_id);
                self.battle_rooms.insert(old_battle_room.player_id.to_string(),old_battle_room.clone());
                // log!("Jugador utilizó escudo");
                // return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");
                env::log(
                    json!(old_battle_room)
                    .to_string()
                    .as_bytes(),
                );
                return old_battle_room;
            }
        }

        
        // Crear estructura burrito
        let burrito = Burrito {
            owner_id : token_owner_id.clone().to_string(),
            name : "".to_string(),
            description : "".to_string(),
            burrito_type : "".to_string(),
            hp : "".to_string(),
            attack : old_battle_room.attack_b1.to_string(),
            defense : old_battle_room.defense_b1.to_string(),
            speed : old_battle_room.speed_b1.to_string(),
            win : "".to_string(),
            global_win : "".to_string(),
            level : old_battle_room.level_b1.to_string(),
            media : "".to_string()
        };

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
            level : old_battle_room.burrito_cpu_level.to_string(),
            media : "".to_string()
        };

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

        // log!("Vida vieja burrito defensor: {}",old_health_burrito_defender);

        let mut attack = 0.0;
        if old_battle_room.turn == "Player"{
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult+old_battle_room.accesories_attack_b1.parse::<f32>().unwrap();
        } else {
            attack = (burrito_attacker.attack.parse::<f32>().unwrap()*attack_mult)+type_mult+old_battle_room.accesories_attack_b2.parse::<f32>().unwrap();
        }

        // log!("Cantidad de daño a realizar: {}",attack);

        // Verificar el tipo de ataque
        if old_battle_room.turn == "Player"{
            if type_move == "2"{
                attack += attack*0.5;
                // log!("Cantidad de daño fuerte a realizar: {}",attack);
            }
        } else {
            if cpu_type_move == "2"{
                attack += attack*0.5;
                // log!("Cantidad de daño fuerte a realizar: {}",attack);
            }
        }
        
        let new_health_burrito_defender = old_health_burrito_defender - attack;
        // log!("Vida nueva burrito defensor: {}",new_health_burrito_defender);
        
        // Actualizar registro de sala de batalla
        if old_battle_room.turn == "Player"{
            if new_health_burrito_defender <= 0.0 {
                old_battle_room.health_cpu = new_health_burrito_defender.to_string();

                // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
                let info = BattlesHistory {
                    player1_id : old_battle_room.player_id.to_string(),
                    player2_id : "CPU".to_string(),
                    winner : old_battle_room.player_id.to_string(),
                    status : "Battle".to_string()
                };
                self.battle_history.insert(old_battle_room.player_id.to_string()+&"-".to_string()+ &self.battle_history.len().to_string(),info);
                // Eliminar sala activa
                self.battle_rooms.remove(&old_battle_room.player_id);
                // log!("Batalla Finalizada, Ganó Jugador");

                // Minar recompensa STRW Tokens
                // log!("Nivel burrito cpu {}",burrito_defender.level.clone().to_string().parse::<f32>().unwrap());
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

                // log!("Tokens a minar {}",tokens_mint*1000000000000000000000000.0);
                let tokens_to_mint = tokens_mint*1000000000000000000000000.0;
                ext_nft::reward_player(
                    old_battle_room.player_id.clone().to_string(),
                    tokens_to_mint.to_string(),
                    STRWTOKEN_CONTRACT.parse::<AccountId>().unwrap(),
                    0000000000000000000000001,
                    MIN_GAS_FOR_NFT_TRANSFER_CALL
                ).then(ext_nft::increment_burrito_wins(
                    old_battle_room.burrito_id.clone().to_string(),
                    BURRITO_CONTRACT.parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    GAS_FOR_NFT_TRANSFER_CALL
                ));

                // return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");
                env::log(
                    json!(old_battle_room)
                    .to_string()
                    .as_bytes(),
                );
                return old_battle_room;
            } else {
                old_battle_room.health_cpu = new_health_burrito_defender.to_string();
                old_battle_room.turn = "CPU".to_string();
                self.battle_rooms.remove(&old_battle_room.player_id);
                self.battle_rooms.insert(old_battle_room.player_id.to_string(),old_battle_room.clone());
            }
        } else {
            if new_health_burrito_defender <= 0.0 {
                // Guardar registro general de la batalla (Jugador, Burrito, Estatus)
                old_battle_room.health_player = new_health_burrito_defender.to_string();

                let info = BattlesHistory {
                    player1_id : old_battle_room.player_id.to_string(),
                    player2_id : "CPU".to_string(),
                    winner : "CPU".to_string(),
                    status : "Battle".to_string()
                };
                self.battle_history.insert(old_battle_room.player_id.to_string()+&"-".to_string()+ &self.battle_history.len().to_string(),info);
                // Eliminar sala activa
                self.battle_rooms.remove(&old_battle_room.player_id);
                // log!("Batalla Finalizada, Ganó CPU");

                // Restar una vida al burrito
                ext_nft::decrease_burrito_hp(
                    old_battle_room.burrito_id.clone().to_string(),
                    BURRITO_CONTRACT.parse::<AccountId>().unwrap(),
                    NO_DEPOSIT,
                    MIN_GAS_FOR_NFT_TRANSFER_CALL
                );

                // return str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'");

                env::log(
                    json!(old_battle_room)
                    .to_string()
                    .as_bytes(),
                );

                return old_battle_room;
            } else {
                old_battle_room.health_player = new_health_burrito_defender.to_string();
                old_battle_room.turn = "Player".to_string();
                self.battle_rooms.remove(&old_battle_room.player_id);
                self.battle_rooms.insert(old_battle_room.player_id.to_string(),old_battle_room.clone());
            }                
        }

        // log!("Ronda Finalizada");

        env::log(
            json!(old_battle_room)
            .to_string()
            .as_bytes(),
        );

        // str::replace(&serde_json::to_string(&old_battle_room.clone()).unwrap(), "\"", "'")

        old_battle_room
    }

}