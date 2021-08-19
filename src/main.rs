pub mod lexer;

use std::env;
use crate::lexer::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_name = &args[1];
        println!("compiling {}", file_name);
        let lexer = Lexer {};
        let result = lexer.from_file(&file_name);
        match result {
            Ok(_v) => println!("scanning done"),
            Err(e) => println!("error scanning file {} because {}", file_name, e),
        }

    } else {
        println!("No file argument found") 
    }
}    
