use std::collections::HashSet;
use utils::*;

mod cookies;
mod data;
mod error;
mod utils;

pub use cookies::*;
pub use data::*;
pub use error::*;

pub fn set_cookie(cookies: CreateCookies) -> Result<(), CookiesError> {
    get_html_document()?
        .set_cookie(&cookies.to_string())
        .map_err(|_| CookiesError::CouldNotSetCookie)
}

pub fn get_cookie(cookie_name: &str) -> Result<Cookies, CookiesError> {
    get_cookies()?
        .into_iter()
        .find(|cookie| cookie.name() == cookie_name)
        .ok_or(CookiesError::CouldNotFindCookie(cookie_name.into()))
}

pub fn get_cookies() -> Result<HashSet<Cookies>, CookiesError> {
    let mut has_of_cookies = HashSet::new();

    let cookies_list: Result<Vec<Cookies>, _> = split_cookies_string_into_vec_cookie(
        &get_html_document()?
            .cookie()
            .map_err(|_| CookiesError::CouldNotFindAnyCookie)?,
    )
    .into_iter()
    .map(|x| x.try_into())
    .collect();

    let _ = cookies_list
        .map_err(|_| CookiesError::ParsingError)?
        .iter()
        .map(|x| has_of_cookies.insert(x.clone()));

    Ok(has_of_cookies)
}
