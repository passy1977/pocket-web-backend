use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::bindings::pocket_t;

#[derive(Clone)]
pub struct Session {
    /// Unique identifier (ulid) for users who are using the client
    pub user_id: String,

    pub jwt: String,
    
    pub timestamp_last_update: u64
}

pub struct Sessions {
    sessions: Mutex<HashMap<String, Session>>, // Using a HashMap to store sessions by user_id
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
    pub fn add_session(&self, session: Session) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session.user_id.clone(), session);
    }

    // Method to get a session by user_id
    pub fn get_session(&self, user_id: &str) -> Option<Session> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(user_id).cloned()
    }

    // Method to remove a session by user_id
    pub fn remove_session(&self, user_id: &str) {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(user_id);
    }
}