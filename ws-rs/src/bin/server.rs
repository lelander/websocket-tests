extern crate ws;

use ws::listen;

fn main() {
  listen("0.0.0.0:3000", |out| {
      println!("Connected to websocket client");
      move |msg| {
         out.send(msg)
      }
  }).unwrap()
}
