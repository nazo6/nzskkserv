use dict_table::DictEditor;
use general_config::GeneralConfigView;
use gpui::*;
use server_toggle_button::StartButton;
use ui::button::Button;

use crate::{config::Config, server::ServerController};

mod dict_table;
mod general_config;
mod server_toggle_button;

pub(super) struct ConfigView {
    start_button: Entity<StartButton>,
    dict_table: Entity<DictEditor>,
    general_config: Entity<GeneralConfigView>,
    server_controller: ServerController,
    _server_recv_task: Task<()>,
    temp_config: Entity<Config>,
}

impl ConfigView {
    pub fn new(
        cx: &mut Context<Self>,
        win: &mut Window,
        server_controller: ServerController,
    ) -> Self {
        let mut receiver = server_controller.subscribe();
        let _server_recv_task = cx.spawn(|this, mut app| async move {
            loop {
                let _ = receiver.changed().await;
                let _ = this.update(&mut app, |this, cx| {
                    this.temp_config.update(cx, |c, _| {
                        *c = receiver.borrow_and_update().config.clone();
                    });
                    cx.notify();
                });
            }
        });

        let config = server_controller.borrow().config.clone();
        let config = cx.new(|_| config);

        ConfigView {
            start_button: cx.new(|cx| StartButton::new(cx, server_controller.clone())),
            dict_table: cx.new(|cx| DictEditor::new(cx, win, config.clone())),
            general_config: cx.new(|cx| GeneralConfigView::new(win, cx, config.clone())),
            server_controller,
            _server_recv_task,
            temp_config: config,
        }
    }
}

impl Render for ConfigView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .child("Run")
            .child(self.start_button.clone())
            .child("Dictionaries")
            .child(div().h_48().child(self.dict_table.clone()))
            .child("General")
            .child(self.general_config.clone())
            .child(
                Button::new("save-config")
                    .label("Save")
                    .on_click(cx.listener(|this, _, _, cx| {
                        let tc = this.temp_config.read(cx);
                        this.server_controller.send_modify(|v| {
                            v.config = tc.clone();
                        });
                    })),
            )
    }
}
