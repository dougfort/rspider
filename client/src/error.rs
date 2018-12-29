use std::fmt;
use std::error::Error;


#[derive(Debug, Clone)]
pub struct ClientError {
    pub message: String,
    pub line: u32,
    pub column: u32,    
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl Error for ClientError {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
