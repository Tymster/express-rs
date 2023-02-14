use crate::request::Method;
use crate::request::Request;
use crate::response::Response;
#[derive(Debug)]
pub struct Route {
    pub url: String,
    pub method: Method,
    pub handler: fn(Request) -> Response,
}
