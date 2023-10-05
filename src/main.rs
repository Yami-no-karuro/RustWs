mod config;
use config::Config;

mod request;
use request::Request;

mod response;
use response::Response;

use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::create_from_args(&args).unwrap_or_else(|err| {
        println!("Unable to load configuration, error: {}", err);
        process::exit(1);
    });

    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .unwrap_or_else(|err: std::io::Error| {
            println!("Unable to bind on port: {}, error: {}", config.port, err);
            process::exit(1);
        });

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        if let Err(error) = handle_connection(stream) {
            println!("Unable to handle connection, error: {}", error);
        }
    }

    process::exit(0);
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let request: Request = Request::from_stream(&mut stream)?;

    let mut response: Response = Response::new("HTTP/1.1 200 OK", "text/plain");
    response.set_content(&format!("{:#?}", request));
    response.set_header("x-powered-by", "rustws");
    response.set_header("x-software-version", "v1.0");

    let response = response.prepare();
    stream.write_all(response.as_bytes())?;

    return Ok(());
}
