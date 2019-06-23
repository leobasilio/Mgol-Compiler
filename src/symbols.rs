use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub type SharedSymbol = Rc<RefCell<Symbol>>;

pub mod tokens {

    pub const IDENTIFIER: &str = "ID";
    pub const NUMBER: &str = "NUM";
    pub const LITERAL: &str = "LITERAL";
    pub const COMMENT: &str = "COMENTARIO";
    pub const WHITESPACE: &str = "BRANCO";
    pub const EOF: &str = "EOF";
    pub const RELATIONAL: &str = "OPR";
    pub const ARITHMETIC: &str = "OPM";
    pub const ATTRIBUTION: &str = "RCB";
    pub const OPEN_PARENTHESIS: &str = "AB_P";
    pub const CLOSE_PARENTHESIS: &str = "FC_P";
    pub const SEMICOLON: &str = "PT_V";
    pub const ERROR: &str = "ERRO";

}

#[derive(Copy,Clone,PartialEq)]
pub enum DataType {
    INTEGER,
    REAL,
    LITERAL
}

pub struct Symbol {
    pub lexeme: String,
    pub token: &'static str,
    pub data_type: Option<DataType>
}

pub struct Table {
    symbols: HashMap<String, SharedSymbol>
}

impl Table {

    pub fn new() -> Self {

        let mut symbols: HashMap<String, SharedSymbol> = HashMap::new();

        let keywords = [
            "inicio",
            "varinicio",
            "varfim",
            "escreva",
            "leia",
            "se",
            "entao",
            "fimse",
            "fim",
            "inteiro",
            "lit",
            "real",
            "enquanto",
            "faca",
            "fimenquanto"
        ];

        for &keyword in &keywords {

            symbols.insert(String::from(keyword), Table::make_symbol(&keyword, &keyword, None));

        }

        Table {
            symbols
        }

    }

    pub fn insert(&mut self, lexeme: &str, token: &'static str) -> SharedSymbol {

        let key = String::from(lexeme);

        if !self.symbols.contains_key(&key) {

            self.symbols.insert(key.clone(), Table::make_symbol(lexeme, token, None));

        }

        return self.symbols.get(&key).unwrap().clone();

    }

    pub fn make_symbol(lexeme: &str, token: &'static str, data_type: Option<DataType>) -> SharedSymbol {
        Rc::new(RefCell::new(Symbol {
            lexeme: String::from(lexeme),
            token,
            data_type
        }))
    }

}