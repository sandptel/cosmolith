use cosmic_comp_config::input::InputConfig;
use cosmic_config::{Config, ConfigGet};
use std::{
	error::Error,
	sync::{mpsc, Arc, Mutex},
	time::Duration,
};

/// Watch Cosmic compositor configuration and optionally emit verbose logs.
pub fn run_watcher(debug: bool) -> Result<(), Box<dyn Error>> {
	// Load the compositor config (stores touchpad & mouse settings).
	let config = Config::new("com.system76.CosmicComp", 1)?;
	let mut last_touchpad: InputConfig = config
		.get("input_touchpad")
		.unwrap_or_else(|_| InputConfig::default());
	if debug {
		println!("Initial touchpad config:\n{:#?}", last_touchpad);
	}

	// Channel used to receive change notifications from the watcher callback.
	let (tx, rx) = mpsc::channel::<Vec<String>>();
	let tx = Arc::new(Mutex::new(tx));

	// Keep the watcher alive for the lifetime of the program.
	let _watcher = config.watch({
		let tx = Arc::clone(&tx);
		move |_cfg, keys| {
			if let Ok(sender) = tx.lock() {
				let _ = sender.send(keys.to_vec());
			}
		}
	})?;

	if debug {
		println!("Watching for touchpad configuration changes…");
	}

	loop {
		match rx.recv_timeout(Duration::from_secs(5)) {
			Ok(keys) => {
				if keys.iter().any(|k| k == "input_touchpad") {
					match config.get::<InputConfig>("input_touchpad") {
						Ok(updated) if updated != last_touchpad => {
							if debug {
								println!("Touchpad config updated:\n{:#?}", updated);
							}
							last_touchpad = updated;
						}
						Ok(_) => {
							if debug {
								println!("Touchpad config rewritten with identical data.");
							}
						}
						Err(err) => eprintln!("Failed to read updated touchpad config: {err}"),
					}
				}
			}
			Err(mpsc::RecvTimeoutError::Timeout) => continue,
			Err(mpsc::RecvTimeoutError::Disconnected) => {
				eprintln!("Watcher channel closed; exiting.");
				break;
			}
		}
	}

	Ok(())
}
