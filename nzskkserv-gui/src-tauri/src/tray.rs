use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Wry,
};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_hide_item = MenuItemBuilder::with_id("show_hide", "Show / Hide").build(app)?;
    let toggle_server_item = MenuItemBuilder::with_id("toggle_server", "Start / Stop Server").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&show_hide_item)
        .item(&toggle_server_item)
        .separator()
        .item(&quit_item)
        .build()?;

    let mut tray_builder = TrayIconBuilder::<Wry>::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("nzskkserv")
        .on_menu_event(move |app_handle, event| {
            match event.id().as_ref() {
                "show_hide" => {
                    crate::window::toggle_main_window(app_handle);
                }
                "toggle_server" => {
                    if let Some(state) = app_handle.try_state::<crate::commands::AppState>() {
                        state.server_ctrl.send_modify(|s| {
                            s.running = !s.running;
                        });
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                crate::window::toggle_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        tray_builder = tray_builder.icon(icon.clone());
    }

    tray_builder.build(app)?;

    Ok(())
}

