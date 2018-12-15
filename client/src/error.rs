use std;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ClientError {
    pub message: String,
    pub line: usize,
    pub column: usize,    
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl std::error::Error for ClientError {
    fn description(&self) -> &str {
        &self.message
    }
}
