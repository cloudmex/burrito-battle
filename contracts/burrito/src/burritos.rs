use near_sdk::{
    env, serde_json::json
};

use crate::*;

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
//const NO_DEPOSIT: Balance = 0;

#[near_bindgen]
impl Contract {
    // Minar un nuevo token 600,000 $STRW tokens + 5 $NEAR tokens
    #[payable]
    pub fn nft_mint(&mut self,token_owner_id: AccountId, token_metadata: TokenMetadata) -> Promise {
        let account_id = env::signer_account_id();
        
        let deposit = env::attached_deposit();
        // 5 Nears
        let deposit_to_treasury = deposit.clone() - 100000000000000000000000;
        log!("Deposito en mint_token treasury {}",deposit_to_treasury);
        // 0.1 Nears
        let deposit_to_mint = deposit.clone() - 4900000000000000000000000;
        log!("Deposito en mint_token mint {}",deposit_to_mint);

        ext_nft::get_balance_and_transfer(
            account_id.clone().to_string(),
            "Mint".to_string(),
            self.strw_contract.parse::<AccountId>().unwrap(),
            deposit_to_treasury,
            MIN_GAS_FOR_NFT_TRANSFER_CALL
        ).then(ext_self::new_burrito(
            token_owner_id,
            token_metadata,
            self.burrito_contract.parse::<AccountId>().unwrap(),
            deposit_to_mint,
            GAS_FOR_NFT_TRANSFER_CALL
        ))
    }

