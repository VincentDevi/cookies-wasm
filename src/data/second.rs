use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Second(i64);

impl Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for Second {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Second {
    pub fn new(value: i64) -> Self {
        Second(value)
    }
}
