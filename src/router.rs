#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;

use crate::http::request::Request;
use crate::http::response::Response;

pub struct Router {
    routes: HashMap<String, fn(&Request) -> Response>,
}

impl Router {

    pub fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    pub fn register_route(&mut self, path: &str, handler: fn(&Request) -> Response) {
        self.routes.insert(path.to_string(), handler);
    }

    pub fn get_handler(&self, path: &str) -> Option<fn(&Request) -> Response> {
        self.routes.get(path).copied()
    }

}