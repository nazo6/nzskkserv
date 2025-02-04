use config::ConfigView;
use gpui::*;
use ui::{
    init,
    tab::{Tab, TabBar},
    Selectable as _,
};

mod config;

pub(super) fn start(
    app_state: tokio::sync::watch::Receiver<crate::AppState>,
    server_controller: crate::server::ServerController,
) {
    let appli = Application::new();

    appli.run(move |app| {
        init(app);

        let mut win_opts = WindowOptions::default();
        if *app_state.borrow() == crate::AppState::Hide {
            win_opts.show = false;
        }

        app.open_window(win_opts, |win, app| {
            app.new(|cx| Root::new(cx, win, app_state, server_controller))
        })
        .unwrap();
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MainTab {
    Log,
    Config,
}

struct Root {
    active_tab: MainTab,
    _app_state_handler: Task<()>,
    config_view: Entity<ConfigView>,
}

impl Root {
    fn new(
        cx: &mut Context<Self>,
        win: &mut Window,
        mut app_state: tokio::sync::watch::Receiver<crate::AppState>,
        server_controller: crate::server::ServerController,
    ) -> Self {
        if *app_state.borrow() == crate::AppState::Hide {
            cx.hide();
        }

        let _app_state_handler = cx.spawn(|_, app| async move {
            loop {
                if let Ok(()) = app_state.changed().await {
                    match *app_state.borrow_and_update() {
                        crate::AppState::Show => {
                            let _ = app.update(|app| app.activate(false));
                        }
                        crate::AppState::Hide => {
                            let _ = app.update(|app| app.hide());
                        }
                        crate::AppState::Quit => {
                            let _ = app.update(|app| app.shutdown());
                        }
                    }
                }
            }
        });

        Root {
            active_tab: MainTab::Config,
            _app_state_handler,
            config_view: cx.new(|cx| ConfigView::new(cx, win, server_controller)),
        }
    }
}

impl Render for Root {
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .bg(white())
            .child(
                TabBar::new("main-tabbar")
                    .child(
                        Tab::new("main-tab", "Config")
                            .selected(self.active_tab == MainTab::Config)
                            .on_click(cx.listener(|this, _, _, _| {
                                this.active_tab = MainTab::Config;
                            })),
                    )
                    .child(
                        Tab::new("main-tab", "Log")
                            .selected(self.active_tab == MainTab::Log)
                            .on_click(cx.listener(|this, _, _, _| {
                                this.active_tab = MainTab::Log;
                            })),
                    ),
            )
            .child(div().p_1().child(match self.active_tab {
                MainTab::Log => div().child("Log"),
                MainTab::Config => div().child(self.config_view.clone()),
            }))
    }
}
