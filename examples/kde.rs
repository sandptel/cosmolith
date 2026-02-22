use std::error::Error;
use std::process::Command;

fn kde_set(group: &str, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let conn = zbus::blocking::Connection::session()?;

    Command::new("kwriteconfig6")
        .args([
            "--file",
            "kcminputrc",
            "--group",
            group,
            "--key",
            key,
            value,
        ])
        .status()?;

    let _ = conn.call_method(
        Some("org.kde.KWin"),
        "/KWin",
        Some("org.kde.KWin"),
        "reconfigure",
        &(),
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Natural Scroll
    kde_set("Libinput", "NaturalScroll", "false")?;

    // 2. Left Handed Mouse
    kde_set("Mouse", "LeftHanded", "true")?;

    // 3. Tap to click
    kde_set("Libinput", "TapToClick", "false")?;

    println!("KDE Plasma settings updated and KWin reconfigured!");
    Ok(())
}
