#[doc="

"]

use std::path::Path;
use std::collections::HashMap;

pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PUT
}

pub enum HttpVersion{
    Http11,
    Http10,
    Http09,
}

pub struct HttpRequest {
    method : Option<HttpMethod>,
    object_url : String,
    host : String,
    http_version : Option<HttpVersion>,
    headers : HashMap<String, String>,
    raw_request : String,
}


impl HttpRequest {
    pub fn new() -> HttpRequest {
            HttpRequest {
                method : None,
                object_url: String::new(),
                host : String::new(),
                http_version : None,
                headers : HashMap::new(),
                raw_request : String::new(),
            }
    }

    pub fn new_from(request : String) -> Result<HttpRequest, &'static str> {
        HttpRequest::parse_request(request)
    }

    fn parse_request(request : String) -> Result<HttpRequest, &'static str> {
        let mut line_counter = 0usize;
        let mut ret = HttpRequest::new();
        ret.raw_request = request.clone();

        for line in request.lines() {
            match line_counter {
                0 => {
                    let mut tokens = line.split_whitespace();

                    if tokens.count() != 3 {
                        return Err("Invalid number of arguments in request line");
                    }

                    tokens = line.split_whitespace();
                    ret.method = HttpRequest::parse_method(tokens.nth(0).unwrap());
                    ret.object_url = tokens.nth(1).unwrap().to_string();
                    ret.http_version = HttpRequest::parse_version(tokens.nth(2).unwrap());
                },

                _ => {
                    let mut tokens = line.split(":");

                    if tokens.count() != 2 {
                        return Err("Invalid header arguments")
                    }

                    tokens = line.split(":");
                    let header = tokens.nth(1).unwrap().to_string();
                    let value = tokens.nth(2).unwrap().to_string();

                    match tokens.nth(1).unwrap() {
                        "Host" => {
                                ret.host = value;
                        },

                        _ => {
                            ret.headers.insert(header, value);
                        }
                    }
                }
            }

            line_counter += 1;
        }//end for line

        Ok(ret)
    }//end parse_request

    fn parse_method(method_str : &str) -> Option<HttpMethod> {
        match method_str {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "DELETE" => Some(HttpMethod::DELETE),
            "PUT" => Some(HttpMethod::PUT),
            _ => None,
        }
    }

    fn parse_version(version_str: &str) -> Option<HttpVersion> {
        match version_str {
            "HTTP/1.1" => Some(HttpVersion::Http11),
            "HTTP/1.0" => Some(HttpVersion::Http10),
            "HTTP/0.9" => Some(HttpVersion::Http09),
            _ => None,
        }
    }
}//end impl HttpParser

#[cfg(test)]
pub mod http_request_tests {

    #[test]
    fn parse_test_1() {
        unimplemented!()
    }

    #[test]
    fn parse_test_1() {
        unimplemented!()
    }

    #[test]
    fn parse_test_1() {
        unimplemented!()
    }


    #[test]
    fn parse_test_1() {
        unimplemented!()
    }

    #[test]
    fn parse_test_1() {
        unimplemented!()
    }
}
