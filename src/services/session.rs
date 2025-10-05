use crate::bindings::{pocket_field_controller_free, pocket_field_controller_t, pocket_free, pocket_group_controller_free, pocket_group_controller_t, pocket_group_field_controller_free, pocket_group_field_controller_t, pocket_new, pocket_t};
use crate::services::data::Data;
use crate::services::session_timer::SessionTimer;
use std::collections::HashMap;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use ulid::Ulid;


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

    pub timestamp_last_update: u64    
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
            session_id: Ulid::new().to_string(),
            pocket: unsafe { pocket_new() },
            group_controller: null_mut(),
            group_field_controller: null_mut(),
            field_controller: null_mut(),
            email: None,
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

    pub fn check_if_already_logged(&self, email: &String) -> bool {
        let sessions = self.sessions.lock().unwrap();
        for (_, session) in sessions.iter() {
            if let Some(session_email) = &session.email {
                if session_email == email {
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
            println!("session_id:{session_id} current_timestamp:{current_timestamp} session.timestamp_last_update + self.session_expiration_time:{}", session.timestamp_last_update + self.session_expiration_time as u64);

            if current_timestamp > session.timestamp_last_update + self.session_expiration_time as u64 {
                println!("Invalidating session: {}", session_id);
                    
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
                
                sessions_to_remove.push(session_id.clone());
            }
        }
        

        for session_id in sessions_to_remove {
            sessions.remove(&session_id);
        }
    }

    pub fn start_validator(&self)  {
        let mut timer = self.timer.lock().unwrap();
        if timer.is_none() {
            *timer = Some(SessionTimer::new());
        }        
    }

    pub fn stop_validator(&self)  {
        let mut timer = self.timer.lock().unwrap();
        if let Some(ref mut t) = *timer {
            t.stop();
        }
    }
}