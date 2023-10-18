use crate::msg::*;
use crate::state::*;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult, Uint128};

use cosmwasm_std::{
    Addr, AllBalanceResponse, BalanceResponse, BankQuery, Coin, QuerierWrapper,
    QueryRequest, WasmQuery
};
pub fn query_config(deps: Deps) -> StdResult<Binary> {
    let config: Config = CONFIG.load(deps.storage)?;
    let response = ConfigResponse{
        balance : config.balance,
    };
    to_binary(&response)
}

pub fn query_user_balance(deps:Deps,user:String) -> StdResult<Binary> {
    let user_addr = deps.api.addr_validate(&user)?;
    let balance = USER_BALANCES.may_load(deps.storage, user_addr)
        .map(|bal| bal.unwrap_or(Uint128::zero()))?;
    let response = UserBalanceResponse{
        user : user,
        balance : balance,
    };
    to_binary(&response)
}

pub fn query_balance(
    querier: &QuerierWrapper,
    address: String,
) -> StdResult<Binary> {
    // let query = querier.query_balance(address, String::from("ustars"))?.amount;
    let balance: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address:address,
        denom:String::from("ustars")
    }))?;
    let response = GetBalanceResponse{balance:balance.amount.amount};
    // let response : GetBalanceResponse = GetBalanceResponse { balance: query };
    to_binary(&response)
}