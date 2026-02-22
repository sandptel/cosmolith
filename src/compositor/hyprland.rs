use crate::compositor::input::{Input, InputResult};
use crate::compositor::{Compositor, CompositorResult};
use crate::event::Event;
use crate::event::input::InputEvent;
use hyprland::keyword::Keyword;
use std::env;

use cosmic_comp_config::input::{
    AccelConfig, AccelProfile, ClickMethod, ScrollConfig, ScrollMethod, TapButtonMap, TapConfig,
};
use cosmic_comp_config::NumlockState;

#[derive(Debug, Default)]
pub struct Hyprland {
    pub instance_signature: Option<String>,
}

// #todo: Restructure:
// 1. Think of a better way to pass on errors
// 2. Trace Error Location: Which Line should be looked at just by looking at the logs.
impl Hyprland {
    pub fn new() -> Self {
        Self {
            instance_signature: None,
        }
    }

    fn set_keyword(&self, key: &str, value: impl ToString) -> InputResult {
        Keyword::set(key, value.to_string())
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
    }

    fn set_bool(&self, key: &str, value: Option<bool>) -> InputResult {
        match value {
            Some(true) => self.set_keyword(key, "true"),
            Some(false) => self.set_keyword(key, "false"),
            None => Ok(()),
        }
    }

    fn map_scroll_method(method: &ScrollMethod) -> &'static str {
        match method {
            ScrollMethod::TwoFinger => "2fg",
            ScrollMethod::Edge => "edge",
            ScrollMethod::OnButtonDown => "on_button",
            ScrollMethod::NoScroll => "none",
            _ => "none",
        }
    }

    fn map_click_method(method: &ClickMethod) -> bool {
        match method {
            ClickMethod::Clickfinger => true,
            ClickMethod::ButtonAreas => false,
            _ => false,
        }
    }

    fn map_tap_button_map(map: &TapButtonMap) -> &'static str {
        match map {
            TapButtonMap::LeftRightMiddle => "lrm",
            TapButtonMap::LeftMiddleRight => "lmr",
            _ => "lrm",
        }
    }
}

