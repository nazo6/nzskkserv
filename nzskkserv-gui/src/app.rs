use std::{cell::RefCell, rc::Rc};

use eframe::egui;

pub(crate) struct App {
    is_exit: Rc<RefCell<bool>>,
}

impl App {
    pub fn new(is_exit: Rc<RefCell<bool>>) -> Self {
        Self { is_exit }
    }
}

impl eframe::App for App {
    fn on_exit_event(&mut self) -> bool {
        *self.is_exit.borrow_mut() = true;
        true
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
            });
        });
        if *self.is_exit.borrow() {
            println!("Exit");
            frame.quit();
        }
    }
}
