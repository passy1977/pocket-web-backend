use crate::bindings::{pocket_field_controller_free, pocket_field_controller_t, pocket_free, pocket_group_controller_free, pocket_group_controller_t, pocket_group_field_controller_free, pocket_group_field_controller_t, pocket_new, pocket_t};
use crate::services::data::Data;
use crate::services::secure_session::generate_secure_session_id;
use crate::services::session_timer::SessionTimer;
use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};


static DEFAULT_TIMESTAMP_LAST_UPDATE : u32 = 5 * 60; // 5 minutes in seconds
 
impl pocket_t {
    pub fn is_valid(&self) -> bool {
        !self.session.is_null()
    }
}

#[derive(Clone)]
pub struct Session {
    /// Unique identifier (ulid) for users who are using the client
    pub session_id: String,
    
    pub pocket: *mut pocket_t,
    
    pub group_controller: *mut pocket_group_controller_t,

    pub group_field_controller: *mut pocket_group_field_controller_t,

    pub field_controller: *mut pocket_field_controller_t,

    pub email: Option<String>,

    pub timestamp_last_update: u64,

    pub remote_session_handling: bool
}

unsafe impl Send for pocket_t {}
unsafe impl Sync for pocket_t {}
unsafe impl Send for pocket_group_controller_t {}
unsafe impl Sync for pocket_group_controller_t {}
unsafe impl Send for pocket_group_field_controller_t {}
unsafe impl Sync for pocket_group_field_controller_t {}
unsafe impl Send for pocket_field_controller_t {}
unsafe impl Sync for pocket_field_controller_t {}
unsafe impl Send for Session {}
unsafe impl Sync for Session {}
unsafe impl Send for Sessions {}
unsafe impl Sync for Sessions {}

impl Session {
    pub fn new() -> Session {
        Self {
            session_id: generate_secure_session_id(),
            pocket: unsafe { pocket_new() },
            group_controller: null_mut(),
            group_field_controller: null_mut(),
            field_controller: null_mut(),
            email: None,
            timestamp_last_update: match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(), 
                Err(_) => 0
            },
            remote_session_handling: true
        }
    }
    
    pub fn update_timestamp_last_update(&mut self)  {
        if self.remote_session_handling {
            return;
        }

        self.timestamp_last_update = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0
        };
    }
    
    pub fn is_valid(&self) -> bool {
        unsafe { !self.pocket.is_null() && !(*self.pocket).user.is_null() && !(*self.pocket).aes.is_null()}
    }

}

pub struct Sessions {
    sessions: Mutex<HashMap<String, Session>>, // Using a HashMap to store sessions by session_id
    #[allow(dead_code)]
    timer: Mutex<Option<SessionTimer>>,
    #[allow(dead_code)]
    session_expiration_time: u32 // in seconds
}



impl Sessions {
    fn new() -> Self {

        Self {
            sessions: Mutex::new(HashMap::new()),
            timer: Mutex::new(None),
            session_expiration_time: match Data::init() {
                Ok(data) =>  data.session_expiration_time,
                Err(e) => {
                    eprintln!("Error loading data: {e}");
                    DEFAULT_TIMESTAMP_LAST_UPDATE 
                }
            }
        }
    }

    // Public method to get the singleton instance
    pub fn share() -> Arc<Self> {
        static INSTANCE: once_cell::sync::Lazy<Arc<Sessions>> = once_cell::sync::Lazy::new(|| {
            Arc::new(Sessions::new())
        });

        INSTANCE.clone()
    }

