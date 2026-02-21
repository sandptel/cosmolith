use crate::compositor::input::{Input, InputResult};
use crate::compositor::{Compositor, CompositorResult};
use crate::event::input::InputEvent;
use crate::event::Event;
use gio::prelude::*;
use gio::Settings;

pub struct Gnome {
    touchpad_settings: Settings,
    mouse_settings: Settings,
}

impl Gnome {
    pub fn new() -> Self {
        Self {
            touchpad_settings: Settings::new("org.gnome.desktop.peripherals.touchpad"),
            mouse_settings: Settings::new("org.gnome.desktop.peripherals.mouse"),
        }
    }

    fn set_str(&self, settings: &Settings, key: &str, value: &str) -> InputResult {
        settings
            .set_string(key, value)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    fn set_bool(&self, settings: &Settings, key: &str, value: bool) -> InputResult {
        settings
            .set_boolean(key, value)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    fn set_opt_bool(&self, settings: &Settings, key: &str, value: Option<bool>) -> InputResult {
        if let Some(v) = value {
            self.set_bool(settings, key, v)?;
        }
        Ok(())
    }

    fn set_double(&self, settings: &Settings, key: &str, val: f64) -> InputResult {
        settings.set_double(key, val)?;
        Ok(())
    }
}

impl Compositor for Gnome {
    fn init(&mut self) -> CompositorResult {
        Ok(())
    }

    fn name(&self) -> &'static str {
        "GNOME"
    }

    fn is_running(&self) -> bool {
        std::env::var("XDG_CURRENT_DESKTOP")
            .map(|val| val.to_uppercase().contains("GNOME"))
            .unwrap_or(false)
    }

    fn supports(&self, event: &Event) -> bool {
        matches!(event, Event::Input(_))
    }

    fn apply_event(&self, event: Event) -> CompositorResult {
        match event {
            Event::Input(InputEvent::Mouse(ev)) => self.apply_mouse_event(ev)?,
            Event::Input(InputEvent::TouchPad(ev)) => self.apply_touchpad_event(ev)?,
            _ => (),
        }
        Ok(())
    }

    fn reload(&self) -> CompositorResult {
        Ok(())
    }
    fn shutdown(&self) -> CompositorResult {
        Ok(())
    }
}

impl Input for Gnome {
    /* Touchpad */

    fn touchpad_tap_enabled(&self, enabled: bool) -> InputResult {
        self.set_bool(&self.touchpad_settings, "tap-to-click", enabled)
    }

    fn touchpad_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        self.set_opt_bool(&self.touchpad_settings, "natural-scroll", enabled)
    }

    fn touchpad_disable_while_typing(&self, enabled: Option<bool>) -> InputResult {
        self.set_opt_bool(&self.touchpad_settings, "disable-while-typing", enabled)
    }

    fn touchpad_left_handed(&self, enabled: Option<bool>) -> InputResult {
        if let Some(v) = enabled {
            let val = if v { "left" } else { "mouse" }; // "mouse" is right-handed
            self.set_str(&self.touchpad_settings, "haptic-output-mode", val)?;
        }
        Ok(())
    }

    /* Mouse */

    fn mouse_left_handed(&self, enabled: Option<bool>) -> InputResult {
        self.set_opt_bool(&self.mouse_settings, "left-handed", enabled)
    }

    fn mouse_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        self.set_opt_bool(&self.mouse_settings, "natural-scroll", enabled)
    }
}
