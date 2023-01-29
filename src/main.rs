// Uncomment this block to pass the first stage
use std::io::Write;
use std::net::TcpListener;

fn main() {
  println!("Logs from your program will appear here!");

  let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        println!("accepted new connection");

        stream.write("+PONG\r\n".as_bytes()).unwrap();
      }
      Err(e) => {
        println!("error: {}", e);
      }
    }
  }
}
