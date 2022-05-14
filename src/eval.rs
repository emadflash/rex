use crate::parser::Parser;
use crate::expr::Expr;

struct Eval {
    target: String,
    curr: usize,
}

impl Eval {
    fn new(target: String) -> Self {
        Self {
            target: target,
            curr: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.curr == self.target.len() {
            return None;
        }

        let ch = self.target.chars().nth(self.curr);
        self.curr += 1;
        ch
    }

    fn expect(&mut self, s: &String) -> bool {
        if &self.target[self.curr..(self.curr + s.len())] != s {
            return false;
        }

        self.curr += s.len();
        true
    }

    fn eval_alphabet(&mut self, alphabet: &String) -> bool {
        self.expect(alphabet)
    }

    fn eval_star(&mut self, expr: Box<Expr>) -> bool {
        self.eval(expr)
    }

    fn eval_plus(&mut self, lhs: Box<Expr>, rhs: Box<Expr>) -> bool {
        if self.eval(lhs) {
            return true;
        }

        self.eval(rhs)
    }

    fn eval_dot(&mut self, expr: Box<Expr>) -> bool {
        false
    }

    fn eval(&mut self, expr: Box<Expr>) -> bool {
        let mut expr = expr;

        match *expr {
            Expr::Alphabet(alphabet) => self.eval_alphabet(&alphabet),
            Expr::Star(e) => self.eval_star(e),
            Expr::Plus(lhs, rhs) => self.eval_plus(lhs, rhs),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabet() {
        let regex = "abcd".to_string();
        let mut parser = Parser::from(&regex);

        let mut eval = Eval::new("abcd".to_string());
        assert!(eval.eval(Box::new(parser.parse())));
    }

    #[test]
    fn star() {
        let regex = "a*".to_string();
        let mut parser = Parser::from(&regex);

        let mut eval = Eval::new("aaaa".to_string());
        assert!(eval.eval(Box::new(parser.parse())));
    }

    #[test]
    fn plus() {
        let regex = "a + b".to_string();
        let mut parser = Parser::from(&regex);

        let mut eval = Eval::new("a".to_string());
        assert!(eval.eval(Box::new(parser.parse())));
    }
}
