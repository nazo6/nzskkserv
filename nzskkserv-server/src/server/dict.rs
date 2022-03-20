use super::{Server, Dicts};

impl Server {
    pub fn set_dicts(&self, dicts: Dicts) {
        let mut config = self.process.blocking_lock();
        config.dicts = dicts;
    }
    pub fn set_google_ime(&self, enable: bool) {
        let mut config = self.process.blocking_lock();
        config.enable_google_ime = enable;
    }
}
