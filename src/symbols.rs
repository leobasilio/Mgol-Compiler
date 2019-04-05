use std::collections::HashMap;

pub mod tokens {

    pub const IDENTIFIER: &str = "_ID_";
    pub const NUMBER: &str = "_NUM_";
    pub const LITERAL: &str = "_LITERAL_";
    pub const COMMENT: &str = "_COMENTARIO_";
    pub const SPACE: &str = "_BRANCO_";
    pub const EOF: &str = "_EOF_";
    pub const RELATIONAL: &str = "_OPR_";
    pub const ARITHMETIC: &str = "_OPM_";
    pub const ATTRIBUTION: &str = "_RCB_";
    pub const OPEN_PARENTHESIS: &str = "_AB_P_";
    pub const CLOSE_PARENTHESIS: &str = "_FC_P_";
    pub const SEMICOLON: &str = "_PT_V_";

}

pub struct Symbol {
    pub lexeme: String,
    pub token: String,
    pub data_type: Option<String>
}

pub struct Table {
    symbols: HashMap<String, Symbol>
}

impl Table {

    pub fn new() -> Self {
        Table {
            symbols: HashMap::new()
        }
    }

    pub fn insert(&mut self, lexeme: &str, token: &str) {

        self.symbols.insert(String::from(lexeme), Symbol {
            lexeme: String::from(lexeme),
            token: String::from(token),
            data_type: None
        });

    }

}
