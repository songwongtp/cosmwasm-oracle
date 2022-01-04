use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub symbols: Vec<String>,
    pub feeder: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // SetPrice updates the price of a given symbol on the contract
    SetPrice { symbol: String, price: u64 },
    // UpdateAdmin updates the contract's owner
    UpdateAdmin { addr: Option<String> },
    // UpdateFeeder updates the feeder addr allowed to feed prices
    UpdateFeeder { addr: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetPrice returns the price of a given symbol stored on the contract
    GetPrice { symbol: String },
    // GetAdmin returns the owner addr
    GetAdmin {},
    // GetFeeder returns the allowed feeder addr
    GetFeeder {},
}

// PriceResponse, the response to GetPrice, return the given symbol price
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub price: u64,
}

// FeederResponse, the response to GetFeeder, return the current allowed feeder addr
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FeederResponse {
    pub feeder: String,
}
