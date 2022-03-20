#[derive(Debug)]
pub(crate) enum SkkIncomingEvent {
    /// 0
    Disconnect,
    /// 1
    Convert(String),
    /// 2
    Version,
    /// 3
    Hostname,
    /// 4
    Server,
}

#[derive(Debug)]
pub struct Candidates {
    pub content: Vec<String>,
    pub anotation: Option<String>
}

#[derive(Debug)]
pub(crate) enum SkkOutcomingEvent {
    Convert(Candidates),
    Version,
    Hostname,
    Server,
}
