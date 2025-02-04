use gpui::*;
use ui::button::Button;

use crate::server::ServerController;

pub struct StartButton {
    server_controller: ServerController,
    _server_recv_task: Task<()>,
    running: bool,
}

impl StartButton {
    pub fn new(cx: &mut Context<'_, Self>, server_controller: ServerController) -> Self {
        let mut receiver = server_controller.subscribe();
        let _server_recv_task = cx.spawn(|this, mut app| async move {
            loop {
                let _ = receiver.changed().await;
                let running = receiver.borrow().running;
                let _ = this.update(&mut app, |this, cx| {
                    this.running = running;
                    cx.notify();
                });
            }
        });

        let running = server_controller.borrow().running;
        StartButton {
            running,
            server_controller,
            _server_recv_task,
        }
    }
}

impl Render for StartButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        Button::new("server-toggle")
            .label(if self.running { "Stop" } else { "Start" })
            .on_click(cx.listener(|this, _, _, _| {
                this.server_controller
                    .send_modify(|v| v.running = !v.running);
            }))
    }
}
