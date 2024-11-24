# Cookies-WASM

## Introduction
This rust library provides a way to manage cookies.


## Features
- Create Cookies
- Get all the cookies
- Find a specific cookie


## Usage

### Create Cookie

1. Create cookie via the builder struct

```
    let new_cookie : Result<CreateCookies, CookiesError> = CreateCookiesBuilder::new()
        .name("cookie_name") 
        .value("cookie_value")
        .expire(Some(Expire::After(3600))) # This is Optional
        .same_site(Some(SameSite::Strict)) # This is Optional
        .path("/") # This is Optional
        .build()
```
Both name and value are mandatory here. I have decided that you cannot create a cookie without providing a name.

2. Call function to create new cookie

```
    let _ : Result<(), CookiesError> = set_cookie(new_cookie);
```
3. That's it


### Find a specific cookie

```
    let all_cookies : Result<HashSet<Cookies>,CookiesError> = get_cookies();
```


### Get all cookies

```
    let specific_cookie : Result<Cookies, CookiesError> = get_cookie("cookie_name");
```
