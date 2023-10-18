use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {pub balance:Uint128}

#[cw_serde]
pub enum ExecuteMsg {
    Gambled {
        token_amount: Uint128,
        is_success: bool,
    },
}

#[cw_serde] 
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config{},
    #[returns(UserBalanceResponse)]
    UserBalance{user:String},
    #[returns(GetBalanceResponse)]
    GetBalance{address:String}
}

#[cw_serde] 
pub struct ConfigResponse{
    pub balance:Uint128,
}
#[cw_serde] 
pub struct UserBalanceResponse{
    pub user:String,
    pub balance:Uint128
}
#[cw_serde] 
pub struct GetBalanceResponse{
    pub balance:Uint128 
}