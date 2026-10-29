use crate::game::types::{InboundMessage, OutboundMessage};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};

pub struct TibiaIdleSocket {
    is_closed: Arc<AtomicBool>,
    sender: Arc<Mutex<Option<futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>>>>,
}

impl TibiaIdleSocket {
    pub async fn connect(ws_url: &str, ticket: &str) -> Result<Self, String> {
        info!("[TibiaIdle WS] Conectando ao WebSocket do jogo: {}", ws_url);
        let (ws_stream, _) = connect_async(ws_url)
            .await
            .map_err(|e| format!("Falha de conexão WS: {e}"))?;

        let (mut write, mut read) = ws_stream.split();

        // Enviar Auth Payload inicial
        let auth_msg = OutboundMessage {
            msg_type: "auth".to_string(),
            data: json!({
                "ticket": ticket,
                "tradeFlowVersion": 2,
            }),
        };
        let auth_str = serde_json::to_string(&auth_msg).unwrap();
        write.send(Message::Text(auth_str.into())).await.map_err(|e| format!("Falha ao enviar auth: {e}"))?;

        let is_closed = Arc::new(AtomicBool::new(false));
        let sender = Arc::new(Mutex::new(Some(write)));

        let closed_clone = is_closed.clone();
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(txt)) => {
                        if let Ok(inbound) = serde_json::from_str::<InboundMessage>(&txt) {
                            debug!("[TibiaIdle WS] Recebido: tipo={}", inbound.msg_type);
                        }
                    }
                    Ok(Message::Close(_)) => {
                        warn!("[TibiaIdle WS] Conexão encerrada pelo servidor.");
                        closed_clone.store(true, Ordering::SeqCst);
                        break;
                    }
                    Err(e) => {
                        error!("[TibiaIdle WS] Erro no stream: {}", e);
                        closed_clone.store(true, Ordering::SeqCst);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(Self {
            is_closed,
            sender,
        })
    }

    pub async fn send_message(&self, msg_type: &str, data: serde_json::Value) -> Result<(), String> {
        let mut sender = self.sender.lock().await;
        if let Some(s) = sender.as_mut() {
            let msg = OutboundMessage {
                msg_type: msg_type.to_string(),
                data,
            };
            let txt = serde_json::to_string(&msg).unwrap();
            s.send(Message::Text(txt.into())).await.map_err(|e| format!("Falha ao enviar mensagem WS: {e}"))?;
            Ok(())
        } else {
            Err("WebSocket desconectado".to_string())
        }
    }

    pub async fn start_hunt(&self, hunt_id: u32) -> Result<(), String> {
        self.send_message("start_hunt", json!({ "huntId": hunt_id })).await
    }

    pub async fn stop_hunt(&self) -> Result<(), String> {
        self.send_message("stop_hunt", json!({})).await
    }

    pub async fn sell_trash(&self) -> Result<(), String> {
        self.send_message("sell_trash", json!({})).await
    }

    pub async fn buy_blessing(&self, blessing_id: u32) -> Result<(), String> {
        self.send_message("buy_blessing", json!({ "blessingId": blessing_id })).await
    }

    pub async fn claim_daily(&self) -> Result<(), String> {
        self.send_message("claim_daily", json!({})).await
    }
}
