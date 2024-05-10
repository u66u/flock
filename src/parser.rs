use crate::ast::*;
use crate::lexer::*;
use std::iter::Peekable;

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

    pub fn parse_ty_times(&mut self) -> Option<Type> {
        let mut left = self.parse_ty_list()?;

        // Loop to handle consecutive TIMES operations (e.g., int * int * int)
        while matches!(self.tokens.peek(), Some(Token::Mult)) {
            self.tokens.next();

            let right = self.parse_ty_list()?;
            left = Type::Mult(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    pub fn parse_ty(&mut self) -> Option<Type> {
        // can handle int -> bool for example
        let ty = self.parse_ty_simple()?; // Start with parsing simple types including parenthesized expressions

        if matches!(self.tokens.peek(), Some(Token::DashArrow)) {
            self.tokens.next();
            let right_ty = self.parse_ty()?; // Recursively parse the right-hand-side type
            return Some(Type::Func(Box::new(ty), Box::new(right_ty)));
        }

        Some(ty)
    }

    pub fn parse_nil(&mut self) -> Option<Type> {
        if matches!(self.tokens.next(), Some(Token::LSquareBrack)) {
            let ty = self.parse_ty()?; // Parse the type within brackets
            if matches!(self.tokens.next(), Some(Token::RSquareBrack)) {
                Some(Type::List(Box::new(ty)))
            } else {
                None // Error: Expected RSQuarBrack
            }
        } else {
            None // Error: Expected LSquareBrack
        }
    }

    pub fn parse_basic_binary_expr(&mut self) -> Option<Expr> {
        match self.tokens.peek() {
            Some(&Token::Integer(value)) => {
                self.tokens.next();
                Some(Expr::Int(value))
            }
            Some(&Token::True) => {
                self.tokens.next();
                Some(Expr::Bool(true))
            }
            Some(&Token::False) => {
                self.tokens.next();
                Some(Expr::Bool(false))
            }
            Some(&Token::LParen) => {
                self.tokens.next();
                let expr = self.parse_binary_expr();
                if matches!(self.tokens.next(), Some(Token::RParen)) {
                    expr
                } else {
                    None // Error: Expected ')'
                }
            }
            // Handle other cases or errors appropriately
            _ => None,
        }
    }

    pub fn parse_binary_expr(&mut self) -> Option<Expr> {
        let left = self.parse_basic_binary_expr()?;
        loop {
            match self.tokens.peek() {
                Some(&Token::Equal) => {
                    self.tokens.next(); // Consume the Equal token
                    let right = self.parse_basic_binary_expr()?;
                    return Some(Expr::Equal(Box::new(left), Box::new(right)));
                }
                Some(&Token::Less) => {
                    self.tokens.next(); // Consume the Less token
                    let right = self.parse_basic_binary_expr()?;
                    return Some(Expr::Less(Box::new(left), Box::new(right)));
                }
                // Here's where you'd handle other operations or end the loop
                _ => break,
            }
        }
        Some(left)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexer::Token;
    use logos::Logos;

    fn tokenize(source: &str) -> Vec<Token> {
        let lexer = Token::lexer(source);
        lexer.filter_map(Result::ok).collect()
    }

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
    #[test]
    fn parse_single_ty_list() {
        let tokens = vec![Token::TypeInt, Token::TypeList]; // Represents: int list
        let mut parser = Parser::new(tokens.into_iter());
        assert_eq!(
            parser.parse_ty_times(),
            Some(Type::List(Box::new(Type::Int)))
        );
    }

    #[test]
    fn parse_simple_ty_times() {
        let tokens = vec![Token::TypeInt, Token::Mult, Token::TypeBool]; // Represents: int * bool
        let mut parser = Parser::new(tokens.into_iter());
        let expected = Type::Mult(Box::new(Type::Int), Box::new(Type::Bool));
        assert_eq!(parser.parse_ty_times(), Some(expected));
    }

    #[test]
    fn parse_nested_ty_times() {
        let tokens = vec![
            Token::TypeInt,
            Token::Mult,
            Token::TypeBool,
            Token::Mult,
            Token::TypeInt,
            Token::TypeList, // Represents: int * bool * int list
        ];
        let mut parser = Parser::new(tokens.into_iter());
        let expected = Type::Mult(
            Box::new(Type::Mult(Box::new(Type::Int), Box::new(Type::Bool))),
            Box::new(Type::List(Box::new(Type::Int))),
        );
        assert_eq!(parser.parse_ty_times(), Some(expected));
    }

    #[test]
    fn parse_ty_times_with_list() {
        let tokens = vec![
            Token::TypeInt,
            Token::TypeList,
            Token::Mult,
            Token::TypeBool,
            Token::TypeList, // Represents: int list * bool list
        ];
        let mut parser = Parser::new(tokens.into_iter());
        let expected = Type::Mult(
            Box::new(Type::List(Box::new(Type::Int))),
            Box::new(Type::List(Box::new(Type::Bool))),
        );
        assert_eq!(parser.parse_ty_times(), Some(expected));
    }
    #[test]
    fn test_parser_integration() {
        let tokens = vec![
            Token::TypeInt,
            Token::TypeList,
            Token::Mult,
            Token::TypeBool,
        ];

        let mut parser = Parser::new(tokens.into_iter());
        let result = parser.parse_ty_times();

        let expected = Some(Type::Mult(
            Box::new(Type::List(Box::new(Type::Int))),
            Box::new(Type::Bool),
        ));

        assert_eq!(result, expected);
    }
    #[test]
    fn test_parse_simple_type() {
        let tokens = vec![Token::TypeInt];
        let mut parser = Parser::new(tokens.into_iter());
        assert_eq!(parser.parse_ty(), Some(Type::Int));
    }

    #[test]
    fn test_parse_arrow_type() {
        let tokens = vec![Token::TypeInt, Token::DashArrow, Token::TypeBool];
        let mut parser = Parser::new(tokens.into_iter());
        assert_eq!(
            parser.parse_ty(),
            Some(Type::Func(Box::new(Type::Int), Box::new(Type::Bool)))
        );
    }
    #[test]
    fn test_parse_nil() {
        let tokens = vec![Token::LSquareBrack, Token::TypeInt, Token::RSquareBrack];
        let mut parser = Parser::new(tokens.into_iter());
        assert_eq!(parser.parse_nil(), Some(Type::List(Box::new(Type::Int))));
    }
    #[test]
    fn full_flow_test() {
        // Source code example: "[int -> bool]"
        let source = "[int -> bool]";
        let tokens = tokenize(source);
        let mut parser = Parser::new(tokens.into_iter());
        let parsed = parser.parse_nil(); // Starts by expecting a 'nil' type construction.

        assert_eq!(
            parsed,
            Some(Type::List(Box::new(Type::Func(
                Box::new(Type::Int),
                Box::new(Type::Bool)
            ))))
        );
    }
    #[test]
    fn test_simple_boolean_expression() {
        let left = Some(Expr::Equal(
            Box::new(Expr::Bool(true)),
            Box::new(Expr::Bool(false)),
        ));
        let right = Some(Expr::Equal(
            Box::new(Expr::Bool(true)),
            Box::new(Expr::Bool(false)),
        ));

        assert_eq!(left, right);
    }
}
