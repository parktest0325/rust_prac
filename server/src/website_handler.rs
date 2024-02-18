use super::server::Handler;
use std::fs;
use super::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                }
                else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                "/test" => Response::new(StatusCode::Ok, self.read_file("test.html")),
                // 모든 정적 파일을 매핑하기 위해 추가한 이 코드때문에 Path Traversal 취약점이 발생할 수 있다
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}