use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{AccountId, Balance, PromiseResult};

use crate::*;

const ONE_NEAR: Balance = 1_000_000_000_000_000_000_000_000;
pub const MIN_TRANSFER_UNIT: u128 = 1000; // to make sibyl attacks more expensive in terms of tokens
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/jpeg;base64,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";


pub fn default_ft_metadata() -> FungibleTokenMetadata {
    FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "Straw Token".to_string(),
        symbol: "STRW".to_string(),
        icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
        reference: Some("https://metapool.app".into()),
        reference_hash: None,
        decimals: 24,
    }
}

impl MetaToken {
    pub fn assert_owner_calling(&self) {
        assert!(
            env::predecessor_account_id() == self.owner_id,
            "can only be called by the owner"
        );
    }

    pub fn assert_minter(&self, account_id: String) {
        assert!(self.minters.contains(&account_id), "not a minter");
    }

    //get stored metadata or default
    pub fn internal_get_ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap_or(default_ft_metadata())
    }

    pub fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> Balance {
        self.accounts.get(&account_id).unwrap_or(0)
    }

    pub fn mint_into(&mut self, account_id: &AccountId, amount: Balance) {
        let balance = self.internal_unwrap_balance_of(account_id);
        self.internal_update_account(&account_id, balance + amount);
        self.total_supply += amount;
    }

    pub fn internal_burn(&mut self, account_id: &AccountId, amount: u128) {
        let balance = self.internal_unwrap_balance_of(account_id);
        assert!(balance >= amount);
        self.internal_update_account(&account_id, balance - amount);
        assert!(self.total_supply >= amount);
        self.total_supply -= amount;
    }

    pub fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        amount: Balance,
        memo: Option<String>,
    ) {
        assert_ne!(
            sender_id, receiver_id,
            "Sender and receiver should be different"
        );

        if self.locked_until_nano > 0 && env::block_timestamp() < self.locked_until_nano {
            panic!(
                "transfers are locked until unix timestamp {}",
                self.locked_until_nano / NANOSECONDS
            );
        }

        let sender_balance = self.internal_unwrap_balance_of(sender_id);
        assert!(
            amount == sender_balance || amount > ONE_NEAR / MIN_TRANSFER_UNIT,
            "The amount should be at least 1/{}",
            MIN_TRANSFER_UNIT
        );

        // remove from sender
        let sender_balance = self.internal_unwrap_balance_of(sender_id);
        assert!(
            amount <= sender_balance,
            "The account doesn't have enough balance {}",
            sender_balance
        );
        let balance_left = sender_balance - amount;
        self.internal_update_account(&sender_id, balance_left);

        // check vesting
        if self.vested_count > 0 {
            match self.vested.get(&sender_id) {
                Some(vesting) => {
                    //compute locked
                    let locked = vesting.compute_amount_locked();
                    if locked == 0 {
                        //vesting is complete. remove vesting lock
                        self.vested.remove(&sender_id);
                        self.vested_count -= 1;
                    } else if balance_left < locked {
                        panic!("Vested account, balance can not go lower than {}", locked);
                    }
                }
                None => {}
            }
        }

        // add to receiver
        let receiver_balance = self.internal_unwrap_balance_of(receiver_id);
        self.internal_update_account(&receiver_id, receiver_balance + amount);

        log!("Transfer {} from {} to {}", amount, sender_id, receiver_id);
        if let Some(memo) = memo {
            log!("Memo: {}", memo);
        }
    }

    /// Inner method to save the given account for a given account ID.
    pub fn internal_update_account(&mut self, account_id: &AccountId, balance: u128) {
        self.accounts.insert(account_id, &balance); //insert_or_update
    }

    // TODO rename
    pub fn int_ft_resolve_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: ValidAccountId,
        amount: U128,
    ) -> (u128, u128) {
        let sender_id: AccountId = sender_id.into();
        let receiver_id: AccountId = receiver_id.into();
        let amount: Balance = amount.into();

        // Get the unused amount from the `ft_on_transfer` call result.
        let unused_amount = match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(value) => {
                if let Ok(unused_amount) = near_sdk::serde_json::from_slice::<U128>(&value) {
                    std::cmp::min(amount, unused_amount.0)
                } else {
                    amount
                }
            }
            PromiseResult::Failed => amount,
        };

        if unused_amount > 0 {
            let receiver_balance = self.accounts.get(&receiver_id).unwrap_or(0);
            if receiver_balance > 0 {
                let refund_amount = std::cmp::min(receiver_balance, unused_amount);
                self.accounts
                    .insert(&receiver_id, &(receiver_balance - refund_amount));

                if let Some(sender_balance) = self.accounts.get(&sender_id) {
                    self.accounts
                        .insert(&sender_id, &(sender_balance + refund_amount));
                    log!(
                        "Refund {} from {} to {}",
                        refund_amount,
                        receiver_id,
                        sender_id
                    );
                    return (amount - refund_amount, 0);
                } else {
                    // Sender's account was deleted, so we need to burn tokens.
                    self.total_supply -= refund_amount;
                    log!("The account of the sender was deleted");
                    return (amount, refund_amount);
                }
            }
        }
        (amount, 0)
    }
}
