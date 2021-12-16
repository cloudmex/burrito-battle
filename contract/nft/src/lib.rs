//Implementación de los standards NFT de near
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC,
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::ValidAccountId;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,
    Promise, PromiseOrValue,};
near_sdk::setup_alloc!();
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    burritos: NonFungibleToken,
    accessories: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u64,
    n_burritos: u64,
    n_accessories: u64
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Burrito {
    name : String,
    description : String,
    burrito_type : String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraBurrito {
    burrito_type: String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Accessory {
    name : String,
    description : String,
    attack : String,
    defense : String,
    speed : String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraAccessory {
    attack : String,
    defense : String,
    speed : String
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init_contract(owner_id: ValidAccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Burrito Battle NFT".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: ValidAccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            burritos: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            accessories: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id.clone(),
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            n_tokens: 0,
            n_burritos: 0,
            n_accessories: 0
        }
    }

    // Obtener cantidad de burritos creados
    pub fn get_number_burritos(&self) -> u64 {
        self.n_burritos
    }

    // Obtener cantidad de accesorios creados
    pub fn get_number_accessories(&self) -> u64 {
        self.n_accessories
    }

    // Minar un nuevo burrito
    #[payable]
    pub fn new_burrito(&mut self,burrito_id: TokenId,receiver_id: ValidAccountId,burrito_metadata: TokenMetadata) -> Burrito {
        let mut new_burrito = burrito_metadata;

        let mut burrito_data = ExtraBurrito {
            hp : "5".to_string(),
            attack : "".to_string(),
            defense : "".to_string(),
            speed : "".to_string(),
            win : "0".to_string(),
            burrito_type : "".to_string()
        };

        // Generar estadísticas random

        let rand_attack = *env::random_seed().get(0).unwrap();
        let rand_defense = *env::random_seed().get(1).unwrap();
        let rand_speed = *env::random_seed().get(2).unwrap();
        let rand_type = *env::random_seed().get(3).unwrap();

        let mut attack: u8 = 0;
        let mut defense: u8 = 0;
        let mut speed: u8 = 0;
        let mut burrito_type: String = "".to_string();

        // Obtener ataque aleatorio
        if rand_attack >= 0 &&  rand_attack <= 70 {
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
        if rand_attack >= 251 &&  rand_attack <= 255 {
            attack = 10;
        }

        // Obtener defensa aleatoria
        if rand_defense >= 0 &&  rand_defense <= 70 {
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
        if rand_defense >= 251 &&  rand_defense <= 255 {
            defense = 10;
        }

        // Obtener velociad aleatoria
        if rand_speed >= 0 &&  rand_speed <= 70 {
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
        if rand_speed >= 251 &&  rand_speed <= 255 {
            speed = 10;
        }

        // Obtener tipo
        if rand_type >= 0 &&  rand_type <= 51 {
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
        if rand_type >= 205 &&  rand_type <= 255 {
            burrito_type = "Volador".to_string();
        }

        // Asignamos valores a las estadisticas del burrito
        burrito_data.attack = attack.to_string();
        burrito_data.defense = defense.to_string();
        burrito_data.speed = speed.to_string();
        burrito_data.burrito_type = burrito_type.to_string();

        let mut extra_data_string = serde_json::to_string(&burrito_data).unwrap();
        extra_data_string = str::replace(&extra_data_string, "\"", "'");
        new_burrito.extra = Some(extra_data_string);

        self.burritos.mint(burrito_id, receiver_id, Some(new_burrito.clone()));

        self.n_burritos += 1;

        let burrito = Burrito {
            name : new_burrito.title.as_ref().unwrap().to_string(),
            description : new_burrito.description.as_ref().unwrap().to_string(),
            burrito_type : burrito_data.burrito_type,
            hp : burrito_data.hp,
            attack : burrito_data.attack,
            defense : burrito_data.defense,
            speed : burrito_data.speed,
            win : burrito_data.win
        };

        burrito
    }

    // Obtener burrito
    pub fn get_burrito(&self, burrito_id: TokenId) -> Burrito {
        let metadata = self
            .burritos
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&burrito_id))
            .unwrap();
        
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win
        };

        burrito
    }

    // Modificar burrito
    pub fn update_burrito(&mut self, burrito_id: TokenId, extra: String) -> Burrito {
        let mut metadata = self
            .burritos
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&burrito_id))
            .unwrap();
        
        metadata.extra = Some(extra);

        self.burritos
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&burrito_id, &metadata));

        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraBurrito = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson.burrito_type,
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
            win : extradatajson.win
        };

        burrito
    }

    //Minar un nuevo accesorio  
    #[payable]
    pub fn new_accessory(&mut self,accessory_id: TokenId,receiver_id: ValidAccountId,accessory_metadata: TokenMetadata) -> Accessory {
        self.accessories.mint(accessory_id, receiver_id, Some(accessory_metadata.clone()));
        self.n_accessories += 1;

        let newextradata = str::replace(&accessory_metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraAccessory = serde_json::from_str(&newextradata).unwrap();

        let accessory = Accessory {
            name : accessory_metadata.title.as_ref().unwrap().to_string(),
            description : accessory_metadata.description.as_ref().unwrap().to_string(),
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed
        };

        accessory
    }

    // Obtener accesorio
    pub fn get_accessory(&self, accessory_id: TokenId) -> Accessory {
        let metadata = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accessory_id))
            .unwrap();
        
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: ExtraAccessory = serde_json::from_str(&newextradata).unwrap();

        let accessory = Accessory {
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed,
        };

        accessory
    }

    // Pelear
    pub fn fight_burritos(&mut self, 
        burrito1_id: TokenId, accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId, 
        burrito2_id: TokenId, accesorio1_burrito2_id: TokenId, accesorio2_burrito2_id: TokenId, accesorio3_burrito2_id: TokenId) -> Burrito {

        // Obtener metadata burrito 1
        let mut metadata_burrito1 = self
            .burritos
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&burrito1_id))
            .unwrap();

        // Obtener metadata accesorio 1 burrito 1
        let mut metadata_accesorio1_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio1_burrito1_id))
            .unwrap();

        // Obtener metadata accesorio 2 burrito 1
        let mut metadata_accesorio2_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio2_burrito1_id))
            .unwrap();

        // Obtener metadata accesorio 3 burrito 1
        let mut metadata_accesorio3_burrito1 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio3_burrito1_id))
            .unwrap();

        // Obtener metadata burrito 2
        let mut metadata_burrito2 = self
            .burritos
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&burrito2_id))
            .unwrap();
        
        // Obtener metadata accesorio 1 burrito 2
        let mut metadata_accesorio1_burrito2 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio1_burrito2_id))
            .unwrap();

        // Obtener metadata accesorio 2 burrito 2
        let mut metadata_accesorio2_burrito2 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio2_burrito2_id))
            .unwrap();

        // Obtener metadata accesorio 3 burrito 2
        let mut metadata_accesorio3_burrito2 = self
            .accessories
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&accesorio3_burrito2_id))
            .unwrap();

        // Extraer extras del token burrito 1
        let newextradata_burrito1 = str::replace(&metadata_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Extraer extras del token accesorios burrito 1
        let newextradata_accesorio1_burrito1 = str::replace(&metadata_accesorio1_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio2_burrito1 = str::replace(&metadata_accesorio2_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio3_burrito1 = str::replace(&metadata_accesorio3_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Extraer extras del token burrito 2
        let newextradata_burrito2 = str::replace(&metadata_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Extraer extras del token accesorios burrito 2
        let newextradata_accesorio1_burrito2 = str::replace(&metadata_accesorio1_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio2_burrito2 = str::replace(&metadata_accesorio2_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_accesorio3_burrito2 = str::replace(&metadata_accesorio3_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");

        // Crear json burrito 1
        let mut extradatajson_burrito1: ExtraBurrito = serde_json::from_str(&newextradata_burrito1).unwrap();

        // Crear json accesorios burrito 1
        let mut extradatajson_accesorio1_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito1).unwrap();
        let mut extradatajson_accesorio2_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito1).unwrap();
        let mut extradatajson_accesorio3_burrito1: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito1).unwrap();

        // Crear json burrito 2
        let mut extradatajson_burrito2: ExtraBurrito = serde_json::from_str(&newextradata_burrito2).unwrap();

        // Crear json accesorios burrito 2
        let mut extradatajson_accesorio1_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio1_burrito2).unwrap();
        let mut extradatajson_accesorio2_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio2_burrito2).unwrap();
        let mut extradatajson_accesorio3_burrito2: ExtraAccessory = serde_json::from_str(&newextradata_accesorio3_burrito2).unwrap();

        // Obtener puntos totales a sumar de cada estadística de los accesorios del burrito 1
        let accesories_attack_burrito1 : f32 = (extradatajson_accesorio1_burrito1.attack.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.attack.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.attack.parse::<f32>().unwrap());
        let accesories_defense_burrito1 : f32 = (extradatajson_accesorio1_burrito1.defense.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.defense.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.defense.parse::<f32>().unwrap());
        let accesories_speed_burrito1 : f32 = (extradatajson_accesorio1_burrito1.speed.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito1.speed.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito1.speed.parse::<f32>().unwrap());

        // Obtener puntos totales a sumar de cada estadística de los accesorios del burrito 2
        let accesories_attack_burrito2 : f32 = (extradatajson_accesorio1_burrito2.attack.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito2.attack.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito2.attack.parse::<f32>().unwrap());
        let accesories_defense_burrito2 : f32 = (extradatajson_accesorio1_burrito2.defense.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito2.defense.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito2.defense.parse::<f32>().unwrap());
        let accesories_speed_burrito2 : f32 = (extradatajson_accesorio1_burrito2.speed.parse::<f32>().unwrap()+extradatajson_accesorio2_burrito2.speed.parse::<f32>().unwrap()+extradatajson_accesorio3_burrito2.speed.parse::<f32>().unwrap());

        // Crear estructura burrito 1
        let burrito1 = Burrito {
            name : metadata_burrito1.title.as_ref().unwrap().to_string(),
            description : metadata_burrito1.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson_burrito1.burrito_type.clone(),
            hp : extradatajson_burrito1.hp.clone(),
            attack : extradatajson_burrito1.attack.clone(),
            defense : extradatajson_burrito1.defense.clone(),
            speed : extradatajson_burrito1.speed.clone(),
            win : extradatajson_burrito1.win.clone()

        };

        // Crear estructura burrito 2
        let burrito2 = Burrito {
            name : metadata_burrito2.title.as_ref().unwrap().to_string(),
            description : metadata_burrito2.description.as_ref().unwrap().to_string(),
            burrito_type : extradatajson_burrito2.burrito_type.clone(),
            hp : extradatajson_burrito2.hp.clone(),
            attack : extradatajson_burrito2.attack.clone(),
            defense : extradatajson_burrito2.defense.clone(),
            speed : extradatajson_burrito2.speed.clone(),
            win : extradatajson_burrito2.win.clone()
        };

        // Validamos que ambos burritos tengan vidas para combatir
        assert!(burrito1.hp.parse::<u8>().unwrap() > 0, "{} no tiene vidas para combatir",metadata_burrito1.title.as_ref().unwrap().to_string());
        assert!(burrito2.hp.parse::<u8>().unwrap() > 0, "{} no tiene vidas para combatir",metadata_burrito2.title.as_ref().unwrap().to_string());

        let logname1 = format!("Nombre Burrito 1: {}", metadata_burrito1.title.as_ref().unwrap().to_string() );
        env::log(logname1.as_bytes());
        
        let logname2 = format!("Nombre Burrito 2: {}", metadata_burrito2.title.as_ref().unwrap().to_string() );
        env::log(logname2.as_bytes());

        // Variable que almacenará al ganador
        let burrito_winner : Burrito;

        //let burrito_winner : Burrito;
        let mut winner : i32 = 0;
        
        // Defensa total del burrito 1
        let mut old_defense_burrito1 = (burrito1.defense.parse::<f32>().unwrap()+accesories_defense_burrito1);
        
        // Defensa total del burrito 2
        let mut old_defense_burrito2 = (burrito2.defense.parse::<f32>().unwrap()+accesories_defense_burrito2);

        let mut rands1: u8 = 0;
        let mut rands2: u8 = 0;
        let mut randa1: u8 = 0;
        let mut randa2: u8 = 0;
        loop {
                // Generar números aleatorios para multiplicadores de velocidad y ataque
                rands1 = *env::random_seed().get(0).unwrap();
                rands2 = *env::random_seed().get(1).unwrap();
                randa1 = *env::random_seed().get(2).unwrap();
                randa2 = *env::random_seed().get(3).unwrap();

                let mut speed_mult1: f32 = 0.0;
                let mut speed_mult2: f32 = 0.0;
                let mut attack_mult1: f32 = 0.0;
                let mut attack_mult2: f32 = 0.0;
                let mut type_mult1: f32 = 0.0;
                let mut type_mult2: f32 = 0.0;

                if rands1 < 10 {
                    speed_mult1 = rands1 as f32 * 0.1;
                }
                if rands1 >= 10 && rands1 < 100 {
                    speed_mult1 = rands1 as f32 * 0.01;
                }
                if rands1 >= 100 && rands1 < 255 {
                    speed_mult1 = rands1 as f32 * 0.001;
                }
                if rands2 < 10 {
                    speed_mult2 = rands2 as f32 * 0.1;
                }
                if rands2 >= 10 && rands2 < 100 {
                    speed_mult2 = rands2 as f32 * 0.01;
                }
                if rands2 >= 100 && rands2 < 255 {
                    speed_mult2 = rands2 as f32 * 0.001;
                }
                if randa1 < 10 {
                    attack_mult1 = randa1 as f32 * 0.1;
                }
                if randa1 >= 10 && randa1 < 100 {
                    attack_mult1 = randa1 as f32 * 0.01;
                }
                if randa1 >= 100 && randa1 < 255 {
                    attack_mult1 = randa1 as f32 * 0.001;
                }
                if randa2 < 10 {
                    attack_mult2 = randa2 as f32 * 0.1;
                }
                if randa2 >= 10 && randa2 < 100 {
                    attack_mult2 = randa2 as f32 * 0.01;
                }
                if randa2 >= 100 && randa2 < 255 {
                    attack_mult2 = randa2 as f32 * 0.001;
                }

                // Verificar cuál burrito tiene mayor velocidad
                if ((burrito1.speed.parse::<f32>().unwrap()*speed_mult1)+accesories_speed_burrito1) > ((burrito2.speed.parse::<f32>().unwrap()*speed_mult2)+accesories_speed_burrito2) {
                    //Obtener multiplicador de tipo
                    if(burrito1.burrito_type == "Fuego" && burrito2.burrito_type == "Planta"){
                        type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito1.burrito_type == "Agua" && burrito2.burrito_type == "Fuego"){
                        type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito1.burrito_type == "Planta" && burrito2.burrito_type == "Eléctrico"){
                        type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito1.burrito_type == "Eléctrico" && burrito2.burrito_type == "Volador"){
                        type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito1.burrito_type == "Volador" && burrito2.burrito_type == "Agua"){
                        type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }

                    old_defense_burrito2 = old_defense_burrito2 - ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)+type_mult1+accesories_attack_burrito1);
                    type_mult1 = 0.0;
                    if old_defense_burrito2 < 0.0 {
                        winner = 1;
                    }
                    if winner == 0 {
                        //Obtener multiplicador de tipo
                        if(burrito2.burrito_type == "Fuego" && burrito1.burrito_type == "Planta"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Agua" && burrito1.burrito_type == "Fuego"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Planta" && burrito1.burrito_type == "Eléctrico"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Eléctrico" && burrito1.burrito_type == "Volador"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito2.burrito_type == "Volador" && burrito1.burrito_type == "Agua"){
                            type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        
                        old_defense_burrito1 = old_defense_burrito1 - ((burrito2.attack.parse::<f32>().unwrap()*attack_mult2)+type_mult2+accesories_attack_burrito2);
                        type_mult2 = 0.0;
                        if old_defense_burrito1 < 0.0 {
                            winner = 2;
                        }
                    }
                } else {
                    //Obtener multiplicador de tipo
                    if(burrito2.burrito_type == "Fuego" && burrito1.burrito_type == "Planta"){
                        type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito2.burrito_type == "Agua" && burrito1.burrito_type == "Fuego"){
                        type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito2.burrito_type == "Planta" && burrito1.burrito_type == "Eléctrico"){
                        type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito2.burrito_type == "Eléctrico" && burrito1.burrito_type == "Volador"){
                        type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }
                    if(burrito2.burrito_type == "Volador" && burrito1.burrito_type == "Agua"){
                        type_mult2 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                    }

                    old_defense_burrito1 = old_defense_burrito1 - ((burrito2.attack.parse::<f32>().unwrap()*attack_mult2)+type_mult2+accesories_attack_burrito2);
                    type_mult2 = 0.0;
                    if old_defense_burrito1 < 0.0 {
                        winner = 2;
                    }
                    if winner == 0 {
                        //Obtener multiplicador de tipo
                        if(burrito1.burrito_type == "Fuego" && burrito2.burrito_type == "Planta"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Agua" && burrito2.burrito_type == "Fuego"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Planta" && burrito2.burrito_type == "Eléctrico"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Eléctrico" && burrito2.burrito_type == "Volador"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }
                        if(burrito1.burrito_type == "Volador" && burrito2.burrito_type == "Agua"){
                            type_mult1 = ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)*0.25)
                        }

                        old_defense_burrito2 = old_defense_burrito2 - ((burrito1.attack.parse::<f32>().unwrap()*attack_mult1)+type_mult1+accesories_attack_burrito1);
                        type_mult1 = 0.0;
                        if old_defense_burrito2 < 0.0 {
                            winner = 1;
                        }
                    }
                }

                if winner != 0 {
                    break;
                }
        }

        if winner == 1 {
            burrito_winner = burrito1;

            let new_hp_burrito2 = burrito2.hp.parse::<u8>().unwrap()-1;
            extradatajson_burrito2.hp = new_hp_burrito2.to_string();

            let mut extra_string_burrito2 = serde_json::to_string(&extradatajson_burrito2).unwrap();
            extra_string_burrito2 = str::replace(&extra_string_burrito2, "\"", "'");
            metadata_burrito2.extra = Some(extra_string_burrito2.clone());

            self.burritos
                .token_metadata_by_id
                .as_mut()
                .and_then(|by_id| by_id.insert(&burrito2_id, &metadata_burrito2));

            let new_win_burrito1 = burrito_winner.win.parse::<u8>().unwrap()+1;
            extradatajson_burrito1.win = new_win_burrito1.to_string();

            let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
            extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
            metadata_burrito1.extra = Some(extra_string_burrito1.clone());

            self.burritos
                .token_metadata_by_id
                .as_mut()
                .and_then(|by_id| by_id.insert(&burrito1_id, &metadata_burrito1));
        } else {
            burrito_winner = burrito2;

            let new_hp_burrito1 = burrito1.hp.parse::<u8>().unwrap()-1;
            extradatajson_burrito1.hp = new_hp_burrito1.to_string();

            let mut extra_string_burrito1 = serde_json::to_string(&extradatajson_burrito1).unwrap();
            extra_string_burrito1 = str::replace(&extra_string_burrito1, "\"", "'");
            metadata_burrito1.extra = Some(extra_string_burrito1.clone());

            self.burritos
                .token_metadata_by_id
                .as_mut()
                .and_then(|by_id| by_id.insert(&burrito1_id, &metadata_burrito1));

            let new_win_burrito2 = burrito_winner.win.parse::<u8>().unwrap()+1;
            extradatajson_burrito2.win = new_win_burrito2.to_string();

            let mut extra_string_burrito2 = serde_json::to_string(&extradatajson_burrito2).unwrap();
            extra_string_burrito2 = str::replace(&extra_string_burrito2, "\"", "'");
            metadata_burrito2.extra = Some(extra_string_burrito2.clone());

            self.burritos
                .token_metadata_by_id
                .as_mut()
                .and_then(|by_id| by_id.insert(&burrito2_id, &metadata_burrito2));
        }

        burrito_winner
    }

}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
