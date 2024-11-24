use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum CookiesError {
    #[error("Expiration date does not exist")]
    ExpirationDateDoesNotExist,
    #[error("Can't find Html document")]
    MissingHtmlDocument,
    #[error("Missing value to pass to the cookie")]
    MissingCookieValue,
    #[error("Missing name to identify your cookie")]
    MissingCookieName,
    #[error("Could not set cookie")]
    CouldNotSetCookie,
    #[error("Could not find cookie : `{0}` ")]
    CouldNotFindCookie(Arc<str>),
    #[error("Could not find any cookie ")]
    CouldNotFindAnyCookie,
    #[error("Parsing error, from string to cookie")]
    ParsingError,
}
