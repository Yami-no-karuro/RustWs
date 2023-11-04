mod config;
use config::Config;

mod http;
use http::request::Request;
use http::response::Response;
use http::response_type::ContentType;
use http::status_code::StatusCode;

use std::env;
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

    println!("Server ready at port {}", config.port);
    println!("Listening...");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        if let Err(error) = run(stream) {
            eprintln!("Unable to handle connection, error: {}", error);
        }
    }

    println!("Exiting..");
    process::exit(0);

}

fn run(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    let request: Request = Request::from_tcp_stream(&mut stream)?;
    println!("{:#?}", request);

    let mut response: Response = Response::new();
    response.set_status(StatusCode::HTTP_OK);
    response.set_content_type(ContentType::CONTENT_TYPE_TEXT_PLAIN);
    response.set_content("Hello! \nWelcome to RustWs!");

    response.set_header("X-Powered-By", "RustWS");
    response.set_header("X-Server-Version", "v1.0");

    let response = response.prepare();
    stream.write_all(response.as_bytes())?;

    return Ok(());

}
