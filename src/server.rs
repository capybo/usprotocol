use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::commands;
use crate::response::Response;

pub struct Server {
    port: u16,
    max_connections: usize,
    current_connections: Arc<Mutex<usize>>
}

impl Server {
    pub fn new(port: u16, max_connections: usize) -> Self {
        Server {
            port,
            max_connections,
            current_connections: Arc::new(Mutex::new(0))
        }
    }

    pub fn start(&self) {
        let listener = TcpListener::bind(("127.0.0.1", self.port))
            .expect("Failed to bind port");

        println!("Server listening on port {}", self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let current_connections = Arc::clone(&self.current_connections);
                    let _ = thread::spawn(move || {
                        handle_client(stream, current_connections);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, current_connections: Arc<Mutex<usize>>) {
    let mut buffer = [0; 1024];
    let mut bytes_read = 0;

    bytes_read = match stream.read(&mut buffer) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("Error reading from stream");
            return;
        }
    };

    let client_address = stream.peer_addr().unwrap();
    println!("Handling client: {}", client_address);

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received request from client {}: {}", client_address, request);

    let response = process_request(&request[..]);
    println!("Sending response to client {}: {}", client_address, response.to_string());

    let _ = stream.write(response.to_string().as_bytes());
}

fn process_request(request: &str) -> Response {
    let mut parts = request.split_whitespace();

    if let Some(command) = parts.next() {
        match command {
            "COMPARE_WORDS" => commands::compare_words(parts.collect()),
            "PLUS" => commands::plus(parts.collect()),
            "MINUS" => commands::minus(parts.collect()),
            _ => Response::unknown_command()
        }
    } else {
        Response::error("General error in the request or processing.")
    }
}