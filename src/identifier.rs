use std::env;

#[allow(dead_code)]
#[derive(Debug)]
// The following is just an intermediatry to be passed to Compsoitor Module
// compositor::init_compositor will match and convert the identified compositor to
// their equivalent structs
pub enum Desktop {
    Hyprland,
    Sway,
    Gnome,
    Kde,
    Plasma,
    Niri,
    Xfce,
    Cosmic,
    Wayland,
    X11,
    Tty,
    Unknown(String),
}

// #todo : Find edge cases where this logic might fail?
// Think of other ways the following can be made more robust :}
pub fn get_current_session() -> Desktop {
    if let Ok(session_type) = env::var("XDG_SESSION_TYPE") {
        match session_type.to_lowercase().as_str() {
            "tty" => return Desktop::Tty,
            "wayland" => {}
            "x11" => {}
            _ => {}
        }
    }

    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        return Desktop::Hyprland;
    }
    if env::var("SWAYSOCK").is_ok() {
        return Desktop::Sway;
    }
    if env::var("NIRI_SOCKET").is_ok() {
        return Desktop::Niri;
    }

    let candidates = [
        env::var("XDG_CURRENT_DESKTOP").ok(),
        env::var("XDG_SESSION_DESKTOP").ok(),
        env::var("DESKTOP_SESSION").ok(),
    ];

    for value in candidates.into_iter().flatten() {
        let lower = value.to_lowercase();
        if lower.contains("hyprland") {
            return Desktop::Hyprland;
        }
        if lower.contains("sway") {
            return Desktop::Sway;
        }
        if lower.contains("niri") {
            return Desktop::Niri;
        }
        if lower.contains("gnome") {
            return Desktop::Gnome;
        }
        if lower.contains("kde") {
            return Desktop::Kde;
        }
        if lower.contains("plasma") {
            return Desktop::Plasma;
        }
        if lower.contains("xfce") {
            return Desktop::Xfce;
        }
        if lower.contains("cosmic") {
            return Desktop::Cosmic;
        }
    }

    if env::var("WAYLAND_DISPLAY").is_ok() {
        return Desktop::Wayland;
    }
    if env::var("DISPLAY").is_ok() {
        return Desktop::X11;
    }

    Desktop::Unknown("Not Detected".into())
}
