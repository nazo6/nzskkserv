use std::{fmt::Display, future::Future};

#[derive(Clone, Debug)]
pub struct Entry {
    pub candidate: String,
    pub description: Option<String>,
}

#[allow(async_fn_in_trait)]
pub trait Handler: Sync + Send + 'static {
    type Error: Display + Send + Sync + 'static;

    const SERVER_VERSION: &'static str;

    fn resolve_word(
        &self,
        input: &str,
    ) -> impl Future<Output = Result<Vec<Entry>, Self::Error>> + Send;
    fn get_hostname(&self) -> Result<String, Self::Error> {
        Ok("localhost".to_string())
    }
}
