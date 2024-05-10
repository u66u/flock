mod ast;
mod lexer;
mod parser;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser::Parser;
// use chumsky::Parser;
use logos::Logos;

fn main() {
    let input = "
        [int -> bool]
";
    let mut lexer = Token::lexer(input);
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => tokens.push(token),
            Err(err) => println!("{:?}", err),
        }
    }
    println!("{:?}", &tokens);
    let mut parser = Parser::new(tokens.into_iter());
    println!("{:?}", &parser.parse_nil());
}
