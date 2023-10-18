use std::io::Stderr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::from_binary;
use cosmwasm_std::{
    coins,Coin, to_binary, Addr, BankMsg, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo,
    Response, StdError, StdResult, Uint128, QuerierWrapper,BankQuery,BalanceResponse,QueryRequest
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use crate::query::*;
/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:gambling-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    CONFIG.save(_deps.storage, &Config{balance:_msg.balance})?;
    let config = CONFIG.load(_deps.storage)?;
    Ok(Response::new().add_attribute("action", "instantiate").add_attribute("balance", config.balance.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Gambled {
            token_amount,
            is_success,
        } => execute_gambled(deps, info, token_amount, is_success),
        //ExecuteMsg::Deposit { ty, amount } => deposit(deps, info, ty, amount),
        //ExecuteMsg::Withdraw { ty, amount, to } => withdraw(deps, info, ty, amount, to),
        
    }
}



fn execute_gambled(
    deps: DepsMut,
    info: MessageInfo,
    token_amount: Uint128,
    is_success: bool,
) -> Result<Response,ContractError> {
    // Check if player sent the "token_amount"
    // let received_amount = info.funds[0].amount;
    // if received_amount != token_amount {
    //     return Err(ContractError::Std(StdError::GenericErr {
    //         msg: "Non-matching balance!".to_string(),
    //     }));
    // }
    // // Calculate the reward
    let reward = token_amount * Decimal::from_ratio(5_u128, 100_u128);
    let config = CONFIG.load(deps.storage)?;
    let player = info.sender.to_string();
    //Get player token balance
    let user_addr = deps.api.addr_validate(&player.clone())?;
    let res = query_balance(&deps.querier, player.clone()).unwrap();
    let from_res :GetBalanceResponse = from_binary(&res).unwrap();
    USER_BALANCES.save(deps.storage, user_addr, &from_res.balance).unwrap();
    if is_success
    {
        if config.balance <= reward {
            return Err(ContractError::Std(StdError::GenericErr {
                msg: "Insufficient Admin Funds".to_string(),
            }));
        }
        let msgs :Vec<CosmosMsg>= vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: player.clone(),
            amount: coins((reward).u128(), "ustars"),
        })];
        let balance = CONFIG.load(deps.storage).unwrap();
        CONFIG.save(deps.storage, &Config{balance : (balance.balance-reward)})?;
        let user_addr = deps.api.addr_validate(&player.clone())?;
        let user_bal = USER_BALANCES.may_load(deps.storage,user_addr.clone())
            .map(|bal| bal.unwrap_or(Uint128::zero()))?;
        USER_BALANCES.save(deps.storage, user_addr, &(user_bal+reward)).unwrap();
        let res = query_balance(&deps.as_ref().querier, player.clone()).unwrap();
        let from_res :GetBalanceResponse = from_binary(&res).unwrap();
        Ok(Response::new().add_messages(msgs)
            .add_attribute("action", "withdraw_loose")
            .add_attribute("reciever", player.clone())
            .add_attribute("amount", from_res.balance.to_string())
        )
            
    }
    else {
        let balance = CONFIG.load(deps.storage).unwrap();
        let user_addr = deps.api.addr_validate(&player.clone())?;
        let user_bal = USER_BALANCES.may_load(deps.storage,user_addr.clone())
            .map(|bal| bal.unwrap_or(Uint128::zero()))?;
        if user_bal<=reward {
            return Err(ContractError::Std(StdError::GenericErr {
                msg: "Insufficient Funds".to_string(),
            }));
        }
        let sent_funds = info
        .funds
        .into_iter()
        .filter(|c| c.denom == "ustars")
        .collect::<Vec<Coin>>();
        
        if sent_funds.is_empty() || sent_funds[0].amount == Uint128::zero() {
            return Err(ContractError::Std(StdError::GenericErr {
                msg: "Insufficient Funds".to_string(),
            }));
        }
        CONFIG.save(deps.storage, &Config{balance : (balance.balance+reward)})?;
        USER_BALANCES.save(deps.storage, user_addr.clone(), &(user_bal-reward))?;
        let res = query_balance(&deps.as_ref().querier, player.clone()).unwrap();
        let from_res :GetBalanceResponse = from_binary(&res).unwrap();
        Ok(Response::new().add_attribute("action", "deposit_win")
            .add_attribute("winner", user_addr.to_string())
            .add_attribute("amount", from_res.balance)
        )
    // // Handle the reward according to "is_success"
    // if is_success {
    //     // Decrease the REWARDS
    //     let available = REWARDS.load(deps.storage).unwrap_or_default();
    //     REWARDS.save(deps.storage, &(available - reward))?;

    //     // Send the "token_amount + reward" to player
    //     let msg = CosmosMsg::Bank(BankMsg::Send {
    //         to_address: player.to_string(),
    //         amount: coins((token_amount + reward).u128(), "ustars"),
    //     });

    //     response = response.add_message(msg);
    //     response = response.add_attributes(vec![
    //         ("action", "gambled"),
    //         ("player", &player.to_string()),
    //         ("reward", &reward.to_string()),
    //     ]);
    // } else {
    //     // Save the profit in PROFITS
    //     let curr_profit = PROFITS.load(deps.storage).unwrap_or_default();
    //     PROFITS.save(deps.storage, &(curr_profit + reward))?;

    //     // Send back the "token_amount - profit" to player
    //     let msg = CosmosMsg::Bank(BankMsg::Send {
    //         to_address: player.to_string(),
    //         amount: coins((token_amount - reward).u128(), "ustars"),
    //     });

    //     response = response.add_message(msg);
    //     response = response.add_attributes(vec![
    //         ("action", "gambled"),
    //         ("player", &player.to_string()),
    //         ("profit", &reward.to_string()),
    //     ]);
    // }
}
// fn deposit(
//     deps: DepsMut,
//     info: MessageInfo,
//     ty: String,
//     amount: Uint128,
// ) -> Result<Response, ContractError> {
//     // Check if "amount" tokens sent
//     if amount != info.funds[0].amount {
//         return Err(ContractError::Std(StdError::GenericErr {
//             msg: "Non-matching deposit amount".to_string(),
//         }));
//     }

