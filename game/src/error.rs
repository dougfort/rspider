use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GameError {
    pub message: String,
    pub line: u32,
    pub column: u32,    
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
