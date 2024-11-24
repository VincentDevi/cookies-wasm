use std::fmt::Display;

use super::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Expire {
    After(Second),
    At(Date),
}

impl Display for Expire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expire::At(value) => write!(f, "{}", value),
            Expire::After(value) => write!(f, "{}", value),
        }
    }
}
