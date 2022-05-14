use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Alphabet(String),
    Plus,
    Star,
    Dot,
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: (usize, usize),
}

impl Token {
    fn new(kind: TokenKind, pos: (usize, usize)) -> Self {
        Self {
            kind: kind,
            pos: pos,
        }
    }
}

impl TokenKind {
    fn to_token(self, pos: (usize, usize)) -> Token {
        Token::new(self, pos)
    }
}

impl fmt::Display for TokenKind {
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

pub fn tokenize(s: &String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
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
                let mut alphabet = String::from(ch);

                while let Some((_, a)) = it.peek() {
                    match a {
                        'a'..='z' | '0'..='9' => {
                            alphabet.push(*a);
                            it.next();
                        }

                        _ => break,
                    };
                }

                tokens.push(TokenKind::Alphabet(alphabet).to_token((0, index)));
            }

            _ => {
                return Err(format!("Encountered unknown character: {}", ch)
                    .to_string()
                    .into());
            }
        };
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizer() {
        let text = "(1 + 2).(3 + 4)*".to_string();

        assert_eq!(
            tokenize(&text),
            Ok(vec![
                Token::new(TokenKind::LeftParen, (0, 0)),
                Token::new(TokenKind::Alphabet("1".to_string()), (0, 1)),
                Token::new(TokenKind::Plus, (0, 3)),
                Token::new(TokenKind::Alphabet("2".to_string()), (0, 5)),
                Token::new(TokenKind::RightParen, (0, 6)),
                Token::new(TokenKind::Dot, (0, 7)),
                Token::new(TokenKind::LeftParen, (0, 8)),
                Token::new(TokenKind::Alphabet("3".to_string()), (0, 9)),
                Token::new(TokenKind::Plus, (0, 11)),
                Token::new(TokenKind::Alphabet("4".to_string()), (0, 13)),
                Token::new(TokenKind::RightParen, (0, 14)),
                Token::new(TokenKind::Star, (0, 15)),
            ])
        );
    }
}
