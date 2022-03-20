pub mod server;

pub use server::error::Error;

#[derive(Clone)]
pub enum Encoding {
    Utf8,
    Eucjp,
}

pub type Dict = std::collections::HashMap<String, String>;
pub type Dicts = Vec<Dict>;
