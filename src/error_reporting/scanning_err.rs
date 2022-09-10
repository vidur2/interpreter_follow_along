use super::error_reporter::Unwindable;

pub type Result<T> = std::result::Result<T, ScanningError>;

#[derive(Clone)]
pub enum ScanningError {
    Tokenization,
    Newline,
    Reading,
    Scanning,
}

impl Unwindable for ScanningError {
    fn get_value(&self) -> String {
        match self {
            ScanningError::Tokenization => String::from("Tokenization Error"),
            ScanningError::Newline => String::new(),
            ScanningError::Reading => String::from("Reading Error"),
            ScanningError::Scanning => String::from("Scanning Error"),
        }
    }
}
