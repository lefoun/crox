#[derive(Debug, Clone, Copy)]
pub enum Value {
    Number(f64),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        let Value::Number(lhs) = self;
        let Value::Number(rhs) = other;
        lhs.approximate_eq(*rhs)
    }
}

pub trait ApproximateEq {
    const EPSILON: f64 = 0.00000001;
    fn approximate_eq(self, other: Self) -> bool;
}

impl ApproximateEq for f64 {
    fn approximate_eq(self, other: Self) -> bool {
        (self - other).abs() < <Self as ApproximateEq>::EPSILON
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}
