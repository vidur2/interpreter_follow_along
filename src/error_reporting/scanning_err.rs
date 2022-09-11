use super::error_reporter::Unwindable;

pub type Result<T> = std::result::Result<T, ScanningException>;

#[derive(Clone)]
pub enum ScanningException {
    Tokenization,
    Newline,
    Ignore,
    Commment,
    String,
    Number,
    UnterminatedString
}

impl Unwindable for ScanningException {
    fn get_value(&self) -> String {
        match self {
            ScanningException::Tokenization => String::from("Tokenization Error"),
            ScanningException::UnterminatedString => String::from("Unterminated String"),
            _ => String::new(),
        }
    }
}
