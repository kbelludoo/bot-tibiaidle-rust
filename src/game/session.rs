use crate::game::types::AccountInfo;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, ORIGIN, REFERER, USER_AGENT};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tracing::{debug, error, info, warn};

pub const DEFAULT_BASE_URL: &str = "https://play.tibidle.com";
pub const USER_AGENT_STR: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub struct SessionClient {
    client: Client,
    base_url: String,
}

impl SessionClient {
    pub fn new(base_url: Option<&str>) -> Self {
        let base = base_url.unwrap_or(DEFAULT_BASE_URL).trim_end_matches('/').to_string();
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            base_url: base,
        }
    }

    fn headers(&self, cookie: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STR));
        headers.insert(ORIGIN, HeaderValue::from_str(&self.base_url).unwrap());
        headers.insert(REFERER, HeaderValue::from_str(&format!("{}/", self.base_url)).unwrap());
        if !cookie.is_empty() {
            if let Ok(c) = HeaderValue::from_str(cookie) {
                headers.insert(COOKIE, c);
            }
        }
        headers
    }

    pub async fn validate_session(&self, cookie: &str) -> Result<AccountInfo, String> {
        let url = format!("{}/auth/me", self.base_url);
        let resp = self.client.get(&url)
            .headers(self.headers(cookie))
            .send()
            .await
            .map_err(|e| format!("Erro de rede em /auth/me: {e}"))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(format!("Sessão inválida (HTTP {}): cookie expirado", status));
        }

        let acc: AccountInfo = resp.json()
            .await
            .map_err(|e| format!("Falha ao decodificar AccountInfo: {e}"))?;

        Ok(acc)
    }

    pub async fn get_world_ticket(&self, cookie: &str, world_id: &str) -> Result<String, String> {
        let url = format!("{}/auth/world-ticket", self.base_url);
        let payload = json!({ "worldId": world_id });

        let resp = self.client.post(&url)
            .headers(self.headers(cookie))
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Erro em /auth/world-ticket: {e}"))?;

        let status = resp.status();
        if !status.is_success() {
            return Err(format!("Falha ao obter ticket do mundo (HTTP {})", status));
        }

        let body: serde_json::Value = resp.json()
            .await
            .map_err(|e| format!("Erro ao ler ticket JSON: {e}"))?;

        body.get("ticket")
            .and_then(|t| t.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "Ticket ausente na resposta".to_string())
    }
}
