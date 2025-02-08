use dioxus::prelude::*;

use crate::server::{ServerState, ServerStateController};

pub fn use_server_state() -> ReadOnlySignal<ServerState> {
    let server_ctrl = use_context::<ServerStateController>();
    let mut server_state = use_signal(|| server_ctrl.borrow().clone());

    use_effect(move || {
        let server_ctrl = server_ctrl.clone();
        let mut receiver = server_ctrl.subscribe();
        spawn(async move {
            loop {
                let _ = receiver.changed().await;
                server_state.set(server_ctrl.borrow().clone());
            }
        });
    });

    ReadOnlySignal::new(server_state)
}

pub fn use_set_server_state() -> ServerStateController {
    use_context::<ServerStateController>()
}
