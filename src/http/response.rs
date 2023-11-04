#![allow(dead_code)]
#![allow(unused_variables)]

use super::status_code::StatusCode;
use super::response_type::ContentType;

#[derive(Debug, Clone)]
pub struct Response<'a> {
    pub status: &'a str,
    pub content_type: &'a str,
    pub content: String,
    pub content_length: usize,
    pub headers: Vec<(String, String)>,
}

impl<'a> Response<'a> {

    pub fn new() -> Self {
        Response {
            status: StatusCode::HTTP_OK,
            content_type: ContentType::CONTENT_TYPE_TEXT_PLAIN,
            content: String::new(),
            content_length: 0,
            headers: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: &'a str) {
        self.status = status;
    }

    pub fn set_content_type(&mut self, content_type: &'a str) {
        self.content_type = content_type;
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
        self.content_length = self.content.len();
    }

    pub fn set_content_lenght(&mut self, content_length: usize) {
        self.content_length = content_length;
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.push((key.to_string(), value.to_string()));
    }

    pub fn prepare(&self) -> String {
        let mut headers_str: String = String::new();
        for (key, value) in &self.headers {
            headers_str.push_str(&format!("{}: {}\r\n", key, value));
        }
        return format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n{}\r\n{}",
            self.status, self.content_type, self.content_length, headers_str, self.content
        );
    }

}
