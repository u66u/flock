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
3 + 5 - 2
";
    let mut lexer = Token::lexer(input);
    // while let Some(token) = lexer.next() {
    //     println!("{:?}: '{}'", token, lexer.slice());
    // }
}
