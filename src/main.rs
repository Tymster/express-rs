use std::io::BufRead;
use std::{io::Read, io::Write, net::TcpListener};

struct Response {
    ty: String,
    //serve images
    //serve redirect
    // serve html
    //serve downloads
    //send data
}
struct Request {
    jer: String,
    //parameters
    //body
    //method
    //user agent
}
struct Route {
    jer: fn(Response, Request),
    //fn(Request, response)
    //path
    //regex
}
struct App {
    port: u16,
    //new
    //listen
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
            println!("Request: {:#?}", http_request);
        }
    }
}
fn main() {
    let x = App::new(1234);
    x.listen();
}
fn home(res: Response, req: Request) {
    // req.params["key"];
}