//     // Handle the deposit
//     match ty.as_str() {
//         "reward" => {
//             let curr_available = REWARDS.load(deps.storage).unwrap_or_default();
//             REWARDS.save(deps.storage, &(curr_available + amount))?;
//         }
//         _ => unreachable!(),
//     };

//     Ok(Response::new().add_attributes(vec![
//         ("action", "deposit"),
//         ("type", &ty),
//         ("amount", &amount.to_string()),
//     ]))
// }

// fn withdraw(
//     deps: DepsMut,
//     info: MessageInfo,
//     ty: String,
//     amount: Uint128,
//     to: Option<Addr>,
// ) -> Result<Response, ContractError> {
//     if amount.is_zero() {
//         return Err(ContractError::Std(StdError::GenericErr {
//             msg: "Invalid zero amount".to_string(),
//         }));
//     }

//     // Handle the withdraw
//     match ty.as_str() {
//         "reward" => {
//             let curr_available = REWARDS.load(deps.storage)?;
//             REWARDS.save(deps.storage, &(curr_available - amount))?;
//         }
//         "profit" => {
//             let curr_profit = PROFITS.load(deps.storage)?;
//             PROFITS.save(deps.storage, &(curr_profit - amount))?;
//         }
//         _ => unreachable!(),
//     };

//     let msg = CosmosMsg::Bank(BankMsg::Send {
//         to_address: to.unwrap_or(info.sender).to_string(),
//         amount: coins(amount.u128(), "ustars".to_string()),
//     });

//     Ok(Response::new().add_message(msg).add_attributes(vec![
//         ("action", "deposit"),
//         ("type", &ty),
//         ("amount", &amount.to_string()),
//     ]))
// }
}
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::Config {  } => query_config(_deps),
        QueryMsg::UserBalance { user } => query_user_balance(_deps, user),
        QueryMsg::GetBalance { address } => query_balance(&_deps.querier, address)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coins,
        testing::{mock_dependencies , mock_env, mock_info, mock_dependencies_with_balance, mock_dependencies_with_balances},
        Addr, StdError, Uint128, from_binary, Response, Coin,
    };

    use crate::{
        msg::{ExecuteMsg, InstantiateMsg, GetBalanceResponse, QueryMsg},
        ContractError, 
        // ,mock_queries::mock_dependencies
    };
    use super::{execute, instantiate, query};

    #[test]
    fn test_initialize() {
        let mut deps = mock_dependencies();
        let info = mock_info("admin", &[]);
        
        // Successful instantiate should be unwrap.
        let _ = instantiate(deps.as_mut(), mock_env(), info,InstantiateMsg {balance:Uint128::from(100u128)}).unwrap();
    }

    #[test]
    fn test_gambled() {
        let mut deps = mock_dependencies_with_balances(&[(&("creater"), &coins(7000,"ustars")),(&("player"), &coins(1000,"ustars"))]);
        // let info = mock_info("sender", &coins(1000, "ustars"));
        let info = mock_info("creater", &coins(4000, "ustars"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), InstantiateMsg {balance:Uint128::from(7000u128)}).unwrap();
        let info = mock_info("player", &coins(100, "ustars"));
        let msg = ExecuteMsg::Gambled { token_amount: Uint128::from(100u128), is_success: false };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!("player" , Response::from(res.clone()).attributes[1].value);
        assert_eq!("995",Response::from(res.clone()).attributes[2].value);
        //assert_eq!("100",info.funds[0].amount.to_string());
        // let msg = QueryMsg::GetBalance { address: "player".to_string() };
        // let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        // let balance : GetBalanceResponse = from_binary(&res).unwrap();
        
        // assert_eq!("1005",balance.balance.to_string());

        // Deposit the "REWARDS"
        // let info = mock_info("admin", &coins(100_u128, "ustars"));
        // let msg = ExecuteMsg::Deposit {
        //     ty: "reward".to_string(),
        //     amount: Uint128::new(100),
        // };
        // let _ = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Try the gamble
        // let info = mock_info("player", &coins(100_u128, "ustars"));
        // let msg = ExecuteMsg::Gambled {
        //     player: Addr::unchecked("player"),
        //     token_amount: Uint128::new(200),
        //     is_success: true,
        // };
        // let err = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();
        // assert_eq!(
        //     err.to_string(),
        //     ContractError::Std(StdError::GenericErr {
        //         msg: "Non-matching balance!".to_string()
        //     })
        //     .to_string()
        // );

        // Success
        // let info = mock_info("player", &coins(100_u128, "ustars"));
        // let msg = ExecuteMsg::Gambled {
        //     player: Addr::unchecked("player"),
        //     token_amount: Uint128::new(100),
        //     is_success: true,
        // };
        // let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        // assert_eq!(res.messages.len(), 1);
        // assert_eq!(res.attributes[2].value, "5");
    }
}
