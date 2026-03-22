use std::fmt;

use crate::syntax::{Binder, Definition, Module, SurfaceTerm};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Ident(String),
    Type,
    Lambda,
    Colon,
    Equals,
    Arrow,
    LParen,
    RParen,
    Newline,
    Eof,
}

pub fn parse_module(input: &str) -> Result<Module, ParseError> {
    let tokens = lex(input)?;
    let mut parser = Parser::new(tokens);
    parser.parse_module()
}

fn lex(input: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\t' | '\r' => {}
            '\n' => tokens.push(Token::Newline),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ':' => tokens.push(Token::Colon),
            '=' => tokens.push(Token::Equals),
            '\\' => tokens.push(Token::Lambda),
            '-' => match chars.next() {
                Some('>') => tokens.push(Token::Arrow),
                other => {
                    return Err(ParseError::new(format!(
                        "expected '>' after '-', found {:?}",
                        other
                    )))
                }
            },
            ch if is_ident_start(ch) => {
                let mut ident = String::from(ch);
                while let Some(next) = chars.peek() {
                    if is_ident_continue(*next) {
                        ident.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if ident == "Type" {
                    tokens.push(Token::Type);
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            other => return Err(ParseError::new(format!("unexpected character '{other}'"))),
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn parse_module(&mut self) -> Result<Module, ParseError> {
        let mut definitions = Vec::new();
        self.skip_newlines();

        while !self.at(&Token::Eof) {
            definitions.push(self.parse_definition()?);
            self.skip_newlines();
        }

        Ok(Module { definitions })
    }

    fn parse_definition(&mut self) -> Result<Definition, ParseError> {
        let name = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let signature = self.parse_term()?;
        self.expect(Token::Newline)?;

        let body_name = self.expect_ident()?;
        if body_name != name {
            return Err(ParseError::new(format!(
                "definition name mismatch: expected '{name}', found '{body_name}'"
            )));
        }
        self.expect(Token::Equals)?;
        let body = self.parse_term()?;
        self.consume(Token::Newline);

        Ok(Definition {
            name,
            signature,
            body,
        })
    }

    fn parse_term(&mut self) -> Result<SurfaceTerm, ParseError> {
        if self.at(&Token::Lambda) {
            self.advance();
            let binder = self.parse_binder()?;
            self.expect(Token::Arrow)?;
            let body = self.parse_term()?;
            return Ok(SurfaceTerm::Lambda {
                binder,
                body: Box::new(body),
            });
        }

        if self.is_binder_arrow() {
            let binder = self.parse_binder()?;
            self.expect(Token::Arrow)?;
            let codomain = self.parse_term()?;
            let domain = (*binder.ty).clone();
            return Ok(SurfaceTerm::Arrow {
                binder: Some(binder),
                domain: Box::new(domain),
                codomain: Box::new(codomain),
            });
        }

        let domain = self.parse_application()?;
        if self.at(&Token::Arrow) {
            self.advance();
            let codomain = self.parse_term()?;
            Ok(SurfaceTerm::Arrow {
                binder: None,
                domain: Box::new(domain),
                codomain: Box::new(codomain),
            })
        } else {
            Ok(domain)
        }
    }

    fn parse_application(&mut self) -> Result<SurfaceTerm, ParseError> {
        let mut term = self.parse_atom()?;

        while self.starts_atom() {
            let argument = self.parse_atom()?;
            term = SurfaceTerm::App(Box::new(term), Box::new(argument));
        }

        Ok(term)
    }

    fn parse_atom(&mut self) -> Result<SurfaceTerm, ParseError> {
        match self.peek() {
            Token::Type => {
                self.advance();
                Ok(SurfaceTerm::Type)
            }
            Token::Ident(_) => Ok(SurfaceTerm::Var(self.expect_ident()?)),
            Token::LParen => {
                self.advance();
                let term = self.parse_term()?;
                self.expect(Token::RParen)?;
                Ok(term)
            }
            token => Err(ParseError::new(format!(
                "expected term, found {:?}",
                token
            ))),
        }
    }

    fn parse_binder(&mut self) -> Result<Binder, ParseError> {
        self.expect(Token::LParen)?;
        let name = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let ty = self.parse_term()?;
        self.expect(Token::RParen)?;
        Ok(Binder {
            name,
            ty: Box::new(ty),
        })
    }

    fn is_binder_arrow(&self) -> bool {
        matches!(self.peek(), Token::LParen)
            && matches!(self.peek_n(1), Token::Ident(_))
            && matches!(self.peek_n(2), Token::Colon)
    }

    fn starts_atom(&self) -> bool {
        matches!(self.peek(), Token::Type | Token::Ident(_) | Token::LParen)
    }

    fn skip_newlines(&mut self) {
        while self.at(&Token::Newline) {
            self.advance();
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.peek().clone() {
            Token::Ident(ident) => {
                self.advance();
                Ok(ident)
            }
            token => Err(ParseError::new(format!(
                "expected identifier, found {:?}",
                token
            ))),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.at(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::new(format!(
                "expected {:?}, found {:?}",
                expected,
                self.peek()
            )))
        }
    }

    fn consume(&mut self, token: Token) {
        if self.at(&token) {
            self.advance();
        }
    }

    fn at(&self, token: &Token) -> bool {
        self.peek() == token
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn peek_n(&self, offset: usize) -> &Token {
        let index = (self.position + offset).min(self.tokens.len() - 1);
        &self.tokens[index]
    }

    fn advance(&mut self) {
        if self.position + 1 < self.tokens.len() {
            self.position += 1;
        }
    }
}
