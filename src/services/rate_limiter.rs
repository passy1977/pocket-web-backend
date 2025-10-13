use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex, LazyLock};
use std::time::SystemTime;
#[allow(unused_imports)]
use tokio::time::{interval, Duration as TokioDuration};
use actix_web::{HttpResponse, HttpRequest};
use serde_json::json;

/// Configuration for rate limiting
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
    
    // Default limits for different endpoints
    pub fn login_limit() -> Self {
        Self::new(5, 300) // 5 attempts every 5 minutes
    }
    
    pub fn registration_limit() -> Self {
        Self::new(3, 3600) // 3 registrations every hour
    }
    
    pub fn password_change_limit() -> Self {
        Self::new(6, 3600) // 6 password changes every hour
    }
    
    pub fn api_limit() -> Self {
        Self::new(1000, 3600) // 1000 API requests every hour
    }
    
    pub fn heartbeat_limit() -> Self {
        Self::new(12, 60) // 12 heartbeats every minute
    }
}

/// Entry to track client requests
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
            true // If we can't calculate, consider it expired
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

/// Main rate limiter
pub struct RateLimiter {
    // Track requests by IP
    ip_requests: Arc<Mutex<HashMap<IpAddr, HashMap<String, RequestEntry>>>>,
    // Track requests by session ID
    session_requests: Arc<Mutex<HashMap<String, HashMap<String, RequestEntry>>>>,
    // Configurations for endpoints
    endpoint_limits: HashMap<String, RateLimit>,
}

impl RateLimiter {
    pub fn new() -> Self {
        let mut endpoint_limits = HashMap::new();
        
        // Configure limits for specific endpoints
        endpoint_limits.insert("/v5/pocket/login".to_string(), RateLimit::login_limit());
        endpoint_limits.insert("/v5/pocket/registration".to_string(), RateLimit::registration_limit());
        endpoint_limits.insert("/v5/pocket/change_passwd".to_string(), RateLimit::password_change_limit());
        endpoint_limits.insert("/v5/pocket/heartbeat".to_string(), RateLimit::heartbeat_limit());
        
        // Generic limit for all other API endpoints
        endpoint_limits.insert("_default_api".to_string(), RateLimit::api_limit());
        
        let rate_limiter = Self {
            ip_requests: Arc::new(Mutex::new(HashMap::new())),
            session_requests: Arc::new(Mutex::new(HashMap::new())),
            endpoint_limits,
        };
        
        // Start automatic cleanup
        rate_limiter.start_cleanup_task();
        
        rate_limiter
    }
    
    /// Verify if a request from an IP can be processed
    pub fn check_ip_rate(&self, ip: IpAddr, endpoint: &str) -> bool {
        let limit = self.get_endpoint_limit(endpoint);
        let endpoint_key = format!("{}_{}", endpoint, "ip");
        
        let mut ip_requests = self.ip_requests.lock().unwrap();
        let ip_map = ip_requests.entry(ip).or_insert_with(HashMap::new);
        
        self.check_rate_internal(ip_map, &endpoint_key, &limit)
    }
    
    /// Verify if a request from a session can be processed
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
                // If the endpoint starts with /v5/pocket/, use the default API limit
                if endpoint.starts_with("/v5/pocket/") {
                    self.endpoint_limits.get("_default_api")
                } else {
                    None
                }
            })
            .cloned()
            .unwrap_or_else(|| RateLimit::new(100, 3600)) // Very restrictive fallback
    }
    
    /// Start a background task to clean expired entries
    fn start_cleanup_task(&self) {
        // Only start cleanup task in non-test mode
        #[cfg(not(test))]
        {
            let ip_requests = self.ip_requests.clone();
            let session_requests = self.session_requests.clone();
            
            tokio::spawn(async move {
                let mut cleanup_interval = interval(TokioDuration::from_secs(300)); // Every 5 minutes
                
                loop {
                    cleanup_interval.tick().await;
                    
                    // Cleanup IP requests
                    {
                        let mut ip_map = ip_requests.lock().unwrap();
                        ip_map.retain(|_, endpoint_map| {
                            endpoint_map.retain(|_, entry| !entry.is_expired(3600)); // Remove entries older than 1 hour
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

/// Global rate limiter instance
static RATE_LIMITER: LazyLock<RateLimiter> = LazyLock::new(|| RateLimiter::new());

/// Helper function to check rate limits and return error response if necessary
pub fn check_rate_limit_or_reject(
    req: &HttpRequest,
    endpoint: &str,
    session_id: Option<&str>,
) -> Option<HttpResponse> {
    // Extract client IP
    let client_ip = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|addr| addr.parse::<IpAddr>().ok())
        .unwrap_or_else(|| "127.0.0.1".parse().unwrap());
    
    // Check rate limit for IP
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
    
    // If there's a session ID, also check rate limit for session
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
    
    None // No rate limit violated
}


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

        // Registration should still work
        assert!(limiter.check_ip_rate(ip, "/v5/pocket/registration"));
    }

    #[test]
    fn test_request_entry_expiration() {
        let mut entry = RequestEntry::new();
        
        // Simulate an old entry
        entry.window_start = SystemTime::now() - Duration::from_secs(400);
        
        // Should be expired (for a 300 second window)
        assert!(entry.is_expired(300));
    }

    #[test]
    fn test_rate_limiter_global_instance() {
        // Test that the global rate limiter is accessible
        let limiter = &*RATE_LIMITER;
        let ip = "192.168.1.1".parse::<std::net::IpAddr>().unwrap();
        let endpoint = "/v5/pocket/test";
        
        // Should work for the first attempts
        assert!(limiter.check_ip_rate(ip, endpoint));
    }
}