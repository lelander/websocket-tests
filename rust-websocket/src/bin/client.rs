#[macro_use] extern crate failure;
extern crate websocket;

use failure::Error;
use std::net::TcpStream;
use std::time::Duration;
use websocket::client::ClientBuilder;
use websocket::sync::Client;
use websocket::{Message, OwnedMessage};

fn main() {
    let mut client = ClientBuilder::new("ws://localhost:3000")
	    .expect("Invalid socket URL")
	    .add_protocol("rust-websocket")
	    .connect_insecure()
        .expect("Unable to connect to socket server");

    println!("Connected to websocket server");

    let mut duration = 0;
    let count = 100_000;

    for _ in 0..count {
        let start = timestamp();

        client.send_message(&Message::text("1538011457915"))
            .expect("Unable to communicate with socket server");
        let _reply = client.recv_text_message()
            .expect("Unable to communicate with socket server");

        let end = timestamp();

        duration += end - start;
    }

    println!("Round-trip time: {}us", duration/count);
}

pub trait ClientExt {
    fn recv_text_message(&mut self) -> Result<String, Error>;
}

impl ClientExt for Client<TcpStream> {
    fn recv_text_message(&mut self) -> Result<String, Error> {
        loop {
            match self.recv_message()? {
                OwnedMessage::Text(msg) => {
                    return Ok(msg);
                },
                OwnedMessage::Binary(_) => {
                    bail!("Expected text message but got binary message");
                },
                OwnedMessage::Close(_data) => {
                    bail!("Web socket closed before expected message");
                },
                OwnedMessage::Ping(data) => {
                    self.send_message(&Message::pong(data))?;
                },
                OwnedMessage::Pong(_dat) => {}
            }
        }
    }
}

fn timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    duration_as_micros(since_the_epoch)
}

fn duration_as_micros(duration: Duration) -> u64 {
    duration.as_secs() * 1_000_000 + duration.subsec_nanos() as u64 / 1_000
}
