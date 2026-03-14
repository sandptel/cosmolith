// Watch Input Config Changes

use std::{error::Error, sync::mpsc::Sender};

use cosmic_comp_config::input::InputConfig;
use cosmic_comp_config::{KeyboardConfig, XkbConfig};
use cosmic_config::Error as ConfigError;
use cosmic_config::{Config, ConfigGet};

use crate::event::{
    Event,
    input::{KeyboardEvent, MouseEvent, TouchpadEvent},
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

fn startup_config_value<T>(result: Result<T, ConfigError>) -> Result<Option<T>, ConfigError> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(ConfigError::NotFound) => Ok(None),
        Err(ConfigError::GetKey(_, err)) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

fn startup_effective_value<T>(
    local: Result<T, ConfigError>,
    effective: Result<T, ConfigError>,
) -> Result<Option<T>, ConfigError> {
    let local = startup_config_value(local)?;
    let effective = startup_config_value(effective)?;

    match (local, effective) {
        (Some(_), Some(value)) => Ok(Some(value)),
        (Some(local_value), None) => Ok(Some(local_value)),
        (None, Some(_)) | (None, None) => Ok(None),
    }
}

fn startup_keyboard_events(
    local_keyboard: Result<XkbConfig, ConfigError>,
    effective_keyboard: Result<XkbConfig, ConfigError>,
    local_numslock: Result<KeyboardConfig, ConfigError>,
    effective_numslock: Result<KeyboardConfig, ConfigError>,
) -> Vec<Event> {
    let keyboard = match startup_effective_value(local_keyboard, effective_keyboard) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to load startup xkb_config: {err}");
            None
        }
    };
    let numslock = match startup_effective_value(local_numslock, effective_numslock) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to load startup keyboard_config: {err}");
            None
        }
    };

    let mut events = Vec::new();
    if let Some(keyboard) = keyboard {
        events.extend(KeyboardEvent::bootstrap_from(keyboard));
    }
    if let Some(numslock) = numslock {
        events.extend(KeyboardEvent::bootstrap_from_keyboard_config(numslock));
    }
    events
}

pub fn load_initial_input_events() -> Result<Vec<Event>, Box<dyn Error>> {
    let config = Config::new(INPUTNAMESPACE, VERSION)?;
    Ok(startup_keyboard_events(
        config.get_local::<XkbConfig>("xkb_config"),
        config.get::<XkbConfig>("xkb_config"),
        config.get_local::<KeyboardConfig>("keyboard_config"),
        config.get::<KeyboardConfig>("keyboard_config"),
    ))
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
                        if let Some(old) = self.touchpad.clone() {
                            events.extend(TouchpadEvent::from(old, new_config.clone()));
                        }
                        self.touchpad = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                "input_default" => match cfg.get::<InputConfig>(key) {
                    Ok(new_config) => {
                        if let Some(old) = self.mouse.clone() {
                            events.extend(MouseEvent::from(old, new_config.clone()));
                        }
                        self.mouse = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                "xkb_config" => match cfg.get::<XkbConfig>(key) {
                    Ok(new_config) => {
                        if let Some(old) = self.keyboard.clone() {
                            events.extend(KeyboardEvent::from(old, new_config.clone()));
                        }
                        self.keyboard = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                "keyboard_config" => match cfg.get::<KeyboardConfig>(key) {
                    Ok(new_config) => {
                        if let Some(old) = self.numslock.clone() {
                            events.extend(KeyboardEvent::from_keyboard_config(
                                old,
                                new_config.clone(),
                            ));
                        }
                        self.numslock = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::input::InputEvent;
    use cosmic_comp_config::NumlockState;
    use cosmic_config::Error as ConfigError;
    use std::io;

    #[test]
    fn startup_keyboard_events_skip_missing_keys_without_defaulting() {
        let events = startup_keyboard_events(
            Err(ConfigError::NotFound),
            Err(ConfigError::NotFound),
            Err(ConfigError::NotFound),
            Err(ConfigError::NotFound),
        );

        assert!(events.is_empty());
    }

    #[test]
    fn startup_keyboard_events_skip_bad_keyboard_read_but_keep_other_values() {
        let events = startup_keyboard_events(
            Err(ConfigError::GetKey(
                "xkb_config".into(),
                io::Error::new(io::ErrorKind::InvalidData, "broken xkb config"),
            )),
            Ok(XkbConfig {
                layout: "us".into(),
                ..XkbConfig::default()
            }),
            Ok(KeyboardConfig {
                numlock_state: NumlockState::BootOn,
            }),
            Ok(KeyboardConfig {
                numlock_state: NumlockState::BootOn,
            }),
        );

        assert_eq!(events.len(), 1);
        assert!(matches!(
            events.first(),
            Some(Event::Input(InputEvent::Keyboard(KeyboardEvent::NumLock(
                NumlockState::BootOn
            ))))
        ));
    }

    #[test]
    fn startup_keyboard_events_skip_missing_default_files() {
        let events = startup_keyboard_events(
            Err(ConfigError::GetKey(
                "xkb_config".into(),
                io::Error::new(io::ErrorKind::NotFound, "missing default"),
            )),
            Err(ConfigError::GetKey(
                "xkb_config".into(),
                io::Error::new(io::ErrorKind::NotFound, "missing default"),
            )),
            Err(ConfigError::GetKey(
                "keyboard_config".into(),
                io::Error::new(io::ErrorKind::NotFound, "missing default"),
            )),
            Err(ConfigError::GetKey(
                "keyboard_config".into(),
                io::Error::new(io::ErrorKind::NotFound, "missing default"),
            )),
        );

        assert!(events.is_empty());
    }

    #[test]
    fn startup_keyboard_events_skip_effective_only_numlock_without_local_override() {
        let events = startup_keyboard_events(
            Err(ConfigError::NotFound),
            Err(ConfigError::NotFound),
            Err(ConfigError::NotFound),
            Ok(KeyboardConfig {
                numlock_state: NumlockState::BootOn,
            }),
        );

        assert!(events.is_empty());
    }

    #[test]
    fn startup_effective_value_skips_packaged_defaults_without_local_override() {
        let value = startup_effective_value::<XkbConfig>(
            Err(ConfigError::NotFound),
            Ok(XkbConfig::default()),
        )
        .unwrap();

        assert!(value.is_none());
    }

    #[test]
    fn startup_effective_value_skips_system_values_without_local_override() {
        let value = startup_effective_value(
            Err(ConfigError::NotFound),
            Ok(XkbConfig {
                layout: "us".into(),
                options: Some("compose:rctrl".into()),
                repeat_delay: 600,
                repeat_rate: 25,
                ..XkbConfig::default()
            }),
        )
        .unwrap();

        assert!(value.is_none());
    }

    #[test]
    fn startup_effective_value_preserves_explicit_local_reset_values() {
        let value = startup_effective_value(
            Ok(XkbConfig::default()),
            Ok(XkbConfig::default()),
        )
        .unwrap();

        assert!(matches!(value, Some(config) if config.layout.is_empty()));
    }

    #[test]
    fn startup_effective_value_uses_effective_value_when_local_override_exists() {
        let value = startup_effective_value(
            Ok(XkbConfig::default()),
            Ok(XkbConfig {
                layout: "us".into(),
                ..XkbConfig::default()
            }),
        )
        .unwrap();

        assert!(matches!(value, Some(config) if config.layout == "us"));
    }
}
