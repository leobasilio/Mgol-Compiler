mod symbols;
mod analyzers;

use std::env;

fn run_compiler(filename: &str) -> std::io::Result<()> {

    let mut table = symbols::Table::new();
    let mut lexical = analyzers::Lexical::new(&mut table);

    lexical.load(filename)?;

    println!("{0: <20} {1: <30} {2: <20}", "TOKEN", "LEXEMA", "TIPO");

    loop {

        let item = lexical.next_token();

        if item.token.eq(symbols::tokens::ERROR) {

            println!("{0: <20} Token inválido: Linha {1} Coluna {2}", item.lexeme, lexical.current_line(), lexical.current_column() - item.lexeme.chars().count());

            break;

        }else{

            println!("{0: <20} {1: <30} {2: <20}", item.token, item.lexeme, item.data_type.unwrap_or("-".to_string()));

            if item.token.eq(symbols::tokens::EOF) {

                break;

            }

        }

    }

    Ok(())

}

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {

        println!("Usage: mgol file.alg");

    }else{

        if let Err(e) = run_compiler(&args[1]) {

            println!("{}", e);

        }

    }

}