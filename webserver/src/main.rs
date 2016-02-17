use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{ErrorKind, Read, Write};
use std::fs::File;
use std::env;



struct Request {
    http_method: String,
    request_path: String, 
    protocol: String,
    is_error: bool,
}

struct Response {
    protocol: String,
    http_method: String,
    status_code: String,
    content_type: String,
    content_length: usize,
    payload: String,
}

fn read_http_request(stream: &mut TcpStream) -> String {
    const BUF_SIZE: usize = 1024;
    let mut buf = [0; BUF_SIZE];
    let mut result = String::new();
    let mut addition: String;

    // continually pass in a buffer until nothing left to read
    while let Ok(length) = stream.read(&mut buf[..]) {
        // add data in buffer to results string
        addition = String::from_utf8(buf.to_owned()).unwrap();
        result.push_str(&addition);
        buf = [0; BUF_SIZE];

        // break if all of input has been read
        if length < BUF_SIZE {
            break;
        }
    }
    return result;
}

fn parse_http_request(request: String) -> Request {
    let req = request.clone(); 
    println!("{}", req);
    let splits: Vec<&str> = req.split(" ").collect();
    if splits.len() < 3 || splits[0] != "GET" {
        println!("request parse error");
        return Request {
            protocol: "HTTP".to_string(),
            http_method: "1.1".to_string(),
            request_path: "".to_string(),
            is_error: true,
        };
    }

    let protocol: Vec<&str> = splits[2].split("\n").collect();

    return Request {
        http_method: splits[0].to_string(),
        request_path: convert_path(splits[1].to_string()),
        protocol: protocol[0].to_string(),
        is_error: false,
    };
}

fn convert_path(path: String) -> String {
    let slash_index = path.find('/');
    match slash_index {
        Some(index) => {
            if index == 0 {
                let slice = &path[1..];
                return slice.to_owned();
            } else {
                return path;
            }
        },
        None => return path
    }
}

fn get_content_type(path: String) -> String {
    let mut tokens: Vec<&str> = path.split(".").collect();
    let extension = tokens.pop().unwrap();
    if extension == "html" {
        "text/html".to_string()
    } else {
        "text/plain".to_string()
    }
}

fn read_file(file_path: String) -> Result<String, ErrorKind>{
    let mut server_path = env::current_dir().unwrap();
    server_path.push(file_path);
    let mut file = File::open(server_path);
    match file {
        Ok(mut file) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content);
            Ok(file_content)
        }
        Err(e) => {
            Err(e.kind())
        }
    }
}

fn make_response(request: &Request, status_code: &str, payload: String) -> Response {
    Response {
        protocol: request.protocol.clone(),
        http_method: request.http_method.clone(),
        status_code: status_code.to_string(),
        content_type: get_content_type(request.request_path.clone()),
        content_length: payload.len(),
        payload: payload,
    }
}

fn handle_client(stream: &mut TcpStream) {
    println!("handling client");
    let http_request = read_http_request(stream);
    let req : Request = parse_http_request(http_request);
    if !req.is_error {
        let file_contents = read_file(req.request_path.clone());
        let resp: Response;
        match file_contents {
            Ok(payload) => {
                resp = make_response(&req, "200", payload);
                send_response(stream, resp);
            },
            Err(err_code) => {
                if err_code == ErrorKind::NotFound {
                    resp = make_response(&req, "404", "".to_string());
                    send_response(stream, resp);
                } else if err_code == ErrorKind::PermissionDenied {
                    resp = make_response(&req, "403", "".to_string());
                    send_response(stream, resp);
                }
            }
        } 
    } else {
        // 400: Bad Request
        let resp = make_response(&req, "400", "".to_string());
        send_response(stream, resp);
    }
}

fn send_response(stream: &mut TcpStream, response: Response) {
    let mut response_text: String = "\n".to_string();
    response_text = response_text + &response.protocol;
    response_text = response_text + &" ";
    response_text = response_text + &response.status_code;
    response_text = response_text + &" ";

    if &response.status_code == &"200" {
        response_text = response_text + &" OK\n";
        response_text = response_text + &" syw973-webserver" + &"\n";
        response_text = response_text + &"Content-type: " + &response.content_type + &"\n";
        response_text = response_text + &"Content-length: " + &response.content_length.to_string() + &"\n";
        response_text = response_text + &"\n\n";
        response_text = response_text + &response.payload;
        response_text = response_text + &"\n\n";
    } else {
        if &response.status_code == &"404" {
            response_text = response_text + &" Not Found" + &"\n";
        } else if &response.status_code == &"400" {
            response_text = response_text + &" Bad Request" + &"\n";
        } else if &response.status_code == &"403" {
            response_text = response_text + &" Forbidden" + &"\n";
        }
    }
    stream.write(response_text.as_bytes());
}

// accept connections and process them, spawning a new thread each one
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            // connection succeeded
            handle_client(&mut stream)
        });
    }
}
