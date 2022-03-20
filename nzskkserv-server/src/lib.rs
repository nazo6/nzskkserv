pub mod error;
pub mod interface;
pub mod server;
pub use server::Server;

mod shutdown;

pub(crate) mod codec;
pub(crate) mod handler;

pub enum Encoding {
    Utf8,
    Eucjp,
}

pub use interface::Candidates;
