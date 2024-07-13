use std::mem::MaybeUninit;

use axum::{
    extract::Request, 
    response::{Response, AppendHeaders, IntoResponse},
    http::header::SET_COOKIE,
    middleware::Next,
};

use crate::error::{Result, Error::*};

pub const COOKIE_NAME:&str = "SESSION";

fn parse_header(headers: &axum::http::HeaderMap)
        ->Result<Option<SessionID>>
{
    let s = match headers.get(axum::http::header::COOKIE) {
        None => None,
        Some(s) => {
            let s = s.to_str().map_err(|_|BadCookie)?;
            let mut cookies = cookie::Cookie::split_parse_encoded(s);
            loop {
                let cookie = match cookies.next() {
                    None => break None,
                    Some(c) => c.map_err(|_|BadCookie)?,
                };
                if cookie.name()==COOKIE_NAME {
                    break match SessionID::from_b64(cookie.value()) {
                        None => {None},
                        Some(x) => {Some(x)},
                    }
                }
            }
        },
    };
    Ok(s)
}

/// middlewaare that inserts SessionID into extensions
pub async fn session(
    mut req: Request,
    next: Next
) -> Result<Response> {
    let idopt = parse_header(&req.headers())?;
    let newid = idopt.is_none();
    let id = idopt.unwrap_or(SessionID::new());
    req.extensions_mut().insert(id.clone());
    let mut resp = next.run(req).await;
    if newid {
        let cookie = cookie::Cookie::build((COOKIE_NAME, id.to_b64()))
            .same_site(cookie::SameSite::Lax)
            .permanent()
            .path("/")
            .build();
        resp = (AppendHeaders([(SET_COOKIE, &cookie.encoded().to_string())]), resp)
            .into_response()
    }
    Ok(resp)
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionID([u128;2]);
impl SessionID {
    pub fn new()->Self 
        {Self(rand::random())}
    pub fn from_b64(b64: &str)->Option<Self> {
        use base64::prelude::*;
        // safety: content unimportant
        #[allow(invalid_value)]
        let mut x:Self = unsafe{MaybeUninit::uninit().assume_init()};
        match BASE64_STANDARD.decode_slice(b64, x.as_mut()) {
            Ok(len) => {
                if len==32 {Some(x)}
                else {None}
            },
            Err(_) => None
        }
    }
    pub fn to_b64(&self)->String {
        use base64::prelude::*;
        BASE64_STANDARD.encode(self)
    }
}

impl AsRef<[u8]> for SessionID {
    fn as_ref(&self) -> &[u8] 
        // safety: properly sized
        {unsafe{&*(&self.0 as *const u128 as *const [u8;32])}}
}
impl AsMut<[u8]> for SessionID {
    fn as_mut(&mut self) -> &mut [u8] 
        {unsafe{&mut *(&mut self.0 as *mut u128 as *mut [u8;32])}}
}
impl std::fmt::Debug for SessionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
        {write!(f, "{}", self.to_b64())}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_id() {
        let id = SessionID::new();
        let s = id.to_b64();
        let id2 = SessionID::from_b64(&s).unwrap();
        assert_eq!(id, id2)
    }
}