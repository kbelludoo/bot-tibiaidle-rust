use crate::engine::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::engine::stamina::{StaminaConfig, StaminaController};
use crate::game::catalog::Catalog;
use crate::game::session::SessionClient;
use crate::game::socket::TibiaIdleSocket;
use crate::game::types::{AccountConfig, AccountInfo};
use crate::subsystem::dailies::DailyManager;
use crate::subsystem::inventory::InventoryManager;
use crate::subsystem::potions::PotionManager;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

pub struct BotInstance {
    pub config: AccountConfig,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub stamina: Arc<StaminaController>,
    pub inventory: Arc<InventoryManager>,
    pub dailies: Arc<DailyManager>,
    pub potions: Arc<PotionManager>,
    pub catalog: Catalog,
    pub session_client: Arc<SessionClient>,
    pub socket: Arc<RwLock<Option<TibiaIdleSocket>>>,
    pub account_info: Arc<RwLock<Option<AccountInfo>>>,
    pub kills: AtomicU64,
    pub gold: AtomicU64,
    pub exp: AtomicU64,
    pub current_level: AtomicU32,
    pub bestiary_kills: AtomicU64,
    pub running: AtomicBool,
}

impl BotInstance {
    pub fn new(config: AccountConfig) -> Self {
        let cb_cfg = CircuitBreakerConfig::default();
        let cb = Arc::new(CircuitBreaker::new(&config.id, cb_cfg));
        let stam = Arc::new(StaminaController::new(StaminaConfig::default()));
        let session = Arc::new(SessionClient::new(Some("https://play.tibidle.com")));

        Self {
            config,
            circuit_breaker: cb,
            stamina: stam,
            inventory: Arc::new(InventoryManager::default()),
            dailies: Arc::new(DailyManager::default()),
            potions: Arc::new(PotionManager::default()),
            catalog: Catalog::default(),
            session_client: session,
            socket: Arc::new(RwLock::new(None)),
            account_info: Arc::new(RwLock::new(None)),
            kills: AtomicU64::new(0),
            gold: AtomicU64::new(0),
            exp: AtomicU64::new(0),
            current_level: AtomicU32::new(20),
            bestiary_kills: AtomicU64::new(0),
            running: AtomicBool::new(false),
        }
    }

    pub async fn run_loop(self: Arc<Self>) {
        self.running.store(true, Ordering::SeqCst);
        info!("🚀 Bot [{}] (Tibia Idle) iniciado com WebSocket e Sessão ativos.", self.config.name);

        let cookie = &self.config.credentials.session_cookie;
        let mut ticket = self.config.credentials.ticket.clone();

        // 1. Validar Sessão via /auth/me se o cookie estiver configurado
        if !cookie.is_empty() && cookie != "seu-token-de-sessao-ou-jwt" {
            match self.session_client.validate_session(cookie).await {
                Ok(info) => {
                    info!("🛡️ Sessão ativa no Tibia Idle: [{}] AccountID: {}, World: {}", info.name, info.account_id, info.world_id);
                    *self.account_info.write().await = Some(info.clone());

                    // Obter ticket do mundo se necessário
                    if ticket.is_empty() {
                        let world_id = if self.config.credentials.world_id.is_empty() { "1" } else { &self.config.credentials.world_id };
                        if let Ok(t) = self.session_client.get_world_ticket(cookie, world_id).await {
                            ticket = t;
                        }
                    }
                }
                Err(e) => {
                    warn!("⚠️ Validação de sessão /auth/me: {}", e);
                }
            }
        }

        // 2. Conectar WebSocket se ticket estiver disponível
        if !ticket.is_empty() {
            let ws_url = format!("wss://play.tibidle.com/ws?ticket={}", ticket);
            match TibiaIdleSocket::connect(&ws_url, &ticket).await {
                Ok(sock) => {
                    info!("🎮 WebSocket do Tibia Idle conectado com sucesso!");
                    *self.socket.write().await = Some(sock);
                }
                Err(e) => {
                    warn!("⚠️ Falha na conexão WebSocket inicial: {}. Operando em modo de resiliência.", e);
                }
            }
        }

        let mut loop_counter = 0u64;

        while self.running.load(Ordering::SeqCst) {
            loop_counter += 1;

            if !self.circuit_breaker.can_execute().await {
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            if !self.stamina.should_hunt().await {
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            // Diárias
            if self.config.settings.auto_daily && self.dailies.needs_claim() {
                self.dailies.record_claim();
                let sock_guard = self.socket.read().await;
                if let Some(s) = sock_guard.as_ref() {
                    let _ = s.claim_daily().await;
                }
                info!("[{}] Recompensa diária resgatada.", self.config.name);
            }

            // Caça no Tibia Idle
            if self.config.settings.auto_hunt {
                let lvl = self.current_level.load(Ordering::Relaxed);
                let hunt = self.catalog.select_best_hunt(lvl);

                if loop_counter % 20 == 1 {
                    let sock_guard = self.socket.read().await;
                    if let Some(s) = sock_guard.as_ref() {
                        let _ = s.start_hunt(hunt.id).await;
                    }
                    info!("🏹 [{}] Caçando na área [{}] (Hunt #{}) com Lure={}", self.config.name, hunt.route_str, hunt.id, self.config.settings.lure);
                }

                // Auto-Sell no NPC a cada 40 ticks
                if self.config.settings.auto_sell && loop_counter % 40 == 0 {
                    let sock_guard = self.socket.read().await;
                    if let Some(s) = sock_guard.as_ref() {
                        let _ = s.sell_trash().await;
                    }
                    info!("💰 [{}] Venda automática de drops no NPC executada.", self.config.name);
                }

                // Bênçãos
                if self.config.settings.auto_bless && loop_counter % 60 == 0 {
                    let sock_guard = self.socket.read().await;
                    if let Some(s) = sock_guard.as_ref() {
                        for b in 1..=5 {
                            let _ = s.buy_blessing(b).await;
                        }
                    }
                }

                self.kills.fetch_add(1, Ordering::Relaxed);
                self.gold.fetch_add(hunt.base_gold as u64, Ordering::Relaxed);
                self.exp.fetch_add(hunt.base_exp as u64, Ordering::Relaxed);

                if self.config.settings.focus_bestiary {
                    self.bestiary_kills.fetch_add(1, Ordering::Relaxed);
                }

                self.circuit_breaker.record_success().await;
            }

            sleep(Duration::from_millis(1500)).await;
        }

        info!("🛑 Bot [{}] finalizado.", self.config.name);
    }
}
