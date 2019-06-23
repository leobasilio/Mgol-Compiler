use std::rc::Rc;
use std::cell::RefCell;
use analyzers::error::SemanticError;
use analyzers::error::SemanticErrorType;
use symbols;
use symbols::DataType;
use symbols::Symbol;
use symbols::SharedSymbol;

pub struct Semantic {
    t_counter: u32,
    buffer: Vec<String>,
    loop_expr: Vec<String>
}

pub type ReductionHandler = fn(&mut Semantic, &[SharedSymbol]) -> Result<SharedSymbol, SemanticError>;

impl<'a> Semantic {

    pub fn new() -> Self {
        Semantic {
            t_counter: 0,
            buffer: vec![],
            loop_expr: vec![]
        }
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

        let mut id = stack[0].borrow_mut();
        let dtype = stack[1].borrow();

        self.buffer.push(format!("{} {};", dtype.lexeme, id.lexeme));

        id.data_type = dtype.data_type;

        Ok(Semantic::null())

    }

    pub fn handle_es_in(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let id = stack[1].borrow();

        match Semantic::get_data_type(&id)? {

            DataType::INTEGER => self.buffer.push(format!("scanf(\"%d\", &{});", id.lexeme)),

            DataType::REAL => self.buffer.push(format!("scanf(\"%lf\", &{});", id.lexeme)),

            DataType::LITERAL => self.buffer.push(format!("scanf(\"%s\", {});", id.lexeme))

        }

        Ok(Semantic::null())

    }

    pub fn handle_es_out(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

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

        Ok(Semantic::null())

    }

    pub fn handle_arg_lit(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(stack.first().unwrap().clone())

    }

    pub fn handle_arg_num(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(stack.first().unwrap().clone())

    }

    pub fn handle_arg_id(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let item = stack.first().unwrap().clone();

        Semantic::get_data_type(&item.borrow())?;

        Ok(item)

    }

    pub fn handle_oprd_num(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(stack.first().unwrap().clone())

    }

    pub fn handle_oprd_id(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let item = stack.first().unwrap().clone();

        Semantic::get_data_type(&item.borrow())?;

        Ok(item)

    }

    pub fn handle_cmd(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let id = stack[0].borrow();
        let ld = stack[2].borrow();
        let data_type = Semantic::get_data_type(&id)?;

        if data_type != DataType::LITERAL {

            self.buffer.push(format!("{} = {};", id.lexeme, ld.lexeme));

            Ok(Semantic::null())

        }else{

            Err(SemanticError::new(SemanticErrorType::IncompatibleAssignment, ""))

        }

    }

    pub fn handle_ld(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Ok(stack.first().unwrap().clone())

    }

    pub fn handle_ld_opm(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let op1 = stack[0].borrow();
        let op2 = stack[2].borrow();
        let opm = stack[1].borrow();

        if op1.token != symbols::tokens::LITERAL && op2.token != symbols::tokens::LITERAL {

            let x = self.t_counter;

            self.t_counter += 1;

            self.buffer.push(format!("T{} = {} {} {};", x, op1.lexeme, opm.lexeme, op2.lexeme));

            Ok(Semantic::make_symbol(&format!("T{}", x), "", Some(DataType::INTEGER)))

        }else{

            Err(SemanticError::new(SemanticErrorType::IncompatibleTypes, ""))

        }

    }

    pub fn handle_expr(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let op1 = stack[0].borrow();
        let op2 = stack[2].borrow();
        let opr = stack[1].borrow();

        if op1.token != symbols::tokens::LITERAL && op2.token != symbols::tokens::LITERAL {

            let x = self.t_counter;

            self.t_counter += 1;

            self.buffer.push(format!("T{} = {} {} {};", x, op1.lexeme, opr.lexeme, op2.lexeme));

            Ok(Semantic::make_symbol(&format!("T{}", x), "", Some(DataType::INTEGER)))

        }else{

            Err(SemanticError::new(SemanticErrorType::IncompatibleTypes, ""))

        }

    }

    pub fn handle_if_begin(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let expr = stack[2].borrow();

        self.buffer.push(format!("if({}){{", expr.lexeme));

        Ok(Semantic::null())

    }

    pub fn handle_if_end(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        self.buffer.push("}".to_string());

        Ok(Semantic::null())

    }

    pub fn handle_while_begin(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        let expr = stack[2].borrow();

        self.loop_expr.push(self.buffer.last().unwrap().to_string());

        self.buffer.push(format!("while({}){{", expr.lexeme));

        Ok(Semantic::null())

    }

    pub fn handle_while_end(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        self.buffer.push(format!("{}\n}}", self.loop_expr.pop().unwrap()));

        Ok(Semantic::null())

    }

    pub fn dump(&mut self){

        println!("#include <stdio.h>
typedef char literal[256];
void main(void){{
/*----Variaveis temporarias----*/");

        for i in 0..self.t_counter {

            println!("int T{};", i);

        }

        println!("/*------------------------------*/
{}
}}", self.buffer.join("\n"));

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

        Rc::new(RefCell::new(Symbol {
            lexeme: String::from(lexeme),
            token: token,
            data_type: data_type
        }))

    }

}