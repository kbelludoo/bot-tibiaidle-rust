use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    pub credentials: GameCredentials,
    #[serde(default)]
    pub settings: GameSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameCredentials {
    #[serde(default)]
    pub session_cookie: String,
    #[serde(default)]
    pub ticket: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub origin: String,
    #[serde(default)]
    pub world_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSettings {
    #[serde(default)]
    pub hunt_target: String,
    #[serde(default = "default_true")]
    pub auto_hunt: bool,
    #[serde(default = "default_true")]
    pub auto_sell: bool,
    #[serde(default = "default_true")]
    pub auto_heal: bool,
    #[serde(default = "default_true")]
    pub auto_daily: bool,
    #[serde(default = "default_true")]
    pub auto_bless: bool,
    #[serde(default = "default_true")]
    pub focus_bestiary: bool,
    #[serde(default = "default_lure")]
    pub lure: u32,
}

fn default_true() -> bool {
    true
}

fn default_lure() -> u32 {
    3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub world_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStats {
    pub level: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub gold: u64,
}
