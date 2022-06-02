use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
    PromiseResult, Gas, require, BorshStorageKey
};

use crate::internal::*;

pub use crate::burritos::*;
pub use crate::evolve::*;
pub use crate::reset_conditions::*;

pub use crate::metadata::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::events::*;
pub use crate::migrate::*;
pub use crate::whitelist::*;

mod internal;

mod burritos;
mod evolve;
mod reset_conditions;
mod whitelist;

mod approval; 
mod enumeration; 
mod metadata; 
mod nft_core; 
mod royalty; 
mod events;
mod migrate;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

pub const BURRITO_CONTRACT: &str = "dev-1652924595303-59024384289373";
pub const ITEMS_CONTRACT: &str = "dev-1647986467816-61735125036881";
pub const STRWTOKEN_CONTRACT: &str = "dev-1653415145729-47929415561597";
pub const PVE_CONTRACT: &str = "dev-1652376335913-86387308955071";

pub const BURRITO1: &str = "QmULzZNvTGrRxEMvFVYPf1qaBc4tQtz6c3MVGgRNx36gAq";
pub const BURRITO2: &str = "QmZEK32JEbJH3rQtXL9BqQJa2omXfpjuXGjbFXLiV2Ge9D";
pub const BURRITO3: &str = "QmQcTRnmdFhWa1j47JZAxr5CT1Cdr5AfqdhnrGpSdr28t6";
pub const BURRITO4: &str = "QmbMS3P3gn2yivKDFvHSxYjVZEZrBdxyZtnnnJ62tVuSVk";

pub const ICON: &str = "data:image/jpeg;base64,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";

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Burrito {
    // token_id : String,
    owner_id : String,
    name : String,
    description : String,
    burrito_type : String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String,
    global_win : String,
    level : String,
    media : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ExtraBurrito {
    burrito_type: String,
    hp : String,
    attack : String,
    defense : String,
    speed : String,
    win : String,
    global_win : String,
    level : String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ExternalContract {
    register_address: AccountId,
    contract_name: String
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldContract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    pub whitelist_contracts: LookupMap<AccountId, ExternalContract>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    //contract owner
    pub owner_id: AccountId,

    //keeps track of all the token IDs for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,

    //keeps track of the token struct for a given token ID
    pub tokens_by_id: LookupMap<TokenId, Token>,

    //keeps track of the token metadata for a given token ID
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,

    //keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,

    pub whitelist_contracts: LookupMap<AccountId, ExternalContract>

}

/// Helper structure for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensById,
    TokenMetadataById,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
    ContractAllowed
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn init_contract(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Burritos (Burrito Battle)".to_string(),
                symbol: "BurritoBattle".to_string(),
                icon: Some(ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        //create a variable of type Self with all the fields initialized. 
        let this = Self {
            //Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            //set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),

            whitelist_contracts: LookupMap::new(StorageKey::ContractAllowed)
        };

        //return the Contract object
        this
    }

    pub fn update_metadata_icon(&mut self, icon: String) {
        self.assert_owner();
        let mut metadata = self.metadata.get().unwrap();
        metadata.icon = Some(icon);
        self.metadata.set(&metadata);
    }

    fn assert_owner(&self) {
        require!(self.signer_is_owner(), "Method is private to owner")
    }

    fn signer_is_owner(&self) -> bool {
        self.is_owner(&env::signer_account_id())
    }

    fn is_owner(&self, minter: &AccountId) -> bool {
        minter.as_str() == self.owner_id.as_str()
    }

}