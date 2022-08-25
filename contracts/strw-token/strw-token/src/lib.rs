use near_sdk::collections::LookupMap;
use near_contract_standards::fungible_token::{
    core::FungibleTokenCore,
    metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC},
    resolver::FungibleTokenResolver,
};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{
    assert_one_yocto, env, ext_contract, log, near_bindgen, AccountId, Balance, Gas,
    Promise, PanicOnDefault, PromiseOrValue, serde_json::json
};
use std::str;

//-- Sputnik DAO remote upgrade requires BLOCKCHAIN_INTERFACE low-level access
#[cfg(target_arch = "wasm32")]
use near_sdk::env::BLOCKCHAIN_INTERFACE;

const TGAS: Gas = 1_000_000_000_000;
const GAS_FOR_RESOLVE_TRANSFER: Gas = 5 * TGAS;
const GAS_FOR_FT_TRANSFER_CALL: Gas = 25 * TGAS;
const NO_DEPOSIT: Balance = 0;
pub const ICON: &str = "data:image/jpeg;base64,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";

// nanoseconds in a second
const NANOSECONDS: u64 = 1_000_000_000;

type U128String = U128;

near_sdk::setup_alloc!();

mod internal;
mod migrations;
mod storage_nep_145;
mod util;
mod vesting;

use util::*;
use vesting::{VestingRecord, VestingRecordJSON};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct MetaToken {
    metadata: LazyOption<FungibleTokenMetadata>,

    pub accounts: LookupMap<AccountId, Balance>,

    pub owner_id: AccountId,

    pub minters: Vec<AccountId>,

    pub total_supply: Balance,

    /// transfers are locked until this moment
    pub locked_until_nano: TimestampNano,

    pub vested: LookupMap<AccountId, VestingRecord>,
    pub vested_count: u32, 
    pub treasury_id: AccountId,
    pub strw_mint_cost: u128,
    pub strw_reset_cost: u128,
    pub strw_evolve_cost: u128,
    pub buyers: LookupMap<AccountId, String>,

}

#[near_bindgen]
impl MetaToken {
    /// Initializes the contract with the given total supply owned by the given `owner_id`.
    #[init]
    pub fn init_contract(owner_id: AccountId, treasury_id: AccountId, strw_mint_cost: u128, strw_reset_cost: u128, strw_evolve_cost: u128) -> Self {
        //validate default metadata
        // internal::default_ft_metadata().assert_valid();
        Self {
            owner_id: owner_id.clone(),
            metadata: LazyOption::new(b"m".to_vec(), None),
            accounts: LookupMap::new(b"a".to_vec()),
            minters: vec![owner_id],
            total_supply: 0,
            locked_until_nano: 0,
            vested: LookupMap::new(b"v".to_vec()),
            vested_count: 0,
            treasury_id : treasury_id,
            strw_mint_cost: strw_mint_cost,
            strw_reset_cost: strw_reset_cost,
            strw_evolve_cost: strw_evolve_cost,
            buyers  : LookupMap::new(b"v".to_vec())      
        }
    }

    // Obtener dueño del contrato
    pub fn get_owner_id(&self) -> AccountId {
        return self.owner_id.clone();
    }

    // Cambiar metadata del FT
    #[payable]
    pub fn set_meta(&mut self) -> String {
        self.assert_owner_calling();
        let mut m = self.internal_get_ft_metadata();
        m.name = "Straw Token".to_string();
        m.symbol = "STRW".to_string();
        m.icon = Some(ICON.to_string());
        self.metadata.set(&m);
        "Metadata cambiada".to_string()
    }

    // Cambiar tesorero
    pub fn set_treasury(&mut self, new_treasury: AccountId) -> String {
        self.assert_owner_calling();
        self.treasury_id = new_treasury;
        "Tesorero actualizado".to_string()
    }
    
    // Cambiar dueño del contrato
    pub fn set_owner_id(&mut self, owner_id: AccountId) {
        self.assert_owner_calling();
        assert!(env::is_valid_account_id(owner_id.as_bytes()));
        self.owner_id = owner_id.into();
    }
    
    pub fn set_locked_until(&mut self, unix_timestamp: u32) {
        self.assert_owner_calling();
        self.locked_until_nano = unix_timestamp as u64 * NANOSECONDS;
    }