    #[payable]
    pub fn new_burrito(&mut self,token_owner_id: AccountId, token_metadata: TokenMetadata) -> Burrito{
        assert_eq!(
            env::promise_results_count(),
            1,
            "Éste es un método callback"
        );
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
                let initial_storage_usage = env::storage_usage();
                let deposit = env::attached_deposit();   
                log!("Deposito en new_burrito {}",deposit);

                let mut new_burrito = token_metadata;
                let burrito_id: TokenId = (self.token_metadata_by_id.len()).to_string();
                let mut burrito_data = ExtraBurrito {
                    hp : "5".to_string(),
                    attack : "5".to_string(),
                    defense : "5".to_string(),
                    speed : "5".to_string(),
                    win : "0".to_string(),
                    global_win : "0".to_string(),
                    burrito_type : "Fuego".to_string(),
                    level : "1".to_string()
                };

                // Generar estadísticas random

                let rand_attack = *env::random_seed().get(0).unwrap();
                let rand_defense = *env::random_seed().get(1).unwrap();
                let rand_speed = *env::random_seed().get(2).unwrap();
                let rand_type = *env::random_seed().get(3).unwrap();
                let rand_image = *env::random_seed().get(4).unwrap();

                let mut attack: u8 = 5;
                let mut defense: u8 = 5;
                let mut speed: u8 = 5;
                let mut burrito_type: String = "Fuego".to_string();
                let mut burrito_image: String = BURRITO1.to_string();

                // Obtener ataque aleatorio
                if rand_attack > 0 &&  rand_attack <= 70 {
                    attack = 5;
                }
                if rand_attack >= 71 &&  rand_attack <= 130 {
                    attack = 6;
                }
                if rand_attack >= 131 &&  rand_attack <= 180 {
                    attack = 7;
                }
                if rand_attack >= 181 &&  rand_attack <= 220 {
                    attack = 8;
                }
                if rand_attack >= 221 &&  rand_attack <= 250 {
                    attack = 9;
                }
                if rand_attack >= 251 &&  rand_attack < 255 {
                    attack = 10;
                }

                // Obtener defensa aleatoria
                if rand_defense > 0 &&  rand_defense <= 70 {
                    defense = 5;
                }
                if rand_defense >= 71 &&  rand_defense <= 130 {
                    defense = 6;
                }
                if rand_defense >= 131 &&  rand_defense <= 180 {
                    defense = 7;
                }
                if rand_defense >= 181 &&  rand_defense <= 220 {
                    defense = 8;
                }
                if rand_defense >= 221 &&  rand_defense <= 250 {
                    defense = 9;
                }
                if rand_defense >= 251 &&  rand_defense < 255 {
                    defense = 10;
                }

                // Obtener velociad aleatoria
                if rand_speed > 0 &&  rand_speed <= 70 {
                    speed = 5;
                }
                if rand_speed >= 71 &&  rand_speed <= 130 {
                    speed = 6;
                }
                if rand_speed >= 131 &&  rand_speed <= 180 {
                    speed = 7;
                }
                if rand_speed >= 181 &&  rand_speed <= 220 {
                    speed = 8;
                }
                if rand_speed >= 221 &&  rand_speed <= 250 {
                    speed = 9;
                }
                if rand_speed >= 251 &&  rand_speed < 255 {
                    speed = 10;
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

                // Obtener imagen
                if rand_image > 0 &&  rand_image <= 64 {
                    burrito_image = BURRITO1.to_string();
                }
                if rand_image > 64 &&  rand_image <= 128 {
                    burrito_image = BURRITO2.to_string();
                }
                if rand_image > 128 &&  rand_image <= 192 {
                    burrito_image = BURRITO3.to_string();
                }
                if rand_image > 192 &&  rand_image < 255 {
                    burrito_image = BURRITO4.to_string();
                }

                // Asignamos valores a las estadisticas del burrito
                burrito_data.attack = attack.to_string();
                burrito_data.defense = defense.to_string();
                burrito_data.speed = speed.to_string();
                burrito_data.burrito_type = burrito_type.to_string();

                let mut extra_data_string = serde_json::to_string(&burrito_data).unwrap();
                extra_data_string = str::replace(&extra_data_string, "\"", "'");
                new_burrito.extra = Some(extra_data_string);
                new_burrito.media = Some(burrito_image);
                let name_burrito = "Burrito ".to_string()+&burrito_type.to_string()+&" #".to_string()+&self.token_metadata_by_id.len().to_string();
                let desription_burrito = "Este es un burrito de tipo ".to_string()+&burrito_type.to_string();

                new_burrito.title = Some(name_burrito);
                new_burrito.description = Some(desription_burrito);

                let royalty = HashMap::new();

                //specify the token struct that contains the owner ID 
                let token = Token {
                    //set the owner ID equal to the receiver ID passed into the function
                    owner_id: token_owner_id.clone(),
                    //we set the approved account IDs to the default value (an empty map)
                    approved_account_ids: Default::default(),
                    //the next approval ID is set to 0
                    next_approval_id: 0,
                    //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
                    royalty,
                };

                //insert the token ID and token struct and make sure that the token doesn't exist
                assert!(
                    self.tokens_by_id.insert(&burrito_id, &token).is_none(),
                    "Token already exists"
                );

                //insert the token ID and metadata
                self.token_metadata_by_id.insert(&burrito_id, &new_burrito);

                //call the internal method for adding the token to the owner
                self.internal_add_token_to_owner(&token.owner_id, &burrito_id);

                // Construct the mint log as per the events standard.
                let nft_mint_log: EventLog = EventLog {
                    // Standard name ("nep171").
                    standard: NFT_STANDARD_NAME.to_string(),
                    // Version of the standard ("nft-1.0.0").
                    version: NFT_METADATA_SPEC.to_string(),
                    // The data related with the event stored in a vector.
                    event: EventLogVariant::NftMint(vec![NftMintLog {
                        // Owner of the token.
                        owner_id: token.owner_id.to_string(),
                        // Vector of token IDs that were minted.
                        token_ids: vec![burrito_id.to_string()],
                        // An optional memo to include.
                        memo: None,
                    }]),
                };

                // Log the serialized json.
                env::log_str(&nft_mint_log.to_string());

                //calculate the required storage which was the used - initial
                let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

                //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
                refund_deposit(required_storage_in_bytes);

                let burrito = Burrito {
                    owner_id : token_owner_id.clone().to_string(),
                    name : new_burrito.title.as_ref().unwrap().to_string(),
                    description : new_burrito.description.as_ref().unwrap().to_string(),
                    burrito_type : burrito_data.burrito_type,
                    hp : burrito_data.hp,
                    attack : burrito_data.attack,
                    defense : burrito_data.defense,
                    speed : burrito_data.speed,
                    win : burrito_data.win,
                    global_win : burrito_data.global_win,
                    level : burrito_data.level,
                    media : new_burrito.media.as_ref().unwrap().to_string()
                };

                env::log(
                    json!(burrito)
                    .to_string()
                    .as_bytes(),
                );

                //serde_json::to_string(&burrito).unwrap()

                burrito
            }
        }

    }

    pub fn get_number_burritos(&self) -> u64 {
        self.token_metadata_by_id.len()
    }

    pub fn get_burrito(&self, burrito_id: TokenId) -> Burrito {
        if burrito_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe el burrito con el id ingresado");
        }
    
        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&burrito_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        if account_id.clone() != owner_id.clone().parse::<AccountId>().unwrap() {
            env::panic_str("El burrito no te pertenece");
        }

        let metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win,
            global_win : extradatajson.global_win,
            level : extradatajson.level,
            media : metadata.media.as_ref().unwrap().to_string()
        };

        burrito
    }

    pub fn get_burrito_incursion(&self, burrito_id: TokenId) -> Burrito {
        if burrito_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe el burrito con el id ingresado");
        }
    
        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&burrito_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        self.assert_whitelist(env::predecessor_account_id());

        let metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win,
            global_win : extradatajson.global_win,
            level : extradatajson.level,
            media : metadata.media.as_ref().unwrap().to_string()
        };

        burrito
    }

    pub fn get_burrito_capsule(&self, burrito_id: TokenId) -> Burrito {
        if burrito_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe el burrito con el id ingresado");
        }
    
        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&burrito_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        self.assert_whitelist(env::predecessor_account_id());

        let metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win,
            global_win : extradatajson.global_win,
            level : extradatajson.level,
            media : metadata.media.as_ref().unwrap().to_string()
        };

        burrito
    }

    pub fn get_burrito_fortress(&self, burrito_id: TokenId) -> Burrito {
        if burrito_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe el burrito con el id ingresado");
        }
    
        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&burrito_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        self.assert_whitelist(env::predecessor_account_id());

        let metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win,
            global_win : extradatajson.global_win,
            level : extradatajson.level,
            media : metadata.media.as_ref().unwrap().to_string()
        };

        burrito
    }

    pub fn update_burrito(&mut self, burrito_id: TokenId, extra: String) -> Burrito {
        self.assert_whitelist(env::predecessor_account_id());

        if burrito_id.clone().parse::<u64>().unwrap() > self.token_metadata_by_id.len()-1 {
            env::panic_str("No existe el burrito con el id ingresado");
        }

        // Validar que el burrito pertenezca al signer
        let account_id = env::signer_account_id();
        let token = self.tokens_by_id.get(&burrito_id.clone());        
        let owner_id = token.unwrap().owner_id.to_string();

        // if account_id.clone() != owner_id.clone().parse::<AccountId>().unwrap() {
        //     env::panic_str("El burrito no te pertenece");
        // }

        let mut metadata_burrito = self.token_metadata_by_id.get(&burrito_id.clone()).unwrap();
        
        metadata_burrito.extra = Some(extra);

        self.token_metadata_by_id.insert(&burrito_id, &metadata_burrito);

        let newextradata = str::replace(&metadata_burrito.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            owner_id : owner_id.to_string(),
            name : metadata_burrito.title.as_ref().unwrap().to_string(),
            description : metadata_burrito.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win,
            global_win : extradatajson.global_win,
            level : extradatajson.level,
            media : metadata_burrito.media.as_ref().unwrap().to_string()
        };

        burrito
    }

    pub fn decrease_burrito_hp(&mut self, burrito_id: TokenId) -> String {
        self.assert_whitelist(env::predecessor_account_id());

        let mut metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let mut extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();
        
        // Crear estructura burrito
        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type.clone(),
            hp : extradatajson.hp.clone(),
            attack : extradatajson.attack.clone(),
            defense : extradatajson.defense.clone(),
            speed : extradatajson.speed.clone(),
            win : extradatajson.win.clone(),
            global_win : extradatajson.global_win.clone(),
            level : extradatajson.level.clone(),
            media : metadata.media.as_ref().unwrap().to_string()
        };

        let new_hp_burrito = burrito.hp.parse::<u8>().unwrap()-1;
        extradatajson.hp = new_hp_burrito.to_string();

        let mut extra_string_burrito = serde_json::to_string(&extradatajson).unwrap();
        extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
        metadata.extra = Some(extra_string_burrito.clone());

        self.token_metadata_by_id.insert(&burrito_id, &metadata);

        "Contador de vidas decrementado".to_string()

    }

    pub fn decrease_all_burrito_hp(&mut self, burrito_id: TokenId) -> String {
        self.assert_whitelist(env::predecessor_account_id());

        let mut metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let mut extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();
        
        // Crear estructura burrito
        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type.clone(),
            hp : extradatajson.hp.clone(),
            attack : extradatajson.attack.clone(),
            defense : extradatajson.defense.clone(),
            speed : extradatajson.speed.clone(),
            win : extradatajson.win.clone(),
            global_win : extradatajson.global_win.clone(),
            level : extradatajson.level.clone(),
            media : metadata.media.as_ref().unwrap().to_string()
        };

        let new_hp_burrito = 0;
        extradatajson.hp = new_hp_burrito.to_string();

        let mut extra_string_burrito = serde_json::to_string(&extradatajson).unwrap();
        extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
        metadata.extra = Some(extra_string_burrito.clone());

        self.token_metadata_by_id.insert(&burrito_id, &metadata);

        "Contador de vidas decrementado".to_string()

    }

    pub fn increase_all_burrito_hp(&mut self, burrito_id: TokenId) -> String {
        self.assert_whitelist(env::predecessor_account_id());

        let mut metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let mut extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();
        
        // Crear estructura burrito
        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type.clone(),
            hp : extradatajson.hp.clone(),
            attack : extradatajson.attack.clone(),
            defense : extradatajson.defense.clone(),
            speed : extradatajson.speed.clone(),
            win : extradatajson.win.clone(),
            global_win : extradatajson.global_win.clone(),
            level : extradatajson.level.clone(),
            media : metadata.media.as_ref().unwrap().to_string()
        };

        let new_hp_burrito = 5;
        extradatajson.hp = new_hp_burrito.to_string();

        let mut extra_string_burrito = serde_json::to_string(&extradatajson).unwrap();
        extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
        metadata.extra = Some(extra_string_burrito.clone());

        self.token_metadata_by_id.insert(&burrito_id, &metadata);

        "Contador de vidas incrementado".to_string()

    }

    pub fn increment_burrito_wins(&mut self, burrito_id: TokenId) -> String {
        self.assert_whitelist(env::predecessor_account_id());

        let mut metadata = self.token_metadata_by_id.get(&burrito_id).unwrap();
        let token = self.tokens_by_id.get(&burrito_id);        

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let mut extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();
        
        // Crear estructura burrito
        let burrito = Burrito {
            owner_id : token.unwrap().owner_id.to_string(),
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type.clone(),
            hp : extradatajson.hp.clone(),
            attack : extradatajson.attack.clone(),
            defense : extradatajson.defense.clone(),
            speed : extradatajson.speed.clone(),
            win : extradatajson.win.clone(),
            global_win : extradatajson.global_win.clone(),
            level : extradatajson.level.clone(),
            media : metadata.media.as_ref().unwrap().to_string()
        };


        // Incrementar victorias del burrito si son < 10
        let mut new_win_burrito1 = extradatajson.win.parse::<u8>().unwrap();
        let new_global_win_burrito1 = extradatajson.global_win.parse::<u8>().unwrap()+1;

        if new_win_burrito1 < 10 {
            new_win_burrito1 += 1;
        }

        extradatajson.win = new_win_burrito1.to_string();
        extradatajson.global_win = new_global_win_burrito1.to_string();

        let mut extra_string_burrito = serde_json::to_string(&extradatajson).unwrap();
        extra_string_burrito = str::replace(&extra_string_burrito, "\"", "'");
        metadata.extra = Some(extra_string_burrito.clone());

        self.token_metadata_by_id.insert(&burrito_id, &metadata);

        "Contador de victorias incrementado".to_string()
    }

    // Minar un nuevo token desde DAO
    #[payable]
    pub fn nft_mint_dao(&mut self,token_owner_id: AccountId, token_metadata: TokenMetadata) -> Burrito {
        // assert!(env::predecessor_account_id() == self.owner_id);

        let account_id = env::predecessor_account_id();
        let deposit = env::attached_deposit(); // 0.1 Nears
        let initial_storage_usage = env::storage_usage();

        let mut new_burrito = token_metadata;
        let burrito_id: TokenId = (self.token_metadata_by_id.len()).to_string();
        let mut burrito_data = ExtraBurrito {
            hp : "5".to_string(),
            attack : "5".to_string(),
            defense : "5".to_string(),
            speed : "5".to_string(),
            win : "0".to_string(),
            global_win : "0".to_string(),
            burrito_type : "Fuego".to_string(),
            level : "1".to_string()
        };

        // Generar estadísticas random

        let rand_attack = *env::random_seed().get(0).unwrap();
        let rand_defense = *env::random_seed().get(1).unwrap();
        let rand_speed = *env::random_seed().get(2).unwrap();
        let rand_type = *env::random_seed().get(3).unwrap();
        let rand_image = *env::random_seed().get(4).unwrap();

        let mut attack: u8 = 5;
        let mut defense: u8 = 5;
        let mut speed: u8 = 5;
        let mut burrito_type: String = "Fuego".to_string();
        let mut burrito_image: String = BURRITO1.to_string();

        // Obtener ataque aleatorio
        if rand_attack > 0 &&  rand_attack <= 70 {
            attack = 5;
        }
        if rand_attack >= 71 &&  rand_attack <= 130 {
            attack = 6;
        }
        if rand_attack >= 131 &&  rand_attack <= 180 {
            attack = 7;
        }
        if rand_attack >= 181 &&  rand_attack <= 220 {
            attack = 8;
        }
        if rand_attack >= 221 &&  rand_attack <= 250 {
            attack = 9;
        }
        if rand_attack >= 251 &&  rand_attack < 255 {
            attack = 10;
        }

        // Obtener defensa aleatoria
        if rand_defense > 0 &&  rand_defense <= 70 {
            defense = 5;
        }
        if rand_defense >= 71 &&  rand_defense <= 130 {
            defense = 6;
        }
        if rand_defense >= 131 &&  rand_defense <= 180 {
            defense = 7;
        }
        if rand_defense >= 181 &&  rand_defense <= 220 {
            defense = 8;
        }
        if rand_defense >= 221 &&  rand_defense <= 250 {
            defense = 9;
        }
        if rand_defense >= 251 &&  rand_defense < 255 {
            defense = 10;
        }

        // Obtener velociad aleatoria
        if rand_speed > 0 &&  rand_speed <= 70 {
            speed = 5;
        }
        if rand_speed >= 71 &&  rand_speed <= 130 {
            speed = 6;
        }
        if rand_speed >= 131 &&  rand_speed <= 180 {
            speed = 7;
        }
        if rand_speed >= 181 &&  rand_speed <= 220 {
            speed = 8;
        }
        if rand_speed >= 221 &&  rand_speed <= 250 {
            speed = 9;
        }
        if rand_speed >= 251 &&  rand_speed < 255 {
            speed = 10;
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

        // Obtener imagen
        if rand_image > 0 &&  rand_image <= 64 {
            burrito_image = BURRITO1.to_string();
        }
        if rand_image > 64 &&  rand_image <= 128 {
            burrito_image = BURRITO2.to_string();
        }
        if rand_image > 128 &&  rand_image <= 192 {
            burrito_image = BURRITO3.to_string();
        }
        if rand_image > 192 &&  rand_image < 255 {
            burrito_image = BURRITO4.to_string();
        }

        // Asignamos valores a las estadisticas del burrito
        burrito_data.attack = attack.to_string();
        burrito_data.defense = defense.to_string();
        burrito_data.speed = speed.to_string();
        burrito_data.burrito_type = burrito_type.to_string();

        let mut extra_data_string = serde_json::to_string(&burrito_data).unwrap();
        extra_data_string = str::replace(&extra_data_string, "\"", "'");
        new_burrito.extra = Some(extra_data_string);
        new_burrito.media = Some(burrito_image);
        let name_burrito = "Burrito ".to_string()+&burrito_type.to_string()+&" #".to_string()+&self.token_metadata_by_id.len().to_string();
        let desription_burrito = "Este es un burrito de tipo ".to_string()+&burrito_type.to_string();

        new_burrito.title = Some(name_burrito);
        new_burrito.description = Some(desription_burrito);

        let royalty = HashMap::new();

        //specify the token struct that contains the owner ID 
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: token_owner_id.clone(),
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
            //the map of perpetual royalties for the token (The owner will get 100% - total perpetual royalties)
            royalty,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&burrito_id, &token).is_none(),
            "Token already exists"
        );

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&burrito_id, &new_burrito);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &burrito_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![burrito_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);

        let burrito = Burrito {
            owner_id : token_owner_id.clone().to_string(),
            name : new_burrito.title.as_ref().unwrap().to_string(),
            description : new_burrito.description.as_ref().unwrap().to_string(),
            burrito_type : burrito_data.burrito_type,
            hp : burrito_data.hp,
            attack : burrito_data.attack,
            defense : burrito_data.defense,
            speed : burrito_data.speed,
            win : burrito_data.win,
            global_win : burrito_data.global_win,
            level : burrito_data.level,
            media : new_burrito.media.as_ref().unwrap().to_string()
        };

        env::log(
            json!(burrito)
            .to_string()
            .as_bytes(),
        );

        //serde_json::to_string(&burrito).unwrap()

        burrito
    }

}