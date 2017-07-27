extern crate hello;
use hello::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap(); // if binding fails then program just stops

    let thread_pool = ThreadPool::new(4); // create a thread pool with 4 spawned threads 
    for stream in listener.incoming() { // iterate through a sequence of TcpStream objects
        let stream = stream.unwrap(); // program stops if stream has any errors

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    } // at the end of the loop, stream goes out of scope and connection gets closed
}

// reads from the TcpStream and sends back a success response
fn handle_connection(mut stream: TcpStream) { // param must be mutable bc stream is modified when read
    let mut buffer = [0; 512]; // buffer stores the data from the stream

    // print out the request
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    let get_root = b"GET / HTTP/1.1\r\n"; // get_root is a byte string
    // send back a response based on what page was being requested
    let (header_line, filename) = if buffer.starts_with(get_root) {
        ("HTTP/1.1 200 OK \r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND \r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", header_line, contents); 
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap(); // wait until all the bytes are written to the connection
}
