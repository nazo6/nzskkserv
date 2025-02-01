#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use gpui::*;
use ui::init;

actions!(a, ["Test"]);

#[tokio::main]
async fn main() -> Result<()> {
    let app = Application::new();

    app.run(move |cx| {
        init(cx);

        cx.set_menus(vec![
            Menu {
                name: "Edit".into(),
                items: vec![MenuItem::action("AAA", Test)],
            },
            Menu {
                name: "Window".into(),
                items: vec![],
            },
        ]);
        cx.activate(true);
    });

    Ok(())
}
