mod ast;
mod lexer;

use crate::ast::*;
use crate::lexer::Token;
use logos::Logos;

fn main() {
    let input = "
let div = fun m : int => rec d : int -> int is
  fun n : int => if n < m then 1420 else 125.0 + (d (n-m))
";
    let mut lexer = Token::lexer(input);

    while let Some(token) = lexer.next() {
        println!("{:?}: '{}'", token, lexer.slice());
    }
}
