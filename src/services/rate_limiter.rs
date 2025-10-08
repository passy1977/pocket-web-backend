use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex, LazyLock};
use std::time::SystemTime;
use tokio::time::{interval, Duration as TokioDuration};
use actix_web::{HttpResponse, HttpRequest};
use serde_json::json;

/// Configurazione per i limiti di rate limiting
#[derive(Debug, Clone)]
pub struct RateLimit {
    pub max_requests: u32,
    pub window_seconds: u64,
}

impl RateLimit {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
    
    // Limiti predefiniti per diversi endpoint
    pub fn login_limit() -> Self {
        Self::new(5, 300) // 5 tentativi ogni 5 minuti
    }
    
    pub fn registration_limit() -> Self {
        Self::new(3, 3600) // 3 registrazioni ogni ora
    }
    
    pub fn password_change_limit() -> Self {
        Self::new(3, 3600) // 3 cambi password ogni ora
    }
    
    pub fn api_limit() -> Self {
        Self::new(1000, 3600) // 1000 richieste API ogni ora
    }
    
    pub fn heartbeat_limit() -> Self {
        Self::new(12, 60) // 12 heartbeat ogni minuto
    }
}

/// Entry per tracciare le richieste di un client
#[derive(Debug, Clone)]
struct RequestEntry {
    count: u32,
    window_start: SystemTime,
}

impl RequestEntry {
    fn new() -> Self {
        Self {
            count: 1,
            window_start: SystemTime::now(),
        }
    }
    
    fn is_expired(&self, window_seconds: u64) -> bool {
        if let Ok(elapsed) = self.window_start.elapsed() {
            elapsed.as_secs() >= window_seconds
        } else {
            true // Se non riusciamo a calcolare, consideriamo scaduto
        }
    }
    
    fn reset_window(&mut self) {
        self.count = 1;
        self.window_start = SystemTime::now();
    }
    
    fn increment(&mut self) {
        self.count += 1;
    }
}

/// Rate limiter principale
pub struct RateLimiter {
    // Traccia richieste per IP
    ip_requests: Arc<Mutex<HashMap<IpAddr, HashMap<String, RequestEntry>>>>,
    // Traccia richieste per session ID
    session_requests: Arc<Mutex<HashMap<String, HashMap<String, RequestEntry>>>>,
    // Configurazioni per endpoint
    endpoint_limits: HashMap<String, RateLimit>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let mut endpoint_limits = HashMap::new();
        
        // Configura limiti per endpoint specifici
        endpoint_limits.insert("/v5/pocket/login".to_string(), RateLimit::login_limit());
        endpoint_limits.insert("/v5/pocket/registration".to_string(), RateLimit::registration_limit());
        endpoint_limits.insert("/v5/pocket/change_passwd".to_string(), RateLimit::password_change_limit());
        endpoint_limits.insert("/v5/pocket/heartbeat".to_string(), RateLimit::heartbeat_limit());
        
        // Limite generico per tutti gli altri endpoint API
        endpoint_limits.insert("_default_api".to_string(), RateLimit::api_limit());
        
        let rate_limiter = Self {
            ip_requests: Arc::new(Mutex::new(HashMap::new())),
            session_requests: Arc::new(Mutex::new(HashMap::new())),
            endpoint_limits,
        };
        
        // Avvia il cleanup automatico
        rate_limiter.start_cleanup_task();
        
        rate_limiter
    }
    
    /// Verifica se una richiesta da un IP può essere processata
    pub fn check_ip_rate(&self, ip: IpAddr, endpoint: &str) -> bool {
        let limit = self.get_endpoint_limit(endpoint);
        let endpoint_key = format!("{}_{}", endpoint, "ip");
        
        let mut ip_requests = self.ip_requests.lock().unwrap();
        let ip_map = ip_requests.entry(ip).or_insert_with(HashMap::new);
        
        self.check_rate_internal(ip_map, &endpoint_key, &limit)
    }
    
    /// Verifica se una richiesta da una session può essere processata
    pub fn check_session_rate(&self, session_id: &str, endpoint: &str) -> bool {
        let limit = self.get_endpoint_limit(endpoint);
        let endpoint_key = format!("{}_{}", endpoint, "session");
        
        let mut session_requests = self.session_requests.lock().unwrap();
        let session_map = session_requests.entry(session_id.to_string()).or_insert_with(HashMap::new);
        
        self.check_rate_internal(session_map, &endpoint_key, &limit)
    }
    
    fn check_rate_internal(
        &self,
        request_map: &mut HashMap<String, RequestEntry>,
        endpoint_key: &str,
        limit: &RateLimit,
    ) -> bool {
        match request_map.get_mut(endpoint_key) {
            Some(entry) => {
                if entry.is_expired(limit.window_seconds) {
                    entry.reset_window();
                    true
                } else if entry.count < limit.max_requests {
                    entry.increment();
                    true
                } else {
                    false // Rate limit exceeded
                }
            }
            None => {
                request_map.insert(endpoint_key.to_string(), RequestEntry::new());
                true
            }
        }
    }
    
    fn get_endpoint_limit(&self, endpoint: &str) -> RateLimit {
        self.endpoint_limits
            .get(endpoint)
            .or_else(|| {
                // Se l'endpoint inizia con /v5/pocket/, usa il limite API di default
                if endpoint.starts_with("/v5/pocket/") {
                    self.endpoint_limits.get("_default_api")
                } else {
                    None
                }
            })
            .cloned()
            .unwrap_or_else(|| RateLimit::new(100, 3600)) // Fallback molto restrittivo
    }
    
    /// Avvia un task in background per pulire le entry scadute
    fn start_cleanup_task(&self) {
        // Solo in modalità non-test avvia il task di cleanup
        #[cfg(not(test))]
        {
            let ip_requests = self.ip_requests.clone();
            let session_requests = self.session_requests.clone();
            
            tokio::spawn(async move {
                let mut cleanup_interval = interval(TokioDuration::from_secs(300)); // Ogni 5 minuti
                
                loop {
                    cleanup_interval.tick().await;
                    
                    // Cleanup IP requests
                    {
                        let mut ip_map = ip_requests.lock().unwrap();
                        ip_map.retain(|_, endpoint_map| {
                            endpoint_map.retain(|_, entry| !entry.is_expired(3600)); // Rimuovi entry più vecchie di 1 ora
                            !endpoint_map.is_empty()
                        });
                    }
                    
                    // Cleanup session requests
                    {
                        let mut session_map = session_requests.lock().unwrap();
                        session_map.retain(|_, endpoint_map| {
                            endpoint_map.retain(|_, entry| !entry.is_expired(3600));
                            !endpoint_map.is_empty()
                        });
                    }
                }
            });
        }
    }
}

