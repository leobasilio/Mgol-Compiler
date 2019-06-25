use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct CompilerErrorItem {
    internal: Box<Error>,
    line_number: usize,
    column_number: usize
}

#[derive(Debug)]
pub struct CompilerErrors {
    errors: Vec<CompilerErrorItem>
}

impl fmt::Display for CompilerErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        if let Some(e) = self.errors.first() {

            write!(f, "{}, linha {}, coluna {}", e.internal, e.line_number, e.column_number)?;

            for e in self.errors.iter().skip(1) {

                write!(f, "\n\n{}, linha {}, coluna {}", e.internal, e.line_number, e.column_number)?;

            }

        }

        Ok(())

    }
}

impl CompilerErrors {

    pub fn new() -> Self {
        CompilerErrors {
            errors: vec![]
        }
    }

    pub fn is_empty(&self) -> bool{
        return self.errors.is_empty();
    }

    pub fn push(&mut self, error: Box<Error>){
        self.errors.push(CompilerErrorItem {
            internal: error,
            line_number: 0,
            column_number: 0
        });
    }

    pub fn push_ln(&mut self, error: Box<Error>, line_number: usize, column_number: usize){
        self.errors.push(CompilerErrorItem {
            internal: error,
            line_number,
            column_number
        });
    }

    pub fn merge_ln(&mut self, others: CompilerErrors, line_number: usize, column_number: usize){

       for mut e in others.errors {

            e.line_number = line_number;
            e.column_number = column_number;

            self.errors.push(e);

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

impl EndOfFileError {
    pub fn new() -> Self {
        EndOfFileError{}
    }
}