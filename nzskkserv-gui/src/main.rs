use app::App;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
};
use tray_item::TrayItem;

mod app;

enum Message {
    Quit,
    ShowHide,
}

fn main() {
    let mut tray = TrayItem::new("nzskkserv", "tray-icon").unwrap();
    let is_hidden = Rc::new(RefCell::new(true));

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    tray.add_menu_item("Show/Hide", move || {
        tx1.send(Message::ShowHide).unwrap();
    })
    .unwrap();

    tray.add_menu_item("Quit", move || {
        tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => break,
            Ok(Message::ShowHide) => {
                if *is_hidden.borrow() {
                    *is_hidden.borrow_mut() = false;
                    let app = app::App::new(Rc::clone(&is_hidden));
                    let options = eframe::NativeOptions::default();
                    eframe::run_native("My egui App", options, Box::new(|_cc| Box::new(app)));
                } else {
                    *is_hidden.borrow_mut() = true;
                }
            }
            _ => {}
        }
    }
}