    // Minar tokens
    pub fn mint(&mut self, account_id: &AccountId, amount: U128String) {
        self.assert_minter(env::predecessor_account_id());
        self.mint_into(account_id, amount.0);
    }

    // Comprar tokens
    #[payable]
    pub fn buy_tokens(&mut self) -> f32 {
        let account_id = env::signer_account_id();

        // Obtener epoca actual
        let block_timestamp = env::block_timestamp();
        // Obtener epoca de la ultima compra del usuario
        let old_timestamp_buyer = self.buyers.get(&account_id);

        // Obtener Nears
        let deposit = env::attached_deposit();

        if deposit < 1000000000000000000000000 {
            env::panic(
                format!("NEARS Insuficientes, esta operación cuesta 1 NEAR").as_bytes(),
            );
        }

        if old_timestamp_buyer.is_none() {
            self.buyers.insert(&account_id.clone(),&block_timestamp.clone().to_string());
        } else {
            // Verificar que la epoca actual sea diferente a la ultima donde realizó una compra
            if block_timestamp <= (old_timestamp_buyer.clone().unwrap().parse::<u64>().unwrap() + 43200000000000) {
                env::panic(
                    format!("Ya realizaste una compra en ésta época, debes esperar a la siguiente").as_bytes(),
                );
            } else {
                self.buyers.insert(&account_id.clone(),&block_timestamp.clone().to_string());
            }
        }

        Promise::new(self.treasury_id.clone()).transfer(deposit as u128);

        // Cantidad de STRW Tokens a minar
        let mut strw_to_mint = 0;

        // Generar cantidad de STRW Tokens a minar entre 1,000 y 10,000
        let random_number = *env::random_seed().get(0).unwrap();

        let mut tokens_mint : f32 = 0.0;

        if random_number <= 25 {
            tokens_mint = 1000.0
        }
        if random_number > 25 && random_number < 254 {
            tokens_mint = random_number as f32*39.0
        }
        if random_number >= 254 {
            tokens_mint = 10000.0
        }

        let tokens_to_mint = tokens_mint*1000000000000000000000000.0;
        log!("{}",tokens_mint);

        // Minar tokens
        self.mint_into(&account_id.clone(), tokens_to_mint as u128);
        //tokens_to_mint.to_string()

        tokens_mint
    }

    pub fn can_buy_tokens(&self, account_id : AccountId) -> String {
        let account_id = account_id.clone();

        // Obtener epoca actual
        let block_timestamp = env::block_timestamp();
        // Obtener epoca de la ultima compra del usuario
        let old_timestamp_buyer = self.buyers.get(&account_id);

        if old_timestamp_buyer.is_none() {
            return "0".to_string();
        } else {
            // Verificar que la epoca actual sea diferente a la ultima donde realizó una compra
            if block_timestamp <= (old_timestamp_buyer.clone().unwrap().parse::<u64>().unwrap() + 43200000000000) {
                let finish_epoch = (old_timestamp_buyer.clone().unwrap().parse::<u64>().unwrap() + 43200000000000);
                return finish_epoch.to_string();
            } else {
                return "0".to_string();
            }
        }
    }
    
    // Otorgar recompensas a jugador
    pub fn reward_player(&mut self, player_owner_id: ValidAccountId, tokens_mint: U128String) {
        self.assert_minter(env::predecessor_account_id());

        let account_id: AccountId = player_owner_id.clone().into();

        self.mint_into(&account_id.clone(), tokens_mint.clone().0);

        let sender_id = env::predecessor_account_id();
        let receiver_id: ValidAccountId = player_owner_id;
        let amount : u128 = 0;
        let memo : Option<String> = Some(String::from("".to_string())) ;     

        self.internal_transfer(&sender_id, receiver_id.as_ref(), amount, memo);

    }

