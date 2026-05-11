#[derive(Debug)]
pub enum ShukaError {
    Cli(String),
    Config(String),
    Explorer(String),
    Parser(String),
    Storage(String),
}

impl std::fmt::Display for ShukaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShukaError::Cli(message) => write!(f, "cli -> {message}"),
            ShukaError::Config(message) => write!(f, "config -> {message}"),
            ShukaError::Explorer(message) => write!(f, "explorer -> {message}"),
            ShukaError::Parser(message) => write!(f, "parser -> {message}"),
            ShukaError::Storage(message) => write!(f, "storage -> {message}"),
        }
    }
}

impl std::error::Error for ShukaError {}