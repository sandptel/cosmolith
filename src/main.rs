// src/main.rs
use cosmic_config::Config;
use std::{
    error::Error,
    sync::{Arc, Mutex, mpsc},
    time::Duration,
};

mod watcher;
use watcher::input::start_input_watcher;
mod event;
use event::Event;

mod identifier;
use identifier::get_current_session;

fn main() -> Result<(), Box<dyn Error>> {
    let _config = Config::new("com.system76.CosmicComp", 1)?;
    // Channel used to receive change notifications from the watcher callback.
    let (tx, rx) = mpsc::channel::<Event>();
    let tx = Arc::new(Mutex::new(tx));

    let _watcher = start_input_watcher(&tx)?;

    println!("Watching for configuration changes…");

    let session = get_current_session();
    println!("You are currently running: {:?}", session);

    loop {
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(event) => {
                println!("Recieved: {:?}", event);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // optional heartbeat to keep the loop responsive to Ctrl+C
                continue;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                eprintln!("Watcher channel closed; exiting.");
                break;
            }
        }
    }

    Ok(())
}
