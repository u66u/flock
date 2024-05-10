mod tokens;

use crate::tokens::*;

fn main() {
    let x = Type::Int;
    println!("{}", x.to_string());
}
