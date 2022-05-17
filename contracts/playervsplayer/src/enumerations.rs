use crate::*;

#[near_bindgen]
impl Contract {
    // Obtener cantidad de batallas activas Player vs CPU
    pub fn get_number_battles_actives(&self) -> u128 {
        self.battle_rooms.len().try_into().unwrap()
    }

    // Obtener numero de batallas finalizadas
    pub fn get_number_battles(&self) -> u128 {
        self.battle_history.len().try_into().unwrap()
    }
}
