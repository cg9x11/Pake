use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant::Value;

/// Register this process as a background application via the XDG portal.
/// `app_name` is the product name (e.g. "zalo") used to derive the DBus name.
pub fn request_background(app_name: &str) {
    let app_name = app_name.to_string();

    std::thread::spawn(move || {
        let conn = match Connection::session() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[Pake] Failed to connect to DBus session: {e}");
                return;
            }
        };

        // Derive unique DBus name from app name.
        let dbus_name = format!("com.pake.{}", app_name.to_lowercase());

        // Own the DBus name so xdg-desktop-portal can identify this app
        // when /proc/PID/exe points inside the AppImage FUSE mount.
        let _ = conn.request_name(dbus_name.as_str());

        // Call the Background portal.
        let mut options: HashMap<&str, Value<'_>> = HashMap::new();
        options.insert("reason", Value::from("Keep running in background"));

        if let Err(e) = conn.call_method(
            Some("org.freedesktop.portal.Desktop"),
            "/org/freedesktop/portal/desktop",
            Some("org.freedesktop.portal.Background"),
            "RequestBackground",
            &("", &options),
        ) {
            eprintln!("[Pake] Failed to request background portal: {e}");
        }

        // Keep thread and DBus connection alive indefinitely.
        loop {
            std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
        }
    });
}


