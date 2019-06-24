use symbols;
use symbols::DataType;
use symbols::Symbol;
use symbols::SharedSymbol;
use analyzers::error::SemanticError;
use analyzers::error::SemanticErrorType;
use std::io::Write;

pub struct Semantic {
    temp: Vec<DataType>,
    buffer: Vec<String>,
    loop_expr: Vec<String>
}

pub type ReductionHandler = fn(&mut Semantic, &[SharedSymbol]) -> Result<SharedSymbol, SemanticError>;

impl Semantic {

    pub fn new() -> Self {
        Semantic {
            temp: vec![],
            buffer: vec![],
            loop_expr: vec![]
        }
    }

    pub fn reset(&mut self){
        self.temp = vec![];
        self.buffer = vec![];
        self.loop_expr = vec![];
    }

    pub fn handle_type_int(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(Semantic::make_symbol("int", "", Some(DataType::INTEGER)))

    }

    pub fn handle_type_real(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(Semantic::make_symbol("double", "", Some(DataType::REAL)))

    }

    pub fn handle_type_lit(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(Semantic::make_symbol("literal", "", Some(DataType::LITERAL)))

    }

    pub fn handle_var_decl(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 2 {

            let mut id = stack[0].borrow_mut();
            let dtype = stack[1].borrow();

            self.buffer.push(format!("{} {};", dtype.lexeme, id.lexeme));

            id.data_type = dtype.data_type;

        }

        Ok(Semantic::null())

    }

    pub fn handle_es_in(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 2 {

            let id = stack[1].borrow();

            match Semantic::get_data_type(&id)? {

                DataType::INTEGER => self.buffer.push(format!("scanf(\"%d\", &{});", id.lexeme)),

                DataType::REAL => self.buffer.push(format!("scanf(\"%lf\", &{});", id.lexeme)),

                DataType::LITERAL => self.buffer.push(format!("scanf(\"%s\", {});", id.lexeme))

            }

        }

        Ok(Semantic::null())

    }

    pub fn handle_es_out(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 2 {

            let item = stack[1].borrow();

            match item.token {

                symbols::tokens::NUMBER => self.buffer.push(format!("printf(\"{}\");", item.lexeme)),

                symbols::tokens::LITERAL => self.buffer.push(format!("printf(\"%s\", {});", item.lexeme)),

                symbols::tokens::IDENTIFIER => {

                    match Semantic::get_data_type(&item)? {

                        DataType::INTEGER => self.buffer.push(format!("printf(\"%d\", {});", item.lexeme)),

                        DataType::REAL => self.buffer.push(format!("printf(\"%lf\", {});", item.lexeme)),

                        DataType::LITERAL => self.buffer.push(format!("printf(\"%s\", {});", item.lexeme))

                    }

                },

                _ => panic!("handle_es_out: Token inesperado")

            }

        }

        Ok(Semantic::null())

    }

