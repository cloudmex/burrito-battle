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
    // Restaurar burrito 30,000 $STRW tokens + 1 $NEAR tokens
    #[payable]
    pub fn reset_burrito(&mut self, burrito_id: TokenId) -> Promise {
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

        ext_nft::get_balance_and_transfer(
            account_id.clone().to_string(),
            "Reset".to_string(),
            self.strw_contract.parse::<AccountId>().unwrap(),
            deposit,
            MIN_GAS_FOR_NFT_TRANSFER_CALL
        ).then(ext_self::reset_conditions(
            burrito_id.to_string(),
            self.burrito_contract.parse::<AccountId>().unwrap(), // Contrato de burritos
            NO_DEPOSIT, // yocto NEAR a ajuntar al callback
            GAS_FOR_NFT_TRANSFER_CALL // gas a ajuntar al callback
        ))
    }

    pub fn reset_conditions(&mut self, burrito_id: TokenId) -> Burrito{
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );

        // handle the result from the cross contract call this method is a callback for
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => {
                let empty_info = Burrito {
                    owner_id : "".to_string(),
                    name : "".to_string(),
                    description : "".to_string(),
                    burrito_type : "".to_string(),
                    hp : "".to_string(),
                    attack : "".to_string(),
                    defense : "".to_string(),
                    speed : "".to_string(),
                    win : "".to_string(),
                    global_win : "".to_string(),
                    level : "".to_string(),
                    media : "".to_string()
                };

                empty_info
            },
            PromiseResult::Successful(_result) => {
        
                let mut metadata_burrito = self.token_metadata_by_id.get(&burrito_id.clone()).unwrap();
        
                let newextradata_burrito = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        
                let mut extradatajson_burrito: ExtraBurrito = serde_json::from_str(&newextradata_burrito).unwrap();
                                       
                extradatajson_burrito.hp = "5".to_string();
        
                let mut extra_string_burrito = serde_json::to_string(&extradatajson_burrito).unwrap();
                extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
                metadata_burrito.extra = Some(extra_string_burrito.clone());
        
                self.token_metadata_by_id.insert(&burrito_id, &metadata_burrito);
                log!("{}",burrito_id);


                //"Burrito Restaurado".to_string()

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

                //burrito_id

                env::log(
                    json!(burrito)
                    .to_string()
                    .as_bytes(),
                );

                burrito
            }
        }

    }

}