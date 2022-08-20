use std::sync::{Arc, Mutex};

use auto_launch::AutoLaunch;
use eframe::{
    egui::{self, RichText},
    epaint::{Color32, FontFamily},
};
use nzskkserv_core::{
    log::LogEvent,
    server::interface::{SkkIncomingEvent, SkkOutGoingEvent},
    Server,
};
use once_cell::sync::Lazy;

use crate::{
    error,
    log::{GlobalLogEntry, LOGGER},
};

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
                        error!("{}", error)
                    }
                }
                ui.label("Server is running.");
            } else if ui.button("Start").clicked() {
                let server = Arc::clone(&self.server);
                tokio::task::spawn(async move {
                    let res = server.start().await;
                    if let Err(error) = res {
                        error!("{}", error)
                    }
                });
            }
        });
    }
    fn log_view(&mut self, ui: &mut egui::Ui) {
        ui.heading("Logs");
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new("log_grid").show(ui, |ui| {
                for item in LOGGER.get_logs().iter().rev() {
                    match item {
                        GlobalLogEntry::ServerLog(log) => {
                            ui.label(
                                RichText::new("Server")
                                    .background_color(Color32::BLUE)
                                    .color(Color32::WHITE),
                            );
                            ui.label(label_from_log_level(log.level));
                            match &log.event {
                                LogEvent::Incoming(e) => {
                                    ui.label(RichText::new("⬅ In").color(Color32::BLUE));
                                    match e {
                                        SkkIncomingEvent::Disconnect => {
                                            ui.label("Disconnecting");
                                        }
                                        SkkIncomingEvent::Convert(str) => {
                                            let mut text = "Converting: ".to_string();
                                            text.push_str(str);
                                            ui.label(text);
                                        }
                                        _ => {
                                            ui.label("Unknown");
                                        }
                                    }
                                }
                                LogEvent::OutGoing(e) => {
                                    ui.label(RichText::new("➡ Out").color(Color32::RED));
                                    match e {
                                        SkkOutGoingEvent::Convert(str) => {
                                            let mut text = "Converted: ".to_string();
                                            if let Some(str) = str {
                                                text.push_str(str);
                                            } else {
                                                text.push_str("(None)");
                                            }
                                            ui.label(text);
                                        }
                                        _ => {
                                            ui.label("Unknown");
                                        }
                                    }
                                }
                                LogEvent::Message(str) => {
                                    ui.label("Message");
                                    ui.label(str);
                                }
                            }
                        }
                        GlobalLogEntry::AppLog(log) => {
                            ui.label(
                                RichText::new("App")
                                    .background_color(Color32::RED)
                                    .color(Color32::WHITE),
                            );
                            ui.label(label_from_log_level(log.level));
                            ui.label("Message");
                            ui.label(&log.message);
                        }
                    }
                    ui.end_row();
                }
            })
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

fn label_from_log_level(level: u8) -> RichText {
    if level == 0 {
        RichText::new("INFO")
            .background_color(Color32::DARK_BLUE)
            .color(Color32::WHITE)
    } else if level == 1 {
        RichText::new("WARN")
            .background_color(Color32::YELLOW)
            .color(Color32::WHITE)
    } else if level == 2 {
        RichText::new("INFO")
            .background_color(Color32::RED)
            .color(Color32::WHITE)
    } else {
        RichText::new("UNKNOWN")
            .background_color(Color32::BLACK)
            .color(Color32::WHITE)
    }
}

static AUTO_LAUNCH: Lazy<AutoLaunch> = Lazy::new(|| {
    let name = "nzskkserv-gui";
    let path = std::env::current_exe().unwrap();
    let path = path.to_str().unwrap();
    AutoLaunch::new(name, path)
});
