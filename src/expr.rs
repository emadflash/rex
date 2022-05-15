#[derive(Debug, PartialEq)]
pub enum Expr<'src> {
    Alphabet(&'src str),
    Star(Box<Expr<'src>>),
    Plus(Box<Expr<'src>>, Box<Expr<'src>>),
    Dot(Box<Expr<'src>>, Box<Expr<'src>>),
}
