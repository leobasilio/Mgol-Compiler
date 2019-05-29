mod symbols;
mod analyzers;

use std::env;
use std::error::Error;

fn run_compiler(filename: &str) -> Result<(), String> {

    let mut table = symbols::Table::new();
    let mut lexical = analyzers::Lexical::new(&mut table);
    let mut syntatic = analyzers::Syntactic::new();

    if let Err(e) = lexical.load(filename) {

        return Err(e.description().to_string());

    }

    match syntatic.run(&mut lexical) {

        Ok(true) => println!("Aceitou!"),

        Ok(false) => println!("Não aceitou!"),

        Err(e) => println!("{}", e)

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