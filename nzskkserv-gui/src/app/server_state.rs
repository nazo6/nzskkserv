use dioxus::prelude::*;
use tracing::error;

use crate::server::{ServerState, ServerStateController};

pub fn use_server_state() -> ReadOnlySignal<ServerState> {
    let server_ctrl = use_context::<ServerStateController>();
    let mut server_state = use_signal(|| server_ctrl.borrow().clone());

    use_future(move || {
        let server_ctrl = server_ctrl.clone();
        let mut receiver = server_ctrl.subscribe();
        async move {
            loop {
                match receiver.changed().await {
                    Ok(_) => server_state.set(server_ctrl.borrow().clone()),
                    Err(e) => {
                        error!("Error receiving server state change: {}", e);
                        break; // Exit the loop if there's an error
                    }
                }
            }
        }
    });

    ReadOnlySignal::new(server_state)
}

pub fn use_set_server_state() -> ServerStateController {
    use_context::<ServerStateController>()
}
