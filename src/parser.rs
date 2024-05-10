use crate::ast::*;
use crate::lexer::Token;
use logos::{Lexer, Logos, Span};
use std::fmt;
use std::result::Result;

// pub struct Parser<'a> {
//     tokens: &'a mut logos::Lexer<'a, Token>,
//     current_token: Option<Token>,
// }
//
// impl<'a> Parser<'a> {
//     pub fn new(tokens: &'a mut logos::Lexer<'a, Token>) -> Self {
//         let current_token = tokens.next();
//         Parser {
//             tokens,
//             current_token,
//         }
//     }
//
//     fn advance(&mut self) -> bool {
//         self.current_token = self.tokens.next();
//         self.current_token.is_some()
//     }
//
//     fn match_token(&mut self, expected: Token) -> Result<(), String> {
//         if self.current_token == Some(expected) {
//             self.advance();
//             Ok(())
//         } else {
//             Err(format!(
//                 "Expected {:?}, found {:?}",
//                 expected, self.current_token
//             ))
//         }
//     }
//
//     fn peek(&self) -> Option<&Token> {
//         self.current_token.as_ref()
//     }
// }
#[derive(Debug)]
pub struct ParseError {
    message: String,
    span: Span,
}

pub type ParseResult<T> = Result<T, ParseError>;

pub fn parse_expr(lexer: &mut Lexer<Token>) -> ParseResult<Expr> {
    let token = lexer.next();
    match token {
        Some(Ok(Token::Integer(value))) => Ok(Expr::Int(value)),
        Some(_) => parse_binary_expr(lexer),
        None => Err(ParseError {
            message: "Unexpected end of input".into(),
            span: lexer.span(),
        }),
    }
}

pub fn parse_binary_expr(lexer: &mut Lexer<Token>) -> ParseResult<Expr> {
    let mut lhs = parse_expr(lexer)?;

    while let Some(op) = lexer.next() {
        match op {
            Ok(Token::Plus) => {
                let rhs = parse_expr(lexer)?;
                lhs = Expr::Plus(Box::new(lhs), Box::new(rhs));
            }
            Ok(Token::Minus) => {
                let rhs = parse_expr(lexer)?;
                lhs = Expr::Minus(Box::new(lhs), Box::new(rhs));
            }
            Ok(Token::Times) => {
                let rhs = parse_expr(lexer)?;
                lhs = Expr::Times(Box::new(lhs), Box::new(rhs));
            }
            Ok(Token::Divide) => {
                let rhs = parse_expr(lexer)?;
                lhs = Expr::Divide(Box::new(lhs), Box::new(rhs));
            }
            _ => {
                return Err(ParseError {
                    message: "Expected operator in expression".into(),
                    span: lexer.span(),
                })
            }
        }
    }

    Ok(lhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lexer_from_str(input: &str) -> Lexer<Token> {
        Token::lexer(input)
    }

    #[test]
    fn test_parse_simple_addition() {
        let mut lexer = Token::lexer("3 + 5");
        let expr = parse_binary_expr(&mut lexer).unwrap();
        assert_eq!(
            expr,
            Expr::Plus(Box::new(Expr::Int(3)), Box::new(Expr::Int(5)))
        );
    }

    #[test]
    fn test_parse_chained_operations() {
        let mut lexer = lexer_from_str("3 + 5 - 2");
        let expr = parse_binary_expr(&mut lexer).unwrap();
        let expected = Expr::Minus(
            Box::new(Expr::Plus(Box::new(Expr::Int(3)), Box::new(Expr::Int(5)))),
            Box::new(Expr::Int(2)),
        );
        assert_eq!(expr, expected);
    }

    #[test]
    fn test_invalid_syntax() {
        let mut lexer = Token::lexer("3 + * 5");
        match parse_binary_expr(&mut lexer) {
            Ok(_) => panic!("Expected an error for invalid syntax"),
            Err(e) => assert_eq!(
                e.message, "Expected operator in expression",
                "Got unexpected error message: {}",
                e.message
            ),
        }
    }
}
