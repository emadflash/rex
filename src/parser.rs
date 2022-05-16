use std::fmt;
use crate::expr::Expr;
use crate::lex::{tokenize, Token, TokenKind};

#[derive(Debug)]
pub enum ParseError {
    Lexer(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Lexer(e) => write!(f, "ParseError::Lexer({})", e),
        }
    }
}

pub struct Parser<'src> {
    tokens: Vec<Token<'src>>,
    curr: usize,
}

impl<'src> Parser<'src> {
    pub fn from(text: &'src str) -> Result<Self, ParseError> {
        let tokens =  tokenize(&text);
        if let Err(e) = tokens {
            return Err(ParseError::Lexer(e));
        }

        Ok(Self {
            tokens: tokens.unwrap(),
            curr: 0,
        })
    }

    fn next(&mut self) -> Option<&Token<'src>> {
        if self.curr == self.tokens.len() {
            return None;
        }

        let tok = &self.tokens[self.curr];
        self.curr += 1;
        Some(tok)
    }

    fn peek(&self) -> Option<&Token<'src>> {
        if self.curr == self.tokens.len() {
            return None;
        }

        let tok = &self.tokens[self.curr];
        Some(tok)
    }

    // star ::= expr *
    fn parse_star_expr(&mut self, lhs: Expr<'src>) -> Expr<'src> {
        self.next();
        Expr::Star(Box::new(lhs))
    }

    // plus ::= expr + expr
    fn parse_plus_expr(&mut self, lhs: Expr<'src>) -> Expr<'src> {
        self.next();
        Expr::Plus(Box::new(lhs), Box::new(self.parse()))
    }

    // dot ::= expr . expr
    fn parse_dot_expr(&mut self, lhs: Expr<'src>) -> Expr<'src> {
        self.next();
        Expr::Dot(Box::new(lhs), Box::new(self.parse()))
    }

    // alphabet ::= 'a'..'z' | '0'..'9'
    // primary ::= alphabet | '(' expr ')'
    fn parse_primary(&mut self) -> Expr<'src> {
        eprintln!("{:?}", self.tokens);
        match self.next() {
            None => panic!("Expected expression"),
            Some(tok) => {
                match &tok.kind {
                    TokenKind::Alphabet(alphabet) => return Expr::Alphabet(alphabet),
                    TokenKind::LeftParen => {
                        let inner = self.parse();

                        match self.next() {
                            None => panic!("Expected closing paren ')'"),
                            Some(tok) => {
                                if tok.kind != TokenKind::RightParen {
                                    panic!("Expected closing paren ')'");
                                }
                            }
                        };

                        return inner;
                    }

                    _ => {
                        panic!("Unexpected token");
                    }
                };
            }
        };
    }

    // expr ::= primary | dot | plus | star
    fn parse_expr(&mut self, lhs: Expr<'src>) -> Expr<'src> {
        let mut lhs = lhs;

        while let Some(tok) = self.peek() {
            match tok.kind {
                TokenKind::Star => lhs = self.parse_star_expr(lhs),
                TokenKind::Plus => lhs = self.parse_plus_expr(lhs),
                TokenKind::Dot => lhs = self.parse_dot_expr(lhs),
                TokenKind::RightParen => break,

                _ => {
                    panic!("Unexpected token");
                }
            };
        }

        lhs
    }

    pub fn parse(&mut self) -> Expr<'src> {
        let primary = self.parse_primary();
        self.parse_expr(primary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_expr() {
        let mut parser = Parser::from("aabb").unwrap();
        assert_eq!(parser.parse(), Expr::Alphabet("aabb"));
    }

    #[test]
    fn plus_expr() {
        let mut parser = Parser::from("69 + 23 + 79 + 59").unwrap();
        assert_eq!(
            parser.parse(),
            Expr::Plus(
                Box::new(Expr::Alphabet("69")),
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("23")),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Alphabet("79")),
                        Box::new(Expr::Alphabet("59"))
                    )),
                )),
            )
        );
    }

    #[test]
    fn star_expr() {
        let mut parser = Parser::from("aabb*").unwrap();
        assert_eq!(parser.parse(), Expr::Star(Box::new(Expr::Alphabet("aabb"))));
    }

    #[test]
    fn dot_expr() {
        let mut parser = Parser::from("a.b.c.d").unwrap();
        assert_eq!(
            parser.parse(),
            Expr::Dot(
                Box::new(Expr::Alphabet("a")),
                Box::new(Expr::Dot(
                    Box::new(Expr::Alphabet("b")),
                    Box::new(Expr::Dot(
                        Box::new(Expr::Alphabet("c")),
                        Box::new(Expr::Alphabet("d"))
                    )),
                )),
            )
        );
    }

    #[test]
    fn paren_expr() {
        let mut parser = Parser::from("(a + b).(c + d)").unwrap();
        assert_eq!(
            parser.parse(),
            Expr::Dot(
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("a")),
                    Box::new(Expr::Alphabet("b"))
                )),
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("c")),
                    Box::new(Expr::Alphabet("d"))
                )),
            )
        );
    }

    #[test]
    fn parse_expr() {
        let mut parser = Parser::from("(1 + 2).(3 + 4)*").unwrap();
        assert_eq!(
            parser.parse(),
            Expr::Dot(
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("1")),
                    Box::new(Expr::Alphabet("2"))
                )),
                Box::new(Expr::Star(Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("3")),
                    Box::new(Expr::Alphabet("4"))
                )))),
            )
        );
    }

    #[test]
    fn parse_expr2() {
        let mut parser = Parser::from("a* + (3 + 4*)").unwrap();
        assert_eq!(
            parser.parse(),
            Expr::Plus(
                Box::new(Expr::Star(Box::new(Expr::Alphabet("a")))),
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("3")),
                    Box::new(Expr::Star(Box::new(Expr::Alphabet("4")))),
                )),
            )
        );
    }
}
