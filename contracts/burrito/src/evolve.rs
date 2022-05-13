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
       // Evolucionar burrito 100,000 $STRW tokens + 2 $NEAR tokens
       #[payable]
       pub fn evolve_burrito(&mut self, burrito_id: TokenId) -> Promise {
           if burrito_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe el burrito con el id ingresado");
           }
   
           // Validar que el burrito pertenezca al signer
           let token = self.tokens_by_id.get(&burrito_id.clone());        
           let account_id = env::signer_account_id();
           let deposit = env::attached_deposit();        
           let owner_id = token.unwrap().owner_id.to_string();
   
           if account_id.clone() != owner_id.clone().parse::<AccountId>().unwrap() {
            env::panic_str("El burrito no te pertenece");
           }
   
           let metadata_burrito = self.token_metadata_by_id.get(&burrito_id.clone()).unwrap();
   
           let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
           let extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();
           let win_burrito = extradatajson_burrito.win.clone().parse::<u8>().unwrap();
           let level_burrito = extradatajson_burrito.level.clone().parse::<u8>().unwrap();
   
           if level_burrito == 40 {
            env::panic_str("El burrito ya no puede evolucionar, el nivel máximo es el 40");
           }
   
           if win_burrito < 10 {
            env::panic_str("El burrito no cumple las victorias para evolucionar deben ser 10");
           }
   
           ext_nft::get_balance_and_transfer(
               account_id.clone().to_string(),
               "Evolve".to_string(),
               STRWTOKEN_CONTRACT.parse::<AccountId>().unwrap(),
               deposit,
               MIN_GAS_FOR_NFT_TRANSFER_CALL
           ).then(ext_self::burrito_level_up(
               burrito_id.to_string(),
               BURRITO_CONTRACT.parse::<AccountId>().unwrap(), // Contrato de burritos
               NO_DEPOSIT, // yocto NEAR a ajuntar al callback
               GAS_FOR_NFT_TRANSFER_CALL // gas a ajuntar al callback
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
                   let mut metadata_burrito = self.token_metadata_by_id.get(&burrito_id.clone()).unwrap();

                   let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
           
                   let mut extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();
           
                   let token = self.tokens_by_id.get(&burrito_id.clone());        
                   let owner_id_burrito = token.unwrap().owner_id.to_string();
           
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
                       level : extradatajson_burrito.level.clone(),
                        media : metadata_burrito.media.as_ref().unwrap().to_string()
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
           

                   self.token_metadata_by_id.insert(&burrito_id, &metadata_burrito);
                   log!("{}",burrito_id);

                   "Burrito Evolucionado".to_string()
               }
           }
   
       }
}