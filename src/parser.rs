pub struct Parser<'a> {
    tokens: &'a mut logos::Lexer<'a, Token>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut logos::Lexer<'a, Token>) -> Self {
        let current_token = tokens.next();
        Parser {
            tokens,
            current_token,
        }
    }

    fn advance(&mut self) -> bool {
        self.current_token = self.tokens.next();
        self.current_token.is_some()
    }

    fn match_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == Some(expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected, self.current_token
            ))
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.current_token.as_ref()
    }
}
