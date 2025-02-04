use gpui::*;
use ui::input::{InputEvent, NumberInput, NumberInputEvent};

use super::Config;

pub struct GeneralConfigView {
    config: Entity<Config>,
    port_input: Entity<NumberInput>,
}

impl GeneralConfigView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, config: Entity<Config>) -> Self {
        let port_input = cx.new(|cx| NumberInput::new(window, cx));

        cx.subscribe_in(&port_input, window, |this, _, evt, window, cx| {
            dbg!("0");
            match evt {
                NumberInputEvent::Input(input_event) => {
                    if let InputEvent::Change(text) = input_event {
                        // if let Ok(port) = text.parse::<u16>() {
                        //     this.config.update(cx, |c, _| {
                        //         c.port = Some(port);
                        //     });
                        // }
                    } else {
                        return;
                    }
                }
                NumberInputEvent::Step(step_action) => match step_action {
                    ui::input::StepAction::Decrement => {
                        this.config.update(cx, |c, cx| {
                            c.port = c.port.map(|p| p.saturating_sub(1));
                        });
                    }
                    ui::input::StepAction::Increment => {
                        this.config.update(cx, |c, cx| {
                            c.port = c.port.map(|p| p.saturating_add(1));
                        });
                    }
                },
            };
            dbg!("1");
            let str = this
                .config
                .read(cx)
                .port
                .map(|p| p.to_string())
                .unwrap_or("".into());
            dbg!("2");

            // this.port_input.update(cx, |i, cx| {
            //     i.set_value(str, window, cx);
            // });
        })
        .detach();

        Self { config, port_input }
    }
}

impl Render for GeneralConfigView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div().flex().flex_col().child(self.port_input.clone())
    }
}
