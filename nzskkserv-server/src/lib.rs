pub mod server;
pub use server::error::Error;

pub enum Encoding {
    Utf8,
    Eucjp,
}
