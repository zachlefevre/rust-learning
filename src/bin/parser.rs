use std::error::Error;

mod tokenize {
    #[derive(Debug, Eq, PartialEq)]
    pub enum Token {
        Select,
        Star,
        From,
        Semicolon,
        Comma,
        Identifier(String),
        As
    }

    impl Token {
        pub fn as_identifier(&self) -> Option<&str> {
            match self {
                Token::Identifier(value) => Some(value),
                _ => None
            }
        }
    }


    pub fn tokenize(items: &str) -> Result<Vec<Token>, String> {
        let mut items = items.chars().peekable();

        let mut state = Vec::new();

        while let Some(char) = items.next() {
            let token = match char {
                '*' => Token::Star,
                ';' => Token::Semicolon,
                ',' => Token::Comma,
                other if other.is_whitespace() => continue,
                other if other.is_alphanumeric() => {
                    let mut datum = other.to_string();
                    while let Some(char) = items.next_if(|&char| char.is_alphabetic() || char == '_') {
                        datum.extend(char.to_lowercase());
                    }
                    match datum.as_str() {
                        "select" => Token::Select,
                        "from" => Token::From,
                        "as" => Token::As,
                        other => Token::Identifier(String::from(other))
                    }

                }
                other => return Err(format!("Unexpected {other}"))
            };
            state.push(token)

        }
        Ok(state)

    }
}

mod parse {
    use crate::tokenize::Token;

    use self::ast::{Column, FromTarget, Select, SelectTarget};
    mod ast {
        #[derive(Debug)]
        pub struct Select {
            pub columns: SelectTarget,
            pub from: FromTarget,
        }

        #[derive(Debug)]
        pub enum SelectTarget {
            Star,
            Columns(Vec<Column>)
        }

        #[derive(Debug)]
        pub struct Column(pub String, pub Option<String>);

        #[derive(Debug)]
        pub struct FromTarget(pub String);
    }

    #[derive(Debug)]
    pub enum ParseError {
        FailedToParse(String, usize),
        UnexpectedEndOfTokens
    }

    pub struct ParseState {
        pub tokens: Vec<Token>,
        pub pos: usize
    }

    impl ParseState {
        fn advance(&mut self) {
            self.pos += 1
        }

        fn next_token(&self) -> Result<&Token, ParseError> {
            self.tokens.get(self.pos + 1).ok_or(ParseError::UnexpectedEndOfTokens)
        }

        fn current_token(&self) -> Result<&Token, ParseError> {
            self.tokens.get(self.pos).ok_or(ParseError::UnexpectedEndOfTokens)
        }

        fn current_token_is(&self, pred: impl FnOnce(&Token) -> bool) -> bool {
            self.current_token().map(pred).unwrap_or(false)
        }

        fn failed_to_parse(&self, string: &str) -> ParseError {
            ParseError::FailedToParse(String::from(string), self.pos)
        }

        pub fn parse_select(&mut self) -> Result<ast::Select, ParseError> {
            if self.current_token_is(|token| token == &Token::Select) {
                self.advance();
                let columns = self.parse_columns()?;
                let from = self.parse_from()?;
                Ok(Select { columns, from})
            } else {
                Err(self.failed_to_parse("Select"))
            }

        }

        fn parse_column(&mut self) -> Result<ast::Column, ParseError> {
            if self.current_token_is(|token| matches!(token, Token::Identifier(_))) {
                let column_name = self.current_token()?.as_identifier().unwrap();
                self.advance();
                if self.current_token_is(|token| token == &Token::As) {
                    self.advance();
                    if self.current_token_is(|token| matches!(token, Token::Identifier(_))) {
                        let alias_name = self.current_token()?.as_identifier().unwrap();
                        return Ok(ast::Column(column_name.to_string(), Some(alias_name.to_string())))
                    } else {
                        Err(self.failed_to_parse("alias name"))
                    }
                } else {
                    todo!()
                }
            } else {
                Err(self.failed_to_parse("column name"))
            }
        }

        fn parse_columns(&mut self) -> Result<ast::SelectTarget, ParseError> {
            if self.current_token_is(|token| token == &Token::Star) {
                self.advance();
                Ok(SelectTarget::Star)
            } else {
                match self.current_token() {
                    Ok(Token::Identifier(identifier)) => {
                        let mut columns = Vec::new();
                        while let Ok(column) = self.parse_column() {
                            columns.push(column)
                        }
                        Ok(ast::SelectTarget::Columns(columns))
                    }
                    _ => Err(self.failed_to_parse("column name"))
                }
            }
        }

        fn parse_from(&mut self) -> Result<ast::FromTarget, ParseError> {
            match self.current_token() {
                Ok(Token::From) => {
                    self.advance();
                    match self.current_token() {
                        Ok(Token::Identifier(value)) => Ok(FromTarget(value.to_owned())),
                        _ => Err(self.failed_to_parse("From table"))
                    }
                }
                _ => Err(self.failed_to_parse("From"))
            }
        }

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tokens = tokenize::tokenize("select foo, bar, baz from foo")?;
    dbg!(parse::ParseState { pos: 0, tokens: tokens }.parse_select());
    Ok(())
}