    #[payable]
    pub fn get_balance_and_transfer(&mut self, account_id: AccountId, action: String) -> bool {
        self.assert_minter(env::predecessor_account_id());

        // Obtener STRW Tokens del jugador
        let balance : u128 = self.accounts.get(&account_id).unwrap_or(0).into();
        // Obtener Nears
        let deposit = env::attached_deposit();
        Promise::new(self.treasury_id.clone()).transfer(deposit as u128);

        log!("{}",action);

        // Costo de STRW
        let mut strw_cost = 0;

        // Tipo de operación
        if action == "Mint" {
            if deposit < 4900000000000000000000000 {
                env::panic(
                    format!("NEARS Insuficientes, enviaste {} y necesitas 5000000000000000000000000", &deposit).as_bytes(),
                );
            }
            // 50,000 STRW
            if balance < self.strw_mint_cost*1000000000000000000000000 {
                env::panic(
                    format!("STRW Tokens Insuficientes, tienes {} y necesitas 50000000000000000000000000000", &balance).as_bytes(),
                );
            }
            strw_cost = self.strw_mint_cost*1000000000000000000000000;
        }
        if action == "Reset" {
            if deposit < 1000000000000000000000000 {
                env::panic(
                    format!("NEARS Insuficientes, enviaste {} y necesitas 1000000000000000000000000", &deposit).as_bytes(),
                );
            }
            // 30,000 STRW
            if balance < self.strw_reset_cost*1000000000000000000000000 {
                env::panic(
                    format!("STRW Tokens Insuficientes, tienes {} y necesitas 30000000000000000000000000000", &balance).as_bytes(),
                );
            }
            strw_cost = self.strw_reset_cost*1000000000000000000000000;
        }
        if action == "Evolve" {
            if deposit < 2000000000000000000000000 {
                env::panic(
                    format!("NEARS Insuficientes, enviaste {} y necesitas 2000000000000000000000000", &deposit).as_bytes(),
                );
            }
            // 70,000 STRW
            if balance < self.strw_evolve_cost*1000000000000000000000000 {
                env::panic(
                    format!("STRW Tokens Insuficientes, tienes {} y necesitas 70000000000000000000000000000", &balance).as_bytes(),
                );
            }
            strw_cost = self.strw_evolve_cost*1000000000000000000000000;

        }
        let receiver_id = self.treasury_id.clone();
        let memo : Option<String> = Some(String::from("".to_string())) ;     
        self.internal_transfer(&account_id, &receiver_id, (strw_cost/100)*5, memo);
        self.internal_burn(&account_id, (strw_cost/100)*95);
        true
    }

    #[payable]
    pub fn get_balance_and_transfer_minigames(&mut self, account_id: AccountId, action: String, treasury_id: ValidAccountId) -> bool {
        self.assert_minter(env::predecessor_account_id());

        // Obtener STRW Tokens del jugador
        let balance : u128 = self.accounts.get(&account_id).unwrap_or(0).into();
        
        // Costo de STRW
        let mut strw_cost = 0;
        let memo : Option<String> = Some(String::from("".to_string())) ;     
        let receiver_id: ValidAccountId = treasury_id;

        // Tipo de operación
        // 10,000
        if action == "Incursion" {
            if balance < 10000000000000000000000000000 {
                log!("STRW Tokens Insuficientes, tienes {} y necesitas 10000000000000000000000000000");
                return false;
            }
            strw_cost = 10000000000000000000000000000;
        }
        
        self.internal_transfer(&account_id, receiver_id.as_ref(), (strw_cost/100)*5, memo);
        self.internal_burn(&account_id, (strw_cost/100)*95);
        return true;
    }

    #[payable]
    pub fn get_balance_and_transfer_hospital(&mut self, account_id: AccountId, action: String, treasury_id: ValidAccountId, cost: u128) -> bool {
        self.assert_minter(env::predecessor_account_id());

        // Obtener STRW Tokens del jugador
        let balance : u128 = self.accounts.get(&account_id).unwrap_or(0).into();
        
        // Costo de STRW
        let mut strw_cost = 0;
        let memo : Option<String> = Some(String::from("".to_string())) ;     
        let receiver_id: ValidAccountId = treasury_id;

        // Tipo de operación
        if action == "Capsule" {
            if balance < cost*1000000000000000000000000 {
                log!("STRW Tokens Insuficientes");
                return false;
            }
            strw_cost = cost*1000000000000000000000000;
        }
        self.internal_transfer(&account_id, receiver_id.as_ref(), (strw_cost/100)*5, memo);
        self.internal_burn(&account_id, (strw_cost/100)*95);
        return true;
    }

