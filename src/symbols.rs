use std::collections::HashMap;

//pub const TOKEN_IDENTIFIER: &str = "_ID_";

pub struct Symbol {
    pub lexeme: String,
    pub token: String,
    pub data_type: Option<String>
}

pub struct Table {
    symbols: HashMap<String, Symbol>
}

impl Table {

    pub fn new() -> Table {
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