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
    SetPrice { symbol: String, price: u64 },
    UpdateAdmin { addr: Option<String> },
    UpdateFeeder { addr: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetPrice returns the price of a given symbol stored in the contract
    GetPrice { symbol: String },
    GetAdmin {},
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
