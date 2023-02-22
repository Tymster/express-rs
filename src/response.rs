use std::collections::HashMap;
use std::io::{BufReader, Read};
#[derive(Debug)]
pub struct Response {
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub reason_phrase: String,
    pub code: u16,
}
impl Response {
    pub fn build(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = format!("HTTP/1.1 {} {} \r\n", self.code, self.reason_phrase)
            .as_bytes()
            .to_vec();
        for (k, v) in self.headers {
            res.append(&mut format!("{k}:{v}").as_bytes().to_vec());
        }
        res.append(&mut "\r\n\r\n".as_bytes().to_vec());
        res.append(&mut self.body);
        res
    }
    pub fn redirect(url: &str) -> Response {
        let mut res = Response::default();
        res.code = 302;
        res.reason_phrase = "Found".to_string();
        res.headers.insert("Location".to_string(), url.to_string());
        res
    }
    pub fn stat(path: &str) -> Response {
        let mut contents = vec![];
        BufReader::new(std::fs::File::open(path).unwrap())
            .read_to_end(&mut contents)
            .unwrap();
        let mut res = Response::default();
        res.headers.insert(
            "Content-Type".to_string(),
            path.clone().split(".").last().unwrap().to_string(),
        );
        res.body = contents;
        res
    }
    pub fn send(text: &str) -> Response {
        let mut res = Response::default();
        res.body = text.as_bytes().to_vec();
        res
    }
    pub fn download(path: &str) {}
}
impl Default for Response {
    fn default() -> Self {
        Response {
            headers: HashMap::new(),
            body: vec![],
            reason_phrase: "OK".to_string(),
            code: 200,
        }
    }
}
