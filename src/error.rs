use std::fmt::Display;

#[derive(Debug, Default)]
pub enum ErrorKind {
    Input,
    Parse,
    Config,
    Output,
    #[default]
    Generic,
}
pub struct SurvexError {
    pub message: String,
    pub kind: ErrorKind,
}
impl SurvexError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            kind: ErrorKind::default()
        }
    }
    pub fn kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }
}
impl From<&str> for ErrorKind {
    fn from(s: &str) -> Self {
        match s {
            "input" => ErrorKind::Input,
            "parse" => ErrorKind::Parse,
            "config" => ErrorKind::Config,
            "output" => ErrorKind::Output,
            _ => ErrorKind::Generic,
        }
    }
}
impl Display for SurvexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
impl From<std::io::Error> for SurvexError {
    fn from(e: std::io::Error) -> Self {
        Self {
            message: e.to_string(),
            kind: ErrorKind::Generic,
        }
    }
}