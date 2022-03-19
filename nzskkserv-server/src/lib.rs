pub mod server;
pub mod error;

pub enum Encoding {
    Utf8,
    Eucjp,
}

pub use server::interface::Candidates;
