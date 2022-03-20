use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

use crate::error::Error;
use crate::Candidates;

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    #[must_use]
    async fn apply(self: Pin<&Self>, request: String) -> Result<Candidates, Error>;

    #[doc(hidden)]
    fn describe(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name::<Self>())
    }
}

impl std::fmt::Debug for dyn Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.describe(f)
    }
}

#[async_trait]
impl<F, Fut> Handler for F
where
    F: Fn(String) -> Fut + Sync + Send + 'static,
    Fut: Future<Output = Candidates> + Send + 'static,
{
    async fn apply(self: Pin<&Self>, request: String) -> Result<Candidates, Error> {
        Ok(self(request).await)
    }
}
