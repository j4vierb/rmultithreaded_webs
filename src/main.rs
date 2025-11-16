use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
    thread,
    time::Duration, 
};

use rmultithreaded_webs::ThreadPool;

fn main() {
    println!("Hello, world!");
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Err(_) => panic!("ERROR: the port is already being use!"),
        Ok(tcp_listener) => tcp_listener,
    };
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        // when stream goes out of scope it's dropped by the Drop trait impl
        let stream = stream.unwrap();

        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream)
        });
    }
}

/*
* [*>] Thread pool 
*
* This technique is just one of many ways to improve the throughput of a web server.
* Other options you might explore are the fork/join model, the single-threaded async 
* I/O model, and the multithreaded async I/O model. If you’re interested in this topic,
* you can read more about other solutions and try to implement them; with a low-level
* language like Rust, all of these options are possible.
* */

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
} 