    // Obtener costos de STRW Tokens
    pub fn get_costs(&self) {
        log!("Treasury Account: {}",self.treasury_id);
        log!("STRW Mint Cost: {}",self.strw_mint_cost);
        log!("STRW Reset Cost: {}",self.strw_reset_cost);
        log!("STRW Evolve Cost: {}",self.strw_evolve_cost);
    }

    // Cambiar costos de STRW Tokens
    pub fn set_costs(&mut self, strw_mint_cost: u128, strw_reset_cost: u128, strw_evolve_cost: u128) {
        self.assert_owner_calling();
        self.strw_mint_cost = strw_mint_cost;
        self.strw_reset_cost = strw_reset_cost;
        self.strw_evolve_cost = strw_evolve_cost;
    }
        
    // Agregar nuevo minero
    pub fn add_minter(&mut self, account_id: AccountId) -> String {
        self.assert_owner_calling();
        if let Some(_) = self.minters.iter().position(|x| *x == account_id) {
            //found
            panic!("already in the list");
        }
        self.minters.push(account_id);
        "Minero agregado".to_string()
    }

    // Remover minero
    pub fn remove_minter(&mut self, account_id: &AccountId) -> String {
        self.assert_owner_calling();
        if let Some(inx) = self.minters.iter().position(|x| x == account_id) {
            //found
            let _removed = self.minters.swap_remove(inx);
        } else {
            panic!("not a minter")
        }
        "Minero removido".to_string()
    }

    // Consultar lista de mineros
    pub fn get_minters(self) -> Vec<AccountId> {
        self.minters
    }

    /// sets metadata_reference
    #[payable]
    pub fn set_metadata_reference(&mut self, reference: String, reference_hash: String) {
        assert_one_yocto();
        self.assert_owner_calling();
        let mut m = self.internal_get_ft_metadata();
        m.reference = Some(reference);
        m.reference_hash = Some(reference_hash.as_bytes().to_vec().into());
        m.assert_valid();
        self.metadata.set(&m);
    }

    //-----------
    //-- Vesting functions in the contract
    //-----------
    /// Get the amount of tokens that are locked in this account due to lockup or vesting.
    pub fn get_locked_amount(&self, account: AccountId) -> U128String {
        match self.vested.get(&account) {
            Some(vesting) => vesting.compute_amount_locked().into(),
            None => 0.into(),
        }
    }

    /// Get vesting information
    pub fn get_vesting_info(&self, account_id: AccountId) -> VestingRecordJSON {
        match self.vested.get(&account_id) {
            Some(vesting) => {
                log!("{}", &account_id);
                return VestingRecordJSON {
                    amount: vesting.amount.into(),
                    locked: vesting.compute_amount_locked().into(),
                    locked_until_timestamp: (vesting.locked_until_timestamp_nano / NANOSECONDS)
                        as u32,
                    linear_start_timestamp: (vesting.linear_start_timestamp_nano / NANOSECONDS)
                        as u32,
                    linear_end_timestamp: (vesting.linear_end_timestamp_nano / NANOSECONDS) as u32,
                }
            }
            _ => panic!("no vesting for account {}", account_id),
        };
    }

    //minters can mint with vesting/locked periods
    #[payable]
    pub fn mint_vested(
        &mut self,
        account_id: &AccountId,
        amount: U128String,
        locked_until_timestamp: u64,
        linear_start_timestamp: u64,
        linear_end_timestamp: u64,
    ) {
        self.mint(account_id, amount);
        let record = VestingRecord::new(
            amount.into(),
            locked_until_timestamp as u64 * NANOSECONDS,
            linear_start_timestamp as u64 * NANOSECONDS,
            linear_end_timestamp as u64 * NANOSECONDS,
        );
        match self.vested.insert(&account_id, &record) {
            Some(previous) => {
                if previous.compute_amount_locked()>0 {
                    panic!("account already vested with locked amount")
                }
            },
            None => self.vested_count += 1,
        }
    }

    #[payable]
    /// terminate vesting before is over
    /// burn the tokens
    pub fn terminate_vesting(&mut self, account_id: &AccountId) {
        assert_one_yocto();
        self.assert_owner_calling();
        match self.vested.get(&account_id) {
            Some(vesting) => {
                let locked_amount = vesting.compute_amount_locked();
                if locked_amount == 0 {
                    panic!("locked_amount is zero")
                }
                self.internal_burn(account_id, locked_amount);
                self.vested.remove(&account_id);
                self.vested_count -= 1;
                log!(
                    "{} vesting terminated, {} burned",
                    account_id,
                    locked_amount
                );
            }
            None => panic!("account not vested"),
        }
    }

