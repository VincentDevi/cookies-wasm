use super::super::error::*;
use std::fmt::Display;

use chrono::NaiveDate;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Date(NaiveDate);

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%a, %d %b %Y %H:%M:%S GMT"))
    }
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, CookiesError> {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .map(Self)
            .ok_or(CookiesError::ExpirationDateDoesNotExist)
    }
}
