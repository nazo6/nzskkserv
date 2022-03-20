pub mod server;
pub use server::error::Error;

#[derive(Clone)]
pub enum Encoding {
    Utf8,
    Eucjp,
}
