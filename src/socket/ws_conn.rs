//! Module to handle the established websocket connection and receiving messages from the websocket client

use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use ulid::Ulid;
use warp::ws::{Message, WebSocket};
use crate::socket::ws_server::{Client, Clients};

/// Establishes a websocket connection with given client
pub async fn client_connection(ws: WebSocket, clients: Clients) {
    println!("Establishing client connection... {:?}", ws);
    
    // Splitting the WebSocket stream object into separate Sender and Receiver objects, for individual tasks ownership
    let (client_ws_sender, mut client_ws_receiver) = ws.split();

    // Creating unbounded channel and splitting into sender and receiver streams
    let (client_sender, client_receiver) = mpsc::unbounded_channel();

    // Defining the receiver stream for receiving messages from channel
    let client_receiver = UnboundedReceiverStream::new(client_receiver);

    // Spawning separate thread for keeping the sender stream open until the client has disconnected
    tokio::task::spawn(client_receiver.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("Failed to send message using websocket - {}", e.to_string());
        }
    }));

    // Generating unique identifier for the user
    let ulid: String = Ulid::new().to_string();

    // Creating new "Client" struct instance for given user and sender stream
    let new_client: Client = Client {
        user_id: ulid.clone(),
        sender: Some(client_sender),
    };

    // Acquiring lock on the client list and inserting the "new_client" object into the clients HashMap
    clients.lock().await.insert(ulid.clone(), new_client);

    // Loop to handle the incoming messages from the client
    // The loop will keep running until the client is disconnected.
    while let Some(result) = client_ws_receiver.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!(
                    "Failed to receive message using websocket - {}",
                    e.to_string()
                );
                break;
            }
        };
        client_msg(&ulid, msg, &clients).await;
    }

    clients.lock().await.remove(&ulid);
    println!("Websocket disconnected.");
}

/// Send and receive message to/from given client
async fn client_msg(user_id: &str, msg: Message, clients: &Clients) {
    println!("Received message from {}: {:?}", user_id, msg);

    let message: &str = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        let locked = clients.lock().await;
        match locked.get(user_id) {
            Some(v) => {
                if let Some(sender) = &v.sender {
                    println!("Sending pong");
                    let _ = sender.send(Ok(Message::text("pong")));
                }
            }
            None => return,
        }
        return;
    };
}