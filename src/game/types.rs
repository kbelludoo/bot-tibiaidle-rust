use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "RawAccountConfig")]
pub struct AccountConfig {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub credentials: GameCredentials,
    pub settings: GameSettings,
}

#[derive(Debug, Clone, Deserialize)]
struct RawAccountConfig {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default)]
    credentials: Option<GameCredentials>,
    #[serde(default)]
    settings: Option<GameSettings>,

    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    password: Option<String>,
    #[serde(rename = "sessionCookie", alias = "session_cookie", default)]
    session_cookie: Option<String>,
    #[serde(default)]
    ticket: Option<String>,
    #[serde(rename = "huntId", alias = "hunt_id", default)]
    hunt_id: Option<i32>,
    #[serde(rename = "huntName", alias = "hunt_name", default)]
    hunt_name: Option<String>,
    #[serde(default)]
    lure: Option<u32>,
    #[serde(rename = "autoSell", alias = "auto_sell", default)]
    auto_sell: Option<bool>,
    #[serde(rename = "autoBlessings", alias = "auto_blessings", default)]
    auto_blessings: Option<bool>,
    #[serde(rename = "autoDaily", alias = "auto_daily", default)]
    auto_daily: Option<bool>,
    #[serde(rename = "bestiaryMode", alias = "bestiary_mode", default)]
    bestiary_mode: Option<bool>,
}

impl From<RawAccountConfig> for AccountConfig {
    fn from(raw: RawAccountConfig) -> Self {
        let name = if !raw.name.is_empty() {
            raw.name
        } else {
            raw.id.clone()
        };

        let mut credentials = raw.credentials.unwrap_or_default();
        if credentials.session_cookie.is_empty() {
            if let Some(sc) = raw.session_cookie {
                credentials.session_cookie = sc;
            }
        }
        if credentials.email.is_empty() {
            if let Some(em) = raw.email {
                credentials.email = em;
            }
        }
        if credentials.password.is_empty() {
            if let Some(pw) = raw.password {
                credentials.password = pw;
            }
        }
        if credentials.ticket.is_empty() {
            if let Some(tk) = raw.ticket {
                credentials.ticket = tk;
            }
        }

        let mut settings = raw.settings.unwrap_or_default();
        if let Some(l) = raw.lure {
            settings.lure = l;
        }
        if let Some(asell) = raw.auto_sell {
            settings.auto_sell = asell;
        }
        if let Some(abless) = raw.auto_blessings {
            settings.auto_bless = abless;
        }
        if let Some(adaily) = raw.auto_daily {
            settings.auto_daily = adaily;
        }
        if let Some(bestiary) = raw.bestiary_mode {
            settings.focus_bestiary = bestiary;
        }
        if let Some(hname) = raw.hunt_name {
            if settings.hunt_target.is_empty() {
                settings.hunt_target = hname;
            }
        }

        Self {
            id: raw.id,
            name,
            enabled: raw.enabled,
            credentials,
            settings,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameCredentials {
    #[serde(rename = "sessionCookie", alias = "session_cookie", default)]
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
