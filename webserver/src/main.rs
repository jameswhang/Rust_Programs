use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;


struct Request {
    http_method: String,
    request_path: String, 
    http_version: String,
}

struct Response {
    http_method: String,
    status_code: String,
    content_type: String,
    content_length: usize,
    content: String,
}

fn parse_http_request(request: String) -> Request {
    let req = request.clone(); 
    println!("{}", req);
    let splits: Vec<&str> = req.split(' ').collect();
    if splits.len() < 3 {
        println!("request parse error");
        return Request {
            http_method: "invalid".to_string(),
            request_path: "none".to_string(),
            http_version: "none".to_string(),
        };
    }

    let method: String = splits[0].to_owned();
    let path: String = splits[1].to_owned(); 
    let ver: String = splits[2].to_owned();

    println!("method: {}, path: {}, ver: {}", method, path, ver);

    // TODO: Sanity check for bad request, forbidden and not found
    return Request {
        http_method: method,
        request_path: path, 
        http_version: ver,
    };
}


fn handle_client(stream: &mut TcpStream) {
    println!("handling client");
    let mut request: String = "".to_owned();
    let read = stream.read_to_string(&mut request); // TODO: Sanity check for http request method
                                                //    by checking string length
    let req : Request = parse_http_request(request);

    let mut resp_string : String = "HTTP/1.1 200 OK".to_string();
    resp_string = resp_string + "\n";
    resp_string = resp_string + "Server: syw973-webserver (MacOS)\n";
    resp_string = resp_string + "Content-Length: 88\n";
    resp_string = resp_string + "Connection: Closed\n";
    resp_string = resp_string + "<html>\n<body>\n<h1>Hello World!</h1>\n</body>\n</html>\n";
    
    println!("resp_string: {}", resp_string);
    stream.write(resp_string.as_bytes());

}

fn send_response(stream: &mut TcpStream) {
    let mut resp_string : String = "HTTP/1.1 200 OK".to_string();
    resp_string = resp_string + "\n";
    resp_string = resp_string + "Server: syw973-webserver (MacOS)\n";
    resp_string = resp_string + "Content-Length: 88\n";
    resp_string = resp_string + "Connection: Closed\n";
    resp_string = resp_string + "<html>\n<body>\n<h1>Hello World!</h1>\n</body>\n</html>\n";
    stream.write(resp_string.as_bytes());
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
