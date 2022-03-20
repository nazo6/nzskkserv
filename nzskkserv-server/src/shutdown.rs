use tokio::sync::broadcast;

#[derive(Debug)]
pub(crate) struct Shutdown {
    /// `true` if the shutdown signal has been received
    shutdown: bool,

    /// The receive half of the channel used to listen for shutdown.
    notify: broadcast::Receiver<bool>,
}

impl Shutdown {
    /// Create a new `Shutdown` backed by the given `broadcast::Receiver`.
    pub(crate) fn new(notify: broadcast::Receiver<(bool)>) -> Shutdown {
        Shutdown {
            shutdown: false,
            notify,
        }
    }

    /// Returns `true` if the shutdown signal has been received.
    pub(crate) fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    /// Receive the shutdown notice, waiting if necessary.
    pub(crate) async fn recv(&mut self) {
        // If the shutdown signal has already been received, then return
        // immediately.
        if self.shutdown {
            return;
        }

        let res = self.notify.recv().await.unwrap();
        self.shutdown = res;
    }
}
