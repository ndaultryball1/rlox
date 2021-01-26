use crate::lexer::*;
use std::fmt;

pub enum S<T> {
    Atom(T),
    Cons(T, Vec<S<T>>)
}

impl<T: fmt::Display> fmt::Display for S<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

pub fn parse_expr(tokens: Vec<Token>) -> S<String> {
    expr_bp(tokens)
}

fn expr_bp(tokens: Vec<Token>) -> S<String> {

    let iter = tokens.iter();

    let lhs = match iter.next() {
        Some(t) => {
            if t.is_atom() {
                S::Atom(t)
            } else {
                panic!("Invalid token: {:?}", t);
            }
        }
        None => panic!("Reached unexpected end of stream while parsing.")
    };

    loop {
        let op = match iter.next() {
            Some(Token::END) => break,
            Some(t) => {
                if t.is_op() {
                    t
                } else {
                    panic!("Invalid token: {:?}", t);
                }
            }
            None => panic!("Unexpected end of stream while parsing.")

        };

        todo!()
    }

    lhs
}

#[cfg(test)]
mod test_parser {
    use super::parse_expr;
    use crate::expression::*;
    use crate::lexer::*;

    #[test]
    fn test_atom() {
        let s = "1";
        let tokens : Vec<Token> = Scanner::new(s).collect();
        assert_eq!(parse_expr(tokens).to_string(), "1")
    }

    #[test]
    fn test_s() {
        let s = "1 + 2 * 3";
        let tokens : Vec<Token> = Scanner::new(s).collect();
        assert_eq!(parse_expr(tokens).to_string(), "(+ 1 (* 2 3))")
    }
}
