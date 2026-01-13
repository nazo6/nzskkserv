use dioxus::desktop::{
    trayicon::{init_tray_icon, menu::MenuItem, DioxusTrayIcon, DioxusTrayMenu},
    use_muda_event_handler, use_window,
};

pub(super) fn use_tray_menu() {
    let menu = DioxusTrayMenu::with_id_and_items(
        0,
        &[
            &MenuItem::with_id("showhide", "Show/Hide", true, None),
            &MenuItem::with_id("quit", "Quit", true, None),
        ],
    )
    .unwrap();

    init_tray_icon(
        menu,
        Some(
            DioxusTrayIcon::from_rgba(
                crate::icon::ICON_DATA.into(),
                crate::icon::ICON_WIDTH,
                crate::icon::ICON_HEIGHT,
            )
            .unwrap(),
        ),
    );

    let window = use_window();
    use_muda_event_handler(move |ev| match ev.id.0.as_str() {
        "showhide" => {
            window.set_visible(!window.is_visible());
        }
        "quit" => {
            window.close();
        }
        _ => {}
    });
}