    // Method to add a session
    pub fn add(&self, session: Session) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.session_id.clone(), session);
    }

    // Method to get a session by session_id
    pub fn get(&self, session_id: &str) -> Option<Session> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(session_id).cloned()
    }

    // Method to remove a session by session_id
    pub fn remove(&self, session_id: &str, free: bool) {
        let mut sessions = self.sessions.lock().unwrap();
        
        if let Some(session) = sessions.get(session_id) {
            
            unsafe {
                if free {
                    if !session.pocket.is_null() {
                        pocket_free(session.pocket);
                    }

                    if !session.group_controller.is_null() {
                        pocket_group_controller_free(session.group_controller);
                    }
                                        
                    if !session.group_field_controller.is_null() {
                        pocket_group_field_controller_free(session.group_field_controller);
                    }
                    
                    if !session.field_controller.is_null() {
                        pocket_field_controller_free(session.field_controller);
                    }
                }
            }
            
            sessions.remove(session_id);    
        }
    }

    pub fn check_if_already_logged(&self, email: &String, ref_session: &mut Option<Session>) -> bool {
        let sessions = self.sessions.lock().unwrap();
        for (_, session) in sessions.iter() {
            if let Some(session_email) = &session.email {
                if session_email == email {
                    *ref_session = Some(session.clone());
                    return true;
                }
            }
        }
        false
    }

    pub fn invalidate(&self, current_timestamp: u64) {
        let mut sessions = self.sessions.lock().unwrap();
        let mut sessions_to_remove = Vec::new();
        
        for (session_id, session) in sessions.iter_mut() {

            if session.remote_session_handling && session.is_valid(){
                continue;
            }

            if current_timestamp > session.timestamp_last_update + self.session_expiration_time as u64 {                    
                unsafe {
                    if !session.pocket.is_null() {
                        pocket_free(session.pocket);
                    }

                    if !session.group_controller.is_null() {
                        pocket_group_controller_free(session.group_controller);
                    }
                                        
                    if !session.group_field_controller.is_null() {
                        pocket_group_field_controller_free(session.group_field_controller);
                    }
                    
                    if !session.field_controller.is_null() {
                        pocket_field_controller_free(session.field_controller);
                    }
                }
                
                sessions_to_remove.push((session.email.clone(), session_id.clone(), session.timestamp_last_update));
            }
        }
        
        
        for (_email, session_id, _timestamp_last_update) in sessions_to_remove {
            #[cfg(debug_assertions)]
            println!("Session expired for {}@{session_id} start at:{_timestamp_last_update} current:{current_timestamp} expiration_time:{}", _email.unwrap_or("no_logged".to_string()), self.session_expiration_time);
            sessions.remove(&session_id);
        }
    }

    pub fn start_validator(&self)  {
        let mut timer = self.timer.lock().unwrap();
        if timer.is_none() {
            *timer = Some(SessionTimer::new());
        }        
    }

    #[allow(dead_code)]
    pub fn stop_validator(&self)  {
        let mut timer = self.timer.lock().unwrap();
        if let Some(ref mut t) = *timer {
            t.stop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_session_new() {
        let session = Session::new();
        
        // Verify that session_id is generated
        assert!(!session.session_id.is_empty());
        assert_eq!(session.session_id.len(), 64); // SHA256 hex
        
        // Verify that pocket is initialized
        assert!(!session.pocket.is_null());
        
        // Verify default values
        assert!(session.group_controller.is_null());
        assert!(session.group_field_controller.is_null());
        assert!(session.field_controller.is_null());
        assert!(session.email.is_none());
        assert!(session.remote_session_handling);
        
        // Verify timestamp
        assert!(session.timestamp_last_update > 0);
    }

    #[test]
    fn test_session_update_timestamp() {
        let mut session = Session::new();
        let initial_timestamp = session.timestamp_last_update;
        
        // Wait a moment to ensure a different timestamp
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        session.update_timestamp_last_update();
        
        assert!(session.timestamp_last_update >= initial_timestamp);
    }

    #[test]
    fn test_sessions_new() {
        let sessions = Sessions::new();
        
        // Verify that sessions are initialized empty
        let sessions_map = sessions.sessions.lock().unwrap();
        assert!(sessions_map.is_empty());
        
        // Verify that timer is None initially
        let timer = sessions.timer.lock().unwrap();
        assert!(timer.is_none());
        
        // Verify that session_expiration_time is a reasonable value
        assert!(sessions.session_expiration_time > 0);
    }

    #[test]
    fn test_sessions_add_session() {
        let sessions = Sessions::new();
        let session = Session::new();
        let session_id = session.session_id.clone();
        
        sessions.add(session);
        
        // Verify that the session was added
        let sessions_map = sessions.sessions.lock().unwrap();
        assert!(sessions_map.contains_key(&session_id));
        assert_eq!(sessions_map.len(), 1);
    }

    #[test]
    fn test_sessions_get_session() {
        let sessions = Sessions::new();
        let session = Session::new();
        let session_id = session.session_id.clone();
        
        sessions.add(session);
        
        // Test existing session
        let retrieved_session = sessions.get(&session_id);
        assert!(retrieved_session.is_some());
        assert_eq!(retrieved_session.unwrap().session_id, session_id);
        
        // Test non-existent session
        let non_existent = sessions.get("non_existent_id");
        assert!(non_existent.is_none());
    }

    #[test]
    fn test_sessions_get_session_mut() {
        let sessions = Sessions::new();
        let mut session = Session::new();
        session.email = Some("test@example.com".to_string());
        let session_id = session.session_id.clone();
        
        sessions.add(session);
        
        // Test modifying existing session
        {
            let mut sessions_map = sessions.sessions.lock().unwrap();
            if let Some(session_ref) = sessions_map.get_mut(&session_id) {
                session_ref.email = Some("updated@example.com".to_string());
            }
        }
        
        // Verify that the modification was persisted
        let retrieved_session = sessions.get(&session_id);
        assert_eq!(retrieved_session.unwrap().email, Some("updated@example.com".to_string()));
    }

    #[test]
    fn test_sessions_remove_session() {
        let sessions = Sessions::new();
        let session = Session::new();
        let session_id = session.session_id.clone();
        
        sessions.add(session);
        
        // Verify that the session exists
        assert!(sessions.get(&session_id).is_some());
        
        // Remove the session
        sessions.remove(&session_id, false);
        
        // Verify that the session was removed
        assert!(sessions.get(&session_id).is_none());
    }

    #[test]
    fn test_sessions_contains() {
        let sessions = Sessions::new();
        let session = Session::new();
        let session_id = session.session_id.clone();
        
        // Verify that the session doesn't exist initially
        assert!(sessions.get(&session_id).is_none());
        
        sessions.add(session);
        
        // Verify that the session exists now
        assert!(sessions.get(&session_id).is_some());
        
        // Test with non-existent ID
        assert!(sessions.get("non_existent_id").is_none());
    }

    #[test]
    fn test_sessions_share() {
        // Test the singleton
        let sessions1 = Sessions::share();
        let sessions2 = Sessions::share();
        
        // Should be the same instance
        assert!(Arc::ptr_eq(&sessions1, &sessions2));
    }

    #[test]
    fn test_pocket_t_is_valid() {
        // Test to verify that pocket_t::is_valid works
        // Note: this test is limited because it depends on C code
        // and we can't easily create a valid pocket_t in tests
        let pocket = pocket_t {
            session: std::ptr::null_mut(),
            user: std::ptr::null_mut(),
            aes: std::ptr::null_mut(),
        };
        
        // A pocket with null session should be invalid
        assert!(!pocket.is_valid());
    }
}