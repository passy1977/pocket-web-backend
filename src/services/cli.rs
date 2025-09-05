use std::net::Ipv4Addr;
use std::str::FromStr;
use clap::Parser;
use crate::services::data::Data;

#[derive(Parser)]
pub struct Cli {
    /// Host ip
    pub ip: Option<String>,

    /// Connection port
    pub port: Option<u16>,

    pub max_threads: Option<usize>
}

impl Cli {
    pub fn update(data: &mut Data) {
        let cli = Cli::parse();

        if let Some(ip) = cli.ip.as_ref() {
            if let Ok(_)  = Ipv4Addr::from_str(ip.as_str()) {
                if data.ip != *ip {
                    data.ip = ip.clone();
                    data.update = true;
                }    
            }
        }
        
        if let Some(port) = cli.port {
            if data.port != port {
                data.port = port;
                data.update = true;
            }
        }

        if let Some(max_threads) = cli.max_threads {
            if data.max_threads != max_threads {
                data.max_threads = max_threads;
                data.update = true;
            }
        }
        
    }
}
