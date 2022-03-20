use super::Server;

impl Server {
    pub async fn set_dicts(&self, dicts: crate::Dicts) {
        let mut config = self.process.lock().await;
        config.dicts = dicts;
    }
    pub async fn set_google_cgi(&self, enable: bool) {
        let mut config = self.process.lock().await;
        config.enable_google_cgi = enable;
    }
}
