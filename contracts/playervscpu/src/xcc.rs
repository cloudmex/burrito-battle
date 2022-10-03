use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(25_000_000_000_000 + GAS_FOR_RESOLVE_TRANSFER.0);
const MIN_GAS_FOR_NFT_TRANSFER_CALL: Gas = Gas(100_000_000_000_000);
const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_nft)]
pub trait ExternsContract {
    fn get_items_for_battle(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId
    ) -> AccessoriesForBattle;
    fn reward_player(&self,player_owner_id: String,tokens_mint: String) -> String;
    fn get_balance_and_transfer(&self,account_id: String, action: String) -> U128;

    fn get_burrito(&self,account_id: String, burrito_id: TokenId) -> Burrito;
    fn decrease_burrito_hp(&self, burrito_id: TokenId) -> Burrito;
    fn increment_burrito_wins(&self, burrito_id: TokenId) -> Burrito;
    fn get_items_for_battle_cpu(&self, 
        accesorio1_burrito1_id: TokenId, accesorio2_burrito1_id: TokenId, accesorio3_burrito1_id: TokenId
    ) -> AccessoriesForBattle;}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    fn get_winner(&mut self,burrito1_id: TokenId,burrito2_id: TokenId) -> String;
    fn burrito_level_up(&mut self,burrito_id: TokenId) -> String;
    //fn new_burrito(&mut self,token_owner_id: AccountId, token_metadata: TokenMetadata) -> String;
    fn reset_conditions(&mut self,burrito_id: TokenId) -> String;

    fn save_burritos_battle_room(&mut self,burrito_id: TokenId,accesorio1_id: TokenId, accesorio2_id: TokenId, accesorio3_id: TokenId) -> String;
    fn save_battle_player_cpu(&mut self,burrito_id: TokenId) -> String;
}