    pub fn handle_arg_lit(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Semantic::null() })

    }

    pub fn handle_arg_num(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Semantic::null() })

    }

    pub fn handle_arg_id(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if let Some(item) = stack.first() {

            Semantic::get_data_type(&item.borrow())?;

            Ok(item.clone())

        }else{

            Ok(Semantic::null())

        }

    }

    pub fn handle_oprd_num(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Semantic::null() })

    }

    pub fn handle_oprd_id(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if let Some(item) = stack.first() {

            Semantic::get_data_type(&item.borrow())?;

            Ok(item.clone())

        }else{

            Ok(Semantic::null())

        }

    }

    pub fn handle_cmd(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 3 {

            let id = stack[0].borrow();
            let ld = stack[2].borrow();
            let data_type = Semantic::get_data_type(&id)?;

            if Some(data_type) == ld.data_type {

                if data_type == DataType::LITERAL {

                    self.buffer.push(format!("sprintf({}, \"%s\", {});", id.lexeme, ld.lexeme));

                }else{

                    self.buffer.push(format!("{} = {};", id.lexeme, ld.lexeme));

                }

            }else{

                return Err(SemanticError::new(SemanticErrorType::IncompatibleAssignment, ""));

            }

        }

        Ok(Semantic::null())

    }

    pub fn handle_ld(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Semantic::null() })

    }

    pub fn handle_ld_opm(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 3 {

            let op1 = stack[0].borrow();
            let op2 = stack[2].borrow();
            let opm = stack[1].borrow();

            if op1.data_type == op2.data_type && op1.data_type != Some(DataType::LITERAL) {

                let x = self.temp.len();

                self.buffer.push(format!("T{} = {} {} {};", x, op1.lexeme, opm.lexeme, op2.lexeme));
                self.temp.push(op1.data_type.unwrap());

                Ok(Semantic::make_symbol(&format!("T{}", x), "", op1.data_type))

            }else{

                Err(SemanticError::new(SemanticErrorType::IncompatibleTypes, ""))

            }

        }else{

            Ok(Semantic::null())

        }

    }

    pub fn handle_expr(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 3 {

            let op1 = stack[0].borrow();
            let op2 = stack[2].borrow();
            let opr = stack[1].borrow();

            if op1.token != symbols::tokens::LITERAL && op2.token != symbols::tokens::LITERAL {

                let x = self.temp.len();
                let opr_lexeme = if opr.lexeme == "=" { "==" }else{ &opr.lexeme };

                self.buffer.push(format!("T{} = {} {} {};", x, op1.lexeme, opr_lexeme, op2.lexeme));
                self.temp.push(op1.data_type.unwrap());

                Ok(Semantic::make_symbol(&format!("T{}", x), "", Some(DataType::INTEGER)))

            }else{

                Err(SemanticError::new(SemanticErrorType::IncompatibleTypes, ""))

            }

        }else{

            Ok(Semantic::null())

        }

    }

    pub fn handle_if_begin(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 3 {

            let expr = stack[2].borrow();

            self.buffer.push(format!("if({}){{", expr.lexeme));

        }

        Ok(Semantic::null())

    }

    pub fn handle_if_end(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        self.buffer.push("}".to_string());

        Ok(Semantic::null())

    }

    pub fn handle_while_begin(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        self.loop_expr.push(if let Some(last) = self.buffer.last() {
            last.clone()
        }else{
            String::from("")
        });

        if stack.len() >= 3 {

            let expr = stack[2].borrow();

            self.buffer.push(format!("while({}){{", expr.lexeme));

        }

        Ok(Semantic::null())

    }

    pub fn handle_while_end(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if let Some(expr) = self.loop_expr.pop() {

            self.buffer.push(format!("{}\n}}", expr));

        }

        Ok(Semantic::null())

    }

    pub fn handle_null(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(Semantic::null())

    }

    pub fn dump(&self, output_file: &str) -> std::io::Result<()> {

        let mut f = std::fs::File::create(output_file)?;

        writeln!(&mut f, "#include <stdio.h>
typedef char literal[256];
void main(void){{
/*----Variaveis temporarias----*/")?;

        for (i, &x) in self.temp.iter().enumerate() {

            if x == DataType::REAL {

                writeln!(&mut f, "double T{};", i)?;

            }else{

                writeln!(&mut f, "int T{};", i)?;

            }

        }

        writeln!(&mut f, "/*------------------------------*/
{}
}}", self.buffer.join("\n"))?;

        Ok(())

    }

    pub fn get_data_type(item: &Symbol) -> Result<DataType, SemanticError> {

        if let Some(data_type) = item.data_type {

            Ok(data_type)

        }else{

            Err(SemanticError::new(SemanticErrorType::Undeclared, &item.lexeme))

        }

    }

    pub fn null() -> SharedSymbol {

        Semantic::make_symbol("", "", None)

    }

    fn make_symbol(lexeme: &str, token: &'static str, data_type: Option<DataType>) -> SharedSymbol {

        symbols::Table::make_symbol(lexeme, token, data_type)

    }

}