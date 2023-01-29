// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
  println!("Logs from your program will appear here!");

  let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        println!("accepted new connection");

        // first request
        let mut buffer = [0; 512];

        let mut is_connection_open = true;

        while is_connection_open {
          let bytes_read = stream.read(&mut buffer).unwrap();

          if bytes_read == 0 {
            println!("Client closed the connection");
            is_connection_open = false;
          }

          let received_string = String::from_utf8(buffer.to_vec()).unwrap();

          println!("Text gotten after first opening connection: {received_string}");

          stream.write("+PONG\r\n".as_bytes()).unwrap();
        }
      }
      Err(e) => {
        println!("error: {}", e);
      }
    }
  }
}
