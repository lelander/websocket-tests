extern crate ws;

use ws::{connect, CloseCode};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

const COUNT: usize = 1_000;

fn main() {
    connect("ws://archlinux.local:3000", |out| {
        println!("Connected to websocket");

        let duration = AtomicUsize::new(0);
        let count = AtomicUsize::new(0);
        let start = AtomicUsize::new(0);

        start.store(timestamp() as usize, Ordering::Relaxed);
        count.store(count.load(Ordering::Relaxed) + 1, Ordering::Relaxed);
        out.send("1538011457915").unwrap();

        move |msg| {
            let end = timestamp() as usize;
            duration.store(duration.load(Ordering::Relaxed) + end - start.load(Ordering::Relaxed),
                           Ordering::Relaxed);
            count.store(count.load(Ordering::Relaxed) + 1, Ordering::Relaxed);

            if count.load(Ordering::Relaxed) >= COUNT {
                println!("Round-trip duration: {}us", duration.load(Ordering::Relaxed)/COUNT);
                out.close(CloseCode::Normal)
            } else {
                start.store(timestamp() as usize, Ordering::Relaxed);
                out.send("1538011457915")
            }
        }
    }).unwrap()
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
