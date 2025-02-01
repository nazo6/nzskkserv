#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use gpui::*;
use ui::{button::Button, init};

struct Root {}

impl Render for Root {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().child(Button::new("0").label("Test"))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Application::new();

    app.run(move |cx| {
        init(cx);

        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Root {}))
            .unwrap();
    });

    Ok(())
}
