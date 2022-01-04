use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};

// ADMIN represents the owner of the contract
pub const ADMIN: Admin = Admin::new("admin");

// STATE contains a single feeder addr allowed to feed data to this contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub feeder: Addr,
}
pub const STATE: Item<State> = Item::new("state");

// PRICES contains the price data of symbols
pub const PRICES: Map<&str, u64> = Map::new("price");
