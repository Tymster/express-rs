use std::collections::HashMap;
use std::io;
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub body: String,
    pub headers: HashMap<String, String>,
}
impl Request {
    pub fn authorization(&self) -> Result<String, io::Error> {
        Ok(self.headers.get("Authorization").unwrap().to_string())
    }
    pub fn user_agent(&self) -> Result<String, io::Error> {
        Ok(self.headers.get("User-Agent").unwrap().to_string())
    }
}
#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    UPDATE,
    DELETE,
    IDK,
}
