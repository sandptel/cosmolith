use std::{
	collections::HashMap,
	error::Error,
	sync::mpsc,
	time::Duration,
};

use cosmic_config::{Config, ConfigGet};
use serde_json::Value;

use crate::{
	debug::{print_diffs, print_set},
	namespaces::DEFAULT_NAMESPACES,
};

#[derive(Debug)]
struct Change {
	namespace: String,
	keys: Vec<String>,
}

/// Watch one or more Cosmic configuration namespaces and emit diffs when debug is enabled.
pub fn run_watcher(debug: bool, namespaces: &[String]) -> Result<(), Box<dyn Error>> {
	let watch_list: Vec<String> = if namespaces.is_empty() {
		DEFAULT_NAMESPACES.iter().map(|s| s.to_string()).collect()
	} else {
		namespaces.to_vec()
	};

	let (tx, rx) = mpsc::channel::<Change>();

	let mut configs: HashMap<String, Config> = HashMap::new();
	let mut watchers = Vec::new();
	let mut last_values: HashMap<String, HashMap<String, Value>> = HashMap::new();

	for ns in watch_list {
		match Config::new(&ns, 1) {
			Ok(config) => {
				let tx_clone = tx.clone();
				let ns_clone = ns.clone();
				let watcher = config.watch(move |_cfg, keys| {
					let _ = tx_clone.send(Change {
						namespace: ns_clone.clone(),
						keys: keys.to_vec(),
					});
				})?;

				if debug {
					println!("Watching namespace: {ns}");
				}

				configs.insert(ns.clone(), config);
				watchers.push(watcher);
			}
			Err(err) => {
				eprintln!("Failed to watch {ns}: {err}");
			}
		}
	}

	// Process change notifications.
	loop {
		match rx.recv_timeout(Duration::from_secs(5)) {
			Ok(change) => {
				if let Some(config) = configs.get(&change.namespace) {
					for key in change.keys {
						match config.get::<Value>(&key) {
							Ok(new_val) => {
								let ns_entry = last_values.entry(change.namespace.clone()).or_default();
								match ns_entry.get(&key) {
									Some(old) if old == &new_val => {
										// No actual change in value; ignore.
									}
									Some(old) => {
										if debug {
											print_diffs(&change.namespace, &key, old, &new_val);
										}
									}
									None => {
										if debug {
											print_set(&change.namespace, &key, &new_val);
										}
									}
								}
								ns_entry.insert(key, new_val);
							}
							Err(err) => eprintln!(
								"[{ns}] failed to read {key}: {err}",
								ns = change.namespace
							),
						}
					}
				} else if debug {
					println!("[{ns}] change received but config handle missing", ns = change.namespace);
				}
			}
			Err(mpsc::RecvTimeoutError::Timeout) => continue,
			Err(mpsc::RecvTimeoutError::Disconnected) => {
				eprintln!("Watcher channel closed; exiting.");
				break;
			}
		}
	}

	// Keep watchers alive for lifetime of process.
	drop(watchers);
	Ok(())
}
