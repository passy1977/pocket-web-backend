use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::sync::mpsc; 

use crate::services::session::Sessions;

static LOOP_SLEEP: u64 = 1_000; // 1s


pub struct SessionTimer {
    handle: Option<JoinHandle<()>>,
    channel: Sender<bool>
}

impl SessionTimer {

    #[allow(dead_code)]
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<bool>();


        let thread = thread::spawn(move || {
                
                let mut enable_loop = true;

                loop {
                    if let Ok(stop_signal) = rx.try_recv() {
                        enable_loop = !stop_signal;
                    }
                    if !enable_loop {
                        break;
                    }
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();

                    Sessions::share().invalidate(current_time);

                    thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP));
                }
            });

        Self {
            handle: Some(thread), 
            channel: tx
        }
    }
    
    pub fn stop(&mut self) {
        let _ = self.channel.send(false);
         
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for SessionTimer {
    fn drop(&mut self) {
        self.stop();
    }
}
