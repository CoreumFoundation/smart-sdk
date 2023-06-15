use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Binary, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::pagination::{PageRequest, PageResponse};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Params {
    pub issue_fee: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ParamsResponse {
    pub params: Params,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Class {
    pub id: String,
    pub issuer: String,
    pub name: String,
    pub symbol: String,
    pub description: Option<String>,
    pub uri: Option<String>,
    pub uri_hash: Option<String>,
    pub data: Option<Binary>,
    pub features: Option<Vec<u32>>,
    pub royalty_rate: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassResponse {
    pub class: Class,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ClassesResponse {
    pub pagination: PageResponse,
    pub classes: Vec<Class>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct FrozenResponse {
    pub frozen: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhitelistedResponse {
    pub whitelisted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WhitelistedAccountsForNFTResponse {
    pub pagination: PageResponse,
    pub accounts: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Msg {
    IssueClass {
        name: String,
        symbol: String,
        description: Option<String>,
        uri: Option<String>,
        uri_hash: Option<String>,
        data: Option<Binary>,
        features: Option<Vec<u32>>,
        royalty_rate: Option<String>,
    },
    Mint {
        class_id: String,
        id: String,
        uri: Option<String>,
        uri_hash: Option<String>,
        data: Option<Binary>,
    },
    Burn {
        class_id: String,
        id: String,
    },
    Freeze {
        class_id: String,
        id: String,
    },
    Unfreeze {
        class_id: String,
        id: String,
    },
    AddToWhitelist {
        class_id: String,
        id: String,
        account: String,
    },
    RemoveFromWhitelist {
        class_id: String,
        id: String,
        account: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
pub enum Query {
    #[returns(ParamsResponse)]
    Params {},

    #[returns(ClassResponse)]
    Class { id: String },

    #[returns(ClassesResponse)]
    Classes {
        pagination: Option<PageRequest>,
        issuer: String,
    },

    #[returns(FrozenResponse)]
    Frozen { id: String, class_id: String },

    #[returns(WhitelistedResponse)]
    Whitelisted {
        id: String,
        class_id: String,
        account: String,
    },

    #[returns(WhitelistedAccountsForNFTResponse)]
    WhitelistedAccountsForNFT {
        pagination: Option<PageRequest>,
        id: String,
        class_id: String,
    },
}
