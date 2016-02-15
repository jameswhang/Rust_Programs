// use super::HttpRequest;
extern crate http_muncher;
use self::http_muncher::{Parser, ParserHandler};
use std::net::{TcpListener, TcpStream, SocketAddr, AddrParseError};
use std::thread;
use std::collections::HashMap;
use std::io::{Read, Write};


struct HttpParser;
impl ParserHandler for HttpParser { }

#[derive(Debug)]
struct SocketClient {
    socket : TcpStream,
    http_parser : Parser<HttpParser>,
}

impl SocketClient {
    fn read(&mut self) {
        loop {
            let mut buf = [0u8; 2048];

            match self.socket.read(&mut buf) {
                Err(e) => {
                    println!("Error reading socket: {:?}", e);
                },

                Ok(content_len) => {
                    self.http_parser.parse(&buf[0..content_len]);

                    if self.http_parser.is_upgrade() {
                        //includes Connection : upgrade header
                        break;
                    }

                    if content_len == 0 {
                        break;
                    }
                }
            }
        }
    }

    fn new(socket : TcpStream) -> SocketClient {
        SocketClient {
            socket : socket,
            http_parser : Parser::request(HttpParser)
        }
    }
}


///
///
///
pub struct HttpServer {
    listener : Option<TcpListener>,
    connections : HashMap<SocketAddr, SocketClient>,
    address : SocketAddr,
    connection_count : usize
}

impl HttpServer {
    ///
    ///
    ///
    pub fn new(address : String )  -> Result<HttpServer, AddrParseError> {
        match  address.parse::<SocketAddr>() {
            Ok(saddr) => Ok(
                HttpServer {
                    listener : None,
                    connections : HashMap::new(),
                    address : saddr,
                    connection_count : 0usize,
            }),

            Err(e) => return Err(e),
        }
    }

    ///
    ///
    ///
    pub fn start(&mut self) {
        println!("Starting server on port: {}", self.address.port());
        self.listener = Some(TcpListener::bind(&self.address).unwrap());
        // need to spawn thread start server loop
        // self.server_loop();
    }

    ///
    ///
    ///
    fn server_loop(&mut self) {

        if let Some(ref mut listener) = self.listener {
            for connection in listener.incoming() {
                match connection {
                    Ok(stream)  => {
                        let new_client = SocketClient::new(stream);

                        if let Ok(pa) = new_client.socket.peer_addr() {
                            self.connections.insert(pa, new_client);
                        } else {
                            unreachable!();
                        }

                        thread::spawn(move|| {
                            HttpServer::handle_client(&mut new_client);
                        });
                    }

                    Err(e) => {
                        println!("Error accepting connection: {:?}", e);
                    },
                }
            }//end for

            drop(listener);
        }
    }//end server_loop

    ///
    ///
    ///
    fn handle_client(client : &mut SocketClient){

    }
}
