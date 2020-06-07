use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use web_server_multi_thread::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening at localhost:7878");
    println!("handling route: /");
    println!("handling route: /sleep");
    println!("handling route: /404");
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "web-server-multi-thread/src/hello.html",
        )
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            "web-server-multi-thread/src/hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            "web-server-multi-thread/src/404.html",
        )
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
