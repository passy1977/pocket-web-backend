use std::net::Ipv4Addr;
use std::str::FromStr;
use clap::Parser;
use crate::services::data::Data;

#[derive(Parser)]
#[command(name = "pocket-web-backend")]
#[command(about = "A secure, high-performance web backend built with Rust and Actix Web")]
pub struct Cli {
    /// Host address
    pub address: Option<String>,

    /// Maximum number of threads
    pub max_threads: Option<usize>,

    /// Session expiration time in seconds
    pub session_expiration_time: Option<u32>
}

impl Cli {
    pub fn update(data: &mut Data) {
        let cli = Cli::parse();

        if let Some(address) = cli.address.as_ref() {
            if let Ok(_)  = Ipv4Addr::from_str(address.as_str()) {
                if data.address != *address {
                    data.address = address.clone();
                    data.update = true;
                }    
            }
        }

        if let Some(max_threads) = cli.max_threads {
            if data.max_threads != max_threads {
                data.max_threads = max_threads;
                data.update = true;
            }
        }

        if let Some(session_expiration_time) = cli.session_expiration_time {
            if data.session_expiration_time != session_expiration_time {
                data.session_expiration_time = session_expiration_time;
                data.update = true;
            }
        }
    }
}
