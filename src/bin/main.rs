use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::time::Duration;
use std::thread;
use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, web_page) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "not_found.html")
    };

    let contents = fs::read_to_string(web_page).unwrap();
    let response = format!("{}{}", status_line, contents);
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {

        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream)
        });
    }
}