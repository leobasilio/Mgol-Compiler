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

    fn print(s: &str){

        println!("\x1B[0;32m{}\n\x1B[0m", s);

    }

    pub fn handle_type_int(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("TIPO.tipo <- int.tipo");

        Ok(Self::make_symbol("int", "", Some(DataType::INTEGER)))

    }

    pub fn handle_type_real(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("TIPO.tipo <- real.tipo");

        Ok(Self::make_symbol("double", "", Some(DataType::REAL)))

    }

    pub fn handle_type_lit(&mut self, _stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("TIPO.tipo <- lit.tipo");

        Ok(Self::make_symbol("literal", "", Some(DataType::LITERAL)))

    }

    pub fn handle_var_decl(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 2 {

            let mut id = stack[0].borrow_mut();
            let dtype = stack[1].borrow();

            Self::print("id.tipo <- TIPO.tipo");

            self.buffer.push(format!("{} {};", dtype.lexeme, id.lexeme));

            id.data_type = dtype.data_type;

        }

        Ok(Self::make_nterminal("D"))

    }

    pub fn handle_es_in(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 2 {

            let id = stack[1].borrow();

            let s = match Self::get_data_type(&id)? {

                DataType::INTEGER => format!("scanf(\"%d\", &{});", id.lexeme),

                DataType::REAL => format!("scanf(\"%lf\", &{});", id.lexeme),

                DataType::LITERAL => format!("scanf(\"%s\", {});", id.lexeme)

            };

            Self::print(&format!("Imprimir: {}", s));

            self.buffer.push(s);

        }

        Ok(Self::make_nterminal("ES"))

    }

    pub fn handle_es_out(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 2 {

            let item = stack[1].borrow();

            let s = match item.token {

                symbols::tokens::NUMBER => format!("printf(\"{}\");", item.lexeme),

                symbols::tokens::LITERAL => format!("printf(\"%s\", {});", item.lexeme),

                symbols::tokens::IDENTIFIER => {

                    match Self::get_data_type(&item)? {

                        DataType::INTEGER => format!("printf(\"%d\", {});", item.lexeme),

                        DataType::REAL => format!("printf(\"%lf\", {});", item.lexeme),

                        DataType::LITERAL => format!("printf(\"%s\", {});", item.lexeme)

                    }

                },

                _ => panic!("handle_es_out: Token inesperado")

            };

            Self::print(&format!("Imprimir: {}", s));

            self.buffer.push(s);

        }

        Ok(Self::make_nterminal("ES"))

    }

    pub fn handle_arg_lit(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("ARG.atributos <- literal.atributos");

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Self::make_nterminal("ARG") })

    }

    pub fn handle_arg_num(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("ARG.atributos <- num.atributos");

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Self::make_nterminal("ARG") })

    }

    pub fn handle_arg_id(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("ARG.atributos <- id.atributos");

        if let Some(item) = stack.first() {

            Self::get_data_type(&item.borrow())?;

            Ok(item.clone())

        }else{

            Ok(Self::make_nterminal("ARG"))

        }

    }

    pub fn handle_oprd_num(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("OPRD.atributos <- num.atributos");

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Self::make_nterminal("OPRD") })

    }

    pub fn handle_oprd_id(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("OPRD.atributos <- id.atributos");

        if let Some(item) = stack.first() {

            Self::get_data_type(&item.borrow())?;

            Ok(item.clone())

        }else{

            Ok(Self::make_nterminal("OPRD"))

        }

    }

    pub fn handle_cmd(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 3 {

            let id = stack[0].borrow();
            let ld = stack[2].borrow();
            let data_type = Self::get_data_type(&id)?;

            if Some(data_type) == ld.data_type {

                let s = if data_type == DataType::LITERAL {

                    format!("sprintf({}, \"%s\", {});", id.lexeme, ld.lexeme)

                }else{

                    format!("{} = {};", id.lexeme, ld.lexeme)

                };

                Self::print(&format!("Imprimir: {}", s));

                self.buffer.push(s);

            }else{

                return Err(SemanticError::new(SemanticErrorType::IncompatibleAssignment, ""));

            }

        }

        Ok(Self::make_nterminal("CMD"))

    }

    pub fn handle_ld(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("LD.atributos <- OPRD.atributos");

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Self::make_nterminal("LD") })

    }

    pub fn handle_ld_opm(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 3 {

            let op1 = stack[0].borrow();
            let op2 = stack[2].borrow();
            let opm = stack[1].borrow();

            if op1.data_type == op2.data_type && op1.data_type != Some(DataType::LITERAL) {

                let x = self.temp.len();
                let s = format!("T{} = {} {} {};", x, op1.lexeme, opm.lexeme, op2.lexeme);

                Self::print(&format!("LD.lexema <- T{}\nImprimir: {}", x, s));

                self.buffer.push(s);
                self.temp.push(op1.data_type.unwrap());

                Ok(Self::make_symbol(&format!("T{}", x), "", op1.data_type))

            }else{

                Err(SemanticError::new(SemanticErrorType::IncompatibleTypes, ""))

            }

        }else{

            Ok(Self::make_nterminal("LD"))

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
                let s = format!("T{} = {} {} {};", x, op1.lexeme, opr_lexeme, op2.lexeme);

                Self::print(&format!("EXP_R.lexema <- T{}\nImprimir: {}", x, s));

                self.buffer.push(s);
                self.temp.push(op1.data_type.unwrap());

                Ok(Self::make_symbol(&format!("T{}", x), "", Some(DataType::INTEGER)))

            }else{

                Err(SemanticError::new(SemanticErrorType::IncompatibleTypes, ""))

            }

        }else{

            Ok(Self::make_nterminal("EXP_R"))

        }

    }

    pub fn handle_if_begin(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if stack.len() >= 3 {

            let expr = stack[2].borrow();
            let s = format!("if({}){{", expr.lexeme);

            Self::print(&format!("Imprimir: {}", s));

            self.buffer.push(s);

        }

        Ok(Self::make_nterminal("CABEÇALHO"))

    }

    pub fn handle_if_end(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        Self::print("Imprimir: }");

        self.buffer.push("}".to_string());

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Self::make_nterminal("COND") })

    }

    pub fn handle_while_begin(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        self.loop_expr.push(if let Some(last) = self.buffer.last() {
            last.clone()
        }else{
            String::from("")
        });

        if stack.len() >= 3 {

            let expr = stack[2].borrow();
            let s = format!("while({}){{", expr.lexeme);

            Self::print(&format!("Imprimir: {}", s));

            self.buffer.push(s);

        }

        Ok(Self::make_nterminal("CABEÇALHOREP"))

    }

    pub fn handle_while_end(&mut self, stack: &[SharedSymbol]) -> Result<SharedSymbol, SemanticError> {

        if let Some(expr) = self.loop_expr.pop() {

            let s = format!("{}\n}}", expr);

            Self::print(&format!("Imprimir: {}", s));

            self.buffer.push(s);

        }

        Ok(if let Some(item) = stack.first() { item.clone() }else{ Self::make_nterminal("REP") })

    }

    pub fn make_nterminal(left_side: &str) -> SharedSymbol {

        Self::make_symbol(left_side, "", None)

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

        Self::make_symbol("_", "", None)

    }

    fn make_symbol(lexeme: &str, token: &'static str, data_type: Option<DataType>) -> SharedSymbol {

        symbols::Table::make_symbol(lexeme, token, data_type)

    }

}