use std::fmt;

#[derive(PartialEq, Eq)]
pub enum Ast {
    Number(i64),
    Operation {
        left: Box<Ast>,
        operator: Op,
        right: Box<Ast>,
    },
    // Parens(Box<Ast>),
}

impl fmt::Debug for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ast::Number(inner) => inner.fmt(f),
            Ast::Operation {
                left,
                operator,
                right,
            } => {
                write!(f, "({:?} {} {:?})", left, operator, right)
            }
        }
    }
}

impl Ast {
    pub fn eval(&self) -> i64 {
        match self {
            Ast::Number(n) => *n,
            Ast::Operation {
                left,
                operator,
                right,
            } => {
                let left = left.eval();
                let right = right.eval();
                match operator {
                    Op::Add => left + right,
                    Op::Sub => left - right,
                    Op::Mul => left * right,
                }
            }
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
        }
    }
}
