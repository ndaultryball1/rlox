#[derive(PartialEq)]
#[derive(Debug)]
enum Value {
    Int(i64),
    Bool(bool)
}

type Name = &'static str;

enum BinaryOp {
    Add,
    Sub,
}

enum UnaryOp {
    Not,
    Pos,
    Neg,
}

pub enum Expr {
    Constant(Value),
    BinaryExpr(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryExpr(UnaryOp, Box<Expr>)
}

fn eval_binary(opp: &BinaryOp, left: &Expr, right: &Expr) -> &Value {
    use BinaryOp::*;
    match opp {
        Add => add(left, right),
        _ => unimplemented!()
    }
}

fn eval_unary(opp: &UnaryOp, expr: &Expr) -> &Value {
    unimplemented!()
}

fn add(left: &Expr, right: &Expr) -> &Value {
    use Value::Int;

    let left_val = left.eval();
    let right_val = right.eval();
    if let (Int(l), Int(r)) = (left_val, right_val) {
        Int(l + r)
    } else {
        panic!("Expect integers when adding!")
    }
}

impl Expr {
    pub fn eval(&self) -> &Value {
        match self {
            Expr::Constant(value) => value,
            Expr::BinaryExpr(left, opp, right) => eval_binary(opp, &*left, &*right),
            Expr::UnaryExpr(opp, expr) => eval_unary(opp, expr),
        }
    }
}

#[test]
fn test_add() {
    use Expr::Constant;
    use Value::Int;

    let two = Int(2);
    let three = Int(3);
    assert_eq!(&Int(5), add(&Constant(two), &Constant(three)))
}