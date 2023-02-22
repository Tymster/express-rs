mod app;
mod request;
mod response;
mod route;
use app::App;
use request::Request;
use response::Response;
fn main() {
    let mut x = App::new();
    x.at("/")
        .get(|_| return Response::redirect("https://www.youtube.com/"));
    x.at("/bananan")
        .get(|_| return Response::stat("static/index.html"));
    x.at("/user/new").post(|req: Request| {
        println!("{:?}", req.headers);
        return Response::stat("static/index.html");
    });
    x.serve("127.0.0.1:8000");
}
