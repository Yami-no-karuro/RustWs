#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn from_tcp_stream(stream: &mut TcpStream) -> Result<Self, Box<dyn std::error::Error>> {
        let mut reader: BufReader<&mut TcpStream> = BufReader::new(stream);
        let mut request_line: String = String::new();

        reader.read_line(&mut request_line)?;
        let request_line_parts: Vec<&str> = request_line.split_whitespace().collect();
        let method: String = request_line_parts[0].to_string();
        let path: String = request_line_parts[1].to_string();

        let mut headers: HashMap<String, String> = HashMap::new();
        let mut cookies: HashMap<String, String> = HashMap::new();
        let mut content_length: usize = 0;

        loop {
            let mut line: String = String::new();
            reader.read_line(&mut line)?;
            if line.trim().is_empty() {
                break;
            }

            let parts: Vec<&str> = line.trim().splitn(2, ": ").collect();
            if parts.len() == 2 {
                if parts[0].eq_ignore_ascii_case("Cookie") {
                    let cookie_string: String = parts[1].to_string();
                    let cookie_parts: Vec<&str> = cookie_string.split("; ").collect();
                    for cookie_part in cookie_parts {
                        let cookie_pair: Vec<&str> = cookie_part.splitn(2, "=").collect();
                        if cookie_pair.len() == 2 {
                            cookies.insert(
                                cookie_pair[0].trim().to_string(),
                                cookie_pair[1].trim().to_string(),
                            );
                        }
                    }
                } else {
                    headers.insert(parts[0].to_string(), parts[1].to_string());
                    if parts[0].eq_ignore_ascii_case("Content-Length") {
                        content_length = parts[1].trim().parse::<usize>().unwrap_or(0);
                    }
                }
            }
        }

        let mut body = String::new();
        if content_length > 0 {
            reader
                .take(content_length as u64)
                .read_to_string(&mut body)?;
        }

        return Ok(Request {
            method,
            path,
            headers,
            cookies,
            body,
        });
    }
}
