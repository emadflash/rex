use crate::expr::Expr;
use crate::tokenizer::{tokenize, Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn from(text: &String) -> Self {
        Self {
            tokens: tokenize(&text).unwrap(),
            curr: 0,
        }
    }

    fn next(&mut self) -> Option<&Token> {
        if self.curr == self.tokens.len() {
            return None;
        }

        let tok = &self.tokens[self.curr];
        self.curr += 1;
        Some(tok)
    }

    fn peek(&self) -> Option<&Token> {
        if self.curr == self.tokens.len() {
            return None;
        }

        let tok = &self.tokens[self.curr];
        Some(tok)
    }

    // star ::= expr *
    fn parse_star_expr(&mut self, lhs: Expr) -> Expr {
        self.next();
        Expr::Star(Box::new(lhs))
    }

    // plus ::= expr + expr
    fn parse_plus_expr(&mut self, lhs: Expr) -> Expr {
        self.next();
        Expr::Plus(Box::new(lhs), Box::new(self.parse()))
    }

    // dot ::= expr . expr
    fn parse_dot_expr(&mut self, lhs: Expr) -> Expr {
        self.next();
        Expr::Dot(Box::new(lhs), Box::new(self.parse()))
    }

    // alphabet ::= 'a'..'z' | '0'..'9'
    // primary ::= alphabet | '(' expr ')'
    fn parse_primary(&mut self) -> Expr {
        match self.next() {
            None => panic!("Expected expression"),
            Some(tok) => {
                match &tok.kind {
                    TokenKind::Alphabet(alphabet) => return Expr::Alphabet(alphabet.to_string()),
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
    fn parse_expr(&mut self, lhs: Expr) -> Expr {
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

        return lhs;
    }

    pub fn parse(&mut self) -> Expr {
        let primary = self.parse_primary();
        return self.parse_expr(primary);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet_expr() {
        let mut parser = Parser::from(&"aabb".to_string());
        assert_eq!(parser.parse(), Expr::Alphabet("aabb".to_string()));
    }

    #[test]
    fn plus_expr() {
        let mut parser = Parser::from(&"69 + 23 + 79 + 59".to_string());
        assert_eq!(
            parser.parse(),
            Expr::Plus(
                Box::new(Expr::Alphabet("69".to_string())),
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("23".to_string())),
                    Box::new(Expr::Plus(
                        Box::new(Expr::Alphabet("79".to_string())),
                        Box::new(Expr::Alphabet("59".to_string()))
                    )),
                )),
            )
        );
    }

    #[test]
    fn star_expr() {
        let mut parser = Parser::from(&"aabb*".to_string());
        let expr = parser.parse();

        assert_eq!(
            expr,
            Expr::Star(Box::new(Expr::Alphabet("aabb".to_string())))
        );
    }

    #[test]
    fn dot_expr() {
        let mut parser = Parser::from(&"a.b.c.d".to_string());
        assert_eq!(
            parser.parse(),
            Expr::Dot(
                Box::new(Expr::Alphabet("a".to_string())),
                Box::new(Expr::Dot(
                    Box::new(Expr::Alphabet("b".to_string())),
                    Box::new(Expr::Dot(
                        Box::new(Expr::Alphabet("c".to_string())),
                        Box::new(Expr::Alphabet("d".to_string()))
                    )),
                )),
            )
        );
    }

    #[test]
    fn paren_expr() {
        let mut parser = Parser::from(&"(a + b).(c + d)".to_string());
        assert_eq!(
            parser.parse(),
            Expr::Dot(
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("a".to_string())),
                    Box::new(Expr::Alphabet("b".to_string()))
                )),
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("c".to_string())),
                    Box::new(Expr::Alphabet("d".to_string()))
                )),
            )
        );
    }

    #[test]
    fn parse_expr() {
        let mut parser = Parser::from(&"(1 + 2).(3 + 4)*".to_string());
        assert_eq!(
            parser.parse(),
            Expr::Dot(
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("1".to_string())),
                    Box::new(Expr::Alphabet("2".to_string()))
                )),
                Box::new(Expr::Star(Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("3".to_string())),
                    Box::new(Expr::Alphabet("4".to_string()))
                )))),
            )
        );
    }

    #[test]
    fn parse_expr2() {
        let mut parser = Parser::from(&"a* + (3 + 4*)".to_string());
        assert_eq!(
            parser.parse(),
            Expr::Plus(
                Box::new(Expr::Star(Box::new(Expr::Alphabet("a".to_string())))),
                Box::new(Expr::Plus(
                    Box::new(Expr::Alphabet("3".to_string())),
                    Box::new(Expr::Star(Box::new(Expr::Alphabet("4".to_string())))),
                )),
            )
        );
    }
}
