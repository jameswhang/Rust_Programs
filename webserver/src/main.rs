#[doc="
    @authors: James Whang (syw973) and Adel Lahlou (adl538)

    Our webserver uses the Rust standard library dominantly, and depends on the chrono crate
    to timestamp responses.

"]
extern crate chrono;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{ErrorKind, Read, Write};
use std::fs::File;
use std::env;
use std::sync::{Arc, Mutex};
sh

use chrono::*;

/// Struct to hold parsed data from HTTP Request
/// is_error signals that values given in Request are not valid
struct HttpRequest {
    http_method: String,
    request_path: String,
    protocol: String,
    is_error: bool,
}

/// Struct to hold data being assembled for HTTP Response
struct HttpResponse {
    protocol: String,
    http_method: String,
    status_code: String,
    content_type: String,
    content_length: usize,
    payload: String,
}

/// Reads from TcpStream. Blocks until completely read
/// @param stream : &TcpStream
///
/// @return String returns the raw HTTP request as a String
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


/// Parses string from raw string from tcpStream into HttpRequest struct
/// A HTTPRequest object is returned instead of an option that way we can
/// respond to errors simply.
///
/// @param stream : String   The raw String from tcp stream
///
/// @return HttpRequest returns parse HttpRequest
fn parse_http_request(request: String) -> HttpRequest {
    let req = request.clone();
    println!("{}", req);
    let splits: Vec<&str> = if let Some(line) = req.lines().nth(0) {
        line.split(" ").collect()
    } else {
        vec![]
    };

    if splits.len() < 3 || splits[0] != "GET" {
        println!("request parse error");
        return HttpRequest {
            protocol: "HTTP".to_string(),
            http_method: "1.1".to_string(),
            request_path: "".to_string(),
            is_error: true,
        };
    }

    return HttpRequest {
        http_method: splits[0].to_string(),
        request_path: convert_path(splits[1].to_string()),
        protocol: splits[2].to_string(),
        is_error: false,
    };
}


/// Used to make the path relative to directory instead of absolute.
/// Aka removes leading forward slash
///
/// @param path : String
///
/// @return String returns path string with no leading slash
fn convert_path(path: String) -> String {
    match path.find('/') {
        Some(index) if index == 0 => {
            let slice = &path[1..];
            return slice.to_owned();
        },
        _ => return path
    }
}


/// Simply takes string segment after last period in file extension to decipher type
/// If not html, returns it as plain text.
///
/// @param path : String
///
/// @return String returns "text/html" or "text/plain"
fn get_content_type(path: String) -> String {
    let mut tokens: Vec<&str> = path.split(".").collect();
    let extension = tokens.pop().unwrap();
    if extension == "html" {
        "text/html".to_string()
    } else {
        "text/plain".to_string()
    }
}


/// Reads from file if it exists at provided path. Blocks until completely read.
/// @param file_path : String
///
/// @return Result<String, ErrorKind> returns a result with the file contents or an Error
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


/// Reads from TcpStream. Blocks until completely read
/// @param request : &HttpRequest
/// @param status_code : &str
/// @param payload : String
///
/// @return HttpResponse returns HttpResponse object
fn make_response(request: &HttpRequest, status_code: &str, payload: String) -> HttpResponse {
    HttpResponse {
        protocol: request.protocol.clone(),
        http_method: request.http_method.clone(),
        status_code: status_code.to_string(),
        content_type: get_content_type(request.request_path.clone()),
        content_length: payload.len(),
        payload: payload,
    }
}

/// Called by listener to handle each connection. Reads the http request from the stream,
/// generates the proper http response, and logs all actions. Logging is sychronous (mutex)
///
/// @param stream : &TcpStream
/// @param log_lock : &Arc<Mutex<File>>
fn handle_client(stream: &mut TcpStream, log_lock: &Arc<Mutex<File>>) {
    println!("handling client");
    let http_request = read_http_request(stream); // blocking
    let req : HttpRequest = parse_http_request(http_request);

    if !req.is_error {
        let file_contents = read_file(req.request_path.clone()); // blocking
        let resp: HttpResponse;

        match file_contents {
            Ok(payload) => {
                resp = make_response(&req, "200", payload);
                let log_content = make_log(&req, &resp);
                let mut logfile = log_lock.lock().unwrap();
                logfile.write(log_content.as_bytes());
                send_response(stream, resp);
            },

            Err(err_code) => {
                if err_code == ErrorKind::NotFound {
                    resp = make_response(&req, "404", "".to_string());
                    send_response(stream, resp);
                } else if err_code == ErrorKind::PermissionDenied {
                    //tried to access file server did not have read permission for
                    resp = make_response(&req, "403", "".to_string());
                    send_response(stream, resp);
                }
            }
        }
    } else {
        // 400: Bad HttpRequest
        let resp = make_response(&req, "400", "".to_string());
        send_response(stream, resp); //blocking
    }
}

/// Helper function to write into log file
///
/// @param req : &HttpRequest
/// @param resp : &HttpResponse
fn make_log(req: &HttpRequest, resp: &HttpResponse) -> String {
    let mut log = "".to_string();
    log = log + &Local::now().format("%m-%d-%Y %H:%M:%S").to_string() + &"\n";
    log = log + &"Request: " + &req.http_method + &" " + &req.request_path + &"\n";
    //log = log + &"Response: " + &resp.status_code + &" " + &resp.content_type = &"\n";
    return log;
}

/// Blocking. Sends HttpResponse into stream
///
/// @param stream : &mut TcpStream
/// @param resp : &HttpResponse
fn send_response(stream: &mut TcpStream, response: HttpResponse) {
    let mut response_text: String = "\n".to_string();
    response_text = response_text + &response.protocol;
    response_text = response_text + &" ";
    response_text = response_text + &response.status_code;
    response_text = response_text + &" ";

    if &response.status_code == &"200" {
        response_text = response_text + &" OK\n";
        response_text = response_text + &" adl538-syw973-webserver" + &"\n";
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

// accept connections and process them, spawning a new thread for each one
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let log = File::create("log.txt").unwrap();
    let log_lock = Arc::new(Mutex::new(log));
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let log_lock = log_lock.clone();
        thread::spawn(move || {
            // connection succeeded
            handle_client(&mut stream, &log_lock)
        });
    }
}
