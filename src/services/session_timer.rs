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

                let mut count = 0u32;
                loop {
                    if let Ok(stop_signal) = rx.try_recv() {
                        enable_loop = !stop_signal;
                    }
                    if !enable_loop {
                        break;
                    }
                    Sessions::share().invalidate(count);

                    thread::sleep(std::time::Duration::from_millis(LOOP_SLEEP));

                    count += 1;
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
