use actix_web::body::BoxBody;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{HttpRequest, HttpResponse, Responder};

pub struct Redirect {
    to: String,
    body: Option<String>,
}

impl Redirect {
    pub fn new<S: AsRef<str>>(to: S) -> Self {
        Self {
            to: to.as_ref().to_string(),
            body: None,
        }
    }

    #[allow(unused)]
    pub fn new_with_body<S: AsRef<str>, B: AsRef<str>>(to: S, body: B) -> Self {
        Self {
            to: to.as_ref().to_string(),
            body: Some(body.as_ref().to_string()),
        }
    }
}

impl Responder for Redirect {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut builder = HttpResponse::Found();
        builder.append_header((
            HeaderName::from_static("location"),
            HeaderValue::from_str(&self.to).unwrap(),
        ));

        if let Some(body) = self.body {
            builder.body(body)
        } else {
            builder.finish()
        }
    }
}
