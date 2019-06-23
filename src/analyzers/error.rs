use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct CompilerError {
    internal_error: Box<Error>,
    line_number: usize,
    column_number: usize
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, linha {}, coluna {}", self.internal_error, self.line_number, self.column_number)
    }
}

impl CompilerError {
    pub fn new(internal_error: Box<Error>, line_number: usize, column_number: usize) -> Self {
        CompilerError {
            internal_error,
            line_number,
            column_number
        }
    }
}

//================================
// LEXICAL
//================================

#[derive(Debug)]
pub struct LexicalError {
    lexeme: String
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Erro Léxico: Token \"{}\" inválido", self.lexeme)
    }
}

impl Error for LexicalError {}

impl LexicalError {
    pub fn new(lexeme: &str) -> Self {
        LexicalError {
            lexeme: lexeme.to_string()
        }
    }
}

//================================
// SYNTACTIC
//================================

#[derive(Debug)]
pub struct SyntacticError {
    terminals: Vec<String>
}

impl fmt::Display for SyntacticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Erro Sintático: Esperado \"{}\"", self.terminals.join("\", \""))
    }
}

impl Error for SyntacticError {}

impl SyntacticError {
    pub fn new(terminals: Vec<String>) -> Self {
        SyntacticError {
            terminals
        }
    }
}

//================================
// SEMANTIC
//================================

#[derive(Debug)]
pub enum SemanticErrorType {
    Undeclared,
    IncompatibleTypes,
    IncompatibleAssignment
}

#[derive(Debug)]
pub struct SemanticError {
    error_type: SemanticErrorType,
    info: String
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type {
            SemanticErrorType::Undeclared => write!(f, "Erro Semântico: Variável não declarada: {}", self.info),
            SemanticErrorType::IncompatibleTypes => write!(f, "Erro Semântico: Operandos com tipos incompatíveis"),
            SemanticErrorType::IncompatibleAssignment => write!(f, "Erro Semântico: Tipos diferentes para atribuição"),
        }
    }
}

impl Error for SemanticError {}

impl SemanticError {
    pub fn new(error_type: SemanticErrorType, info: &str) -> Self {
        SemanticError {
            error_type,
            info: String::from(info)
        }
    }
}

//================================
// END OF FILE
//================================

#[derive(Debug)]
pub struct EndOfFileError {}

impl fmt::Display for EndOfFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Final inesperado do arquivo")
    }
}

impl Error for EndOfFileError {}