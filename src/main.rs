mod app;
mod request;
mod response;
mod route;
use app::App;
use request::Request;
use response::Response;
fn main() {
    let mut x = App::new();
    x.at("/").get(base);
    x.serve("127.0.0.1:8000");
}
fn base(req: Request) -> Response {
    println!("User : {}", req.user_agent().unwrap());
    println!("Auth : {}", req.authorization().unwrap());

    Response::stat("static/icon.png")
}
