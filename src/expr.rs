use std::{fmt, ops};

#[derive(Debug, Copy, Clone)]
pub enum ExprParts {
    Int(i64),
    Double(f64),
    AddOp,
    SubOp,
    MulOp,
    DivOp,
}

impl ExprParts {
    pub fn is_op(&self) -> bool {
        matches!(self, ExprParts::AddOp | ExprParts::SubOp | ExprParts::MulOp |
                ExprParts::DivOp)
    }

    pub fn op_prio(&self) -> u8 {
        match self {
            ExprParts::SubOp => 1,
            ExprParts::AddOp => 2,
            ExprParts::MulOp => 3,
            ExprParts::DivOp => 4,
            _ => unreachable!() // should not happen
        }
    }
}

impl fmt::Display for ExprParts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprParts::Int(i) => write!(f, "{i}"),
            ExprParts::Double(d) => write!(f, "{d}"),
            ExprParts::AddOp => write!(f, "+"),
            ExprParts::SubOp => write!(f, "-"),
            ExprParts::MulOp => write!(f, "*"),
            ExprParts::DivOp => write!(f, "/")
        }
    }
}

impl ops::Add for ExprParts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            ExprParts::Int(i1) => {
                match rhs {
                    ExprParts::Int(i2) => ExprParts::Int(i1 + i2),
                    ExprParts::Double(f) => ExprParts::Double(i1 as f64 + f),
                    _ => unreachable!("adding operators")
                }
            },

            ExprParts::Double(f1) => {
                match rhs {
                    ExprParts::Int(i) => ExprParts::Double(i as f64 + f1),
                    ExprParts::Double(f2) => ExprParts::Double(f1 + f2),
                    _ => unreachable!("adding operators")
                }
            },
            _ => unreachable!("adding operators")
        }
    }
}

impl ops::Sub for ExprParts {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            ExprParts::Int(i1) => {
                match rhs {
                    ExprParts::Int(i2) => ExprParts::Int(i1 - i2),
                    ExprParts::Double(f) => ExprParts::Double(i1 as f64 - f),
                    _ => unreachable!("subtracting operators")
                }
            },

            ExprParts::Double(f1) => {
                match rhs {
                    ExprParts::Int(i) => ExprParts::Double(i as f64 - f1),
                    ExprParts::Double(f2) => ExprParts::Double(f1 - f2),
                    _ => unreachable!("subtracting operators")
                }
            },
            _ => unreachable!("subtracting operators")
        }
    }
}

impl ops::Mul for ExprParts {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            ExprParts::Int(i1) => {
                match rhs {
                    ExprParts::Int(i2) => ExprParts::Int(i1 * i2),
                    ExprParts::Double(f) => ExprParts::Double(i1 as f64 * f),
                    _ => unreachable!("multiplying operators")
                }
            },

            ExprParts::Double(f1) => {
                match rhs {
                    ExprParts::Int(i) => ExprParts::Double(i as f64 * f1),
                    ExprParts::Double(f2) => ExprParts::Double(f1 * f2),
                    _ => unreachable!("multiplying operators")
                }
            },
            _ => unreachable!("multiplying operators")
        }
    }
}

impl ops::Div for ExprParts {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            ExprParts::Int(i1) => {
                match rhs {
                    ExprParts::Int(i2) => ExprParts::Int(i1 / i2),
                    ExprParts::Double(f) => ExprParts::Double(i1 as f64 / f),
                    _ => unreachable!("dividing operators")
                }
            },

            ExprParts::Double(f1) => {
                match rhs {
                    ExprParts::Int(i) => ExprParts::Double(i as f64 / f1),
                    ExprParts::Double(f2) => ExprParts::Double(f1 / f2),
                    _ => unreachable!("dividing operators")
                }
            },
            _ => unreachable!("dividing operators")
        }
    }
}