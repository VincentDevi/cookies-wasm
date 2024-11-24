use super::data::*;
use crate::error::CookiesError;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateCookies {
    value: String,
    name: String,
    expire: Option<Expire>,
    domain: Option<String>,
    path: Option<String>,
    same_site: SameSite,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Cookies {
    value: String,
    name: String,
}
impl Cookies {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl TryFrom<String> for Cookies {
    type Error = CookiesError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let (name, value) = value
            .split_once('=')
            .ok_or(CookiesError::MissingCookieValue)?;
        Ok(Cookies {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

impl Display for CreateCookies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value = String::new();
        value.push_str(&format!("{}=", self.name));
        value.push_str(&self.value);
        if let Some(expire) = &self.expire {
            value.push_str(&format!("; {}", expire));
        }
        if let Some(domain) = &self.domain {
            value.push_str(&format!("; Domain={}", domain));
        }
        if let Some(path) = &self.path {
            value.push_str(&format!("; Path={}", path));
        }
        value.push_str(&format!("; SameSite={}", self.same_site));
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CreateCookiesBuilder {
    value: Option<String>,
    name: Option<String>,
    expire: Option<Expire>,
    domain: Option<String>,
    path: Option<String>,
    same_site: Option<SameSite>,
}

impl CreateCookiesBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
    pub fn value(&mut self, value: String) -> &mut Self {
        self.value = Some(value);
        self
    }
    pub fn path(&mut self, value: Option<String>) -> &mut Self {
        self.path = value;
        self
    }
    pub fn same_site(&mut self, value: SameSite) -> &mut Self {
        self.same_site = Some(value);
        self
    }
    pub fn expire(&mut self, value: Option<Expire>) -> &mut Self {
        self.expire = value;
        self
    }
    pub fn build(&self) -> Result<CreateCookies, CookiesError> {
        Ok(CreateCookies {
            value: self.value.clone().ok_or(CookiesError::MissingCookieName)?,
            name: self.name.clone().ok_or(CookiesError::MissingCookieValue)?,
            expire: self.expire,
            domain: self.domain.clone(),
            path: self.path.clone(),
            same_site: self.same_site.unwrap_or_default(),
        })
    }
}
