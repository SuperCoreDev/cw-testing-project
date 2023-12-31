use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]

pub struct Config{
    pub balance : Uint128
}
pub const CONFIG : Item<Config> = Item::new("config");
pub const USER_BALANCES : Map<Addr,Uint128> = Map::new("user_balances");