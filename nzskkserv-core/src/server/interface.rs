#[derive(Debug, Clone)]
pub enum SkkIncomingEvent {
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

#[derive(Debug, Clone)]
pub enum SkkOutGoingEvent {
    Convert(Option<String>),
    Version,
    Hostname,
    Server,
}
