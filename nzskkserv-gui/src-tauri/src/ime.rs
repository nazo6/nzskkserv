#[cfg(target_os = "linux")]
pub async fn notify_ime() {
    tokio::spawn(async {
        use std::time::Duration;
        use tracing::{debug, info};

        let connection = match zbus::Connection::session().await {
            Ok(conn) => conn,
            Err(e) => {
                debug!("Failed to connect to D-Bus session bus: {}", e);
                return;
            }
        };

        // If fcitx5 is currently starting up, retry for a few seconds.
        for attempt in 1..=5 {
            let res = connection
                .call_method(
                    Some("org.fcitx.Fcitx5"),
                    "/controller",
                    Some("org.fcitx.Fcitx.Controller1"),
                    "ReloadAddonConfig",
                    &("skk",),
                )
                .await;

            match res {
                Ok(_) => {
                    info!("Successfully notified fcitx5 to reload skk addon");
                    return;
                }
                Err(zbus::Error::MethodError(name, _, _))
                    if name.as_str() == "org.freedesktop.DBus.Error.UnknownMethod" =>
                {
                    // Fall back to reloading general config if ReloadAddonConfig isn't available
                    if let Ok(_) = connection
                        .call_method(
                            Some("org.fcitx.Fcitx5"),
                            "/controller",
                            Some("org.fcitx.Fcitx.Controller1"),
                            "ReloadConfig",
                            &(),
                        )
                        .await
                    {
                        info!("Successfully notified fcitx5 via ReloadConfig");
                        return;
                    }
                }
                Err(e) => {
                    debug!("Attempt {}/5: fcitx5 not reachable: {}", attempt, e);
                }
            }

            tokio::time::sleep(Duration::from_millis(1000)).await;
        }

        debug!("fcitx5 was not reachable after 5 attempts; skipped notification.");
    });
}

#[cfg(not(target_os = "linux"))]
pub async fn notify_ime() {}

