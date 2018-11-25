use std;
use std::fmt;

#[derive(Debug, Clone)]
pub struct GameError {
    pub message: String,
    pub line: usize,
    pub column: usize,    
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl std::error::Error for GameError {
    fn description(&self) -> &str {
        &self.message
    }
}
