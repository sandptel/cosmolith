// Watch Input Config Changes

use std::{error::Error, sync::mpsc::Sender};

use cosmic_comp_config::{XkbConfig, KeyboardConfig};
use cosmic_comp_config::input::InputConfig;
use cosmic_config::{Config, ConfigGet};

use crate::event::{
    Event,
    input::{InputConfigDiff, XkbConfigDiff, KeyboardConfigDiff},
};
use std::sync::{Arc, Mutex};

// #todo : Find all the keys linked to  com.system76.CosmicComp and catch those and read events
// implemented
// 1. input_touchpad
// 2. input_default
// 3. xkb_config 
// 4. keyboard_config 
// to be implemented
// 5. workspaces
// 6. pinned_workspaces
// 7. input_touchpad_override
// 8. input_devices 
// 9. autotile
// 10. autotile_behaviour
// 11. active_hint
// 12. focus_follows_cursor
// 13. cursor_follows_focus
// 14. focus_follows_cursor_delay
// 15. descale_xwayland
// 16. xwayland_eavesdropping
// 17. edge_snap_threshold
// 18. accessbility_zoom

pub const INPUTNAMESPACE: &str = "com.system76.CosmicComp";
pub const VERSION: u64 = 1;

pub struct InputState {
    touchpad: Option<InputConfig>,
    mouse: Option<InputConfig>,
    // #todo: Find which exact type is used to emit and monitor changes for this
    // Add that here and then
    // 1. pattern match / 2. add events / 3. impl from() / 4. Events -> Ipc Calls Mapping
    keyboard: Option<XkbConfig>,
    numslock: Option<KeyboardConfig>,
}

pub fn start_input_watcher(
    tx: &Arc<Mutex<Sender<Event>>>,
) -> Result<Box<dyn std::any::Any + Send>, Box<dyn Error>> {
    let config = Config::new(INPUTNAMESPACE, VERSION)?;
    let state = Arc::new(Mutex::new(InputState {
        touchpad: config.get::<InputConfig>("input_touchpad").ok(),
        mouse: config.get::<InputConfig>("input_default").ok(),
        keyboard: config.get::<XkbConfig>("xkb_config").ok(),
        numslock: config.get::<KeyboardConfig>("keyboard_config").ok(),
    }));

    // Keep the watcher alive for the lifetime of the program.
    let watcher = config.watch({
        let tx = Arc::clone(&tx);
        let state = Arc::clone(&state);
        move |cfg: &Config, keys| {
            if let Ok(sender) = tx.lock() {
                if let Ok(mut state) = state.lock() {
                    let events = state.from(cfg, keys);
                    for event in events {
                        if let Err(err) = sender.send(event) {
                            eprintln!("Failed to send input event: {err}");
                        }
                    }
                }
            }
        }
    })?;

    Ok(Box::new(watcher))
}

impl InputState {
    pub fn from(&mut self, cfg: &Config, keys: &[String]) -> Vec<Event> {
        let mut events = Vec::new();
        for key in keys {
            match key.as_str() {
                "input_touchpad" => match cfg.get::<InputConfig>(key) {
                    Ok(new_config) => {
                        if let Some(ref old) = self.touchpad {
                            events.extend(old.from_touchpad(&new_config));
                        }
                        self.touchpad = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                "input_default" => match cfg.get::<InputConfig>(key) {
                    Ok(new_config) => {
                        if let Some(ref old) = self.mouse {
                            events.extend(old.from_mouse(&new_config));
                        }
                        self.mouse = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                "xkb_config" => match cfg.get::<XkbConfig>(key) {
                    Ok(new_config) => {
                        if let Some(ref old) = self.keyboard {
                            events.extend(old.from(&new_config));
                        }
                        self.keyboard = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                "keyboard_config" => match cfg.get::<KeyboardConfig>(key) {
                    Ok(new_config) => {
                        if let Some(ref old) = self.numslock {
                            events.extend(old.from(&new_config));
                        }
                        self.numslock = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                }
                x => {
                    eprintln!(
                        "Unknown key found in Input (com.system76.CosmicComp): {}",
                        x
                    );
                }
            }
        }
        events
    }
}
