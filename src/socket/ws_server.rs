//! Module to implement websocket server to receive and process incoming requests from clients

use std::{collections::HashMap, convert::Infallible, net::Ipv4Addr, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{
    reject::Rejection,
    reply::Reply,
    ws::{Message, Ws},
    Filter,
};
use crate::services::data::Data;
use crate::socket::{ws_conn};

#[derive(Debug, Clone)]
/// Defines the structure for connected websocket client
pub struct Client {
    /// Unique identifier (ulid) for users who are using the client
    pub user_id: String,

    /// Sender is used to send messages to the connected client (mpsc::UnboundedReceiver)
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

/// Map of connection IDs for clients that can be safely passed across threads
pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

/// Function to extract the "Clients" data and return a Filter matching any route.
pub fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

/// Function to run the websocket server on given IP address and Port number
pub async fn start(data: &Data) {

    let ip_addr: Ipv4Addr = data.ip.parse().unwrap();

    // ws_server::start(ip_addr, data.port, &data.ssl_cert, &data.ssl_key).await;
    
    // Creating new instance of the "Clients" type
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Configuring websocket route");

    // Creating the websocket route for the server
    // 1) Define the path as "ws". So the full path for the client will become: <ip_addr>:<port>/ws
    // 2) Add a WebSocket filter that yields a "Ws" object that will be used to upgrade the connection to a WebSocket connection.
    // 3) Add new instance of "Clients" type
    // 4) Configure the handler function that is called to handle this route.
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(ws_handler);

    // Adding a CORS filter that allows any origin
    let routes = ws_route.with(warp::cors().allow_any_origin());

    println!("Starting server @ {}:{}", ip_addr, data.port);

    // Running the Warp server on given IP address and Port number
    //warp::serve(routes).run((ip_addr, port)).await;
    warp::serve(routes)
        .tls()
        .cert_path(&data.ssl_cert)
        .key_path(&data.ssl_key)
        .run((ip_addr, data.port)).await;
}

/// Handler function to receive the HashMap of clients, and pass this to the client_connection function in the ws module
async fn ws_handler(ws: Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    // Websocket protocol upgrade for handling incoming communications
    Ok(ws.on_upgrade(move |socket| ws_conn::client_connection(socket, clients)))
}