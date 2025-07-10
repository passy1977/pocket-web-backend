use crate::bindings::{pocket_field_controller_t, pocket_free, pocket_group_controller_t, pocket_new, pocket_t};
use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use ulid::Ulid;

#[derive(Clone)]
pub struct Session {
    /// Unique identifier (ulid) for users who are using the client
    pub session_id: String,
    
    pub pocket: *mut pocket_t,
    
    pub group_controller: *mut pocket_group_controller_t,

    pub field_controller: *mut pocket_field_controller_t,

    pub timestamp_last_update: u64
}

unsafe impl Send for Sessions {}
unsafe impl Sync for Sessions {}

impl Session {
    pub fn new() -> Session {
        Self {
            session_id: Ulid::new().to_string(),
            pocket: unsafe { pocket_new() },
            group_controller: null_mut(),
            field_controller: null_mut(),
            timestamp_last_update: match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_secs(), 
                Err(_) => 0
            }
        }
    }
    
    pub fn update_timestamp_last_update(&mut self)  {
        self.timestamp_last_update = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0
        };
    }
    
}

pub struct Sessions {
    sessions: Mutex<HashMap<String, Session>>, // Using a HashMap to store sessions by session_id
}



impl Sessions {
    fn new() -> Self {
        Sessions {
            sessions: Mutex::new(HashMap::new()),
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
    pub fn remove(&self, session_id: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        
        if let Some(session) = sessions.get(session_id) {
            
            unsafe {
                if session.pocket != null_mut() {
                    pocket_free(session.pocket);    
                }
            }
            
            sessions.remove(session_id);    
        }
    }
}