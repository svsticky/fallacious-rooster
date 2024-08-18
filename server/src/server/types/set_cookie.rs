use actix_web::cookie::Cookie;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub struct SetCookie<T: Responder> {
    inner: T,
    cookie_name: String,
    cookie_value: String,
}

impl<T: Responder> SetCookie<T> {
    pub fn new<N: AsRef<str>, V: AsRef<str>>(inner: T, cookie_name: N, cookie_value: V) -> Self {
        Self {
            inner,
            cookie_name: cookie_name.as_ref().to_string(),
            cookie_value: cookie_value.as_ref().to_string(),
        }
    }
}

impl<T: Responder> Responder for SetCookie<T> {
    type Body = <T as Responder>::Body;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut response = self.inner.respond_to(req);
        let mut cookie = Cookie::new(self.cookie_name, self.cookie_value);
        cookie.set_path("/");
        response.add_cookie(&cookie).unwrap();

        response
    }
}
