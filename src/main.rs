mod symbols;
mod analyzers;

use std::env;

fn run_compiler(input_file: &str) {

    let mut table = symbols::Table::new();
    let mut lexical = analyzers::Lexical::new(&mut table);

    if let Err(e) = lexical.load(input_file) {

        println!("{}", e);

    }else{

        let mut syntatic = analyzers::Syntactic::new();
        let output_file = [input_file, ".c"].concat();

        match syntatic.run(&mut lexical, &output_file) {

            Ok(()) => println!("Pronto!"),

            Err(e) => println!("\n{}\n", e)

        }

    }

}

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {

        println!("Usage: mgol file.alg");

    }else{

        run_compiler(&args[1]);

    }

}