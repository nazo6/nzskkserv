pub mod server;
pub use server::error::Error;
pub use server::Server;

#[derive(Clone)]
pub enum Encoding {
    Utf8,
    Eucjp,
}

pub type Dict = std::collections::HashMap<String, String>;
