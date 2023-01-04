use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw1155::Expiration;
use cw_storage_plus::{Item, Map};

/// Store the minter address who have permission to mint new tokens.
pub const MINTER: Item<Addr> = Item::new("minter");
/// Store the balance map, `(owner, token_id) -> balance`
pub const BALANCES: Map<(&Addr, &str), Uint128> = Map::new("balances");
/// Store the approval status, `(owner, spender) -> expiration`
pub const APPROVES: Map<(&Addr, &Addr), Expiration> = Map::new("approves");
/// Store the tokens metadata url, also supports enumerating tokens,
/// An entry for token_id must exist as long as there's tokens in circulation.
pub const TOKENS: Map<&str, String> = Map::new("tokens");

// autoincrement token sequen
pub const TOKEN_SEQ: Item<Uint128> = Item::new("token_seq");

pub const ROYALTIES: Map<&str, RoyaltyInfo> = Map::new("royalties");

#[cw_serde]
pub struct RoyaltyInfo {
    /// This is how much the minter takes as a cut when sold
    /// royalties are owed on this token if it is Some
    pub royalty_percentage: Option<u64>,
    /// The payment address, may be different to or the same
    /// as the minter addr
    /// question: how do we validate this?
    pub royalty_payment_address: Option<String>,
}
