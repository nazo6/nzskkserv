use std::sync::{Arc, Mutex};

use eframe::egui;
use nzskkserv_core::Server;

use crate::log::LOGGER;

pub(crate) struct App {
    is_hidden: Arc<Mutex<bool>>,
    is_exit: Arc<Mutex<bool>>,
    server: Arc<Server>,
}

impl App {
    pub fn new(
        is_hidden: Arc<Mutex<bool>>,
        is_exit: Arc<Mutex<bool>>,
        server: Arc<Server>,
    ) -> Self {
        Self {
            is_hidden,
            is_exit,
            server,
        }
    }
}

impl eframe::App for App {
    fn on_exit_event(&mut self) -> bool {
        *self.is_hidden.lock().unwrap() = true;
        *self.is_exit.lock().unwrap()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("nzskkserv-gui");
            ui.horizontal(|ui| {
                if self.server.get_running() {
                    ui.label("Server is running.");
                    if ui.button("Stop").clicked() {
                        let res = self.server.stop();
                        if let Err(error) = res {
                            LOGGER.log(format!("{}", error));
                        }
                    }
                } else if ui.button("Start").clicked() {
                    let server = Arc::clone(&self.server);
                    tokio::task::spawn(async move {
                        let res = server.start().await;
                        if let Err(error) = res {
                            LOGGER.log(format!("{}", error));
                        }
                    });
                }
            });
        });
        if *self.is_hidden.lock().unwrap() {
            frame.set_visible(false);
        } else {
            frame.set_visible(true);
        }

        if *self.is_exit.lock().unwrap() {
            frame.quit();
        }
    }
}
