pub mod tokens;

use std::fs;

use crate::lexer::tokens::*;
pub struct Lexer {
   
}

impl Lexer {
    pub fn from_file(&self, file_name:&str) -> Result<Vec<Token>, &'static str> {
        println!("reading file {}", file_name);
        let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
        println!("start scanning");
        self.scan(contents)
    }

    fn scan(&self, content: String) -> Result<Vec<Token>, &'static str> {
        println!("{}", content);
        Ok(vec![create_token(TokenType::Symbol(String::from("bla")), String::from("bla"), String::from("bla"), 0, 0)
        
        ])
    }
}
