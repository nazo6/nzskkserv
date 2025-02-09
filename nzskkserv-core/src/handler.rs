use std::{fmt::Display, future::Future};

#[allow(async_fn_in_trait)]
pub trait Handler: Sync + Send + 'static {
    type Error: Display + Send + Sync + 'static;

    const SERVER_VERSION: &'static str;

    fn resolve_word(
        &self,
        input: &str,
    ) -> impl Future<Output = Result<Option<Vec<String>>, Self::Error>> + Send;
    fn get_hostname(&self) -> Result<String, Self::Error> {
        Ok("localhost".to_string())
    }
}
