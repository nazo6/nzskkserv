use std::sync::{Arc, Mutex};

use eframe::egui;
use poll_promise::Promise;

use crate::log::LOGGER;

pub(crate) struct App {
    is_hidden: Arc<Mutex<bool>>,
    server: Arc<nzskkserv_core::Server>,
    server_promise: Option<Promise<()>>,
}

impl App {
    pub fn new(is_hidden: Arc<Mutex<bool>>, server: Arc<nzskkserv_core::Server>) -> Self {
        Self {
            is_hidden,
            server,
            server_promise: None,
        }
    }
}

impl eframe::App for App {
    fn on_exit_event(&mut self) -> bool {
        *self.is_hidden.lock().unwrap() = true;
        false
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("nzskkserv-gui");
            ui.horizontal(|ui| {
                if let Some(server_promise) = &self.server_promise {
                    match server_promise.ready() {
                        Some(()) => {
                            self.server_promise = None;
                        }
                        None => {
                            ui.label("Server is running.");
                            if ui.button("Stop").clicked() {
                                let res = self.server.stop();
                                if let Err(error) = res {
                                    LOGGER.log(format!("{}", error));
                                }
                            }
                        }
                    }
                } else if ui.button("Start").clicked() {
                    let server = Arc::clone(&self.server);
                    self.server_promise = Some(Promise::spawn_async(async move {
                        let res = server.start().await;
                        if let Err(error) = res {
                            LOGGER.log(format!("{}", error));
                        }
                    }));
                }
            });
        });
        if *self.is_hidden.lock().unwrap() {
            frame.set_visible(false);
        } else {
            frame.set_visible(true);
        }
        println!("update");
    }
}
