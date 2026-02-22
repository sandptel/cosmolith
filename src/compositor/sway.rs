use std::env;
use std::sync::Mutex;

use swayipc::Connection;

use crate::compositor::input::{Input, InputResult};
use crate::compositor::{Compositor, CompositorResult};
use crate::event::Event;
use crate::event::input::InputEvent;

use cosmic_comp_config::input::{
    AccelConfig, AccelProfile, ClickMethod, ScrollConfig, ScrollMethod, TapConfig,
};
use cosmic_comp_config::NumlockState;

#[derive(Debug, Default)]
pub struct Sway {
    connection: Mutex<Option<Connection>>,
}

impl Sway {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }

    fn bool_to_sway(value: bool) -> &'static str {
        if value { "enabled" } else { "disabled" }
    }

    fn map_click_method(method: &ClickMethod) -> &'static str {
        match method {
            ClickMethod::ButtonAreas => "button_areas",
            ClickMethod::Clickfinger => "clickfinger",
            _ => "none",
        }
    }

    fn map_scroll_method(method: &ScrollMethod) -> &'static str {
        match method {
            ScrollMethod::TwoFinger => "two_finger",
            ScrollMethod::Edge => "edge",
            ScrollMethod::OnButtonDown => "on_button",
            ScrollMethod::NoScroll => "none",
            _ => "none",
        }
    }

    fn map_accel_profile(profile: &AccelProfile) -> &'static str {
        match profile {
            AccelProfile::Flat => "flat",
            AccelProfile::Adaptive => "adaptive",
            _ => "none",
        }
    }

    fn clamp_speed(speed: f64) -> f64 {
        speed.max(-1.0).min(1.0)
    }

    fn run_command(&self, cmd: String) -> InputResult {
        let mut guard = self.connection.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Sway connection lock poisoned")
        })?;

        if guard.is_none() {
            *guard = Some(Connection::new()?);
        }

        let result = guard.as_mut().unwrap().run_command(&cmd);
        match result {
            Ok(results) => {
                for res in results {
                    if let Err(err) = res {
                        eprintln!("Sway command error: {err}");
                    }
                }
                Ok(())
            }
            Err(err) => {
                eprintln!("Sway IPC error: {err}. Reconnecting...");
                *guard = Some(Connection::new()?);
                let results = guard.as_mut().unwrap().run_command(&cmd)?;
                for res in results {
                    if let Err(err) = res {
                        eprintln!("Sway command error: {err}");
                    }
                }
                Ok(())
            }
        }
    }

    fn set_bool(&self, target: &str, setting: &str, value: Option<bool>) -> InputResult {
        if let Some(value) = value {
            let val = Self::bool_to_sway(value);
            return self.run_command(format!("input {target} {setting} {val}"));
        }
        Ok(())
    }

    fn set_bool_required(&self, target: &str, setting: &str, value: bool) -> InputResult {
        let val = Self::bool_to_sway(value);
        self.run_command(format!("input {target} {setting} {val}"))
    }

    fn normalize_kb_options(options: &str) -> String {
        // Sway expects a clean comma-separated list without leading commas or empty segments.
        options
            .trim_matches(|c: char| c == ',' || c.is_whitespace())
            .split(',')
            .filter_map(|part| {
                let trimmed = part.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Compositor for Sway {
    fn init(&mut self) -> CompositorResult {
        let mut guard = self.connection.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Sway connection lock poisoned")
        })?;
        *guard = Some(Connection::new()?);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Sway"
    }

    fn is_running(&self) -> bool {
        env::var("SWAYSOCK").is_ok()
    }

    fn supports(&self, event: &Event) -> bool {
        matches!(event, Event::Input(_))
    }

    fn apply_event(&self, event: Event) -> CompositorResult {
        match event {
            Event::Input(InputEvent::TouchPad(ev)) => self.apply_touchpad_event(ev),
            Event::Input(InputEvent::Mouse(ev)) => self.apply_mouse_event(ev),
            Event::Input(InputEvent::Keyboard(ev)) => self.apply_keyboard_event(ev),
        }
    }

    fn reload(&self) -> CompositorResult {
        Ok(())
    }

    fn shutdown(&self) -> CompositorResult {
        Ok(())
    }
}

// #todo: For all Ok(()) if there exists a if let Some(),
// define a error variants for such errors and send upwards
impl Input for Sway {
    fn keyboard_rules(&self, rules: String) -> InputResult {
        self.run_command(format!("input type:keyboard xkb_rules {rules}"))
    }

    fn keyboard_model(&self, model: String) -> InputResult {
        self.run_command(format!("input type:keyboard xkb_model {model}"))
    }

    fn keyboard_layout(&self, layout: String) -> InputResult {
        self.run_command(format!("input type:keyboard xkb_layout {layout}"))
    }

    fn keyboard_variant(&self, variant: String) -> InputResult {
        self.run_command(format!("input type:keyboard xkb_variant {variant}"))
    }

    fn keyboard_options(&self, options: Option<String>) -> InputResult {
        if let Some(options) = options {
            let cleaned = Self::normalize_kb_options(&options);
            return self.run_command(format!("input type:keyboard xkb_options {cleaned}"));
        }
        Ok(())
    }

    fn keyboard_repeat_delay(&self, delay: u32) -> InputResult {
        self.run_command(format!("input type:keyboard repeat_delay {delay}"))
    }

    fn keyboard_repeat_rate(&self, rate: u32) -> InputResult {
        self.run_command(format!("input type:keyboard repeat_rate {rate}"))
    }

    fn numslock_state(&self, state: NumlockState) -> InputResult {
        match state {
            NumlockState::BootOn => self.run_command("input type:keyboard xkb_numlock enabled".to_string()),
            NumlockState::BootOff => self.run_command("input type:keyboard xkb_numlock disabled".to_string()),
            NumlockState::LastBoot => Ok(()), // Don't change
        }
    }

    // fn touchpad_state(&self, _state: DeviceState) -> InputResult {
    //     // TODO: Requires device-specific identifiers; DisabledOnExternalMouse not supported.
    //     dbg!("Sway: touchpad enable/disable not supported via type:touchpad");
    //     Ok(())
    // }

    fn touchpad_acceleration(&self, accel: Option<AccelConfig>) -> InputResult {
        if let Some(accel) = accel {
            let speed = Self::clamp_speed(accel.speed);
            self.run_command(format!("input type:touchpad pointer_accel {speed}"))?;
            if let Some(profile) = accel.profile {
                let value = Self::map_accel_profile(&profile);
                self.run_command(format!("input type:touchpad accel_profile {value}"))?;
            }
        }
        Ok(())
    }

    // fn touchpad_calibration(&self, _cal: Option<[f32; 6]>) -> InputResult {
    //     // TODO: No calibration support in Sway IPC.
    //     dbg!("Sway: touchpad calibration not supported");
    //     Ok(())
    // }

    fn touchpad_click_method(&self, method: Option<ClickMethod>) -> InputResult {
        if let Some(method) = method {
            let value = Self::map_click_method(&method);
            return self.run_command(format!("input type:touchpad click_method {value}"));
        }
        Ok(())
    }

    fn touchpad_disable_while_typing(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:touchpad", "dwt", enabled)
    }

    fn touchpad_left_handed(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:touchpad", "left_handed", enabled)
    }

    fn touchpad_middle_button_emulation(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:touchpad", "middle_emulation", enabled)
    }

    // fn touchpad_rotation_angle(&self, _angle: Option<u32>) -> InputResult {
    //     // TODO: Rotation is not supported in Sway IPC.
    //     dbg!("Sway: touchpad rotation not supported");
    //     Ok(())
    // }

    fn touchpad_scroll_config(&self, config: Option<ScrollConfig>) -> InputResult {
        if let Some(config) = config {
            if let Some(factor) = config.scroll_factor {
                self.run_command(format!("input type:touchpad scroll_factor {factor}"))?;
            }
            if let Some(natural) = config.natural_scroll {
                let value = Self::bool_to_sway(natural);
                self.run_command(format!("input type:touchpad natural_scroll {value}"))?;
            }
        }
        Ok(())
    }

    fn touchpad_scroll_method(&self, method: Option<ScrollMethod>) -> InputResult {
        if let Some(method) = method {
            let value = Self::map_scroll_method(&method);
            return self.run_command(format!("input type:touchpad scroll_method {value}"));
        }
        Ok(())
    }

    fn touchpad_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:touchpad", "natural_scroll", enabled)
    }

    fn touchpad_scroll_factor(&self, factor: Option<f64>) -> InputResult {
        if let Some(factor) = factor {
            return self.run_command(format!("input type:touchpad scroll_factor {factor}"));
        }
        Ok(())
    }

    fn touchpad_scroll_button(&self, button: Option<u32>) -> InputResult {
        if let Some(button) = button {
            return self.run_command(format!("input type:touchpad scroll_button {button}"));
        }
        Ok(())
    }

    fn touchpad_tap_config(&self, config: Option<TapConfig>) -> InputResult {
        if let Some(config) = config {
            self.set_bool_required("type:touchpad", "tap", config.enabled)?;
            self.set_bool_required("type:touchpad", "tap_and_drag", config.drag)?;
            self.set_bool_required("type:touchpad", "drag_lock", config.drag_lock)?;
        }
        Ok(())
    }

    fn touchpad_tap_enabled(&self, enabled: bool) -> InputResult {
        self.set_bool_required("type:touchpad", "tap", enabled)
    }

    // fn touchpad_tap_button_map(&self, _map: Option<TapButtonMap>) -> InputResult {
    //     // TODO: Tap button map not exposed in Sway IPC.
    //     dbg!("Sway: touchpad tap_button_map not supported");
    //     Ok(())
    // }

    fn touchpad_tap_drag(&self, enabled: bool) -> InputResult {
        self.set_bool_required("type:touchpad", "tap_and_drag", enabled)
    }

    fn touchpad_tap_drag_lock(&self, enabled: bool) -> InputResult {
        self.set_bool_required("type:touchpad", "drag_lock", enabled)
    }

    // fn touchpad_map_to_output(&self, _output: Option<String>) -> InputResult {
    //     // TODO: Requires device-specific identifiers; not supported via type:touchpad.
    //     dbg!("Sway: touchpad map_to_output not supported");
    //     Ok(())
    // }

    // fn mouse_state(&self, _state: DeviceState) -> InputResult {
    //     // TODO: Requires device-specific identifiers; DisabledOnExternalMouse not supported.
    //     dbg!("Sway: mouse enable/disable not supported via type:pointer");
    //     Ok(())
    // }

    fn mouse_acceleration(&self, accel: Option<AccelConfig>) -> InputResult {
        if let Some(accel) = accel {
            let speed = Self::clamp_speed(accel.speed);
            self.run_command(format!("input type:pointer pointer_accel {speed}"))?;
            if let Some(profile) = accel.profile {
                let value = Self::map_accel_profile(&profile);
                self.run_command(format!("input type:pointer accel_profile {value}"))?;
            }
        }
        Ok(())
    }

    // fn mouse_calibration(&self, _cal: Option<[f32; 6]>) -> InputResult {
    //     // TODO: No calibration support in Sway IPC.
    //     dbg!("Sway: mouse calibration not supported");
    //     Ok(())
    // }

    fn mouse_click_method(&self, method: Option<ClickMethod>) -> InputResult {
        if let Some(method) = method {
            let value = Self::map_click_method(&method);
            return self.run_command(format!("input type:pointer click_method {value}"));
        }
        Ok(())
    }

    fn mouse_disable_while_typing(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:pointer", "dwt", enabled)
    }

    fn mouse_left_handed(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:pointer", "left_handed", enabled)
    }

    fn mouse_middle_button_emulation(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:pointer", "middle_emulation", enabled)
    }

    // fn mouse_rotation_angle(&self, _angle: Option<u32>) -> InputResult {
    //     // TODO: Rotation is not supported in Sway IPC.
    //     dbg!("Sway: mouse rotation not supported");
    //     Ok(())
    // }

    fn mouse_scroll_config(&self, config: Option<ScrollConfig>) -> InputResult {
        if let Some(config) = config {
            if let Some(factor) = config.scroll_factor {
                self.run_command(format!("input type:pointer scroll_factor {factor}"))?;
            }
            if let Some(natural) = config.natural_scroll {
                let value = Self::bool_to_sway(natural);
                self.run_command(format!("input type:pointer natural_scroll {value}"))?;
            }
        }
        Ok(())
    }

    fn mouse_scroll_method(&self, method: Option<ScrollMethod>) -> InputResult {
        if let Some(method) = method {
            let value = Self::map_scroll_method(&method);
            return self.run_command(format!("input type:pointer scroll_method {value}"));
        }
        Ok(())
    }

    fn mouse_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("type:pointer", "natural_scroll", enabled)
    }

    fn mouse_scroll_factor(&self, factor: Option<f64>) -> InputResult {
        if let Some(factor) = factor {
            return self.run_command(format!("input type:pointer scroll_factor {factor}"));
        }
        Ok(())
    }

    fn mouse_scroll_button(&self, button: Option<u32>) -> InputResult {
        if let Some(button) = button {
            return self.run_command(format!("input type:pointer scroll_button {button}"));
        }
        Ok(())
    }

    // fn mouse_tap_config(&self, _config: Option<TapConfig>) -> InputResult {
    //     // TODO: Mouse tap config is not supported in Sway IPC.
    //     dbg!("Sway: mouse tap_config not supported");
    //     Ok(())
    // }

    // fn mouse_map_to_output(&self, _output: Option<String>) -> InputResult {
    //     // TODO: Requires device-specific identifiers; not supported via type:pointer.
    //     dbg!("Sway: mouse map_to_output not supported");
    //     Ok(())
    // }
}
