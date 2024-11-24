use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum SameSite {
    None,
    #[default]
    Lax,
    Strict,
}

impl Display for SameSite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SameSite::Lax => write!(f, "Lax"),
            SameSite::Strict => write!(f, "Strict"),
            SameSite::None => write!(f, "None"),
        }
    }
}
