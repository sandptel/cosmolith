use gio::prelude::*;
use gio::Settings;
use std::error::Error;

struct GnomeConfig {
    touchpad: Settings,
    mouse: Settings,
}

impl GnomeConfig {
    fn new() -> Self {
        Self {
            touchpad: Settings::new("org.gnome.desktop.peripherals.touchpad"),
            mouse: Settings::new("org.gnome.desktop.peripherals.mouse"),
        }
    }

    fn set_bool(&self, settings: &Settings, key: &str, val: bool) -> Result<(), Box<dyn Error>> {
        settings.set_boolean(key, val)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let gnome = GnomeConfig::new();

    // 1. Enable Natural Scrolling on Touchpad
    gnome.set_bool(&gnome.touchpad, "natural-scroll", true)?;

    // 2. Enable Left-Handed mode for the Mouse
    gnome.set_bool(&gnome.mouse, "left-handed", true)?;

    // 3. Disable Tap-to-Click
    gnome.set_bool(&gnome.touchpad, "tap-to-click", false)?;

    // force the backend (dconf) to write the changes
    Settings::sync();

    println!("GNOME GSettings updated successfully!");
    Ok(())
}
