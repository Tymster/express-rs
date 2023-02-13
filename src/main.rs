use std::io::BufRead;
use std::net::TcpStream;
use std::{io::Read, io::Write, net::TcpListener};

use regex::Regex;
#[derive(Debug)]
enum Method {
    GET,
    UPDATE,
    DELETE,
    POST,
    UNKNOWN,
}
struct Response {
    stream: TcpStream,
    //serve images
    //serve redirect
    // serve html
    //serve downloads
    //send data
}
impl Response {
    fn stat(&mut self, path: &str) {
        self.write(format!(
            "HTTP/1.1 200 OK\r\n\r\n{}",
            std::fs::read_to_string(path).unwrap()
        ));
    }
    fn redirect(&mut self, url: &str) {
        self.write(format!("HTTP/1.1 302 Found\r\nLocation: {url}\r\n\r\n"));
    }
    fn write(&mut self, response: String) {
        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}
#[derive(Debug)]
struct Request {
    user_agent: String,
    path: String,
    body: String,
    method: Method,
    //parameters
    //body
    //method
    //user agent
}
struct Route {
    handler: fn(Request, Response),
    path: String,
    regex: Regex,
}
struct App {
    port: u16,
}
impl App {
    pub fn new(port: u16) -> App {
        App { port }
    }
    pub fn listen(self) {
        for stream in TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .unwrap()
            .incoming()
        {
            let mut stream = stream.unwrap();
            let buf_reader = std::io::BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            let path_method: Vec<String> =
                http_request[0].split(" ").map(|f| f.to_string()).collect();
            let request = Request {
                user_agent: http_request[2].clone(),
                method: get_method(path_method[0].clone()),
                path: path_method[1].clone(),
                body: String::new(),
            };
            let mut response = Response { stream };
            response.stat("static/index.html")
        }
    }
}
fn main() {
    // let x = App::new(1234);
    // x.get("/users/:id", home);
    // x.listen();
    let path = "/users/:id";
    let url = "/users/f13j9f31/jerome";
    let re = Regex::new(r"/users/(?P<id>[^/]+)/(?P<jer>[^/]+)").unwrap();
    for n in re.captures(url).unwrap().iter().skip(1) {
        println!("N : {}", n.unwrap().as_str());
    }
}
fn home(res: Response, req: Request) {
    // req.params["key"];
    //req.user_agent
}
fn get_method(method: String) -> Method {
    match method.as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "DELETE" => Method::DELETE,
        "UPDATE" => Method::UPDATE,
        _ => Method::UNKNOWN,
    }
}
