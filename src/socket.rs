use std::net::Ipv4Addr;

pub(crate) mod ws_conn;
pub(crate) mod ws_server;


//openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout ssl_privkey.pem -out ssl_cert.pem
pub async fn start(ip: String, port: u16, ssl_cert: &str, ssl_key: &str) {
    
    let ip_addr: Ipv4Addr = ip.parse().unwrap();
    
    ws_server::start(ip_addr, port, ssl_cert, ssl_key).await;
}