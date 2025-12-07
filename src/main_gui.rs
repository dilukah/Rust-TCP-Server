// GUI build: Same TCP server, but also with UI.

mod shared;
mod server;
mod gui;

use server::{start_tcp_server, ClientMap};
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;

fn main() {
    let clients: ClientMap = Arc::new(Mutex::new(Default::default()));

    // Start TCP server on its own thread
    std::thread::spawn({
        let c = clients.clone();
        move || {
            Runtime::new()
                .unwrap()
                .block_on(start_tcp_server("0.0.0.0:9000", c))
                .unwrap();
        }
    });

    println!("Starting GUI mode...");
    gui::start_gui(clients);
}