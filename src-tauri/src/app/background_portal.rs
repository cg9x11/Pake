use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant::Value;

/// Register this process as a background application via the XDG portal.
/// Owns a DBus name derived from `app_id` so the portal can identify the app.
pub fn request_background(app_id: &str) {
    let app_id = app_id.to_string();

    std::thread::spawn(move || {
        let conn = match Connection::session() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[Pake] Failed to connect to DBus session: {e}");
                return;
            }
        };

        let _ = conn.request_name(app_id.as_str());

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

        loop {
            std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
        }
    });
}
