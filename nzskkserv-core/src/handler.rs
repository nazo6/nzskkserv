use std::fmt::Display;

#[allow(async_fn_in_trait)]
pub trait Handler: Sync {
    type Error: Display;

    const SERVER_VERSION: &'static str;

    async fn resolve_word(&self, input: &str) -> Result<Option<Vec<String>>, Self::Error>;
    fn get_hostname(&self) -> Result<String, Self::Error> {
        Ok("localhost".to_string())
    }
}
