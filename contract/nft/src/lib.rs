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
    metadata: LazyOption<NFTContractMetadata>,
    n_tokens: u64,
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 288 288'%3E%3Cg id='l' data-name='l'%3E%3Cpath d='M187.58,79.81l-30.1,44.69a3.2,3.2,0,0,0,4.75,4.2L191.86,103a1.2,1.2,0,0,1,2,.91v80.46a1.2,1.2,0,0,1-2.12.77L102.18,77.93A15.35,15.35,0,0,0,90.47,72.5H87.34A15.34,15.34,0,0,0,72,87.84V201.16A15.34,15.34,0,0,0,87.34,216.5h0a15.35,15.35,0,0,0,13.08-7.31l30.1-44.69a3.2,3.2,0,0,0-4.75-4.2L96.14,186a1.2,1.2,0,0,1-2-.91V104.61a1.2,1.2,0,0,1,2.12-.77l89.55,107.23a15.35,15.35,0,0,0,11.71,5.43h3.13A15.34,15.34,0,0,0,216,201.16V87.84A15.34,15.34,0,0,0,200.66,72.5h0A15.35,15.35,0,0,0,187.58,79.81Z'/%3E%3C/g%3E%3C/svg%3E";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Burrito {
    name : String,
    description : String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Extras {
    hp : String,
    attack : String,
    defense : String,
    speed : String,
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
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            n_tokens: 0
        }
    }

    // Obtener cantidad de tokens creaos
    pub fn get_number_burritos(&self) -> u64 {
        self.n_tokens
    }

    // Obtener burrito
    pub fn get_burrito(&self, token_id: TokenId) -> Burrito {
        let metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        
        let newextradata = str::replace(&metadata.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson: Extras = serde_json::from_str(&newextradata).unwrap();

        let burrito = Burrito {
            name : metadata.title.as_ref().unwrap().to_string(),
            description : metadata.description.as_ref().unwrap().to_string(),
            hp : extradatajson.hp,
            attack : extradatajson.attack,
            defense : extradatajson.defense,
            speed : extradatajson.speed
        };

        burrito

    }

    // Modificar burrito
    pub fn update_burrito(&mut self, token_id: TokenId, extra: String) -> TokenMetadata {
        let mut metadata = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id))
            .unwrap();
        
        metadata.extra = Some(extra);

        self.tokens
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.insert(&token_id, &metadata));

        metadata
    }

    // Minar un nuevo token
    #[payable]
    pub fn new_burrito(
        &mut self,
        token_id: TokenId,
        receiver_id: ValidAccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.n_tokens += 1;
        self.tokens.mint(token_id, receiver_id, Some(token_metadata))
    }

    // Pelear
    pub fn fight_burritos(&mut self, token_id_burrito1: TokenId, token_id_burrito2: TokenId) -> Burrito {
        // Obtener metadata burrito 1
        let metadata_burrito1 = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id_burrito1))
            .unwrap();

        // Obtener metadata burrito 2
        let metadata_burrito2 = self
            .tokens
            .token_metadata_by_id
            .as_ref()
            .and_then(|by_id| by_id.get(&token_id_burrito2))
            .unwrap();
        
        // Crear json
        let newextradata_burrito1 = str::replace(&metadata_burrito1.extra.as_ref().unwrap().to_string(), "'", "\"");
        let newextradata_burrito2 = str::replace(&metadata_burrito2.extra.as_ref().unwrap().to_string(), "'", "\"");
        let extradatajson_burrito1: Extras = serde_json::from_str(&newextradata_burrito1).unwrap();
        let extradatajson_burrito2: Extras = serde_json::from_str(&newextradata_burrito2).unwrap();

        // Crear estructura burrito 1
        let burrito1 = Burrito {
            name : metadata_burrito1.title.as_ref().unwrap().to_string(),
            description : metadata_burrito1.description.as_ref().unwrap().to_string(),
            hp : extradatajson_burrito1.hp,
            attack : extradatajson_burrito1.attack,
            defense : extradatajson_burrito1.defense,
            speed : extradatajson_burrito1.speed
        };

        // Crear estructura burrito 2
        let burrito2 = Burrito {
            name : metadata_burrito2.title.as_ref().unwrap().to_string(),
            description : metadata_burrito2.description.as_ref().unwrap().to_string(),
            hp : extradatajson_burrito2.hp,
            attack : extradatajson_burrito2.attack,
            defense : extradatajson_burrito2.defense,
            speed : extradatajson_burrito2.speed
        };

        let logname1 = format!("Nombre Burrito 1: {}", metadata_burrito1.title.as_ref().unwrap().to_string() );
        env::log(logname1.as_bytes());
        
        let logname2 = format!("Nombre Burrito 2: {}", metadata_burrito2.title.as_ref().unwrap().to_string() );
        env::log(logname2.as_bytes());

        // Variable que almacenará al ganador
        let burrito_winner : Burrito;
        let mut winner : i32 = 0;
        let mut old_defense_burrito1 = burrito1.defense.parse::<f32>().unwrap();
        let mut old_defense_burrito2 = burrito2.defense.parse::<f32>().unwrap();
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
                if (burrito1.speed.parse::<f32>().unwrap()*speed_mult1) > (burrito2.speed.parse::<f32>().unwrap()*speed_mult2) {
                    // let attackb1 = format!("Ataque Burrito 1: {}", (burrito1.attack.parse::<f32>().unwrap()*attack_mult1).to_string() );
                    // env::log(attackb1.as_bytes());
                    old_defense_burrito2 = old_defense_burrito2 - (burrito1.attack.parse::<f32>().unwrap()*attack_mult1);
                    // let defenserb2 = format!("Defensa Restante Burrito 2: {}", old_defense_burrito2.to_string() );
                    // env::log(defenserb2.as_bytes());
                    if old_defense_burrito2 < 0.0 {
                        winner = 1;
                    }
                    if winner == 0 {
                        // let attackb2 = format!("Ataque Burrito 2: {}", (burrito2.attack.parse::<f32>().unwrap()*attack_mult2).to_string() );
                        // env::log(attackb2.as_bytes());
                        old_defense_burrito1 = old_defense_burrito1 - (burrito2.attack.parse::<f32>().unwrap()*attack_mult2);
                        // let defenserb1 = format!("Defensa Restante Burrito 1: {}", old_defense_burrito1.to_string() );
                        // env::log(defenserb1.as_bytes());
                        if old_defense_burrito1 < 0.0 {
                            winner = 2;
                        }
                    }
                } else {
                    // let attackb2 = format!("Ataque Burrito 2: {}", (burrito2.attack.parse::<f32>().unwrap()*attack_mult2).to_string() );
                    // env::log(attackb2.as_bytes());
                    old_defense_burrito1 = old_defense_burrito1 - (burrito2.attack.parse::<f32>().unwrap()*attack_mult2);
                    // let defenserb1 = format!("Defensa Restante Burrito 1: {}", old_defense_burrito1.to_string() );
                    // env::log(defenserb1.as_bytes());
                    if old_defense_burrito1 < 0.0 {
                        winner = 2;
                    }
                    if winner == 0 {
                        // let attackb1 = format!("Ataque Burrito 1: {}", (burrito1.attack.parse::<f32>().unwrap()*attack_mult1).to_string() );
                        // env::log(attackb1.as_bytes());
                        old_defense_burrito2 = old_defense_burrito2 - (burrito1.attack.parse::<f32>().unwrap()*attack_mult1);
                        // let defenserb2 = format!("Defensa Restante Burrito 2: {}", old_defense_burrito2.to_string() );
                        // env::log(defenserb2.as_bytes());
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
            burrito_winner = burrito1
        } else {
            burrito_winner = burrito2
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
