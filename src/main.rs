mod config;
use config::Config;

mod router;
use router::Router;

mod http;
use http::request::Request;
use http::response::Response;
use http::response_type::ContentType;
use http::status_code::StatusCode;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::create_from_args(&args).unwrap_or_else(|err| {
        eprintln!("Unable to load configuration, error: {}", err);
        println!("Exiting..");
        process::exit(1);
    });

    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .unwrap_or_else(|err: std::io::Error| {
            eprintln!("Unable to bind on port: {}, error: {}", config.port, err);
            println!("Exiting..");
            process::exit(1);
        });

    let mut router: Router = Router::new();
    register_routes(&mut router);

    println!("Server ready at port {}", config.port);
    println!("Listening...");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        if let Err(error) = run(&router, stream) {
            eprintln!("Unable to handle connection, error: {}", error);
        }
    }

    println!("Exiting..");
    process::exit(0);

}

fn register_routes(router: &mut Router) {

    // #[Route(path: "/", name: "index")]
    router.register_route("/", |_request: &Request| {
        let mut response: Response = Response::new();
        response.set_status(StatusCode::HTTP_OK);
        response.set_content_type(ContentType::CONTENT_TYPE_TEXT_HTML);

        let mut file: File = File::open("static/index.html").unwrap();
        let mut content: String = String::new();
        file.read_to_string(&mut content).unwrap();

        response.set_content(&content);
        return response;
    });

    // #[Route(path: "/about", name: "about")]
    router.register_route("/about", |_request: &Request| {
        let mut response: Response = Response::new();
        response.set_status(StatusCode::HTTP_OK);
        response.set_content_type(ContentType::CONTENT_TYPE_TEXT_HTML);

        let mut file: File = File::open("static/about.html").unwrap();
        let mut content: String = String::new();
        file.read_to_string(&mut content).unwrap();

        response.set_content(&content);
        return response;
    });

}

fn run(router: &Router, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    let request: Request = Request::from_tcp_stream(&mut stream)?;
    let handler: Option<fn(&Request) -> Response<'_>> = router.get_handler(&request.path);

    let mut response: Response;
    if let Some(handler) = handler {
        response = handler(&request);
    } else {
        response = Response::new();
        response.set_status(StatusCode::HTTP_NOT_FOUND);
        response.set_content_type(ContentType::CONTENT_TYPE_TEXT_PLAIN);
    }

    response.set_header("X-Powered-By", "RustWS");
    response.set_header("X-Server-Version", "v1.0");

    let response = response.prepare();
    stream.write_all(response.as_bytes())?;
    return Ok(());

}
