mod symbols;
mod analyzers;

use std::env;

fn run_compiler(filename: &str) -> std::io::Result<()> {

    let mut table = symbols::Table::new();
    let mut lexical = analyzers::Lexical::new(&mut table);

    lexical.load(filename)?;

    loop {

        let item = lexical.next_token();

        println!("{0: <20} {1: <20}", item.lexeme, item.token);

        if item.token.eq(symbols::tokens::EOF) {
            break;
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