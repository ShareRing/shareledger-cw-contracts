use crate::{
    error::ContractError,
    execute,
    msg::{Cw1155RoyaltiesExecuteMsg, Cw1155RoyaltiesQueryMsg, InstantiateMsg},
    query,
    state::{MINTER, TOKEN_SEQ},
};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw1155-royalties";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let minter = deps.api.addr_validate(&msg.minter)?;
    MINTER.save(deps.storage, &minter)?;
    TOKEN_SEQ.save(deps.storage, &Uint128::new(0))?;
    Ok(Response::default())
}

/// To mitigate clippy::too_many_arguments warning
pub struct ExecuteEnv<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw1155RoyaltiesExecuteMsg,
) -> Result<Response, ContractError> {
    let env = ExecuteEnv { deps, env, info };
    match msg {
        Cw1155RoyaltiesExecuteMsg::SendFrom {
            from,
            to,
            token_id,
            value,
            msg,
        } => execute::send_from(env, from, to, token_id, value, msg),
        Cw1155RoyaltiesExecuteMsg::BatchSendFrom {
            from,
            to,
            batch,
            msg,
        } => execute::batch_send_from(env, from, to, batch, msg),
        Cw1155RoyaltiesExecuteMsg::Mint {
            to,
            token_id,
            value,
            msg,
            royalty_payment_address,
            royalty_percentage,
        } => execute::mint(
            env,
            to,
            token_id,
            value,
            royalty_payment_address,
            royalty_percentage,
            msg,
        ),
        Cw1155RoyaltiesExecuteMsg::BatchMint { to, batch, msg } => {
            execute::batch_mint(env, to, batch, msg)
        }
        Cw1155RoyaltiesExecuteMsg::Burn {
            from,
            token_id,
            value,
        } => execute::burn(env, from, token_id, value),
        Cw1155RoyaltiesExecuteMsg::BatchBurn { from, batch } => {
            execute::batch_burn(env, from, batch)
        }
        Cw1155RoyaltiesExecuteMsg::ApproveAll { operator, expires } => {
            execute::approve_all(env, operator, expires)
        }
        Cw1155RoyaltiesExecuteMsg::RevokeAll { operator } => execute::revoke_all(env, operator),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: Cw1155RoyaltiesQueryMsg) -> StdResult<Binary> {
    match msg {
        Cw1155RoyaltiesQueryMsg::Balance { owner, token_id } => {
            to_binary(&query::balance(deps, owner, token_id)?)
        }
        Cw1155RoyaltiesQueryMsg::BatchBalance { owner, token_ids } => {
            to_binary(&query::batch_balance(deps, owner, token_ids)?)
        }
        Cw1155RoyaltiesQueryMsg::IsApprovedForAll { owner, operator } => {
            to_binary(&query::is_approved_for_all(deps, env, owner, operator)?)
        }
        Cw1155RoyaltiesQueryMsg::ApprovedForAll {
            owner,
            include_expired,
            start_after,
            limit,
        } => to_binary(&query::approved_for_all(
            deps,
            env,
            owner,
            include_expired.unwrap_or(false),
            start_after,
            limit,
        )?),
        Cw1155RoyaltiesQueryMsg::TokenInfo { token_id } => {
            to_binary(&query::token_info(deps, token_id)?)
        }
        Cw1155RoyaltiesQueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_binary(&query::tokens(deps, owner, start_after, limit)?),
        Cw1155RoyaltiesQueryMsg::AllTokens { start_after, limit } => {
            to_binary(&query::all_tokens(deps, start_after, limit)?)
        }
        Cw1155RoyaltiesQueryMsg::RoyaltyInfo {
            token_id,
            sale_price,
        } => to_binary(&query::query_royalties_info(deps, token_id, sale_price)?),
        Cw1155RoyaltiesQueryMsg::CheckRoyalties {} => to_binary(&query::check_royalties(deps)?),
    }
}
