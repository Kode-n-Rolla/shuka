#[derive(Debug)]
pub enum ShukaError {
    Cli(String),
    Config(String),
    Explorer(String),
    Parser(String),
    Storage(String),
}
