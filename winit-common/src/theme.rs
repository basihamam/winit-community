//! System theme detection for Linux using the XDG Desktop Portal (`org.freedesktop.portal.Settings`).

use std::process::Command;
use winit_core::window::Theme;

/// Query the system theme on Linux via the XDG desktop portal (`org.freedesktop.portal.Settings.Read`).
///
/// Returns:
/// - `Some(Theme::Dark)` if the portal reports color-scheme 1 (prefer dark)
/// - `Some(Theme::Light)` if the portal reports color-scheme 2 (prefer light)
/// - `None` if no preference is indicated (0) or if the portal is unreachable.
pub fn read_linux_system_theme() -> Option<Theme> {
    // 1. Try gdbus
    if let Ok(output) = Command::new("gdbus")
        .args([
            "call",
            "--session",
            "--dest",
            "org.freedesktop.portal.Desktop",
            "--object-path",
            "/org/freedesktop/portal/desktop",
            "--method",
            "org.freedesktop.portal.Settings.Read",
            "org.freedesktop.appearance",
            "color-scheme",
        ])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.contains("uint32 1") {
                return Some(Theme::Dark);
            } else if s.contains("uint32 2") {
                return Some(Theme::Light);
            }
        }
    }

    // 2. Try busctl
    if let Ok(output) = Command::new("busctl")
        .args([
            "--user",
            "call",
            "org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings",
            "Read",
            "ss",
            "org.freedesktop.appearance",
            "color-scheme",
        ])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.contains(" 1") {
                return Some(Theme::Dark);
            } else if s.contains(" 2") {
                return Some(Theme::Light);
            }
        }
    }

    // 3. Try dbus-send
    if let Ok(output) = Command::new("dbus-send")
        .args([
            "--session",
            "--print-reply=literal",
            "--dest=org.freedesktop.portal.Desktop",
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings.Read",
            "string:org.freedesktop.appearance",
            "string:color-scheme",
        ])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.contains("uint32 1") || s.contains(" 1") {
                return Some(Theme::Dark);
            } else if s.contains("uint32 2") || s.contains(" 2") {
                return Some(Theme::Light);
            }
        }
    }

    // 4. Fallback to gsettings for GNOME environments where the portal service might be absent
    if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "color-scheme"])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            if s.contains("prefer-dark") {
                return Some(Theme::Dark);
            } else if s.contains("prefer-light") || s.contains("default") {
                return Some(Theme::Light);
            }
        }
    }

    None
}