impl Compositor for Hyprland {
    fn init(&mut self) -> CompositorResult {
        self.instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE").ok();
        if self.instance_signature.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HYPRLAND_INSTANCE_SIGNATURE not set",
            )
            .into());
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Hyprland"
    }

    fn is_running(&self) -> bool {
        env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
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

// #todo: For all the todos -> Find equivalent functions in documentation and update
impl Input for Hyprland {

    // fn touchpad_state(&self, _state: DeviceState) -> InputResult {
    //     // TODO: Hyprland does not expose a direct enable/disable for touchpad.
    //     dbg!("Hyprland: touchpad enable/disable not supported");
    //     Ok(())
    // }

    fn touchpad_acceleration(&self, accel: Option<AccelConfig>) -> InputResult {
        // Mapped to general input sensitivity + accel_profile
        if let Some(accel) = accel {
            self.set_keyword("input:sensitivity", accel.speed)?;
            if let Some(profile) = accel.profile {
                let value = match profile {
                    AccelProfile::Flat => "flat",
                    AccelProfile::Adaptive => "adaptive",
                    _ => "adaptive",
                };
                self.set_keyword("input:accel_profile", value)?;
            }
        }
        Ok(())
    }

    // fn touchpad_calibration(&self, _cal: Option<[f32; 6]>) -> InputResult {
    //     // TODO: No touchpad calibration keyword in Hyprland.
    //     dbg!("Hyprland: touchpad calibration not supported");
    //     Ok(())
    // }

    fn touchpad_click_method(&self, method: Option<ClickMethod>) -> InputResult {
        if let Some(method) = method {
            let enabled = Self::map_click_method(&method);
            return self.set_keyword("input:touchpad:clickfinger_behavior", enabled);
        }
        Ok(())
    }

    fn touchpad_disable_while_typing(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("input:touchpad:disable_while_typing", enabled)
    }

    fn touchpad_left_handed(&self, enabled: Option<bool>) -> InputResult {
        // Mapped to general input left_handed
        self.set_bool("input:left_handed", enabled)
    }

    fn touchpad_middle_button_emulation(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("input:touchpad:middle_button_emulation", enabled)
    }

    // fn touchpad_rotation_angle(&self, _angle: Option<u32>) -> InputResult {
    //     // TODO: No touchpad rotation keyword in Hyprland.
    //     dbg!("Hyprland: touchpad rotation not supported");
    //     Ok(())
    // }

    fn touchpad_scroll_config(&self, config: Option<ScrollConfig>) -> InputResult {
        // Split into scroll_factor + natural_scroll
        if let Some(config) = config {
            if let Some(factor) = config.scroll_factor {
                self.set_keyword("input:touchpad:scroll_factor", factor)?;
            }
            self.set_bool("input:touchpad:natural_scroll", config.natural_scroll)?;
        }
        Ok(())
    }

    fn touchpad_scroll_method(&self, method: Option<ScrollMethod>) -> InputResult {
        if let Some(method) = method {
            let value = Self::map_scroll_method(&method);
            return self.set_keyword("input:scroll_method", value);
        }
        Ok(())
    }

    fn touchpad_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("input:touchpad:natural_scroll", enabled)
    }

    fn touchpad_scroll_factor(&self, factor: Option<f64>) -> InputResult {
        if let Some(factor) = factor {
            return self.set_keyword("input:touchpad:scroll_factor", factor);
        }
        Ok(())
    }

    // fn touchpad_scroll_button(&self, _button: Option<u32>) -> InputResult {
    //     // TODO: No touchpad scroll_button keyword in Hyprland.
    //     dbg!("Hyprland: touchpad scroll_button not supported");
    //     Ok(())
    // }

    fn touchpad_tap_config(&self, config: Option<TapConfig>) -> InputResult {
        // Split into tap-to-click, tap-and-drag, drag_lock
        if let Some(config) = config {
            self.set_keyword("input:touchpad:tap-to-click", config.enabled)?;
            self.set_keyword("input:touchpad:tap-and-drag", config.drag)?;
            self.set_keyword("input:touchpad:drag_lock", config.drag_lock)?;
        }
        Ok(())
    }

    fn touchpad_tap_enabled(&self, enabled: bool) -> InputResult {
        self.set_keyword("input:touchpad:tap-to-click", enabled)
    }

    fn touchpad_tap_button_map(&self, map: Option<TapButtonMap>) -> InputResult {
        if let Some(map) = map {
            let value = Self::map_tap_button_map(&map);
            return self.set_keyword("input:touchpad:tap_button_map", value);
        }
        Ok(())
    }

    fn touchpad_tap_drag(&self, enabled: bool) -> InputResult {
        self.set_keyword("input:touchpad:tap-and-drag", enabled)
    }

    fn touchpad_tap_drag_lock(&self, enabled: bool) -> InputResult {
        self.set_keyword("input:touchpad:drag_lock", enabled)
    }

    // fn touchpad_map_to_output(&self, _output: Option<String>) -> InputResult {
    //     // TODO: Hyprland touchpad mapping to output is not exposed.
    //     dbg!("Hyprland: touchpad map_to_output not supported");
    //     Ok(())
    // }

    // fn mouse_state(&self, _state: DeviceState) -> InputResult {
    //     // TODO: Hyprland does not expose a direct enable/disable for mouse.
    //     dbg!("Hyprland: mouse enable/disable not supported");
    //     Ok(())
    // }

    fn mouse_acceleration(&self, accel: Option<AccelConfig>) -> InputResult {
        if let Some(accel) = accel {
            self.set_keyword("input:sensitivity", accel.speed)?;
            if let Some(profile) = accel.profile {
                let value = match profile {
                    AccelProfile::Flat => "flat",
                    AccelProfile::Adaptive => "adaptive",
                    _ => "adaptive",
                };
                self.set_keyword("input:accel_profile", value)?;
            }
        }
        Ok(())
    }

    // fn mouse_calibration(&self, _cal: Option<[f32; 6]>) -> InputResult {
    //     // TODO: No mouse calibration keyword in Hyprland.
    //     dbg!("Hyprland: mouse calibration not supported");
    //     Ok(())
    // }

    // fn mouse_click_method(&self, _method: Option<ClickMethod>) -> InputResult {
    //     // TODO: No mouse click method keyword in Hyprland.
    //     dbg!("Hyprland: mouse click_method not supported");
    //     Ok(())
    // }

    // fn mouse_disable_while_typing(&self, _enabled: Option<bool>) -> InputResult {
    //     // TODO: No mouse-specific disable_while_typing in Hyprland.
    //     dbg!("Hyprland: mouse disable_while_typing not supported");
    //     Ok(())
    // }

    fn mouse_left_handed(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("input:left_handed", enabled)
    }

    // fn mouse_middle_button_emulation(&self, _enabled: Option<bool>) -> InputResult {
    //     // TODO: No mouse middle-button emulation keyword in Hyprland.
    //     dbg!("Hyprland: mouse middle_button_emulation not supported");
    //     Ok(())
    // }

    // fn mouse_rotation_angle(&self, _angle: Option<u32>) -> InputResult {
    //     // TODO: No mouse rotation keyword in Hyprland.
    //     dbg!("Hyprland: mouse rotation not supported");
    //     Ok(())
    // }

    // fn mouse_scroll_config(&self, _config: Option<ScrollConfig>) -> InputResult {
    //     // TODO: Redundant when fine-grained events are emitted.
    //     dbg!("Hyprland: mouse scroll_config is redundant");
    //     Ok(())
    // }

    fn mouse_scroll_method(&self, method: Option<ScrollMethod>) -> InputResult {
        if let Some(method) = method {
            let value = Self::map_scroll_method(&method);
            return self.set_keyword("input:scroll_method", value);
        }
        Ok(())
    }

    fn mouse_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        self.set_bool("input:natural_scroll", enabled)
    }

    // fn mouse_scroll_factor(&self, _factor: Option<f64>) -> InputResult {
    //     // TODO: No mouse scroll_factor keyword in Hyprland.
    //     dbg!("Hyprland: mouse scroll_factor not supported");
    //     Ok(())
    // }

    fn mouse_scroll_button(&self, button: Option<u32>) -> InputResult {
        if let Some(button) = button {
            return self.set_keyword("input:scroll_button", button);
        }
        Ok(())
    }

    // fn mouse_tap_config(&self, _config: Option<TapConfig>) -> InputResult {
    //     // TODO: Mouse tap config is not supported in Hyprland.
    //     dbg!("Hyprland: mouse tap_config not supported");
    //     Ok(())
    // }

    // fn mouse_map_to_output(&self, _output: Option<String>) -> InputResult {
    //     // TODO: Hyprland does not expose mouse mapping to output.
    //     dbg!("Hyprland: mouse map_to_output not supported");
    //     Ok(())
    // }

    fn keyboard_rules(&self, rules: String) -> InputResult {
        self.set_keyword("input:kb_rules", rules)
    }

    fn keyboard_layout(&self, layout: String) -> InputResult {
        self.set_keyword("input:kb_layout", layout)
    }

    fn keyboard_model(&self, model: String) -> InputResult {
        self.set_keyword("input:kb_model", model)
    }

    fn keyboard_options(&self, options: Option<String>) -> InputResult {
        if let Some(options) = options {
            // Hyprland expects a clean comma-separated list with no leading/trailing commas
            // and no empty segments. Normalize by trimming edge commas/whitespace, dropping
            let cleaned = options
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
                // empty segments, and re-joining with commas.
                .join(",");

            return self.set_keyword("input:kb_options", cleaned);
        }
        Ok(())
    }

    fn keyboard_variant(&self, variant: String) -> InputResult {
        self.set_keyword("input:kb_variant", variant)
    }

    fn keyboard_repeat_delay(&self, delay: u32) -> InputResult {
        return self.set_keyword("input:repeat_delay", delay);
    }

    fn keyboard_repeat_rate(&self, rate: u32) -> InputResult {
        return self.set_keyword("input:repeat_rate", rate);
    }

    fn numslock_state(&self, state: NumlockState) -> InputResult {
        match state {
            NumlockState::BootOn => self.set_keyword("input:numlock_by_default", "true"),
            NumlockState::BootOff => self.set_keyword("input:numlock_by_default", "false"),
            NumlockState::LastBoot => Ok(()), // Don't change
        }
    }
}
