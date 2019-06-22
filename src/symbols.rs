use std::collections::HashMap;

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

pub mod types {

    pub const INTEGER: &str = "inteiro";
    pub const REAL: &str = "real";
    pub const LITERAL: &str = "literal";

}

#[derive(Clone)]
pub struct Symbol {
    pub lexeme: String,
    pub token: &'static str,
    pub data_type: Option<&'static str>
}

pub struct Table {
    symbols: HashMap<String, Symbol>
}

impl<'a> Table {

    pub fn new() -> Self {

        let mut symbols: HashMap<String, Symbol> = HashMap::new();

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

            symbols.insert(String::from(keyword), Table::make_symbol(&keyword, &keyword));

        }

        Table {
            symbols
        }

    }

    pub fn insert(&'a mut self, lexeme: &str, token: &'static str) -> &'a Symbol {

        let key = String::from(lexeme);

        if !self.symbols.contains_key(&key) {

            self.symbols.insert(key.clone(), Table::make_symbol(lexeme, token));

        }

        return self.symbols.get(&key).unwrap();

    }

    pub fn make_symbol(lexeme: &str, token: &'static str) -> Symbol {
        Symbol {
            lexeme: String::from(lexeme),
            token: token,
            data_type: None
        }
    }

}