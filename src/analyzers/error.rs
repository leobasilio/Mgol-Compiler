use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum SemanticErrorType {
    Undeclared,
    IncompatibleTypes,
    IncompatibleAssignment
}

#[derive(Debug)]
pub struct PdaError {
    terminals: Vec<String>
}

#[derive(Debug)]
pub struct SemanticError {
    error_type: SemanticErrorType,
    info: String
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

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error_type {
            SemanticErrorType::Undeclared => write!(f, "Variável não declarada: {}", self.info),
            SemanticErrorType::IncompatibleTypes => write!(f, "Operandos com tipos incompatíveis"),
            SemanticErrorType::IncompatibleAssignment => write!(f, "Tipos diferentes para atribuição"),
        }
    }
}

impl Error for SemanticError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl SemanticError {
    pub fn new(error_type: SemanticErrorType, info: &str) -> Self {
        SemanticError {
            error_type,
            info: String::from(info)
        }
    }
}