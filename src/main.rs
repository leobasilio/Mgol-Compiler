mod symbols;
mod analyzers;

use std::env;

fn run_compiler(filename: &str){

    let mut table = symbols::Table::new();

    let lexical = analyzers::Lexical::new();

}

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {

        println!("Usage: mgol file.alg");

    }else{

        run_compiler(&args[1])

    }

}