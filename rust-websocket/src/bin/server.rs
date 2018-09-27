extern crate websocket;

use websocket::{Message, OwnedMessage, WebSocketError};
use websocket::sync::Server;

fn main() {
    let server = Server::bind("0.0.0.0:3000")
        .expect("Unable to listen on port 3000");

    println!("Listening on port 3000...");

    for request in server.filter_map(Result::ok) {
        let mut client = request.accept().unwrap();

        let ip = client.peer_addr().unwrap();

        println!("Connection from {}", ip);

        let (mut receiver, mut sender) = client.split().unwrap();

        for msg in receiver.incoming_messages() {
            match msg {
                Ok(msg) => match msg {
                    OwnedMessage::Text(msg) => {
                        sender.send_message(&Message::text(msg)).unwrap();
                    },
                    OwnedMessage::Binary(data) => {
                        sender.send_message(&Message::binary(data)).unwrap();
                    },
                    OwnedMessage::Close(reason) => {
                        if let Some(reason) = reason {
                            println!("WebSocket closed: [{}] {}", reason.status_code, reason.reason);
                        } else {
                            println!("WebSocket closed");
                        }

                        break;
                    },
                    OwnedMessage::Ping(data) => {
                        sender.send_message(&Message::pong(data)).unwrap();
                    },
                    OwnedMessage::Pong(_dat) => {}
                },
                Err(error) => match error {
                    WebSocketError::NoDataAvailable => {
                        println!("WebSocket terminated unexpectedly");
                        break;
                    },
                    _ => panic!("WebSocket error: {:?}", error)
                }
            }
        }
    }
}
