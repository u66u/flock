mod ast;
mod lexer;
mod parser;

use crate::ast::*;
use crate::lexer::Token;
use crate::parser::*;
use logos::Logos;

fn main() {
    let input = "
3 + 5 - 2
";
    let mut lexer = Token::lexer(input);
    let mut res = parser::parse_binary_expr(&mut lexer).unwrap();
    println!("{:?}", res);
    // while let Some(token) = lexer.next() {
    //     println!("{:?}: '{}'", token, lexer.slice());
    // }
}