    /// return how many vested accounts are still active
    pub fn vested_accounts_count(&self) -> u32 {
        self.vested_count
    }

    //---------------------------------------------------------------------------
    /// Sputnik DAO remote-upgrade receiver
    /// can be called by a remote-upgrade proposal
    ///
    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        assert!(env::predecessor_account_id() == self.owner_id);
        //input is code:<Vec<u8> on REGISTER 0
        //log!("bytes.length {}", code.unwrap().len());
        const GAS_FOR_UPGRADE: u64 = 10 * TGAS; //gas occupied by this fn
        const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id().into_bytes();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - GAS_FOR_UPGRADE;
        unsafe {
            BLOCKCHAIN_INTERFACE.with(|b| {
                // Load input (new contract code) into register 0
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .input(0);

                //prepare self-call promise
                let promise_id = b
                    .borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_create(current_id.len() as _, current_id.as_ptr() as _);

                //1st action, deploy/upgrade code (takes code from register 0)
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

                //2nd action, schedule a call to "migrate()".
                //Will execute on the **new code**
                b.borrow()
                    .as_ref()
                    .expect(BLOCKCHAIN_INTERFACE_NOT_SET_ERR)
                    .promise_batch_action_function_call(
                        promise_id,
                        migrate_method_name.len() as _,
                        migrate_method_name.as_ptr() as _,
                        0 as _,
                        0 as _,
                        0 as _,
                        attached_gas,
                    );
            });
        }
    }
}

//----------------------------------------------
// ft metadata standard
// Q: Is ignoring storage costs the only reason for the re-implementation?
// A: making the user manage storage costs adds too much friction to account creation
// it's better to impede sybil attacks by other means
#[near_bindgen]
impl FungibleTokenCore for MetaToken {
    fn ft_transfer(&mut self, receiver_id: ValidAccountId, amount: U128, memo: Option<String>) {
        let sender_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        self.internal_transfer(&sender_id, receiver_id.as_ref(), amount, memo);
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: ValidAccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert_one_yocto();
        let sender_id = env::predecessor_account_id();
        let amount: Balance = amount.into();
        self.internal_transfer(&sender_id, receiver_id.as_ref(), amount, memo);
        // Initiating receiver's call and the callback
        // ext_fungible_token_receiver::ft_on_transfer(
        ext_ft_receiver::ft_on_transfer(
            sender_id.clone(),
            amount.into(),
            msg,
            receiver_id.as_ref(),
            NO_DEPOSIT,
            env::prepaid_gas() - GAS_FOR_FT_TRANSFER_CALL - GAS_FOR_RESOLVE_TRANSFER, // assign rest of gas to callback
        )
        .then(ext_self::ft_resolve_transfer(
            sender_id,
            receiver_id.into(),
            amount.into(),
            &env::current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_RESOLVE_TRANSFER,
        ))
        .into()
    }

    fn ft_total_supply(&self) -> U128 {
        self.total_supply.into()
    }

    fn ft_balance_of(&self, account_id: ValidAccountId) -> U128 {
        self.accounts.get(account_id.as_ref()).unwrap_or(0).into()
    }
}

#[near_bindgen]
impl FungibleTokenResolver for MetaToken {
    /// Returns the amount of burned tokens in a corner case when the sender
    /// has deleted (unregistered) their account while the `ft_transfer_call` was still in flight.
    /// Returns (Used token amount, Burned token amount)
    #[private]
    fn ft_resolve_transfer(
        &mut self,
        sender_id: ValidAccountId,
        receiver_id: ValidAccountId,
        amount: U128,
    ) -> U128 {
        let sender_id: AccountId = sender_id.into();
        let (used_amount, burned_amount) =
            self.int_ft_resolve_transfer(&sender_id, receiver_id, amount);
        if burned_amount > 0 {
            log!("{} tokens burned", burned_amount);
        }
        return used_amount.into();
    }
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for MetaToken {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.internal_get_ft_metadata()
    }
}

#[ext_contract(ext_ft_receiver)]
pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_self)]
trait FungibleTokenResolver {
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128;
}
