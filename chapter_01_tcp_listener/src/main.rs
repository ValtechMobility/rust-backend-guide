use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("Starting the listener!");

    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    loop {
        let (mut socket, _) = listener.accept().unwrap();

        let mut buffer = [0; 1024];

        // Read data from the socket
        let n = socket.read(&mut buffer).unwrap();
        if n == 0 {
            return;
        }

        // Convert the buffer to a string for parsing
        let request = String::from_utf8_lossy(&buffer[..n]);

        let response;

        if request.starts_with("GET") {
            response = "HTTP/1.1 200 OK\r\n\r\nHello, Client! You sent a GET request!";
        } else if request.starts_with("POST") {
            response = "HTTP/1.1 200 OK\r\n\r\nHello, Client! You sent a POST request!";
        } else {
            response = "HTTP/1.1 404 Bad Request\r\n\r\nHello, Client! You sent a request with an unsupported method!";
        };

        socket.write(response.as_bytes()).unwrap();
        socket.flush().unwrap();

        println!("Sent a reply to the client!")
    }
}
