#[derive(Debug, PartialEq)]
pub enum Expr {
    Alphabet(String),
    Star(Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Dot(Box<Expr>, Box<Expr>),
}
