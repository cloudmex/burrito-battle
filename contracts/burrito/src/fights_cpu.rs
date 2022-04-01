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
    // Obtener cantidad de batallas activas Player vs CPU
    pub fn get_number_battles_actives_cpu(&self) -> u128 {
        self.n_battle_rooms_cpu
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
            env::panic_str("No existe sala creada de esta cuenta");
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
            env::panic_str("Ya tienes una partida iniciada, debes terminarla o rendirte");
        }
        
        // Validar que exista el id
        if burrito_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe el Burrito a utilizar para el combate");
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
        .then(ext_self::save_battle_player_cpu(
            burrito_id,
            BURRITO_CONTRACT.parse::<AccountId>().unwrap(), // Contrato de burritos
            NO_DEPOSIT, // yocto NEAR a ajuntar al callback
            GAS_FOR_NFT_TRANSFER_CALL // gas a ajuntar al callback
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
                let value = std::str::from_utf8(&result).unwrap();
                let accessories_for_battle: AccessoriesForBattle = serde_json::from_str(&value).unwrap();

                let token_owner_id = env::signer_account_id();

                // Obtener metadata burrito 1
                let metadata_burrito = self.token_metadata_by_id.get(&burrito_id.clone()).unwrap();
        
                // Extraer extras del token burrito 1
                let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        
                // Crear json burrito 1
                let extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();

                let token = self.tokens_by_id.get(&burrito_id.clone());        
                let owner_id_burrito = token.unwrap().owner_id.to_string();
                
                if extradatajson_burrito.hp.clone().parse::<u8>().unwrap() == 0 {
                    env::panic_str("El Burrito a utilizar no tiene vidas");
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
            env::panic_str("No tienes una batalla activa");
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
        let mut metadata_burrito = self.token_metadata_by_id.get(&battle_room.burrito_id.clone()).unwrap();

        // Extraer extras del token burrito 1
        let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Crear json burrito 1
        let mut extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();

        let token = self.tokens_by_id.get(&battle_room.burrito_id.clone());        
        let owner_id_burrito = token.unwrap().owner_id.to_string();
        
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

        self.token_metadata_by_id.insert(&battle_room.burrito_id.clone(), &metadata_burrito);

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
        self.battle_room_cpu.remove(&token_owner_id.to_string());
        self.n_battle_rooms_cpu -= 1;

        "Finalizó batalla".to_string()
    }

}