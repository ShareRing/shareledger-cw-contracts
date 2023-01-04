use cosmwasm_std::{Addr, Decimal, Deps, Env, Order, StdResult, Uint128};
use cw1155::{
    ApprovedForAllResponse, BalanceResponse, BatchBalanceResponse, IsApprovedForAllResponse,
    TokenInfoResponse, TokensResponse,
};
use cw_storage_plus::Bound;
use cw_utils::{maybe_addr, Expiration};

use crate::{
    helpers::check_can_approve,
    state::{APPROVES, BALANCES, ROYALTIES, TOKENS},
};

use crate::msg::{CheckRoyaltiesResponse, RoyaltiesInfoResponse};

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 30;

pub fn balance(deps: Deps, owner: String, token_id: String) -> StdResult<BalanceResponse> {
    let owner = deps.api.addr_validate(&owner)?;

    let balance = BALANCES
        .may_load(deps.storage, (&owner, &token_id))?
        .unwrap_or_default();

    Ok(BalanceResponse { balance })
}

pub fn batch_balance(
    deps: Deps,
    owner: String,
    token_ids: Vec<String>,
) -> StdResult<BatchBalanceResponse> {
    let owner = deps.api.addr_validate(&owner)?;

    let balances = token_ids
        .into_iter()
        .map(|token_id| -> StdResult<_> {
            Ok(BALANCES
                .may_load(deps.storage, (&owner, &token_id))?
                .unwrap_or_default())
        })
        .collect::<StdResult<_>>()?;

    Ok(BatchBalanceResponse { balances })
}

fn build_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<cw1155::Approval> {
    item.map(|(addr, expires)| cw1155::Approval {
        spender: addr.into(),
        expires,
    })
}

pub fn approved_for_all(
    deps: Deps,
    env: Env,
    owner: String,
    include_expired: bool,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ApprovedForAllResponse> {
    let owner = deps.api.addr_validate(&owner)?;
    let start_after = maybe_addr(deps.api, start_after)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(Bound::exclusive);

    let operators = APPROVES
        .prefix(&owner)
        .range(deps.storage, start, None, Order::Ascending)
        .filter(|r| include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block))
        .take(limit)
        .map(build_approval)
        .collect::<StdResult<_>>()?;

    Ok(ApprovedForAllResponse { operators })
}

pub fn token_info(deps: Deps, token_id: String) -> StdResult<TokenInfoResponse> {
    let url = TOKENS.load(deps.storage, &token_id)?;

    Ok(TokenInfoResponse { url })
}

pub fn is_approved_for_all(
    deps: Deps,
    env: Env,
    owner: String,
    operator: String,
) -> StdResult<IsApprovedForAllResponse> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let operator_addr = deps.api.addr_validate(&operator)?;

    let approved = check_can_approve(deps, &env, &owner_addr, &operator_addr)?;

    Ok(IsApprovedForAllResponse { approved })
}

pub fn tokens(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let owner = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

    let tokens = BALANCES
        .prefix(&owner)
        .keys(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect::<StdResult<_>>()?;

    Ok(TokensResponse { tokens })
}

pub fn all_tokens(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

    let tokens = TOKENS
        .keys(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .collect::<StdResult<_>>()?;

    Ok(TokensResponse { tokens })
}

/// NOTE: default behaviour here is to round down
/// EIP2981 specifies that the rounding behaviour is at the discretion of the implementer
pub fn query_royalties_info(
    deps: Deps,
    token_id: String,
    sale_price: Uint128,
) -> StdResult<RoyaltiesInfoResponse> {
    let royalty_info = ROYALTIES.load(deps.storage, &token_id)?;

    let royalty_percentage = match royalty_info.royalty_percentage {
        Some(percentage) => Decimal::percent(percentage),
        None => Decimal::percent(0),
    };

    let royalty_amount = sale_price * royalty_percentage;

    let address = match royalty_info.royalty_payment_address {
        Some(addr) => addr,
        None => String::from(""),
    };

    Ok(RoyaltiesInfoResponse {
        address,
        royalty_amount,
    })
}

/// As our default implementation here specifies royalties at token level
/// and not at contract level, it is therefore logically true that
/// on sale, every token managed by this contract should be checked
/// to see if royalties are owed, and to whom. If you are importing
/// this logic, you may want a custom implementation here
pub fn check_royalties(_deps: Deps) -> StdResult<CheckRoyaltiesResponse> {
    Ok(CheckRoyaltiesResponse {
        royalty_payments: true,
    })
}
