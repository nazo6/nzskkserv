use tokio::sync::watch;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIconBuilder,
};

use crate::AppState;

#[derive(Debug, Clone, Copy)]
pub enum MenuId {
    ShowHide,
    Quit,
}

impl MenuId {
    fn as_str(&self) -> &'static str {
        match self {
            MenuId::ShowHide => "Show/Hide",
            MenuId::Quit => "Quit",
        }
    }
}

pub fn start(app_state_sender: watch::Sender<AppState>) {
    let menu = Menu::new();

    let a_sh = MenuItem::with_id(
        MenuId::ShowHide.as_str(),
        MenuId::ShowHide.as_str(),
        true,
        None,
    );
    let a_q = MenuItem::with_id(MenuId::Quit.as_str(), MenuId::Quit.as_str(), true, None);

    let _ = menu.append_items(&[&a_sh, &a_q]);

    let icon = Icon::from_resource_name("tray-icon", None).unwrap();

    let tray = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(icon)
        .build()
        .unwrap();
    Box::leak(Box::new(tray));

    MenuEvent::set_event_handler(Some(move |e: MenuEvent| match e.id.0.as_str() {
        id if id == MenuId::ShowHide.as_str() => {
            app_state_sender.send_modify(|val| {
                if *val == AppState::Show {
                    *val = AppState::Hide;
                } else {
                    *val = AppState::Show;
                }
            });
        }
        id if id == MenuId::Quit.as_str() => {
            let _ = app_state_sender.send(AppState::Quit);
        }
        _ => {}
    }));
}
