use symbols;
use symbols::Symbol;

pub struct Semantic {
    t_counter: u32
}

pub type ReductionHandler = fn(&mut Semantic, &[Symbol]) -> Symbol;

impl<'a> Semantic {

    pub fn new() -> Self {
        Semantic {
            t_counter: 0
        }
    }

    pub fn handle_type(&mut self, stack: &[Symbol]) -> Symbol {

        let item = stack.first().unwrap();

        match item.lexeme.as_ref(){

            "inteiro" => Semantic::make_symbol("int", item.token, Some(symbols::types::INTEGER)),

            "real" => Semantic::make_symbol("double", item.token, Some(symbols::types::REAL)),

            "lit" => Semantic::make_symbol("literal", item.token, Some(symbols::types::LITERAL)),

            _ => panic!("Tipo desconhecido")

        }

    }

    pub fn handle_var_decl(&mut self, stack: &[Symbol]) -> Symbol {

        let id = &stack[0];
        let data_type = &stack[1];

        println!("{} {};", data_type.lexeme, id.lexeme);

        return Semantic::make_symbol("D", "", None);

    }

    pub fn handle_input(&mut self, stack: &[Symbol]) -> Symbol {

        let id = &stack[1];

        println!("scanf(\"%s\", {});", id.lexeme);

        self.null()

    }

    pub fn null(&self) -> Symbol {

        Semantic::make_symbol("", "", None)

    }

    fn make_symbol(lexeme: &str, token: &'static str, data_type: Option<&'static str>) -> Symbol{

        Symbol {
            lexeme: String::from(lexeme),
            token: token,
            data_type: data_type
        }

    }

}