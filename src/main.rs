use jupyter_client::Client;
use std::thread;

fn main() {
    let client = Client::existing().unwrap();

    // Set up the heartbeat watcher
    let hb_receiver = client.heartbeat();
    thread::spawn(move || {
        for _ in hb_receiver {
            println!("Received heartbeat from kernel");
        }
    });
}
