use crate::compositor::input::{Input, InputResult};
use crate::compositor::{Compositor, CompositorResult};
use crate::event::{Event, InputEvent};
use std::sync::Mutex;
use zbus::blocking::Connection;

pub struct Kde {
    connection: Mutex<Option<Connection>>,
}

impl Kde {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }

    fn run_kde_cmd(&self, group: &str, key: &str, value: &str) -> InputResult {
        std::process::Command::new("kwriteconfig6")
            .args([
                "--file",
                "kcminputrc",
                "--group",
                group,
                "--key",
                key,
                value,
            ])
            .status()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

        self.reload()
    }

    fn set_bool(&self, group: &str, key: &str, value: bool) -> InputResult {
        self.run_kde_cmd(group, key, &value.to_string())
    }

    fn set_opt_bool(&self, group: &str, key: &str, value: Option<bool>) -> InputResult {
        if let Some(v) = value {
            return self.set_bool(group, key, v);
        }
        Ok(())
    }

    fn set_opt_double(&self, group: &str, key: &str, value: Option<f64>) -> InputResult {
        if let Some(v) = value {
            return self.run_kde_cmd(group, key, &v.to_string());
        }
        Ok(())
    }
}

impl Compositor for Kde {
    fn init(&mut self) -> CompositorResult {
        let conn = Connection::session()?;
        let mut guard = self.connection.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "KDE connection lock poisoned")
        })?;
        *guard = Some(conn);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "KDE Plasma"
    }

    fn is_running(&self) -> bool {
        std::env::var("XDG_CURRENT_DESKTOP")
            .map(|val| val.to_uppercase().contains("KDE"))
            .unwrap_or(false)
    }

    fn reload(&self) -> CompositorResult {
        let guard = self.connection.lock().unwrap();
        if let Some(conn) = guard.as_ref() {
            conn.call_method(
                Some("org.kde.KWin"),
                "/KWin",
                Some("org.kde.KWin"),
                "reconfigure",
                &(),
            )?;
        }
        Ok(())
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
    fn shutdown(&self) -> CompositorResult {
        Ok(())
    }
}

impl Input for Kde {
    /* Touchpad */
    fn touchpad_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        self.set_opt_bool("Libinput", "NaturalScroll", enabled)
    }

    fn touchpad_tap_enabled(&self, enabled: bool) -> InputResult {
        self.set_bool("Libinput", "TapToClick", enabled)
    }

    /* Mouse */
    fn mouse_left_handed(&self, enabled: Option<bool>) -> InputResult {
        self.set_opt_bool("Mouse", "LeftHanded", enabled)
    }

    fn mouse_scroll_factor(&self, factor: Option<f64>) -> InputResult {
        self.set_opt_double("Mouse", "WheelScrollLines", factor)
    }
}
