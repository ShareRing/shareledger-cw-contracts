use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128};
use cw_utils::Expiration;

pub type TokenId = String;

#[cw_serde]
pub struct InstantiateMsg {
    /// The minter is the only one who can create new tokens.
    /// This is designed for a base token platform that is controlled by an external program or
    /// contract.
    pub minter: String,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw1155RoyaltiesQueryMsg {
    // Cw1155QueryMsg(Cw1155QueryMsg),
    /// Should be called on sale to see if royalties are owed
    /// by the marketplace selling the NFT, if CheckRoyalties
    /// returns true
    /// See https://eips.ethereum.org/EIPS/eip-2981
    #[returns(RoyaltiesInfoResponse)]
    RoyaltyInfo {
        token_id: String,
        // the denom of this sale must also be the denom returned by RoyaltiesInfoResponse
        // this was originally implemented as a Coin
        // however that would mean you couldn't buy using CW20s
        // as CW20 is just mapping of addr -> balance
        sale_price: Uint128,
    },
    /// Called against contract to determine if this NFT
    /// implements royalties. Should return a boolean as part of
    /// CheckRoyaltiesResponse - default can simply be true
    /// if royalties are implemented at token level
    /// (i.e. always check on sale)
    #[returns(CheckRoyaltiesResponse)]
    CheckRoyalties {},

    /// Returns the current balance of the given address, 0 if unset.
    #[returns(BalanceResponse)]
    Balance { owner: String, token_id: TokenId },
    /// Returns the current balance of the given address for a batch of tokens, 0 if unset.
    #[returns(BatchBalanceResponse)]
    BatchBalance {
        owner: String,
        token_ids: Vec<TokenId>,
    },
    /// List all operators that can access all of the owner's tokens.
    #[returns(ApprovedForAllResponse)]
    ApprovedForAll {
        owner: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Query approved status `owner` granted to `operator`.
    #[returns(IsApprovedForAllResponse)]
    IsApprovedForAll { owner: String, operator: String },

    /// With MetaData Extension.
    /// Query metadata of token
    #[returns(TokenInfoResponse)]
    TokenInfo { token_id: TokenId },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub enum Cw1155RoyaltiesExecuteMsg {
    /// SendFrom is a base message to move tokens,
    /// if `env.sender` is the owner or has sufficient pre-approval.
    SendFrom {
        from: String,
        /// If `to` is not contract, `msg` should be `None`
        to: String,
        token_id: TokenId,
        value: Uint128,
        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// BatchSendFrom is a base message to move multiple types of tokens in batch,
    /// if `env.sender` is the owner or has sufficient pre-approval.
    BatchSendFrom {
        from: String,
        /// if `to` is not contract, `msg` should be `None`
        to: String,
        batch: Vec<(TokenId, Uint128)>,
        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// Mint is a base message to mint tokens.
    Mint {
        /// If `to` is not contract, `msg` should be `None`
        to: String,
        token_id: Option<String>,
        value: Uint128,
        royalty_percentage: Option<u64>,
        royalty_payment_address: Option<String>,

        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// BatchMint is a base message to mint multiple types of tokens in batch.
    BatchMint {
        /// If `to` is not contract, `msg` should be `None`
        to: String,
        batch: Vec<(TokenId, Uint128)>,
        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// Burn is a base message to burn tokens.
    Burn {
        from: String,
        token_id: TokenId,
        value: Uint128,
    },
    /// BatchBurn is a base message to burn multiple types of tokens in batch.
    BatchBurn {
        from: String,
        batch: Vec<(TokenId, Uint128)>,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },
}

#[cw_serde]
pub struct RoyaltiesInfoResponse {
    pub address: String,
    // Note that this must be the same denom as that passed in to RoyaltyInfo
    // rounding up or down is at the discretion of the implementer
    pub royalty_amount: Uint128,
}

/// Shows if the contract implements royalties
/// if royalty_payments is true, marketplaces should pay them
#[cw_serde]
pub struct CheckRoyaltiesResponse {
    pub royalty_payments: bool,
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct BatchBalanceResponse {
    pub balances: Vec<Uint128>,
}

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: String,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[cw_serde]
pub struct ApprovedForAllResponse {
    pub operators: Vec<Approval>,
}

#[cw_serde]
pub struct IsApprovedForAllResponse {
    pub approved: bool,
}

#[cw_serde]
pub struct TokenInfoResponse {
    /// Should be a url point to a json file
    pub url: String,
}

#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_from` in future queries
    /// to achieve pagination.
    pub tokens: Vec<TokenId>,
}
