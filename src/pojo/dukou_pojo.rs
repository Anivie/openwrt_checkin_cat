use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    pub ret: u8,
    pub status: Option<u8>,
    pub username: Option<String>,
    pub id: Option<u32>,
    pub token: Option<String>,
    pub msg: Option<String>,
    pub result: Option<TokenResponseResult>,
    pub data: Option<TokenResponseData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponseResult {
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponseData {
    pub token: String,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(rename = "user_hash")]
    pub user_hash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckinResponse {
    pub result: String,
    pub ret: u8,
}