use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct PdaError {
    terminals: Vec<String>
}

impl fmt::Display for PdaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Esperado \"{}\"", self.terminals.join("\", \""))
    }
}

impl Error for PdaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl PdaError {
    pub fn new(terminals: Vec<String>) -> Self {
        PdaError {
            terminals
        }
    }
}