/// Istanza globale del rate limiter
static RATE_LIMITER: LazyLock<RateLimiter> = LazyLock::new(|| RateLimiter::new());

/// Funzione helper per verificare i rate limit e restituire una risposta di errore se necessario
pub fn check_rate_limit_or_reject(
    req: &HttpRequest,
    endpoint: &str,
    session_id: Option<&str>,
) -> Option<HttpResponse> {
    // Estrai IP del client
    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|addr| addr.parse::<IpAddr>().ok())
        .unwrap_or_else(|| "127.0.0.1".parse().unwrap());
    
    // Verifica rate limit per IP
    if !RATE_LIMITER.check_ip_rate(client_ip, endpoint) {
        return Some(
            HttpResponse::TooManyRequests()
                .json(json!({
                    "error": "Rate limit exceeded. Too many requests from your IP.",
                    "retry_after": 60,
                    "endpoint": endpoint
                }))
        );
    }
    
    // Se c'è un session ID, verifica anche il rate limit per session
    if let Some(sid) = session_id {
        if !RATE_LIMITER.check_session_rate(sid, endpoint) {
            return Some(
                HttpResponse::TooManyRequests()
                    .json(json!({
                        "error": "Rate limit exceeded. Too many requests for this session.",
                        "retry_after": 60,
                        "endpoint": endpoint
                    }))
            );
        }
    }
    
    None // Nessun rate limit violato
}

/// Funzione helper per verificare manualmente i rate limit
// pub fn check_rate_limit(ip: IpAddr, endpoint: &str, session_id: Option<&str>) -> bool {
//     let ip_ok = RATE_LIMITER.check_ip_rate(ip, endpoint);
    
//     if let Some(session) = session_id {
//         let session_ok = RATE_LIMITER.check_session_rate(session, endpoint);
//         ip_ok && session_ok
//     } else {
//         ip_ok
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_rate_limit_creation() {
        let limit = RateLimit::login_limit();
        assert_eq!(limit.max_requests, 5);
        assert_eq!(limit.window_seconds, 300);
    }

    #[test]
    fn test_rate_limiter_ip_limit() {
        let limiter = RateLimiter::new();
        let ip = "127.0.0.1".parse().unwrap();
        let endpoint = "/v5/pocket/login";

        // I primi 5 tentativi dovrebbero passare
        for _ in 0..5 {
            assert!(limiter.check_ip_rate(ip, endpoint));
        }

        // Il 6° tentativo dovrebbe fallire
        assert!(!limiter.check_ip_rate(ip, endpoint));
    }

    #[test]
    fn test_rate_limiter_session_limit() {
        let limiter = RateLimiter::new();
        let session_id = "test_session_123";
        let endpoint = "/v5/pocket/login";

        // I primi 5 tentativi dovrebbero passare
        for _ in 0..5 {
            assert!(limiter.check_session_rate(session_id, endpoint));
        }

        // Il 6° tentativo dovrebbe fallire
        assert!(!limiter.check_session_rate(session_id, endpoint));
    }

    #[test]
    fn test_different_endpoints_separate_limits() {
        let limiter = RateLimiter::new();
        let ip = "127.0.0.1".parse().unwrap();

        // Usa tutti i tentativi per login
        for _ in 0..5 {
            assert!(limiter.check_ip_rate(ip, "/v5/pocket/login"));
        }
        assert!(!limiter.check_ip_rate(ip, "/v5/pocket/login"));

        // Registration dovrebbe ancora funzionare
        assert!(limiter.check_ip_rate(ip, "/v5/pocket/registration"));
    }

    #[test]
    fn test_request_entry_expiration() {
        let mut entry = RequestEntry::new();
        
        // Simula una entry vecchia
        entry.window_start = SystemTime::now() - Duration::from_secs(400);
        
        // Dovrebbe essere scaduta (per una finestra di 300 secondi)
        assert!(entry.is_expired(300));
    }

    #[test]
    fn test_rate_limiter_global_instance() {
        // Test che il rate limiter globale sia accessibile
        let limiter = &*RATE_LIMITER;
        let ip = "192.168.1.1".parse::<std::net::IpAddr>().unwrap();
        let endpoint = "/v5/pocket/test";
        
        // Dovrebbe funzionare per i primi tentativi
        assert!(limiter.check_ip_rate(ip, endpoint));
    }
}