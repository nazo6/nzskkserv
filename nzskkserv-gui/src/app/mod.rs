use gpui::*;
use ui::{
    init,
    tab::{Tab, TabBar},
    Selectable as _,
};

pub(super) fn start() {
    let appli = Application::new();

    appli.run(move |app| {
        init(app);

        app.open_window(WindowOptions::default(), |_, app| app.new(Root::new))
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
}

impl Root {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(|this, mut cx| async move {}).detach();

        Root {
            active_tab: MainTab::Log,
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
                        Tab::new("main-tab", "Log")
                            .selected(self.active_tab == MainTab::Log)
                            .on_click(cx.listener(|this, _, _, _| {
                                this.active_tab = MainTab::Log;
                            })),
                    )
                    .child(
                        Tab::new("main-tab", "Config")
                            .selected(self.active_tab == MainTab::Config)
                            .on_click(cx.listener(|this, _, _, _| {
                                this.active_tab = MainTab::Config;
                            })),
                    ),
            )
            .child(match self.active_tab {
                MainTab::Log => div().child("Log"),
                MainTab::Config => div().child("Config"),
            })
    }
}
