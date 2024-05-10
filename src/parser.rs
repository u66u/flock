use crate::ast::*;
use crate::lexer::*;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse_ty_simple(&mut self) -> Option<Type> {
        match self.tokens.peek()? {
            Token::TypeBool => {
                self.tokens.next();
                Some(Type::Bool)
            }
            Token::TypeInt => {
                self.tokens.next();
                Some(Type::Int)
            }
            Token::LParen => {
                self.tokens.next();
                let ty = self.parse_ty_simple();
                if matches!(self.tokens.next(), Some(Token::RParen)) {
                    ty
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    pub fn parse_ty_list(&mut self) -> Option<Type> {
        let base = self.parse_ty_simple()?;
        let mut result = base;

        while matches!(self.tokens.peek(), Some(Token::TypeList)) {
            self.tokens.next();
            result = Type::List(Box::new(result));
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::Token;

    #[test]
    fn test_parse_ty_simple_int() {
        let tokens = vec![Token::TypeInt];
        let mut parser = Parser::new(tokens.into_iter());
        assert_eq!(parser.parse_ty_simple(), Some(Type::Int));
    }

    #[test]
    fn test_parse_ty_simple_bool() {
        let tokens = vec![Token::TypeBool];
        let mut parser = Parser::new(tokens.into_iter());
        assert_eq!(parser.parse_ty_simple(), Some(Type::Bool));
    }

    #[test]
    fn test_parse_ty_list_int() {
        let tokens = vec![Token::TypeInt, Token::TypeList];
        let mut parser = Parser::new(tokens.into_iter());
        assert_eq!(
            parser.parse_ty_list(),
            Some(Type::List(Box::new(Type::Int)))
        );
    }

    #[test]
    fn test_parse_ty_list_list_of_int() {
        let tokens = vec![Token::TypeInt, Token::TypeList, Token::TypeList];
        let mut parser = Parser::new(tokens.into_iter());
        let expected = Type::List(Box::new(Type::List(Box::new(Type::Int))));
        assert_eq!(parser.parse_ty_list(), Some(expected));
    }
}
