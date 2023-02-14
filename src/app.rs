use crate::request::{Method, Request};
use crate::response::Response;
use crate::route::Route;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
pub struct App {
    pub routes: Vec<Route>,
}
impl App {
    pub fn new() -> App {
        App { routes: vec![] }
    }
    pub fn at(&mut self, url: &str) -> At {
        At {
            url: url.to_string(),
            app: self,
        }
    }
    pub fn serve(self, port: &str) {
        for n in TcpListener::bind(port).unwrap().incoming() {
            let mut stream = n.unwrap();
            let mut headers: HashMap<String, String> = HashMap::new();
            let buf_reader = BufReader::new(&mut stream);
            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();
            for n in http_request.iter().skip(1) {
                let k_v: Vec<String> = n.split(":").map(|f| f.trim().to_string()).collect();
                headers.insert(k_v[0].to_string(), k_v[1].to_string());
            }
            let first_line = http_request[0]
                .split(" ")
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            for route in self.routes.iter() {
                if route.url == first_line[1] || route.url == "*" {
                    println!("Found path");

                    let response: Response = (route.handler)(Request {
                        method: get_method(&first_line[0]),
                        body: "jerome".to_string(),
                        headers,
                    });
                    stream.write(&response.build()).unwrap();
                    stream.flush().unwrap();
                    break;
                }
            }
        }
    }
}
pub struct At<'a> {
    pub url: String,
    pub app: &'a mut App,
}
impl At<'_> {
    pub fn get(mut self, handler: fn(Request) -> Response) -> Self {
        self.add(handler, Method::GET);
        self
    }
    pub fn post(mut self, handler: fn(Request) -> Response) -> Self {
        self.add(handler, Method::POST);
        self
    }
    pub fn add(&mut self, handler: fn(Request) -> Response, method: Method) {
        self.app.routes.push(Route {
            method,
            url: self.url.clone(),
            handler,
        })
    }
}
fn get_method(method: &str) -> Method {
    match method {
        "GET" => Method::GET,
        "DELETE" => Method::DELETE,
        "UPDATE" => Method::UPDATE,
        "POST" => Method::POST,
        _ => Method::IDK,
    }
}
