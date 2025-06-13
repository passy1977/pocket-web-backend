use std::net::Ipv4Addr;
use std::str::FromStr;
use clap::Parser;
use crate::services::data::Data;

#[derive(Parser)]
pub struct Cli {
    /// Ip where WebSocket listen
    pub ip: Option<String>,

    /// Connection port
    pub port: Option<u16>,

    ///Path and filename where find ssl cert
    pub ssl_cert: Option<String>,

    ///Path and filename where find key cert
    pub ssl_key: Option<String>
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
        
        if let Some(port) = cli.port.as_ref() {
            if data.port != *port {
                data.port = *port;
                data.update = true;
            }
        }

        if let Some(ssl_cert) = cli.ssl_cert.as_ref() {
            if data.ssl_cert != *ssl_cert {
                data.ssl_cert = ssl_cert.clone();
                data.update = true;
            }
        }

        if let Some(ssl_key) = cli.ssl_key.as_ref() {
            if data.ssl_key != *ssl_key {
                data.ssl_key = ssl_key.clone();
                data.update = true;
            }
        }
    }
}
