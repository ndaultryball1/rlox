#![allow(dead_code)]

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

type Name = &'static str;

#[derive(Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    Div,
    And,
    Or,
}

#[derive(Clone, Copy)]
pub enum UnaryOp {
    Not,
    Neg,
}

pub enum Expr {
    Constant(Value),
    BinaryExpr(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryExpr(UnaryOp, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> Value {
        match self {
            Expr::Constant(value) => *value,
            Expr::BinaryExpr(left, opp, right) => eval_binary(*opp, &left, &right),
            Expr::UnaryExpr(opp, expr) => eval_unary(*opp, &expr),
        }
    }
}

fn eval_binary(opp: BinaryOp, left: &Expr, right: &Expr) -> Value {
    use BinaryOp::*;
    use Value::*;

    let left_val = left.eval();
    let right_val = right.eval();
    if let (Int(l), Int(r)) = (left_val, right_val) {
        // Do a typecheck and group all the integer operations together
        match opp {
            Add => Int(l + r),
            Sub => Int(l - r),
            Mult => Int(l * r),
            Div => Int(l / r),
            _ => panic!("Operation not defined for type: Int"),
        }
    } else if let (Bool(l), Bool(r)) = (left_val, right_val) {
        // Same for bool expressions
        match opp {
            And => Bool(l & r),
            Or => Bool(l | r),
            _ => panic!("Operation not defined for type: Bool"),
        }
    } else {
        panic!("Mismatched types.")
    }
}

fn eval_unary(opp: UnaryOp, expr: &Expr) -> Value {
    use UnaryOp::*;
    use Value::*;

    let val = expr.eval();

    if let Int(e) = val {
        // Do a typecheck and group all the integer operations together
        match opp {
            Neg => Int(-e),
            _ => panic!("Operation not defined for type: Int"),
        }
    } else if let Bool(e) = val {
        // Same for bool expressions
        match opp {
            Not => Bool(!e),
            _ => panic!("Operation not defined for type: Bool"),
        }
    } else {
        panic!("Mismatched types.")
    }
}

#[cfg(test)]
mod test_expression {
    use super::BinaryOp::*;
    use super::Expr::*;
    use super::UnaryOp::*;
    use super::Value::*;

    #[test]
    fn test_binary() {
        let two = Int(2);
        let three = Int(3);

        let res_add = BinaryExpr(Box::new(Constant(two)), Add, Box::new(Constant(three))).eval();
        let res_sub = BinaryExpr(Box::new(Constant(two)), Sub, Box::new(Constant(three))).eval();

        assert_eq!(Int(5), res_add);
        assert_eq!(Int(-1), res_sub);
    }

    #[test]
    #[should_panic]
    fn test_type_error_ints() {
        let two = Int(2);
        let three = Int(3);

        let _ = BinaryExpr(Box::new(Constant(two)), Or, Box::new(Constant(three))).eval();
    }

    #[test]
    fn test_unary() {
        let True = Bool(true);
        let res = UnaryExpr(Not, Box::new(Constant(True))).eval();

        assert_eq!(res, Bool(false))
    }
}
