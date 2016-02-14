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

pub struct HttpRequest<'a> {
    method : Option<HttpMethod>,
    object_url : &'a Path,
    host : &'a Path,
    http_version : Option<HttpVersion>,
    headers : HashMap<String, String>,
    raw_request : String,
}


impl<'a, 'b> HttpRequest<'b> {
    pub fn new_from(request : String) -> Result<HttpRequest<'b>, &'a str> {
        HttpRequest::parse_request(request)
    }

    fn parse_request(request : String) -> Result<HttpRequest<'b>, &'a str> {
        let line_counter = 0usize;
        let mut ret : HttpRequest;

        for line in request.lines() {
            let mut tokens = line.split_whitespace();

            if line_counter == 0 {
                if tokens.count() != 3 {
                    return Err("Expected 3 arguments in request line");
                } else {
                    ret.method = HttpRequest::parse_method(tokens.nth(0).unwrap());
                    ret.object_url = Path::new(tokens.nth(1).unwrap());
                    ret.http_version = HttpRequest::parse_version(tokens.nth(2).unwrap());
                }
            } else if line_counter == 1 {

            } else {

            }
        }//end for

        ret
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
