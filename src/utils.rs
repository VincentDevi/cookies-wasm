use super::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;

pub fn get_html_document() -> Result<HtmlDocument, CookiesError> {
    web_sys::window()
        .ok_or(CookiesError::MissingHtmlDocument)?
        .document()
        .ok_or(CookiesError::MissingHtmlDocument)?
        .dyn_into::<HtmlDocument>()
        .map_err(|_| CookiesError::MissingHtmlDocument)
}

pub fn split_cookies_string_into_vec_cookie(cookie: &str) -> Vec<String> {
    cookie.split(';').map(|x| x.trim().to_string()).collect()
}
