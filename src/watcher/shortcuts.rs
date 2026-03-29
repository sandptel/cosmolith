use std::{error::Error, sync::mpsc::Sender};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use cosmic_config::Config;
use cosmic_settings_config::shortcuts::{self, Action, Binding};

use crate::event::{Event, ShortcutEvent};

pub const SHORTCUTS_NAMESPACE: &str = shortcuts::ID;
pub const VERSION: u64 = 1;

pub struct ShortcutsState {
    pub shortcuts: HashMap<Binding, Action>,
}

pub fn start_shortcuts_watcher(
    tx: &Arc<Mutex<Sender<Event>>>,
) -> Result<Box<dyn std::any::Any + Send>, Box<dyn Error>> {
    let config = Config::new(SHORTCUTS_NAMESPACE, VERSION)?;
    
    let initial_shortcuts = shortcuts::shortcuts(&config).0;
    
    let state = Arc::new(Mutex::new(ShortcutsState {
        shortcuts: initial_shortcuts.clone(),
    }));
    
    if let Ok(sender) = tx.lock() {
        for (binding, action) in initial_shortcuts.iter() {
            let _ = sender.send(Event::Shortcut(ShortcutEvent::Add {
                binding: binding.clone(),
                shortcut: action.clone().into(),
            }));
        }
    }

    let watcher = config.watch({
        let tx = Arc::clone(&tx);
        let state = Arc::clone(&state);
        move |cfg: &Config, _keys| {
            if let Ok(sender) = tx.lock() {
                if let Ok(mut state) = state.lock() {
                    let new_shortcuts = shortcuts::shortcuts(cfg).0;
                    let old_shortcuts = state.shortcuts.clone();
                    
                    for (binding, action) in old_shortcuts.iter() {
                        if !new_shortcuts.contains_key(binding) || new_shortcuts.get(binding) != Some(action) {
                            let _ = sender.send(Event::Shortcut(ShortcutEvent::Remove {
                                shortcut: action.clone().into(),
                                binding: binding.clone(),
                            }));
                        }
                    }
                    
                    for (binding, action) in new_shortcuts.iter() {
                        if !old_shortcuts.contains_key(binding) || old_shortcuts.get(binding) != Some(action) {
                            let _ = sender.send(Event::Shortcut(ShortcutEvent::Add {
                                shortcut: action.clone().into(),
                                binding: binding.clone(),
                            }));
                        }
                    }
                    
                    state.shortcuts = new_shortcuts;
                }
            }
        }
    })?;

    Ok(Box::new(watcher))
}
