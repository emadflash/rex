use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenKind<'src> {
    Alphabet(&'src str),
    Plus,
    Star,
    Dot,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub pos: (usize, usize),
}

impl<'src> Token<'src> {
    fn new(kind: TokenKind<'src>, pos: (usize, usize)) -> Self {
        Self {
            kind: kind,
            pos: pos,
        }
    }
}

impl<'src> TokenKind<'src> {
    fn to_token(self, pos: (usize, usize)) -> Token<'src> {
        Token::new(self, pos)
    }
}

impl<'src> fmt::Display for TokenKind<'src> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Alphabet(alphabet) => write!(f, "TokenKind::Alphabet({})", alphabet),
            TokenKind::Plus => write!(f, "TokenKind::Plus"),
            TokenKind::Star => write!(f, "TokenKind::Star"),
            TokenKind::Dot => write!(f, "TokenKind::Dot"),
            TokenKind::LeftParen => write!(f, "TokenKind::LeftParen"),
            TokenKind::RightParen => write!(f, "TokenKind::RightParen"),
        }
    }
}

pub fn tokenize<'src>(s: &'src str) -> Result<Vec<Token<'src>>, String> {
    let mut tokens: Vec<Token<'src>> = Vec::new();
    let mut it = s.char_indices().peekable();

    while let Some((index, ch)) = it.next() {
        match ch {
            ' ' => {
                while let Some((_, w)) = it.peek() {
                    if !w.is_whitespace() {
                        break;
                    }

                    it.next();
                }
            }

            '+' => tokens.push(TokenKind::Plus.to_token((0, index))),
            '*' => tokens.push(TokenKind::Star.to_token((0, index))),
            '(' => tokens.push(TokenKind::LeftParen.to_token((0, index))),
            ')' => tokens.push(TokenKind::RightParen.to_token((0, index))),
            '.' => tokens.push(TokenKind::Dot.to_token((0, index))),

            'a'..='z' | '0'..='9' => {
                let mut end = index;

                while let Some((_, a)) = it.peek() {
                    match a {
                        'a'..='z' | '0'..='9' => {
                            end += 1;
                            it.next();
                        }

                        _ => break,
                    };
                }

                tokens.push(TokenKind::Alphabet(&s[index..=end]).to_token((0, index)));
            }

            _ => {
                return Err(format!("Encountered unknown character: {}", ch).to_string());
            }
        };
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizeer() {
        let text: &str = "(1 + 2).(3 + 4)*(1.3)";

        assert_eq!(
            tokenize(&text),
            Ok(vec![
                Token::new(TokenKind::LeftParen, (0, 0)),
                Token::new(TokenKind::Alphabet("1"), (0, 1)),
                Token::new(TokenKind::Plus, (0, 3)),
                Token::new(TokenKind::Alphabet("2"), (0, 5)),
                Token::new(TokenKind::RightParen, (0, 6)),
                Token::new(TokenKind::Dot, (0, 7)),
                Token::new(TokenKind::LeftParen, (0, 8)),
                Token::new(TokenKind::Alphabet("3"), (0, 9)),
                Token::new(TokenKind::Plus, (0, 11)),
                Token::new(TokenKind::Alphabet("4"), (0, 13)),
                Token::new(TokenKind::RightParen, (0, 14)),
                Token::new(TokenKind::Star, (0, 15)),
                Token::new(TokenKind::LeftParen, (0, 16)),
                Token::new(TokenKind::Alphabet("1"), (0, 17)),
                Token::new(TokenKind::Dot, (0, 18)),
                Token::new(TokenKind::Alphabet("3"), (0, 19)),
                Token::new(TokenKind::RightParen, (0, 20)),
            ])
        );
    }
}
