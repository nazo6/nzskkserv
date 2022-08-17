use std::sync::{Arc, Mutex};

use auto_launch::AutoLaunch;
use eframe::{egui, epaint::FontFamily};
use nzskkserv_core::{
    log::LogEvent,
    server::interface::{SkkIncomingEvent, SkkOutcomingEvent},
    Server,
};
use once_cell::sync::Lazy;

use crate::log::{GlobalLogEntry, LOGGER};

pub(crate) struct App {
    is_hidden: Arc<Mutex<bool>>,
    is_exit: Arc<Mutex<bool>>,
    server: Arc<Server>,
    ui_state: UiState,
}

struct UiState {
    tab: TopTabBar,
    run_mode: RunMode,
}
#[derive(PartialEq)]
pub enum TopTabBar {
    DashBoard,
    Log,
    Config,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    Reactive,
    Continuous,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        is_hidden: Arc<Mutex<bool>>,
        is_exit: Arc<Mutex<bool>>,
        server: Arc<Server>,
    ) -> Self {
        let mut fonts = egui::FontDefinitions::default();
        let font = std::fs::read("c:/Windows/Fonts/YuGothM.ttc").unwrap();
        fonts
            .font_data
            .insert("my_font".to_owned(), egui::FontData::from_owned(font));
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Self {
            is_hidden,
            is_exit,
            server,
            ui_state: UiState {
                tab: TopTabBar::DashBoard,
                run_mode: RunMode::Reactive,
            },
        }
    }
}

impl eframe::App for App {
    fn on_exit_event(&mut self) -> bool {
        *self.is_hidden.lock().unwrap() = true;
        *self.is_exit.lock().unwrap()
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.ui_state.run_mode {
            RunMode::Continuous => {
                // Tell the backend to repaint as soon as possible
                ctx.request_repaint();
            }
            RunMode::Reactive => {
                // let the computer rest for a bit
                ctx.request_repaint_after(std::time::Duration::from_secs_f32(1.0));
            }
        }
        egui::TopBottomPanel::top("tabbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.ui_state.tab, TopTabBar::DashBoard, "Dashboard");
                ui.selectable_value(&mut self.ui_state.tab, TopTabBar::Log, "Log");
                ui.selectable_value(&mut self.ui_state.tab, TopTabBar::Config, "Config");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| match self.ui_state.tab {
            TopTabBar::DashBoard => {
                self.ui_state.run_mode = RunMode::Reactive;
                self.dashboard_view(ui)
            }
            TopTabBar::Log => {
                if *self.is_hidden.lock().unwrap() {
                    self.ui_state.run_mode = RunMode::Reactive;
                } else {
                    self.ui_state.run_mode = RunMode::Continuous;
                }
                self.log_view(ui)
            }
            TopTabBar::Config => {
                self.ui_state.run_mode = RunMode::Reactive;
                self.config_view(ui);
            }
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

impl App {
    fn dashboard_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("nzskkserv-gui");
        ui.horizontal(|ui| {
            if self.server.get_running() {
                if ui.button("Stop").clicked() {
                    let res = self.server.stop();
                    if let Err(error) = res {
                        LOGGER.log(format!("{}", error));
                    }
                }
                ui.label("Server is running.");
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
    }
    fn log_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("Logs");
        ui.vertical(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    for item in LOGGER.get_logs().iter().rev() {
                        match item {
                            GlobalLogEntry::ServerLog(log) => {
                                let text;
                                match &log.event {
                                    LogEvent::Incoming(e) => match e {
                                        SkkIncomingEvent::Disconnect => {
                                            text = "Incoming: Disconnected".to_string();
                                        }
                                        SkkIncomingEvent::Convert(str) => {
                                            let mut new_t = "Incoming: Converting: ".to_string();
                                            new_t.push_str(str);
                                            text = new_t;
                                        }
                                        _ => {
                                            text = "Incoming: Unknown".to_string();
                                        }
                                    },
                                    LogEvent::Outcoming(e) => match e {
                                        SkkOutcomingEvent::Convert(str) => {
                                            let mut new_t = "Outgoing: Converted: ".to_string();
                                            if let Some(str) = str {
                                                new_t.push_str(str);
                                            } else {
                                                new_t.push_str("(None)");
                                            }
                                            text = new_t;
                                        }
                                        _ => {
                                            text = "Outgoing: Unknown".to_string();
                                        }
                                    },
                                    LogEvent::Message(str) => {
                                        text = str.to_string();
                                    }
                                }
                                ui.label(format!("SERV: {}", text));
                            }
                            GlobalLogEntry::AppLog(log) => {
                                ui.label(format!("APP : {}", log));
                            }
                        }
                    }
                })
            });
        });
    }
    fn config_view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Auto-start");
            if ui.button("Enable").clicked() {
                AUTO_LAUNCH.enable();
            }
            if ui.button("Disable").clicked() {
                AUTO_LAUNCH.disable();
            }
        });
    }
}

static AUTO_LAUNCH: Lazy<AutoLaunch> = Lazy::new(|| {
    let name = "nzskkserv-gui";
    let path = std::env::current_exe().unwrap();
    let path = path.to_str().unwrap();
    AutoLaunch::new(name, path)
});
